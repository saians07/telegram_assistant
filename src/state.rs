use std::{ops::Deref, sync::Arc};

use secrecy::SecretString;
use teloxide::Bot;
use tokio::sync::RwLock;

use crate::core::ais::base::BotAgent;

#[derive(Clone)]
pub struct AppState(Arc<AppStateInner>);

pub struct AppStateInner {
    pub bot: Bot, // teloxide bot has been a singleton
    pub owner_chat_id: i64,
    pub secret_token: SecretString,
    pub gemini_3_1_flash_lite: RwLock<Option<BotAgent>>,
}

impl AppState {
    pub fn new(bot: Bot, owner_chat_id: i64, secret_token: SecretString) -> Self {
        Self(Arc::new(AppStateInner {
            bot,
            owner_chat_id,
            secret_token,
            gemini_3_1_flash_lite: RwLock::new(None),
        }))
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
