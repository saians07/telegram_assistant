use std::{collections::HashMap, ops::Deref, sync::Arc};

use secrecy::SecretString;

use crate::{core::ais::base::BotAgent, service::telegram::TelegramService};

#[derive(Clone)]
pub struct AppState(Arc<AppStateInner>);

pub struct AppStateInner {
    pub telegram: TelegramService, // teloxide bot has been a singleton
    pub owner_chat_id: i64,
    pub secret_token: SecretString,
    pub bot_agents: HashMap<String, BotAgent>,
}

impl AppState {
    pub fn new(
        telegram: TelegramService,
        owner_chat_id: i64,
        secret_token: SecretString,
        bot_agents: HashMap<String, BotAgent>,
    ) -> Self {
        Self(Arc::new(AppStateInner {
            telegram,
            owner_chat_id,
            secret_token,
            bot_agents,
        }))
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
