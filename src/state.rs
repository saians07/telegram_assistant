use secrecy::SecretString;
use teloxide::Bot;

#[derive(Clone)]
pub struct AppState {
    pub bot: Bot, // teloxide bot has been a singleton
    pub owner_chat_id: i64,
    pub secret_token: SecretString,
}

impl AppState {
    pub fn new(bot: Bot, owner_chat_id: i64, secret_token: SecretString) -> Self {
        Self {
            bot,
            owner_chat_id,
            secret_token,
        }
    }
}
