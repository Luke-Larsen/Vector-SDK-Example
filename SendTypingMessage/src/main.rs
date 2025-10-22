use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32
};

// Vector SDK
use vector_sdk::{VectorBot};
use std::error::Error;

// Thread, time for pausing response
use std::{thread, time};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    // Generate new random keys
    let keys = Keys::generate();

    println!("Vector bot initialized with public key: {:?}", keys.public_key());
    let bech32_pubkey: String = keys.public_key().to_bech32()?;
    let bech32_private_key: String = keys.secret_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);
    println!("Bech32 PrivateKey: {}", bech32_private_key);

    // Create a new VectorBot with default metadata
    let bot = VectorBot::quick(keys).await;

    let _ = bot.client.handle_notifications(|notification| {
        let bot_clone = bot.clone();
        async move {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if event.kind == Kind::GiftWrap {
                    match bot_clone.client.unwrap_gift_wrap(&event).await {
                        Ok(UnwrappedGift { rumor, sender }) => {
                            if rumor.kind == Kind::PrivateDirectMessage {
                                println!("Received message: {:?}", rumor.content.trim());

                                // Get the chat channel for the sender
                                let chat = bot_clone.get_chat(sender).await;

                                // Send a typing indicator because it might take a minute
                                let send_typing_indicator = chat.send_typing_indicator().await;
                                println!("Sending Typing indicator: {:#?}", send_typing_indicator);

                                let ten_millis = time::Duration::from_millis(10000);

                                thread::sleep(ten_millis);

                                // Respond with "Hello World"
                                let success = chat.send_private_message("Hello World").await;
                                if success {
                                    println!("Sent response to {:?}", sender);
                                } else {
                                    println!("Failed to send response to {:?}", sender);
                                }
                            }
                        }
                        Err(e) => println!("Impossible to decrypt direct message: {e}"),
                    }
                }
            }
        Ok(false) // Set to true to exit from the loop
        }
    }).await;

     Ok(())
}