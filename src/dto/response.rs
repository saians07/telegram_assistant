use axum::http::StatusCode;
use rig::message::Message;
use sea_orm::{FromJsonQueryResult, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromJsonQueryResult, FromQueryResult, Serialize, Deserialize, Default)]
pub struct TelegramHistory {
    pub author: String,
    pub message: String,
}

pub struct TelegramHistoryList(pub Vec<TelegramHistory>);

#[derive(serde::Serialize, utoipa::ToSchema, Debug)]
pub struct BaseResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<()>,
}

pub struct ResponseWithData<T> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

impl<T> ResponseWithData<T> {
    pub fn reply(stat: StatusCode, message: &str, data: T) -> Self {
        Self {
            status: stat.as_u16(),
            message: message.into(),
            data,
        }
    }
}

impl BaseResponse {
    pub fn reply(stat: StatusCode, message: &str) -> Self {
        Self {
            status: stat.as_u16(),
            message: message.into(),
            data: None,
        }
    }
}

impl TelegramHistoryList {
    pub async fn create_message_vector(self) -> Vec<Message> {
        let mut messages: String = "".to_string();
        let mut prev_author: String = "".to_string();
        let mut history: Vec<Message> = Vec::new();
        let len_hist = self.0.len() - 1;
        for (idx, message) in self.0.iter().enumerate() {
            let message = message.clone();
            if idx == 0 {
                messages = message.message.clone();
                prev_author = "user".to_string();
                continue;
            }
            if idx == len_hist {
                match message.author.as_str() {
                    "user" => {
                        if prev_author == "user" {
                            messages = format!("{} {}", &messages, &message.message);
                            history.push(Message::user(messages.clone()));
                        }
                        if prev_author == "assistant" {
                            history.push(Message::user(message.message.clone()));
                        }
                    }
                    "assistant" => {
                        history.push(Message::assistant(message.message.clone()));
                    }
                    _ => {}
                }
            }
            match message.author.as_str() {
                "user" => {
                    if prev_author == "user" {
                        messages = format!("{} {}", &messages, &message.message);
                    }
                    if prev_author == "assistant" {
                        history.push(Message::assistant(messages));
                        messages = message.message;
                    }
                    prev_author = "user".to_string();
                }
                "assistant" => {
                    if prev_author == "user" {
                        history.push(Message::user(messages));
                        messages = message.message;
                        prev_author = "assistant".to_string();
                    } else {
                        continue;
                    }
                }
                _ => {}
            }
        }
        history
    }
}
