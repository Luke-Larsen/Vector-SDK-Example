use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32, PublicKey
};
use vector_sdk::VectorBot;
use std::error::Error;
use rusqlite::{Connection, params, OptionalExtension};
use chrono::{DateTime, Utc, Local};
use anyhow::{Context, Result};
use std::path::Path;

/// Database schema version
const SCHEMA_VERSION: i32 = 1;

/// Database connection wrapper
struct Database {
    conn: Connection,
}

impl Database {
    /// Initialize or open the database
    fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Initialize database schema
        Self::initialize_schema(&conn)?;

        Ok(Database { conn })
    }

    /// Initialize database schema
    fn initialize_schema(conn: &Connection) -> Result<()> {
        // Create schema_version table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            )",
            [],
        )?;

        // Check if we need to initialize
        let current_version: Option<i32> = conn.query_row(
            "SELECT version FROM schema_version LIMIT 1",
            [],
            |row| row.get(0),
        ).optional()?;

        if current_version.is_none() {
            // Create conversations table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS conversations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_pubkey TEXT NOT NULL,
                    group_pubkey TEXT,
                    created_at TEXT NOT NULL,
                    UNIQUE(user_pubkey, group_pubkey)
                )",
                [],
            )?;

            // Create messages table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS messages (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    conversation_id INTEGER NOT NULL,
                    sender_pubkey TEXT NOT NULL,
                    content TEXT NOT NULL,
                    message_time TEXT NOT NULL,
                    is_from_bot BOOLEAN NOT NULL DEFAULT 0,
                    FOREIGN KEY (conversation_id) REFERENCES conversations (id)
                )",
                [],
            )?;

            // Create user_preferences table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS user_preferences (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_pubkey TEXT NOT NULL,
                    key TEXT NOT NULL,
                    value TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    UNIQUE(user_pubkey, key)
                )",
                [],
            )?;

            // Insert current schema version
            conn.execute(
                "INSERT INTO schema_version (version) VALUES (?)",
                [SCHEMA_VERSION],
            )?;
        }

        Ok(())
    }

    /// Get or create a conversation
    fn get_or_create_conversation(&self, user_pubkey: &str, group_pubkey: Option<&str>) -> Result<i32> {
        let conversation_id: Option<i32> = if let Some(group_pubkey) = group_pubkey {
            self.conn.query_row(
                "SELECT id FROM conversations WHERE user_pubkey = ? AND group_pubkey = ?",
                [user_pubkey, group_pubkey],
                |row| row.get(0),
            ).optional()?
        } else {
            self.conn.query_row(
                "SELECT id FROM conversations WHERE user_pubkey = ? AND group_pubkey IS NULL",
                [user_pubkey],
                |row| row.get(0),
            ).optional()?
        };

        if let Some(id) = conversation_id {
            Ok(id)
        } else {
            let created_at = Utc::now().to_rfc3339();
            let group_pubkey_val = group_pubkey.unwrap_or("");

            self.conn.execute(
                "INSERT INTO conversations (user_pubkey, group_pubkey, created_at) VALUES (?, ?, ?)",
                params![user_pubkey, group_pubkey_val, created_at],
            )?;

            self.conn.query_row(
                "SELECT last_insert_rowid()",
                [],
                |row| row.get(0),
            )
        }
    }

    /// Save a message to the database
    fn save_message(&self, conversation_id: i32, sender_pubkey: &str, content: &str, is_from_bot: bool) -> Result<()> {
        let message_time = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO messages (conversation_id, sender_pubkey, content, message_time, is_from_bot) VALUES (?, ?, ?, ?, ?)",
            params![conversation_id, sender_pubkey, content, message_time, is_from_bot],
        )?;

        Ok(())
    }

    /// Get conversation history
    fn get_history(&self, conversation_id: i32, limit: i32) -> Result<Vec<(String, String, String, bool)>> {
        let mut stmt = self.conn.prepare(
            "SELECT sender_pubkey, content, message_time, is_from_bot
             FROM messages
             WHERE conversation_id = ?
             ORDER BY message_time DESC
             LIMIT ?"
        )?;

        let rows = stmt.query_map(params![conversation_id, limit], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut history = Vec::new();
        for row in rows {
            history.push(row?);
        }

        // Reverse to get chronological order
        history.reverse();
        Ok(history)
    }

    /// Save user preference
    fn save_preference(&self, user_pubkey: &str, key: &str, value: &str) -> Result<()> {
        let updated_at = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR REPLACE INTO user_preferences (user_pubkey, key, value, updated_at) VALUES (?, ?, ?, ?)",
            params![user_pubkey, key, value, updated_at],
        )?;

        Ok(())
    }

    /// Load user preference
    fn load_preference(&self, user_pubkey: &str, key: &str) -> Result<Option<String>> {
        let value: Option<String> = self.conn.query_row(
            "SELECT value FROM user_preferences WHERE user_pubkey = ? AND key = ?",
            params![user_pubkey, key],
            |row| row.get(0),
        ).optional()?;

        Ok(value)
    }

    /// Get bot statistics
    fn get_stats(&self) -> Result<(i32, i32, i32)> {
        let total_conversations: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM conversations",
            [],
            |row| row.get(0),
        )?;

        let total_messages: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM messages",
            [],
            |row| row.get(0),
        )?;

        let total_users: i32 = self.conn.query_row(
            "SELECT COUNT(DISTINCT user_pubkey) FROM conversations",
            [],
            |row| row.get(0),
        )?;

        Ok((total_conversations, total_messages, total_users))
    }
}

