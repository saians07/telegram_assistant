use sea_orm::{JoinType, QueryOrder, QuerySelect, Set, TransactionTrait, prelude::*};
use uuid::Uuid;

use crate::{
    core::error::SwanError,
    dto::response::TelegramHistory,
    entities::{
        telegram_chat_history::{
            ActiveModel as TelegramChatHistoryActiveModel, Column as TelegramChatHistoryColumn,
            Entity as TelegramChatHistory, Relation as TelegramChatHistoryRelation,
        },
        telegram_guest::{
            ActiveModel as TelegramGuestActiveModel, Column as TelegramGuestColumn,
            Entity as TelegramGuest, Model as TelegramGuestModel,
        },
        telegram_guest_chat_history::{
            ActiveModel as TelegramGuestChatHistoryActiveModel, Entity as TelegramGuestChatHistory,
            Relation as TelegramGuestChatHistoryRelation,
        },
        telegram_users::{
            Column as TelegramUserColumn, Entity as TelegramUser, Model as TelegramUserModel,
        },
        users::ActiveModel as UserActiveModel,
    },
};

#[derive(Debug, Clone)]
pub struct TelegramRepo(pub DatabaseConnection);

impl TelegramRepo {
    pub fn new(db_con: DatabaseConnection) -> Self {
        Self(db_con)
    }

    pub async fn fetch_guest(&self, chat_id: i64) -> Result<TelegramGuestModel, SwanError> {
        let guest = self
            .0
            .transaction::<_, TelegramGuestModel, SwanError>(|txn| {
                Box::pin(async move {
                    let guest = TelegramGuest::find()
                        .filter(TelegramGuestColumn::TelegramChatId.eq(chat_id))
                        .one(txn)
                        .await?;

                    match guest {
                        Some(gst) => Ok(gst),
                        None => Err(SwanError::UnauthorizedBot),
                    }

                    // Ok(guest)
                })
            })
            .await
            .map_err(|e| SwanError::operation_with_source("Failed to fetch guest list: ", e))?;

        Ok(guest)
    }

    pub async fn fetch_user(&self, chat_id: i64) -> Result<TelegramUserModel, SwanError> {
        let guest = self
            .0
            .transaction::<_, TelegramUserModel, SwanError>(|txn| {
                Box::pin(async move {
                    let guest = TelegramUser::find()
                        .filter(TelegramUserColumn::TelegramChatId.eq(chat_id))
                        .one(txn)
                        .await?;

                    match guest {
                        Some(gst) => Ok(gst),
                        None => Err(SwanError::UnauthorizedBot),
                    }
                })
            })
            .await
            .map_err(|e| SwanError::operation_with_source("Failed to fetch guest list: ", e))?;

        Ok(guest)
    }

    pub async fn register_user(&self) -> Result<Uuid, SwanError> {
        let user_id = Uuid::new_v4();
        let user = UserActiveModel {
            id: Set(user_id),
            ..Default::default()
        };
        user.insert(&self.0).await?;

        Ok(user_id)
    }

    pub async fn register_guest(&self, telegram_chat_id: i32) -> Result<bool, SwanError> {
        self.0
            .transaction::<_, (), SwanError>(|txn| {
                Box::pin(async move {
                    let new_guest = TelegramGuestActiveModel {
                        telegram_chat_id: Set(telegram_chat_id),
                        authorized: Set(false),
                        ..Default::default()
                    };
                    new_guest.insert(txn).await?;

                    Ok(())
                })
            })
            .await
            .map_err(|e| SwanError::operation_with_source("Failed to register guest", e))?;

        Ok(true)
    }

    pub async fn insert_guest_chat(
        &self,
        chat_id: i32,
        message: String,
    ) -> Result<bool, SwanError> {
        self.0
            .transaction::<_, (), SwanError>(|txn| {
                Box::pin(async move {
                    let guest_id = TelegramGuest::find()
                        .filter(TelegramGuestColumn::TelegramChatId.eq(chat_id))
                        .one(txn)
                        .await?;
                    let Some(guest_model) = guest_id else {
                        return Err(SwanError::UnauthorizedBot);
                    };
                    let chat = TelegramGuestChatHistoryActiveModel {
                        telegram_guest_id: Set(guest_model.id),
                        message: Set(message),
                        ..Default::default()
                    };
                    chat.insert(txn).await?;
                    Ok(())
                })
            })
            .await
            .map_err(|e| {
                SwanError::operation_with_source("Failed to insert guest chat history!", e)
            })?;

        Ok(true)
    }

