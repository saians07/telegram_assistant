use rig::{message::Message, wasm_compat::WasmCompatSend};

use crate::{core::error::SwanError, dto::response::TelegramHistory};

#[async_trait::async_trait]
pub trait AgentTrait {
    async fn create_history(&self, telegram_history: Vec<TelegramHistory>) -> Vec<Message>;
    async fn send_chat_message(
        &self,
        promt: impl Into<Message> + WasmCompatSend,
        chat_history: &mut Vec<Message>,
    ) -> Result<String, SwanError>;
}
