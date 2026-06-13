use rig::{
    agent::{Agent, AgentBuilder},
    providers::{anthropic, cohere, gemini, openai, together},
    tool::ToolDyn,
};

pub enum ProviderName {
    OpenAI,
    Cohere,
    Anthropic,
    Gemini,
    Together,
}

pub enum BuilderAgentProvider {
    OpenAI(AgentBuilder<openai::responses_api::ResponsesCompletionModel>),
    Cohere(AgentBuilder<cohere::CompletionModel>),
    Anthropic(AgentBuilder<anthropic::completion::CompletionModel>),
    Gemini(AgentBuilder<gemini::completion::CompletionModel>),
    Together(AgentBuilder<together::completion::CompletionModel>),
}

pub enum BuilderAgentProviderWithTool {
    OpenAI(AgentBuilder<openai::responses_api::ResponsesCompletionModel>),
    Cohere(AgentBuilder<cohere::CompletionModel>),
    Anthropic(AgentBuilder<anthropic::completion::CompletionModel>),
    Gemini(AgentBuilder<gemini::completion::CompletionModel>),
    Together(AgentBuilder<together::completion::CompletionModel>),
}

impl BuilderAgentProvider {
    pub async fn build(self, tools: Option<Vec<Box<dyn ToolDyn + 'static>>>) -> ProviderAgent {
        match self {
            BuilderAgentProvider::OpenAI(builder) => match tools {
                Some(tool) => {
                    let agent = builder.tools(tool).build();
                    // let resp = agent
                    //     .prompt("tell me anything about you!")
                    //     .await
                    //     .expect("I AM FAILED HERE!");
                    // tracing::info!("The response: {}", resp);
                    ProviderAgent::OpenAI(agent)
                }
                _ => {
                    let agent = builder.build();
                    ProviderAgent::OpenAI(agent)
                }
            },
            BuilderAgentProvider::Cohere(builder) => match tools {
                Some(tool) => ProviderAgent::Cohere(builder.tools(tool).build()),
                _ => ProviderAgent::Cohere(builder.build()),
            },
            BuilderAgentProvider::Anthropic(builder) => match tools {
                Some(tool) => ProviderAgent::Anthropic(builder.tools(tool).build()),
                _ => ProviderAgent::Anthropic(builder.build()),
            },
            BuilderAgentProvider::Gemini(builder) => match tools {
                Some(tool) => ProviderAgent::Gemini(builder.tools(tool).build()),
                _ => ProviderAgent::Gemini(builder.build()),
            },
            BuilderAgentProvider::Together(builder) => match tools {
                Some(tool) => ProviderAgent::Together(builder.tools(tool).build()),
                _ => {
                    let agent = builder.build();
                    // let resp = agent.prompt("Tell me who you are!").await.unwrap();
                    // tracing::info!("AI Response: {}", resp);
                    ProviderAgent::Together(agent)
                }
            },
        }
    }
}

pub enum ProviderAgent {
    OpenAI(Agent<openai::responses_api::ResponsesCompletionModel>),
    Cohere(Agent<cohere::CompletionModel>),
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    Gemini(Agent<gemini::completion::CompletionModel>),
    Together(Agent<together::completion::CompletionModel>),
}

pub enum TaskType {
    QnA,
    RAG,
    Research,
}