    pub async fn insert_user_chat(
        &self,
        role: &str,
        chat: &str,
        user_id: i32,
        session_id: &str,
    ) -> Result<bool, SwanError> {
        let chat_model = TelegramChatHistoryActiveModel {
            author: Set(role.to_owned()),
            message: Set(chat.to_owned()),
            telegram_user_id: Set(user_id),
            session_id: Set(session_id.to_owned()),
            ..Default::default()
        };

        chat_model.insert(&self.0).await?;

        Ok(true)
    }

    pub async fn fetch_user_chat_history(
        &self,
        chat_id: i64,
        session_id: &str,
    ) -> Result<Vec<TelegramHistory>, SwanError> {
        let chat_history = TelegramChatHistory::find()
            .join(
                JoinType::InnerJoin,
                TelegramChatHistoryRelation::TelegramUsers.def(),
            )
            .filter(
                TelegramUserColumn::TelegramChatId
                    .eq(chat_id)
                    .and(TelegramChatHistoryColumn::SessionId.eq(session_id.to_string())),
            )
            .order_by_asc(TelegramChatHistoryColumn::CreatedAt)
            .into_model::<TelegramHistory>()
            .all(&self.0)
            .await?;

        Ok(chat_history)
    }

    pub async fn fetch_guest_quota(&self, chat_id: i64) -> Result<i32, SwanError> {
        let duration_limit = chrono::Utc::now() - chrono::Duration::hours(24);
        let quota = TelegramGuestChatHistory::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                TelegramGuestChatHistoryRelation::TelegramGuest.def(),
            )
            .filter(
                TelegramGuestColumn::TelegramChatId
                    .eq(chat_id)
                    .and(TelegramGuestColumn::CreatedAt.gte(duration_limit)),
            )
            .count(&self.0)
            .await?;

        Ok(quota as i32)
    }

    pub async fn fetch_user_quota(&self, chat_id: i64) -> Result<i32, SwanError> {
        let quota = TelegramUser::find()
            .filter(TelegramUserColumn::TelegramChatId.eq(chat_id))
            .one(&self.0)
            .await?;
        match quota {
            Some(mdl) => Ok(mdl.daily_limit),
            None => Ok(10i32),
        }
    }

    pub async fn fetch_user_used_quota(&self, chat_id: i64) -> Result<i32, SwanError> {
        let duration_limit = chrono::Utc::now() - chrono::Duration::hours(24);
        let quota = TelegramChatHistory::find()
            .join(
                JoinType::InnerJoin,
                TelegramChatHistoryRelation::TelegramUsers.def(),
            )
            .filter(
                TelegramUserColumn::TelegramChatId
                    .eq(chat_id)
                    .and(TelegramChatHistoryColumn::CreatedAt.gte(duration_limit)),
            )
            .count(&self.0)
            .await?;

        Ok(quota as i32)
    }

    pub async fn fetch_last_session_id(&self, chat_id: i64) -> Result<String, SwanError> {
        let duration = chrono::Utc::now() - chrono::Duration::minutes(30);
        let chat_history = TelegramChatHistory::find()
            .join(
                JoinType::InnerJoin,
                TelegramChatHistoryRelation::TelegramUsers.def(),
            )
            .filter(TelegramUserColumn::TelegramChatId.eq(chat_id))
            .order_by_desc(TelegramChatHistoryColumn::CreatedAt)
            .one(&self.0)
            .await?;

        match chat_history {
            Some(mdl) => {
                let last_session_duration = duration < mdl.created_at;

                if last_session_duration {
                    Ok(mdl.session_id)
                } else {
                    Err(SwanError::SessionEnded)
                }
            }
            None => Err(SwanError::NotFoundError("Chat History".to_string())),
        }
    }
}
