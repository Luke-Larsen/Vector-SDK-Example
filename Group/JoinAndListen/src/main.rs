use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32
};

// Vector SDK
use vector_sdk::{VectorBot};
use std::error::Error;

use reqwest::Client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    // Generate new random keys
    //let keys = Keys::generate();
    let keys = Keys::parse("nsec1n5vmml3eh4rqz9z6qzyekm3eml5w63qn0kerdfs0hq6tfj6tueysp96vrm")?;

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
                // println!("Incoming event: {:#?} ", event);
                if event.kind == Kind::GiftWrap {
                    match bot_clone.client.unwrap_gift_wrap(&event).await {
                        Ok(UnwrappedGift { rumor, sender }) => {
                            // println!("Incoming rumor: {:#?} ", rumor);

                            if rumor.kind == Kind::PrivateDirectMessage {
                                // println!("Received message: {:?}", rumor.content.trim());

                                // Get the chat channel for the sender
                                let chat = bot_clone.get_chat(sender).await;

                                // Respond with "Hello World"
                                let success = chat.send_private_message("Hello World").await;
                                if success {
                                    println!("Sent response to {:?}", sender);
                                } else {
                                    println!("Failed to send response to {:?}", sender);
                                }
                            } else if rumor.kind == Kind::MlsWelcome {

                                println!("Welcome Event: {:#?}", rumor);
                                println!("rumor_event: {:#?}", &rumor);

                                println!("Welcome Event Nostr Id: {:#?}", rumor.id);

                                // First we get our group
                                let group = match bot_clone.quick_join_group(rumor).await{
                                    Ok(g) => g,
                                    Err(_) => panic!("Could not join group")
                                };


                                // TODO: Set up checkout_group so that a bot can validate things before joining
                                // let group = bot_clone.checkout_group(rumor);
                                // println!("Group data: {:#?}", group);

                                // let join = bot_clone.join_group(group.mls_group_id);

                                let _ = tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
                                
                                // Send typing reaction
                                let _ = group.send_group_typing_indication().await;

                                // Lets send a message to our newly joined group
                                let message_result = group.send_group_message("Hello World").await;

                                println!("Our message result: {:#?}", message_result);

                                let _ = group.check_group_messages();

                            }

                        }
                        Err(e) => println!("Impossible to decrypt direct message: {e}"),
                    }
                } else if event.kind == Kind::MlsGroupMessage {
                    println!("We got a group message! {:#?}",event);
                    // Process group message here
                    let group_message = match bot_clone.process_group_message(&event).await{
                        Ok(g) => g,
                        Err(_) => panic!("Could not join group")
                    };

                    println!("{:#?}", group_message);

                    match group_message.kind{
                        Kind::ApplicationSpecificData => {
                            if group_message.content == "typing" {
                                println!("typing data");
                            }
                        }
                        Kind::PrivateDirectMessage => {
                            // Handle your command system
                            match group_message.content.trim().to_lowercase().as_str() {

                                "/help" =>{
                                    let group = match bot_clone.get_group(group_message.mls_group_id).await{
                                        Ok(g) => g,
                                        Err(_) => panic!("Could not join group")
                                    };
                                    // Lets send a message to our newly joined group
                                    let message_result = group.send_group_message("I will not help you").await;

                                    println!("Our message result: {:#?}", message_result);
                                },
                                "/cat" => {
                                    // Fetch the cat image from the URL
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
                                                                    let group = match bot_clone.get_group(group_message.mls_group_id).await{
                                                                        Ok(g) => g,
                                                                        Err(_) => panic!("Could not join group")
                                                                    };
                                                                    println!("group channel grabbed");

                                                                    // // Send a reaction to validate we got the command
                                                                    // let send_checkmark = normal_chat.send_reaction(rumor.id.unwrap().to_string(), "🆗".to_string()).await;
                                                                    // println!("Sending reaction: {:#?}", send_checkmark);

                                                                    // // Send a typing indicator because it might take a minute
                                                                    // let send_typing_indicator = normal_chat.send_typing_indicator().await;
                                                                    // println!("Sending Typing indicator: {:#?}", send_typing_indicator);

                                                                    // Send the image
                                                                    let send_attatched = group.send_group_attachment(Some(attached_file)).await;
                                                                    println!("AttatchedMessageSend: {:#?}", send_attatched);
                                                                } else {
                                                                    panic!("Failed to fetch cat image")
                                                                }
                                                            }
                                                            Err(_) => panic!("Error fetching cat image"),
                                                        }
                                                    } else {
                                                        panic!("Invalid cat image response")
                                                    }
                                                } else {
                                                    panic!("Failed to parse cat image response")
                                                }
                                            } else {
                                                panic!("Failed to fetch cat image metadata")
                                            }
                                        }
                                        Err(_) => panic!("Error fetching cat image"),
                                    }
                                },
                                _ =>{
                                    let group = match bot_clone.get_group(group_message.mls_group_id).await{
                                        Ok(g) => g,
                                        Err(_) => panic!("Could not join group")
                                    };
                                    // Lets send a message to our newly joined group
                                    let message_result = group.send_group_message("Not a command").await;

                                    println!("Our message result: {:#?}", message_result);
                                }
                            }
                        }
                        _ =>{
                            println!("Not filter for Kind");
                        }
                    }
                        
                }
            }
            
        Ok(false) // Set to true to exit from the loop
        }
    }).await;

    Ok(())
}