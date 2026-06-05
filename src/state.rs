use teloxide::Bot;

#[derive(Clone)]
pub struct AppState {
    pub bot: Bot, // teloxide bot has been a singleton
    pub owner_chat_id: i64,
}

impl AppState {
    pub fn new(bot: Bot, owner_chat_id: i64) -> Self {
        Self { bot, owner_chat_id }
    }
}
