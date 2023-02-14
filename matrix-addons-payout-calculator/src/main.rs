use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::message::{
    MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
};
use matrix_sdk::{config::SyncSettings, ruma::user_id, Client};
use std::sync::{Arc, Mutex};

struct State {
    dl: f32,
    cpm: f32,
}

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
    client
        .login_username(username, "TYPE-YOUR-PASSWORD-HERE")
        .send()
        .await
        .unwrap();
    println!(">> Logged in <<");
    let state = Arc::new(Mutex::new(State { dl: 0.0, cpm: 0.0 }));
    client.add_event_handler(move |event: OriginalSyncRoomMessageEvent, room: Room| {
        on_room_message(event, room, state.clone())
    });
    println!(">> Ready <<\n");
    // Syncing is important to synchronize the client state with the server. This method will never return.
    client
        .sync(SyncSettings::default())
        .await
        .expect("TODO: panic message");
}

async fn on_room_message(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    state: Arc<Mutex<State>>,
) {
    if let Room::Joined(room) = room {
        let MessageType::Text(text_content) = event.content.msgtype else {
            return;
        };
        if text_content.body.starts_with("!dl") {
            let content = RoomMessageEventContent::text_plain(
                "You've entered a download count of ".to_owned()
                    + &text_content.body.replace("!dl ", ""),
            );
            room.send(content, None).await.expect("TODO: panic message");

            let dl: f32 = text_content
                .body
                .replace("!dl ", "")
                .trim()
                .parse()
                .ok()
                .expect("Numbers only");
            println!("{}", dl);
            state.lock().unwrap().dl = dl;
        } else if text_content.body.starts_with("!cpm") {
            let content = RoomMessageEventContent::text_plain(
                "You've entered an eCPM of ".to_owned() + &text_content.body.replace("!cpm ", ""),
            );
            room.send(content, None).await.expect("TODO: panic message");

            let cpm: f32 = text_content
                .body
                .replace("!cpm ", "")
                .trim()
                .parse()
                .ok()
                .expect("Numbers only");
            println!("{}", cpm);
            state.lock().unwrap().cpm = cpm;
        } else if text_content.body.starts_with("!calculate") {
            let dl = state.lock().unwrap().dl;
            let cpm = state.lock().unwrap().cpm;
            let count: f32 = 1000.0; // Divide the download count by this, 1K default
            let split: f32 = 0.7; // Payments split, 70/30 default
            let answer: f32 = dl / count * cpm * split;
            let output = (answer as i32).to_string();

            let content = RoomMessageEventContent::text_plain("You'll earn $".to_owned() + &output);
            room.send(content, None).await.expect("TODO: panic message");
        }
    }
}
