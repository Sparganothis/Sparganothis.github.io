use matrix_sdk::{ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent}, Room};
pub async fn on_public_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    // if room.state() != RoomState::Joined {
    //     return;
    // }
    let MessageType::Text(text_content) = event.content.msgtype else {
        return;
    };
    if text_content.body.contains("!party") {
        let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");
        room.send(content).await.unwrap();
    }
}