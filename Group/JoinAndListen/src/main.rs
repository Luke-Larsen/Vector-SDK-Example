use vector_sdk::nostr::{
    Keys, Kind, UnwrappedGift, RelayPoolNotification, ToBech32
};

// Vector SDK
use vector_sdk::{VectorBot};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    // Generate new random keys
     let keys = Keys::generate();
    //let keys = Keys::parse("nsec12kcgs78l06p30jz7z7h3n2x2cy99nw2z6zspjdp7qc206887mwvs95lnkx")?;

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
                println!("Incoming event: {:#?} ", event);
                if event.kind == Kind::GiftWrap {
                    match bot_clone.client.unwrap_gift_wrap(&event).await {
                        Ok(UnwrappedGift { rumor, sender }) => {
                            println!("Incoming rumor: {:#?} ", rumor);

                            if rumor.kind == Kind::PrivateDirectMessage {
                                println!("Received message: {:?}", rumor.content.trim());

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
                                // TODO: add in a passthrough for the group id
                                let group = bot_clone.join_group(rumor).await;

                                // Now we accept the group welcome if we like it
                                // let _ = group.accept_group_invite(rumor.id).await;

                                let _ = tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;

                                // Lets send a message to our newly joined group
                                let message_result = group.send_group_message("Hello World").await;

                                println!("Our message result: {:#?}", message_result);

                                let _ = group.check_group_messages();

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