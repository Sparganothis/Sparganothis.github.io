use game::{api::{game_replay::GameId, user::UserProfile}, tet::GameOverReason};


#[derive(Debug,Clone)]
pub enum ChatbotMessage {
    UserActivity(UserActivityMessage),
    GameUpdate(GameUpdateMessage),
    FeedbackForm(FeedbackFormContent)
}

#[derive(Debug,Clone)]
pub struct UserActivityMessage {
    pub user_uuid: uuid::Uuid,
    pub user_activity_type: UserActivityEventType,
}

#[derive(Debug,Clone)]
pub enum UserActivityEventType {
    UserAccountCreated(UserProfile),
    UserBeatPersonalBest(uuid::Uuid),  // game id
}

#[derive(Debug,Clone,Copy)]
pub struct GameUpdateMessage {
    pub game_id: GameId,
    pub event_type: GameUpdateEventType,
}

#[derive(Debug,Clone,Copy)]
pub enum GameUpdateEventType {
    GameStarted,
    GameFinished(GameOverReason),
}

#[derive(Debug,Clone)]
pub struct FeedbackFormContent {
    pub user_uuid: uuid::Uuid,
    pub user_display_name: String,
    pub feedback_type: String,
    pub feedback_text: String,
    pub feedback_image: Vec<u8>,
}