/// Main function to demonstrate persistence using the Vector SDK.
///
/// This bot demonstrates how to persist conversation history and user preferences
/// using SQLite database. The bot maintains state between restarts.
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

    // Initialize database
    let db_path = "persistence_bot.db";
    let db = Database::new(db_path)
        .with_context(|| format!("Failed to initialize database at {}", db_path))?;

    println!("Database initialized at {}", db_path);

    // Create a new VectorBot with default metadata
    let bot = VectorBot::quick(keys).await;

    // Set up notification handler for gift wrap events
    bot.client.handle_notifications(|notification| {
        let bot_clone = bot.clone();
        let db_clone = db.clone();

        async move {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if event.kind == Kind::GiftWrap {
                    match bot_clone.client.unwrap_gift_wrap(&event).await {
                        Ok(UnwrappedGift { rumor, sender }) => {
                            if rumor.kind == Kind::PrivateDirectMessage {
                                let content = rumor.content.trim().to_lowercase();
                                let sender_pubkey = sender.to_string();

                                // Parse and handle commands
                                let response = match content {
                                    cmd if cmd.starts_with("/save") => {
                                        handle_save_command(&content, &sender_pubkey, &db_clone).await
                                    }
                                    cmd if cmd.starts_with("/load") => {
                                        handle_load_command(&content, &sender_pubkey, &db_clone).await
                                    }
                                    cmd if cmd.starts_with("/history") => {
                                        handle_history_command(&content, &sender_pubkey, &db_clone).await
                                    }
                                    cmd if cmd.starts_with("/stats") => {
                                        handle_stats_command(&db_clone).await
                                    }
                                    cmd if cmd.starts_with("/help") => {
                                        help_text()
                                    }
                                    _ => {
                                        // Save the message to history
                                        let conversation_id = db_clone.get_or_create_conversation(&sender_pubkey, None)
                                            .map_err(|e| println!("Error saving message: {}", e))
                                            .ok();

                                        if let Some(conv_id) = conversation_id {
                                            let _ = db_clone.save_message(conv_id, &sender_pubkey, rumor.content.trim(), false);
                                        }

                                        "Invalid command. Send /help to see available commands.".to_string()
                                    }
                                };

                                // Save bot's response to history
                                let conversation_id = db_clone.get_or_create_conversation(&sender_pubkey, None)
                                    .map_err(|e| println!("Error saving message: {}", e))
                                    .ok();

                                if let Some(conv_id) = conversation_id {
                                    let _ = db_clone.save_message(conv_id, "bot", &response, true);
                                }

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

/// Handle the /save command
async fn handle_save_command(content: &str, sender_pubkey: &str, db: &Database) -> String {
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.len() < 3 {
        return "Usage: /save [key] [value]".to_string();
    }

    let key = parts[1];
    let value = parts[2..].join(" ");

    match db.save_preference(sender_pubkey, key, &value) {
        Ok(_) => format!("Saved preference '{}' with value '{}'", key, value),
        Err(e) => format!("Error saving preference: {}", e),
    }
}

/// Handle the /load command
async fn handle_load_command(content: &str, sender_pubkey: &str, db: &Database) -> String {
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.len() != 2 {
        return "Usage: /load [key]".to_string();
    }

    let key = parts[1];

    match db.load_preference(sender_pubkey, key) {
        Ok(Some(value)) => format!("Preference '{}': {}", key, value),
        Ok(None) => format!("No preference found with key '{}'", key),
        Err(e) => format!("Error loading preference: {}", e),
    }
}

/// Handle the /history command
async fn handle_history_command(content: &str, sender_pubkey: &str, db: &Database) -> String {
    let parts: Vec<&str> = content.split_whitespace().collect();

    let limit = if parts.len() == 2 {
        parts[1].parse().unwrap_or(10)
    } else {
        10
    };

    let conversation_id = match db.get_or_create_conversation(sender_pubkey, None) {
        Ok(id) => id,
        Err(e) => {
            return format!("Error getting conversation: {}", e);
        }
    };

    match db.get_history(conversation_id, limit) {
        Ok(history) => {
            if history.is_empty() {
                return "No messages in history.".to_string();
            }

            let mut output = format!("Last {} messages:\n\n", history.len());
            for (sender, content, time, is_from_bot) in history {
                let sender_label = if is_from_bot { "Bot" } else { "You" };
                let time_local = DateTime::parse_from_rfc3339(&time)
                    .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|_| time);

                output.push_str(&format!("[{}] {}: {}\n\n", time_local, sender_label, content));
            }
            output
        }
        Err(e) => format!("Error getting history: {}", e),
    }
}

/// Handle the /stats command
async fn handle_stats_command(db: &Database) -> String {
    match db.get_stats() {
        Ok((conversations, messages, users)) => {
            format!(
                "Bot Statistics:\n\nTotal Conversations: {}\nTotal Messages: {}\nTotal Users: {}",
                conversations, messages, users
            )
        }
        Err(e) => format!("Error getting statistics: {}", e),
    }
}

/// Provides help information with available commands.
fn help_text() -> String {
    let mut output = String::new();
    output.push_str("Persistence Bot - Commands:\n\n");
    output.push_str("/save [key] [value] - Save a user preference\n");
    output.push_str("  Example: /save theme dark\n\n");
    output.push_str("/load [key] - Load a user preference\n");
    output.push_str("  Example: /load theme\n\n");
    output.push_str("/history [n] - Show last n messages in conversation (default: 10)\n");
    output.push_str("  Example: /history 5\n\n");
    output.push_str("/stats - Show bot usage statistics\n");
    output.push_str("/help - Show this help message\n\n");
    output.push_str("All messages are automatically saved to history and persist between bot restarts.\n");
    output
}
