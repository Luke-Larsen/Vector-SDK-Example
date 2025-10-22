use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32
};

use vector_sdk::{VectorBot};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

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
                match bot_clone.client.unwrap_gift_wrap(&event).await {
                    Ok(UnwrappedGift { rumor, sender }) => {
                        if rumor.kind == Kind::PrivateDirectMessage {
                            println!("Received message: {:?}", rumor.content.trim());

                            // Get the chat channel for the sender
                            let chat = bot_clone.get_chat(sender).await;

                            // Send a reaction to validate we got the command
                            let send_checkmark = chat.send_reaction(rumor.id.unwrap().to_string(), "🆗".to_string()).await;
                            println!("Sending reaction: {:#?}", send_checkmark);
                            Ok(false) // Return result here to handle it properly
                        } else {
                            Ok(false) // Default to false if not a direct message
                        }
                    }
                    Err(e) => {
                        println!("Impossible to decrypt direct message: {e}");
                        Ok(false)
                    }
                }
            } else {
                Ok(false)
            }
        }
    }).await;

    Ok(())
}