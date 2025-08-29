//! # AI Provider Abstractions 🔌
//!
//! Definición de traits y tipos para providers de IA

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::errors::Result;

/// Trait principal para providers de IA
#[async_trait]
pub trait AIProvider: Send + Sync + std::fmt::Debug {
    /// Nombre del provider
    fn name(&self) -> String;
    
    /// Configuración del provider
    fn config(&self) -> &ProviderConfig;
    
    /// Generar texto con el provider
    async fn generate_text(&self, prompt: &str, creativity: f32) -> Result<String>;
    
    /// Verificar si el provider está disponible
    async fn is_available(&self) -> bool;
    
    /// Obtener límites del provider (tokens, requests, etc.)
    fn get_limits(&self) -> ProviderLimits;
    
    /// Estimar costo de una generación
    async fn estimate_cost(&self, prompt: &str) -> Option<f64>;
}

/// Configuración base para providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub max_tokens: u32,
    pub timeout_seconds: u32,
    pub custom_params: HashMap<String, String>,
}

/// Límites del provider
#[derive(Debug, Clone)]
pub struct ProviderLimits {
    pub max_tokens_per_request: u32,
    pub max_requests_per_minute: u32,
    pub max_tokens_per_minute: u32,
    pub supports_streaming: bool,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            api_key: None,
            base_url: None,
            model: "default".to_string(),
            max_tokens: 2000,
            timeout_seconds: 30,
            custom_params: HashMap::new(),
        }
    }
}

// === OpenAI Provider ===

#[cfg(feature = "openai")]
#[derive(Debug)]
pub struct OpenAIProvider {
    config: ProviderConfig,
    client: Option<openai_api_rs::v1::api::Client>,
}

#[cfg(feature = "openai")]
impl OpenAIProvider {
    pub fn new(api_key: String, model: Option<String>) -> Result<Self> {
        let config = ProviderConfig {
            name: "OpenAI".to_string(),
            api_key: Some(api_key.clone()),
            base_url: Some("https://api.openai.com/v1".to_string()),
            model: model.unwrap_or_else(|| "gpt-3.5-turbo".to_string()),
            max_tokens: 2000,
            timeout_seconds: 30,
            custom_params: HashMap::new(),
        };
        
        let client = openai_api_rs::v1::api::Client::new(api_key);
        
        Ok(Self {
            config,
            client: Some(client),
        })
    }
    
    pub fn with_custom_model(api_key: String, model: String, max_tokens: u32) -> Result<Self> {
        let mut provider = Self::new(api_key, Some(model))?;
        provider.config.max_tokens = max_tokens;
        Ok(provider)
    }
}

#[cfg(feature = "openai")]
#[async_trait]
impl AIProvider for OpenAIProvider {
    fn name(&self) -> String {
        format!("OpenAI ({})", self.config.model)
    }
    
    fn config(&self) -> &ProviderConfig {
        &self.config
    }
    
