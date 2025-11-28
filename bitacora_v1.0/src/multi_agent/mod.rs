//! # Multi-Agent System
//!
//! Sistema de orquestación multi-LLM con routing inteligente.

pub mod hubspoke;
pub mod llm_client;

pub use hubspoke::{
    Hub, HubSpokeConfig, ContextTensor7D, HubSpokeError, RoutingStrategy,
};
pub use llm_client::{
    LLMClient, LLMRequest, LLMResponse, ClientConfig, LLMClientError, LLMProvider,
};
