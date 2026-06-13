use rig::{message::Message, wasm_compat::WasmCompatSend};

use crate::core::error::SwanError;

#[async_trait::async_trait]
pub trait AgentTrait {
    async fn create_history(&self) -> Vec<Message>;
    async fn test_func(
        &self,
        promt: impl Into<Message> + WasmCompatSend,
        chat_history: &mut Vec<Message>,
    ) -> Result<String, SwanError>;
}