    async fn generate_text(&self, prompt: &str, creativity: f32) -> Result<String> {
        let client = self.client.as_ref()
            .ok_or_else(|| crate::errors::AIGeneratorError::ProviderNotAvailable)?;
            
        // Convert creativity to temperature (0.0 - 1.0 -> 0.0 - 2.0)
        let temperature = creativity * 2.0;
        
        let request = openai_api_rs::v1::chat_completion::ChatCompletionRequest::new(
            self.config.model.clone(),
            vec![openai_api_rs::v1::chat_completion::ChatCompletionMessage {
                role: openai_api_rs::v1::chat_completion::MessageRole::user,
                content: openai_api_rs::v1::chat_completion::Content::Text(prompt.to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        )
        .temperature(temperature)
        .max_tokens(self.config.max_tokens);
        
        let response = client.chat_completion(request).await
            .map_err(|e| crate::errors::AIGeneratorError::GenerationFailed(e.to_string()))?;
        
        let content = response.choices
            .first()
            .and_then(|choice| choice.message.content.as_ref())
            .ok_or_else(|| crate::errors::AIGeneratorError::GenerationFailed(
                "No content in response".to_string()
            ))?;
        
        Ok(content.clone())
    }
    
    async fn is_available(&self) -> bool {
        // Simple check - attempt to create a minimal request
        self.client.is_some() && self.config.api_key.is_some()
    }
    
    fn get_limits(&self) -> ProviderLimits {
        ProviderLimits {
            max_tokens_per_request: self.config.max_tokens,
            max_requests_per_minute: 60, // Standard OpenAI limit
            max_tokens_per_minute: 40000, // Approximation
            supports_streaming: true,
        }
    }
    
    async fn estimate_cost(&self, prompt: &str) -> Option<f64> {
        // Rough estimation - varies by model
        let tokens = prompt.len() / 4; // Rough approximation
        let cost_per_1k_tokens = match self.config.model.as_str() {
            "gpt-4" => 0.03,
            "gpt-3.5-turbo" => 0.002,
            _ => 0.002,
        };
        
        Some((tokens as f64 / 1000.0) * cost_per_1k_tokens)
    }
}

// === Anthropic Provider ===

#[cfg(feature = "anthropic")]
#[derive(Debug)]
pub struct AnthropicProvider {
    config: ProviderConfig,
    client: Option<anthropic::Client>,
}

#[cfg(feature = "anthropic")]
impl AnthropicProvider {
    pub fn new(api_key: String) -> Result<Self> {
        let config = ProviderConfig {
            name: "Anthropic".to_string(),
            api_key: Some(api_key.clone()),
            base_url: Some("https://api.anthropic.com".to_string()),
            model: "claude-3-sonnet-20240229".to_string(),
            max_tokens: 2000,
            timeout_seconds: 30,
            custom_params: HashMap::new(),
        };
        
        let client = anthropic::Client::new().api_key(api_key);
        
        Ok(Self {
            config,
            client: Some(client),
        })
    }
}

#[cfg(feature = "anthropic")]
#[async_trait]
impl AIProvider for AnthropicProvider {
    fn name(&self) -> String {
        format!("Anthropic ({})", self.config.model)
    }
    
    fn config(&self) -> &ProviderConfig {
        &self.config
    }
    
    async fn generate_text(&self, prompt: &str, creativity: f32) -> Result<String> {
        let client = self.client.as_ref()
            .ok_or_else(|| crate::errors::AIGeneratorError::ProviderNotAvailable)?;
            
        let temperature = creativity;
        
        let request = anthropic::messages::CreateMessageRequest::new(
            self.config.model.clone(),
            self.config.max_tokens,
            vec![anthropic::messages::Message {
                role: anthropic::messages::Role::User,
                content: vec![anthropic::messages::Content::Text {
                    text: prompt.to_string(),
                }],
            }],
        )
        .temperature(temperature);
        
        let response = client.messages().create(request).await
            .map_err(|e| crate::errors::AIGeneratorError::GenerationFailed(e.to_string()))?;
        
        let content = response.content
            .first()
            .ok_or_else(|| crate::errors::AIGeneratorError::GenerationFailed(
                "No content in response".to_string()
            ))?;
        
        match content {
            anthropic::messages::ContentBlock::Text { text } => Ok(text.clone()),
            _ => Err(crate::errors::AIGeneratorError::GenerationFailed(
                "Unexpected content type".to_string()
            )),
        }
    }
    
    async fn is_available(&self) -> bool {
        self.client.is_some() && self.config.api_key.is_some()
    }
    
    fn get_limits(&self) -> ProviderLimits {
        ProviderLimits {
            max_tokens_per_request: self.config.max_tokens,
            max_requests_per_minute: 50, // Anthropic limit
            max_tokens_per_minute: 40000,
            supports_streaming: true,
        }
    }
    
    async fn estimate_cost(&self, prompt: &str) -> Option<f64> {
        let tokens = prompt.len() / 4;
        let cost_per_1k_tokens = 0.015; // Claude 3 Sonnet pricing
        Some((tokens as f64 / 1000.0) * cost_per_1k_tokens)
    }
}

// === Ollama Provider (Local) ===

#[cfg(feature = "ollama")]
#[derive(Debug)]
pub struct OllamaProvider {
    config: ProviderConfig,
    base_url: String,
}

#[cfg(feature = "ollama")]
impl OllamaProvider {
    pub fn new(base_url: Option<String>, model: String) -> Result<Self> {
        let base_url = base_url.unwrap_or_else(|| "http://localhost:11434".to_string());
        
        let config = ProviderConfig {
            name: "Ollama".to_string(),
            api_key: None, // Ollama doesn't use API keys
            base_url: Some(base_url.clone()),
            model,
            max_tokens: 2000,
            timeout_seconds: 60, // Local models might be slower
            custom_params: HashMap::new(),
        };
        
        Ok(Self {
            config,
            base_url,
        })
    }
    
    pub fn local_llama(model: String) -> Result<Self> {
        Self::new(None, model)
    }
    
    pub fn local_codellama() -> Result<Self> {
        Self::new(None, "codellama:latest".to_string())
    }
}

#[cfg(feature = "ollama")]
#[async_trait]
impl AIProvider for OllamaProvider {
    fn name(&self) -> String {
        format!("Ollama ({})", self.config.model)
    }
    
    fn config(&self) -> &ProviderConfig {
        &self.config
    }
    
    async fn generate_text(&self, prompt: &str, creativity: f32) -> Result<String> {
        use reqwest::Client;
        use serde_json::{json, Value};
        
        let client = Client::new();
        let url = format!("{}/api/generate", self.base_url);
        
        let request_body = json!({
            "model": self.config.model,
            "prompt": prompt,
            "temperature": creativity,
            "stream": false,
            "options": {
                "num_predict": self.config.max_tokens
            }
        });
        
        let response = client
            .post(&url)
            .json(&request_body)
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds as u64))
            .send()
            .await
            .map_err(|e| crate::errors::AIGeneratorError::GenerationFailed(e.to_string()))?;
        
        let response_json: Value = response
            .json()
            .await
            .map_err(|e| crate::errors::AIGeneratorError::GenerationFailed(e.to_string()))?;
        
        let content = response_json["response"]
            .as_str()
            .ok_or_else(|| crate::errors::AIGeneratorError::GenerationFailed(
                "No response content from Ollama".to_string()
            ))?;
        
        Ok(content.to_string())
    }
    
    async fn is_available(&self) -> bool {
        use reqwest::Client;
        
        let client = Client::new();
        let url = format!("{}/api/version", self.base_url);
        
        client.get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
    
    fn get_limits(&self) -> ProviderLimits {
        ProviderLimits {
            max_tokens_per_request: self.config.max_tokens,
            max_requests_per_minute: 1000, // Local - no real limit
            max_tokens_per_minute: 100000, // Local - no real limit
            supports_streaming: true,
        }
    }
    
    async fn estimate_cost(&self, _prompt: &str) -> Option<f64> {
        Some(0.0) // Local models are free
    }
}

// === Mock Provider for Testing ===

#[derive(Debug)]
pub struct MockProvider {
    config: ProviderConfig,
    response_template: String,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            config: ProviderConfig {
                name: "Mock".to_string(),
                model: "mock-model-v1".to_string(),
                ..Default::default()
            },
            response_template: Self::default_template(),
        }
    }
    
