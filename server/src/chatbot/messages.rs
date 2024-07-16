use game::api::game_match::GameMatchType;



#[derive(Debug,Clone)]
pub enum ChatbotMessage {
    UserProfileCreated(UserActivityMessage),
    GameUpdate(GameUpdateMessage),
    MatchUpdate(UserActivityMessage),
    FeedbackForm(FeedbackFormContent)
}

#[derive(Debug,Clone,Copy)]
pub struct UserActivityMessage {
    pub user_uuid: uuid::Uuid,
    pub user_activity_type: UserActivityEventType,
}


#[derive(Debug,Clone,Copy)]
pub enum UserActivityEventType {
    UserAccountCreated,
    UserLogin,
}


#[derive(Debug,Clone,Copy)]
pub struct GameUpdateMessage {
    pub game_uuid: uuid::Uuid,
    pub event_type: GameUpdateEventType,
}


#[derive(Debug,Clone,Copy)]
pub enum GameUpdateEventType {
    GameStarted,
    GameWon,
    GameLost,
}

#[derive(Debug,Clone)]
pub struct FeedbackFormContent {
    pub user_uuid: uuid::Uuid,
    pub user_display_name: String,
    pub feedback_type: String,
    pub feedback_text: String,
    pub feedback_image: Vec<u8>,
}


