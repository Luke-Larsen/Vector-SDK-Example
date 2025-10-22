use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32
};

use vector_sdk::{VectorBot};
use std::error::Error;

// reqwest for connecting to catass
use reqwest::Client;

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

    // Handle notifications and properly handle the Result
    let notifications_result = bot.client.handle_notifications(|notification| {
        let bot_clone = bot.clone();
        async move {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if event.kind == Kind::GiftWrap {
                    match bot_clone.client.unwrap_gift_wrap(&event).await {
                        Ok(UnwrappedGift { rumor, sender }) => {
                            if rumor.kind == Kind::PrivateDirectMessage {
                                println!("Received message: {:?}", rumor.content.trim());

                                let cat_url = "https://cataas.com/cat?json=true";
                                let r_client = Client::new();

                                match r_client.get(cat_url).send().await {
                                    Ok(response) => {
                                        if response.status().is_success() {
                                            // Parse the JSON response to get the image URL
                                            if let Ok(json) = response.json::<serde_json::Value>().await {
                                                if let Some(image_url) = json.get("url").and_then(|url| url.as_str()) {
                                                    // Fetch the actual image
                                                    let image_response = r_client.get(image_url).send().await;

                                                    match image_response {
                                                        Ok(img_response) => {
                                                            if img_response.status().is_success() {
                                                                // Create an AttachmentFile with the image data
                                                                let bytes = img_response.bytes().await.unwrap().to_vec();
                                                                let extension = match json.get("mimetype").and_then(|mimetype| mimetype.as_str()) {
                                                                    Some("image/png") => "png",
                                                                    Some("image/jpeg") => "jpg",
                                                                    Some("image/gif") => "gif",
                                                                    Some("image/webp") => "webp",
                                                                    Some(_) => "png",
                                                                    None => "png",
                                                                };

                                                                let attached_file = vector_sdk::AttachmentFile {
                                                                    bytes,
                                                                    img_meta: None,
                                                                    extension: extension.to_string(),
                                                                };

                                                                // Send the image file
                                                                let normal_chat = bot_clone.get_chat(sender).await;
                                                                println!("chat channel created");

                                                                // Send the image
                                                                let send_attached = normal_chat.send_private_file(Some(attached_file)).await;
                                                                println!("AttachedMessageSend: {:#?}", send_attached);

                                                                if !send_attached{
                                                                    println!("Error sending image {}", send_attached)
                                                                }

                                                                // Return the string that will be sent to the user as normal private message
                                                                println!("Here is your cat image!");
                                                            } else {
                                                                println!("Failed to fetch cat image");
                                                            }
                                                        }
                                                        Err(_) => println!("Error fetching cat image"),
                                                    }
                                                } else {
                                                    println!("Invalid cat image response")
                                                }
                                            } else {
                                                println!("Failed to parse cat image response")
                                            }
                                        } else {
                                            println!("Failed to fetch cat image metadata")
                                        }
                                    }
                                    Err(_) => println!("Error fetching cat image"),
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

    // Handle the Result from handle_notifications
    notifications_result?;

    Ok(())
}