    pub fn with_template(template: String) -> Self {
        Self {
            config: ProviderConfig {
                name: "Mock".to_string(),
                model: "mock-model-v1".to_string(),
                ..Default::default()
            },
            response_template: template,
        }
    }
    
    fn default_template() -> String {
        r#"# Mock Generated Template

alias: BITA-NAV-DEBUG-ERROR-AI-v1

## Context Variables
```
error_message: "{{error_message}}"
stack_trace: "{{stack_trace}}"
project_context: "{{project_context}}"
```

## Navigation Steps

1. **Analyze Error Context**
   - Review the error message: {{error_message}}
   - Examine stack trace for root cause
   
2. **Investigate Root Cause**
   - Check recent code changes
   - Verify dependencies and configuration
   
3. **Implement Solution**
   - Apply identified fix
   - Test solution thoroughly
   
4. **Validate Resolution**
   - Run full test suite
   - Monitor for recurring issues

## Interactive Questions
- What was the exact sequence of actions that led to this error?
- Are there any recent changes that might be related?
- Have you seen similar errors before?

## Metrics
- Time to resolution: {{resolution_time}}
- Error recurrence rate: {{recurrence_rate}}
- Solution effectiveness: {{effectiveness_score}}
"#.to_string()
    }
}

#[async_trait]
impl AIProvider for MockProvider {
    fn name(&self) -> String {
        "Mock Provider".to_string()
    }
    
    fn config(&self) -> &ProviderConfig {
        &self.config
    }
    
    async fn generate_text(&self, _prompt: &str, _creativity: f32) -> Result<String> {
        // Simulate some processing time
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(self.response_template.clone())
    }
    
    async fn is_available(&self) -> bool {
        true
    }
    
    fn get_limits(&self) -> ProviderLimits {
        ProviderLimits {
            max_tokens_per_request: 2000,
            max_requests_per_minute: 1000,
            max_tokens_per_minute: 100000,
            supports_streaming: false,
        }
    }
    
    async fn estimate_cost(&self, _prompt: &str) -> Option<f64> {
        Some(0.0)
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}
