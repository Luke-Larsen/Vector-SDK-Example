use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32
};

// Vector SDK
use vector_sdk::{VectorBot};
use std::error::Error;


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

                                // TODO: Set up checkout_group so that a bot can validate things before joining
                                

                                // First we get our group
                                let group = bot_clone.join_group(rumor).await;

                                let _ = tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;

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
                    let group_message = bot_clone.process_group_message(&event).await;

                    println!("{:#?}", group_message);
                    // Check what "Kind" of message we got
                   
                    // if group_message.kind == Kind::ApplicationSpecificData {
                    //     println!("ApplicationSpecificData");
                    //     if group_message.content == "typing" {
                    //         println!("typing data");
                    //     }
                    // }else{
                    //     println!("Not filter for Kind");
                    // }
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
                                    let group = bot_clone.get_group(group_message.mls_group_id).await;
                                    // Lets send a message to our newly joined group
                                    let message_result = group.send_group_message("I will not help you").await;

                                    println!("Our message result: {:#?}", message_result);
                                },
                                _ =>{
                                    let group = bot_clone.get_group(group_message.mls_group_id).await;
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