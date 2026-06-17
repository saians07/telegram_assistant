use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;
use rig::{
    client::CompletionClient,
    completion::Chat,
    message::Message,
    providers::{anthropic, cohere, gemini, openai, openrouter, together},
    tool::ToolDyn,
    wasm_compat::WasmCompatSend,
};

use crate::core::{
    enums::{BuilderAgentProvider, ProviderAgent, ProviderName},
    error::SwanError,
    traits::ai_agent::AgentTrait,
};

// every part of the bot info basically changeable in the future.
#[derive(Debug, Clone)]
pub struct BotInfo {
    pub bot_name: String,
    pub base_url: String,
    pub api_key: String,
    pub system_prompt: &'static str,
    pub model: String,
}

impl Default for BotInfo {
    fn default() -> Self {
        BotInfo {
            bot_name: "Swan".to_string(),
            base_url: "".to_string(),
            api_key: "".to_string(),
            system_prompt: r#"
                You are Swan, a virtual assistant created by PT Trias Sigma Technology.
                Your jobs are as follow:
                    * Engage in conversation with users about basic topics
                    * Answer question related to PT TRIAS SIGMA TECHNOLOGY
                    * Helping users by executing tools when possible and available.
                "#,
            model: "".to_string(),
        }
    }
}

pub struct BotAgent {
    pub agent: ProviderAgent,
    pub bot_name: String,
}

impl BotAgent {
    pub fn new_open_ai(bot_info: BotInfo) -> BuilderAgentProvider {
        let agent = openai::Client::builder()
            .base_url(bot_info.base_url)
            .api_key(bot_info.api_key)
            .build()
            .unwrap()
            .agent(bot_info.model)
            .preamble(&bot_info.system_prompt);

        BuilderAgentProvider::OpenAI(agent)
    }

    pub fn new_cohere(bot_info: BotInfo) -> BuilderAgentProvider {
        BuilderAgentProvider::Cohere(
            cohere::Client::builder()
                .base_url(bot_info.base_url)
                .api_key(bot_info.api_key)
                .build()
                .unwrap()
                .agent(bot_info.model)
                .preamble(&bot_info.system_prompt),
        )
    }

    pub fn new_anthropic(bot_info: BotInfo) -> BuilderAgentProvider {
        BuilderAgentProvider::Anthropic(
            anthropic::Client::builder()
                .base_url(bot_info.base_url)
                .api_key(bot_info.api_key)
                .build()
                .unwrap()
                .agent(bot_info.model)
                .preamble(&bot_info.system_prompt),
        )
    }

    pub fn new_gemini(bot_info: BotInfo) -> BuilderAgentProvider {
        BuilderAgentProvider::Gemini(
            gemini::Client::builder()
                .base_url(bot_info.base_url)
                .api_key(bot_info.api_key)
                .build()
                .unwrap()
                .agent(bot_info.model)
                .preamble(&bot_info.system_prompt),
        )
    }

    pub fn new_together(bot_info: BotInfo) -> BuilderAgentProvider {
        BuilderAgentProvider::Together(
            together::Client::builder()
                .base_url(bot_info.base_url)
                .api_key(bot_info.api_key)
                .build()
                .unwrap()
                .agent(bot_info.model)
                .preamble(&bot_info.system_prompt),
        )
    }

    pub fn new_openrouter(bot_info: BotInfo) -> BuilderAgentProvider {
        BuilderAgentProvider::OpenRouter(
            openrouter::Client::builder()
                .base_url(bot_info.base_url)
                .api_key(bot_info.api_key)
                .build()
                .unwrap()
                .agent(bot_info.model)
                .preamble(&bot_info.system_prompt),
        )
    }
}

impl BotAgent {
    /// Initialize the bot agent at the first time.
    /// This will save your time especially when you have
    /// all the kind of information needed without retyping them
    /// again and again.
    pub async fn new(
        bot_info: BotInfo,
        agent_type: ProviderName,
        tools: Option<Vec<Box<dyn ToolDyn + 'static>>>,
    ) -> Self {
        let bot_info_c = bot_info.clone();
        let agent_provider = match agent_type {
            ProviderName::OpenAI => BotAgent::new_open_ai(bot_info),
            ProviderName::Anthropic => BotAgent::new_anthropic(bot_info),
            ProviderName::Cohere => BotAgent::new_cohere(bot_info),
            ProviderName::Gemini => BotAgent::new_gemini(bot_info),
            ProviderName::Together => BotAgent::new_together(bot_info),
            ProviderName::OpenRouter => BotAgent::new_openrouter(bot_info),
        };

        Self {
            agent: agent_provider.build(tools).await,
            bot_name: bot_info_c.bot_name,
        }
    }

    /// You already have the bot, you just want to patch the element of them, such as the base URL
    /// without necessarily pass in the Provider type again and again
    pub async fn reinitialize_bot(
        &mut self,
        bot_info: BotInfo,
        tools: Option<Vec<Box<dyn ToolDyn + 'static>>>,
    ) -> Result<(), SwanError> {
        self.bot_name = bot_info.clone().bot_name;
        let agt = match self.agent {
            ProviderAgent::OpenAI(_) => BotAgent::new_open_ai(bot_info),
            ProviderAgent::Anthropic(_) => BotAgent::new_anthropic(bot_info),
            ProviderAgent::Cohere(_) => BotAgent::new_cohere(bot_info),
            ProviderAgent::Gemini(_) => BotAgent::new_gemini(bot_info),
            ProviderAgent::Together(_) => BotAgent::new_together(bot_info),
            ProviderAgent::OpenRouter(_) => BotAgent::new_openrouter(bot_info),
        };
        self.agent = agt.build(tools).await;
        Ok(())
    }
}

#[async_trait]
impl AgentTrait for BotAgent {
    /// Function to convert user and ai turn
    async fn create_history(&self) -> Vec<Message> {
        let mut a = Vec::new();
        a.push(Message::user(
            "Halo nama saya adalah Bob! Saya tinggal dipinggiran kota California.",
        ));
        a
    }
    async fn test_func(
        &self,
        prompt: impl Into<Message> + WasmCompatSend,
        chat_history: &mut Vec<Message>,
    ) -> Result<String, SwanError> {
        let resp = match &self.agent {
            ProviderAgent::Together(agent) => {
                agent.chat(prompt, chat_history).await.map_err(|e| {
                    SwanError::operation_with_source(
                        "Failed to get response from Open AI server: ",
                        e,
                    )
                })?
            }
            ProviderAgent::OpenRouter(agent) => {
                agent.chat(prompt, chat_history).await.map_err(|e| {
                    SwanError::operation_with_source(
                        "Failed to get response from Open Open Router Server: ",
                        e,
                    )
                })?
            }
            ProviderAgent::OpenAI(agent) => {
                agent.chat(prompt, chat_history).await.map_err(|e| {
                    SwanError::operation_with_source(
                        "Failed to get response from Open Open Router Server: ",
                        e,
                    )
                })?
            }
            _ => "".to_string(),
        };

        Ok(resp)
    }
}
