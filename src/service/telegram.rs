use std::{collections::HashMap, env};

use anyhow::Result;
use reqwest::StatusCode;
use teloxide::{
    Bot,
    payloads::{SendMessageSetters, SendVoiceSetters},
    prelude::Requester,
    types::{ChatId, InputFile, Message},
};

use crate::{
    core::{
        ais::base::BotAgent, error::SwanError, formatter::markdown_to_html,
        traits::ai_agent::AgentTrait,
    },
    dto::response::{BaseResponse, TelegramHistoryList},
    repositories::telegram::TelegramRepo,
};

#[derive(Debug, Clone)]
pub struct TelegramService {
    pub bot: Bot,
    pub repo: TelegramRepo,
}

impl TelegramService {
    pub fn new(bot: Bot, repo: TelegramRepo) -> Self {
        Self { bot, repo }
    }

    /// Receiving message from API handler for the first time.
    pub async fn receive_message(
        &self,
        message: Message,
        agents: &HashMap<String, BotAgent>,
        session_id: String,
    ) -> Result<BaseResponse, SwanError> {
        let chat_id = message.chat.id;

        if !self.is_authorized_user(chat_id.0).await {
            tracing::warn!("User is not authorized.");
            match self.handle_guest(message).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Failed to handle user request: {:#?}", e);
                    return Ok(BaseResponse::reply(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to handle request!",
                    ));
                }
            }
        } else {
            tracing::warn!("User is authorized.");
            // handling the session from the very beginning
            let mut session_id = session_id;
            match self.repo.fetch_last_session_id(chat_id.0).await {
                Ok(session) => session_id = session,
                Err(_) => {} // when error, we do nothing since we have had the session_id
            }
            match self
                .handle_authorized_user_message(message, session_id, agents)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Failed to handle user request: {:#?}", e);
                    return Ok(BaseResponse::reply(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to handle request!",
                    ));
                }
            }
        }
        // no matter what is the result of our processing, we
        Ok(BaseResponse::reply(
            StatusCode::OK,
            "Your request has been accepted!",
        ))
    }

    /// This will handle messages from authorized user
    /// For each message from authorized user we will:
    ///     * check if there is < 30 minutes conversation session.
    async fn handle_authorized_user_message(
        &self,
        message: Message,
        session_id: String,
        agents: &HashMap<String, BotAgent>,
    ) -> Result<(), SwanError> {
        tracing::info!("Start processing user message ...");
        let chat_id = message.chat.id;
        let user_id = self.repo.fetch_user(chat_id.0).await?.id;
        let quota_count = self.repo.fetch_user_quota(chat_id.0).await?;
        let used_quota = self.repo.fetch_user_used_quota(chat_id.0).await?;

        if (quota_count > 0) & (quota_count > used_quota) {
            return Ok(());
        }

        let Some(text) = message.text() else {
            self.bot
                .send_message(chat_id, "Maaf, saat ini pesan selain text belum didukung!")
                .await?;

            return Ok(());
        };

        // let agent = agents.get("gemini_3.1_flash_google").unwrap();
        let agent = agents.get("gemma_4_openrouter").unwrap();
        let last_session = match self.repo.fetch_last_session_id(chat_id.0).await {
            Ok(session) => session,
            Err(_) => session_id.clone(),
        };
        let chat_history = self
            .repo
            .fetch_user_chat_history(chat_id.0, &last_session)
            .await?;
        let mut telegram_history = TelegramHistoryList(chat_history)
            .create_message_vector()
            .await;

        let response = agent.test_func(text, &mut telegram_history).await?;

        self.bot
            .send_message(chat_id, markdown_to_html(&response))
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;

        self.repo
            .insert_user_chat("user", text, user_id, &session_id)
            .await?;
        self.repo
            .insert_user_chat("assistant", &response, user_id, &session_id)
            .await?;

        Ok(())
    }

    /// This will handle the guest (unauthorized users)
    /// For each unauthorized user we have to:
    ///     1. Register user as guest if not yet listed as guest
    ///     2. Check user quota: 1 chat per user per day.
    ///     2. Tell the user to register to use our service.
    ///     3. Update the database
    async fn handle_guest(&self, message: Message) -> Result<(), SwanError> {
        let chat_id = message.chat.id.0;
        if !self.is_registered_guest(chat_id).await {
            let _ = self.repo.register_guest(chat_id as i32).await?;
        };
        let quota_count = self.check_guest_quota(chat_id).await?;
        if quota_count > 0 {
            return Ok(());
        }
        let sent = self.send_unregistered_message(chat_id).await?;

        if sent {
            self.repo
                .insert_guest_chat(chat_id as i32, message.text().unwrap().to_string())
                .await?;
        };

        Ok(())
    }

    async fn send_unregistered_message(&self, chat_id: i64) -> Result<bool, SwanError> {
        let Ok(path) = env::current_dir() else {
            return Err(SwanError::operation(
                "Failed to get the current working directory!",
            ));
        };

        let voice = InputFile::file(format!(
            "{}/assets/voices/Belum_Terdaftar.mp3",
            path.display()
        ));

        let _ = self
            .bot
            .send_voice(ChatId(chat_id), voice)
            .caption("You are not registered yet. Please kindly register")
            .await?;

        Ok(true)
    }

    async fn is_authorized_user(&self, chat_id: i64) -> bool {
        match self.repo.fetch_user(chat_id).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn is_registered_guest(&self, chat_id: i64) -> bool {
        match self.repo.fetch_guest(chat_id).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn check_guest_quota(&self, chat_id: i64) -> Result<i32, SwanError> {
        let guest_quota = self.repo.fetch_guest_quota(chat_id).await?;

        Ok(guest_quota)
    }
}
