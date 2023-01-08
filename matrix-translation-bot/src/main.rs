use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id},
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
    let username = user_id!("@test:uxn.one");
    let client_builder = Client::builder().homeserver_url("https://matrix.uxn.one");
    let client = client_builder.build().await.unwrap();
    // First we need to log in.
    client.login_username(username, "YOUR_BOTS_PASSWORD").send().await.unwrap();
    println!(">> Logged in <<");
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
        if text_content.body.starts_with("!t en ") {

            let api = DeepLApi::new("YOUR_DEEPL_API_KEY", false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!t en ", "");
            let translated = api.translate(&pre_translated, Some(Lang::DE), Lang::EN).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } else if text_content.body.starts_with("!t zh ") {

            let api = DeepLApi::new("YOUR_DEEPL_API_KEY", false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!t zh ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::ZH).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } else if text_content.body.starts_with("!t es ") {
            let api = DeepLApi::new("YOUR_DEEPL_API_KEY", false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!t es ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::ES).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } else if text_content.body.starts_with("!t fr ") {
            let api = DeepLApi::new("YOUR_DEEPL_API_KEY", false);
            let pre_translated = "".to_owned() + &text_content.body.replace("!t fr ", "");
            let translated = api.translate(&pre_translated, Some(Lang::EN), Lang::FR).await.unwrap();
            assert!(!translated.translations.is_empty());
            let sentences = translated.translations;
            println!("{}", sentences[0].text);
            let content = RoomMessageEventContent::text_plain("".to_owned() + &sentences[0].text);
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        } else if text_content.body.starts_with("!bothelp") {
            let content = RoomMessageEventContent::text_plain("Type !t followed by what you want to translate.");
            println!("sending");
            room.send(content, None).await.expect("TODO: panic message");
            println!("message sent");
        }
    }
}
