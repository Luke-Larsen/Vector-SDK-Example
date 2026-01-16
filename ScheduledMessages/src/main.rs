use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32, PublicKey
};
use vector_sdk::VectorBot;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc, Local, TimeZone};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a scheduled message with metadata
#[derive(Debug, Clone)]
struct ScheduledMessage {
    id: String,
    sender: PublicKey,
    message: String,
    scheduled_time: DateTime<Utc>,
    is_recurring: bool,
    recurrence_interval: Option<String>, // "daily" or "weekly"
}

/// Main function to demonstrate scheduled messages using the Vector SDK.
///
/// This bot allows users to schedule messages to be sent at specific times or on a recurring basis.
/// It demonstrates background task management and time-based operations.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();

    // Generate new random keys
    let keys = Keys::generate();

    println!("Vector bot initialized with public key: {:?}", keys.public_key());
    let bech32_pubkey: String = keys.public_key().to_bech32()?;
    let bech32_private_key: String = keys.secret_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);
    println!("Bech32 PrivateKey: {}", bech32_private_key);

    // Create shared state for scheduled messages
    let scheduled_messages: Arc<Mutex<HashMap<String, ScheduledMessage>>> = Arc::new(Mutex::new(HashMap::new()));

    // Create a new VectorBot with default metadata
    let bot = VectorBot::quick(keys).await;

    // Spawn a background task to process scheduled messages
    let scheduled_messages_clone = scheduled_messages.clone();
    let bot_clone = bot.clone();
    tokio::spawn(async move {
        loop {
            // Check for messages to send
            let now = Utc::now();
            let mut messages_to_send = Vec::new();

            {
                let mut messages = scheduled_messages_clone.lock().await;
                messages.retain(|id, msg| {
                    if msg.scheduled_time <= now {
                        messages_to_send.push(msg.clone());
                        false // Remove from map
                    } else {
                        true // Keep in map
                    }
                });
            }

            // Send the messages
            for msg in messages_to_send {
                let chat = bot_clone.get_chat(msg.sender).await;
                let _ = chat.send_private_message(&msg.message).await;
                println!("Sent scheduled message to {:?}: {}", msg.sender, msg.message);

                // Reschedule if recurring
                if msg.is_recurring {
                    if let Some(interval) = msg.recurrence_interval {
                        let new_time = match interval.as_str() {
                            "daily" => (msg.scheduled_time + chrono::Duration::days(1)).with_time(msg.scheduled_time.time()),
                            "weekly" => (msg.scheduled_time + chrono::Duration::weeks(1)).with_time(msg.scheduled_time.time()),
                            _ => None,
                        };

                        if let Some(new_time) = new_time {
                            let mut messages = scheduled_messages_clone.lock().await;
                            messages.insert(msg.id.clone(), ScheduledMessage {
                                id: msg.id,
                                sender: msg.sender,
                                message: msg.message,
                                scheduled_time: new_time,
                                is_recurring: true,
                                recurrence_interval: msg.recurrence_interval,
                            });
                        }
                    }
                }
            }

            // Sleep for a short period before checking again
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    // Set up notification handler for gift wrap events
    bot.client.handle_notifications(|notification| {
        let bot_clone = bot.clone();
        let scheduled_messages_clone = scheduled_messages.clone();

        async move {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if event.kind == Kind::GiftWrap {
                    match bot_clone.client.unwrap_gift_wrap(&event).await {
                        Ok(UnwrappedGift { rumor, sender }) => {
                            if rumor.kind == Kind::PrivateDirectMessage {
                                let content = rumor.content.trim().to_lowercase();

                                // Parse and handle commands
                                let response = match content {
                                    cmd if cmd.starts_with("/schedule") => {
                                        handle_schedule_command(&content, sender, scheduled_messages_clone).await
                                    }
                                    cmd if cmd.starts_with("/list") => {
                                        handle_list_command(sender, scheduled_messages_clone).await
                                    }
                                    cmd if cmd.starts_with("/cancel") => {
                                        handle_cancel_command(&content, sender, scheduled_messages_clone).await
                                    }
                                    cmd if cmd.starts_with("/help") => {
                                        help_text()
                                    }
                                    _ => "Invalid command. Send /help to see available commands.".to_string(),
                                };

                                let chat = bot_clone.get_chat(sender).await;
                                let _ = chat.send_private_message(&response).await;
                            }
                        }
                        Err(e) => println!("Impossible to decrypt direct message: {e}"),
                    }
                }
            }
            Ok(false) // Set to true to exit from the loop
        }
    }).await?;

    Ok(())
}

/// Handle the /schedule command
async fn handle_schedule_command(
    content: &str,
    sender: PublicKey,
    scheduled_messages: Arc<Mutex<HashMap<String, ScheduledMessage>>>,
) -> String {
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.len() < 3 {
        return "Usage: /schedule [time] [message] or /schedule daily [time] [message] or /schedule weekly [time] [message]".to_string();
    }

    // Check if this is a recurring schedule
    let (recurring, time_part) = if parts[1] == "daily" || parts[1] == "weekly" {
        (true, parts[2])
    } else {
        (false, parts[1])
    };

    // Parse the time
    let scheduled_time = match parse_time(time_part) {
        Some(time) => time,
        None => return "Invalid time format. Use HH:MM or HH:MM AM/PM".to_string(),
    };

    // The rest is the message
    let message = if recurring {
        parts[3..].join(" ")
    } else {
        parts[2..].join(" ")
    };

    // Create scheduled message
    let id = Uuid::new_v4().to_string();
    let scheduled_msg = ScheduledMessage {
        id: id.clone(),
        sender,
        message: message.clone(),
        scheduled_time,
        is_recurring: recurring,
        recurrence_interval: if recurring {
            Some(parts[1].to_string())
        } else {
            None
        },
    };

    // Add to scheduled messages
    {
        let mut messages = scheduled_messages.lock().await;
        messages.insert(id, scheduled_msg);
    }

    format!("Message scheduled! ID: {}\nTime: {}\nMessage: {}", id, scheduled_time, message)
}

/// Handle the /list command
async fn handle_list_command(
    sender: PublicKey,
    scheduled_messages: Arc<Mutex<HashMap<String, ScheduledMessage>>>,
) -> String {
    let messages = scheduled_messages.lock().await;
    let user_messages: Vec<_> = messages.values()
        .filter(|msg| msg.sender == sender)
        .collect();

    if user_messages.is_empty() {
        return "You have no scheduled messages.".to_string();
    }

    let mut output = String::from("Your scheduled messages:\n\n");
    for msg in user_messages {
        output.push_str(&format!(
            "ID: {}\nTime: {}\nMessage: {}\n",
            msg.id,
            msg.scheduled_time.with_timezone(&Local),
            msg.message
        ));
        if msg.is_recurring {
            output.push_str(&format!("Recurrence: {} every {}\n", msg.message, msg.recurrence_interval.as_ref().unwrap()));
        }
        output.push_str("\n");
    }

    output
}

/// Handle the /cancel command
async fn handle_cancel_command(
    content: &str,
    sender: PublicKey,
    scheduled_messages: Arc<Mutex<HashMap<String, ScheduledMessage>>>,
) -> String {
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() != 2 {
        return "Usage: /cancel [id]".to_string();
    }

    let id = parts[1];
    let mut messages = scheduled_messages.lock().await;

    if let Some(msg) = messages.remove(id) {
        if msg.sender == sender {
            format!("Cancelled scheduled message ID: {}", id)
        } else {
            "You can only cancel your own scheduled messages.".to_string()
        }
    } else {
        "No scheduled message found with that ID.".to_string()
    }
}

/// Parse time string into DateTime<Utc>
fn parse_time(time_str: &str) -> Option<DateTime<Utc>> {
    // Try parsing as HH:MM
    if let Ok(time) = chrono::NaiveTime::parse_from_str(time_str, "%H:%M") {
        let now = Local::now();
        let today = now.date();
        let datetime = today.and_time(time);
        return Some(datetime.with_timezone(&Utc));
    }

    // Try parsing as HH:MM AM/PM
    if let Ok(time) = chrono::NaiveTime::parse_from_str(time_str, "%I:%M %p") {
        let now = Local::now();
        let today = now.date();
        let datetime = today.and_time(time);
        return Some(datetime.with_timezone(&Utc));
    }

    None
}

/// Provides help information with available commands.
fn help_text() -> String {
    let mut output = String::new();
    output.push_str("Scheduled Messages Bot - Commands:\n\n");
    output.push_str("/schedule [time] [message] - Schedule a one-time message\n");
    output.push_str("  Example: /schedule 14:30 Hello there!\n\n");
    output.push_str("/schedule daily [time] [message] - Schedule a daily recurring message\n");
    output.push_str("  Example: /schedule daily 09:00 Good morning!\n\n");
    output.push_str("/schedule weekly [time] [message] - Schedule a weekly recurring message\n");
    output.push_str("  Example: /schedule weekly 17:00 Have a great weekend!\n\n");
    output.push_str("/list - List all your scheduled messages\n");
    output.push_str("/cancel [id] - Cancel a scheduled message\n");
    output.push_str("/help - Show this help message\n");
    output
}
