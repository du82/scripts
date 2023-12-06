use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id, api::client::uiaa::Password},
};
use matrix_sdk::room::Room;
use matrix_sdk::ruma::_macros::room_id;
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};
use deepl::{DeepLApi, Lang};


#[tokio::main]
async fn main() {
    println!(">> The bot has started <<");
    authentication().await;
}

async fn authentication() {
    // Set up your own matrix translation bot! Make a new account. Add the login credentials here. Add your DeepL key below.
    let username = user_id!("YOUR_USERNAME");
    let password = "YOUR_PASSWORD";
    let client_builder = Client::builder().homeserver_url("https://YOUR_SERVER");
    let client = client_builder.build().await.unwrap();
    // First we need to log in.
    client.login_username(username, password).send().await.unwrap();
    println!(">> Logged in to Matrix Account <<");
    // Call on_room_message to handle the messages.
    client.add_event_handler(on_room_message);
    println!(">> Ready <<\n");
    // Syncing is important to synchronize the client state with the server. This method will never return.
    client.sync(SyncSettings::default()).await.expect("TODO: panic message");
}

async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if let Room::Joined(room) = room {
        let MessageType::Text(text_content) = event.content.msgtype else {
            return;
        };
            let deepl_api_key = "YOUR_DEEPL_API_KEY";
            
            // MANDARIN (SIMPLIFIED) => ENGLISH
            if text_content.body.starts_with("!T EN ") {
            let api = DeepLApi::new(deepl_api_key, false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!T EN ", "");
            let translated = api.translate(&pre_translated, Some(Lang::ZH), Lang::EN).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        }   
            // ENGLISH => MANDARIN (SIMPLIFIED)
            else if text_content.body.starts_with("!T ZH ") {
            let api = DeepLApi::new(deepl_api_key, false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!T ZH ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::ZH).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        }   
            // ENGLISH => SPANISH
            else if text_content.body.starts_with("!T ES ") {
            let api = DeepLApi::new(deepl_api_key, false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!T ES ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::ES).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        }   
            // ENGLIH => FRENCH
            else if text_content.body.starts_with("!T FR ") {
            let api = DeepLApi::new(deepl_api_key, false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!T FR ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::FR).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } 
            // ENGLISH => RUSSIAN 
            else if text_content.body.starts_with("!T RU ") {
            let api = DeepLApi::new(deepl_api_key, false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!T RU ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::RU).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } 
            // ENGLISH => JAPANESE
            else if text_content.body.starts_with("!T JA ") {
            let api = DeepLApi::new(deepl_api_key, false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!T JA ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::JA).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } 
            // Run !help for Bot Help
            else if text_content.body.starts_with("!help") {
            let content = RoomMessageEventContent::text_plain("type: !T [Language Code] [message to translate] \n\nSupported Language Codes: EN, ZH, ES, FR, RU, JA"); 
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } {
    
}
    }
}
