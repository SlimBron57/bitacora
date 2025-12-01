//! # Conversational Engine
//!
//! Motor para conversación general y narrativa biográfica.
//!
//! ## Responsabilidades
//!
//! 1. **GeneralConversation**: Dialogar sobre cualquier tema (no solo first-time)
//! 2. **ContextRetention**: Mantener contexto de conversación activa
//! 3. **ToneAdaptation**: Adaptar tono según sentiment del usuario
//! 4. **BiographicalNarrative**: Crear narrativas de vida/experiencias
//!
//! ## Modo Conversational
//!
//! Cuando el usuario dice:
//! - "cuéntame sobre tu día"
//! - "¿cómo te sientes?"
//! - "déjame contarte algo"
//! - "quiero hablar de mi infancia"
//!
//! ShuiDao detecta intención **Conversational** y activa este engine.
//!
//! ## Diferencia con IceBreaker
//!
//! - **IceBreaker**: Primera vez, construir relación inicial (4 stages)
//! - **Conversational**: Usuario ya conocido, conversación fluida
//!
//! ## Filosofía
//!
//! ```text
//! IceBreaker: "¿Cómo te llamas?" (first-time)
//!       ↓
//! Conversational: "Hola Eduardo, ¿cómo estuvo tu día?" (returning user)
//! ```
//!
//! **Performance Target**: <180ms engine processing
//!
//! **Versión**: 1.0.0-beta  
//! **Fecha**: 2025-11-24 (Week 3 Day 5)

use crate::shuidao::error::{Result, ShuiDaoError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

// ============================================================================
// STRUCTURES
// ============================================================================

/// Conversational Engine
pub struct ConversationalEngine {
    /// Active conversations (conversation_id → Conversation)
    conversations: HashMap<String, Conversation>,
    
    /// Sentiment keywords for analysis
    positive_keywords: Vec<String>,
    negative_keywords: Vec<String>,
}

/// Conversación activa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub user_name: Option<String>,
    pub started_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: ConversationStatus,
    
    // Context
    pub messages: Vec<ConversationMessage>,
    pub topics_discussed: Vec<String>,
    pub current_tone: ConversationalTone,
    
    // Tracking
    pub total_exchanges: u32,
    pub sentiment_history: Vec<SentimentScore>,
}

/// Mensaje de conversación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub speaker: Speaker,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub sentiment: SentimentScore,
}

/// Quién habla
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Speaker {
    User,
    System,
}

/// Tono conversacional
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConversationalTone {
    Casual,      // Relajado, informal
    Empathetic,  // Empático, comprensivo
    Curious,     // Curioso, preguntando
    Reflective,  // Reflexivo, profundo
    Supportive,  // Apoyo emocional
}

/// Estado de conversación
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConversationStatus {
    Active,
    Paused,
    Ended,
}

/// Sentiment score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentScore {
    pub score: f32, // -1.0 (muy negativo) a +1.0 (muy positivo)
    pub detected_at: DateTime<Utc>,
}

/// Response del engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationalResponse {
    pub conversation_id: String,
    pub response_content: String,
    pub suggested_tone: ConversationalTone,
    pub detected_topics: Vec<String>,
    pub sentiment: SentimentScore,
    pub processing_time_ms: f64,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl ConversationalEngine {
    /// Crear nuevo ConversationalEngine
    pub fn new() -> Self {
        Self {
            conversations: HashMap::new(),
            positive_keywords: vec![
                "feliz".to_string(),
                "contento".to_string(),
                "genial".to_string(),
                "excelente".to_string(),
                "bien".to_string(),
                "alegre".to_string(),
                "emocionado".to_string(),
                "gracias".to_string(),
                "perfecto".to_string(),
                "increíble".to_string(),
            ],
            negative_keywords: vec![
                "triste".to_string(),
                "mal".to_string(),
                "frustrado".to_string(),
                "enojado".to_string(),
                "preocupado".to_string(),
                "difícil".to_string(),
                "problema".to_string(),
                "horrible".to_string(),
                "terrible".to_string(),
                "ansioso".to_string(),
            ],
        }
    }
    
    /// Iniciar nueva conversación
    pub fn start_conversation(&mut self, user_name: Option<String>) -> Result<String> {
        let conversation_id = uuid::Uuid::new_v4().to_string();
        
        let conversation = Conversation {
            id: conversation_id.clone(),
            user_name,
            started_at: Utc::now(),
            last_updated: Utc::now(),
            status: ConversationStatus::Active,
            messages: Vec::new(),
            topics_discussed: Vec::new(),
            current_tone: ConversationalTone::Casual,
            total_exchanges: 0,
            sentiment_history: Vec::new(),
        };
        
        self.conversations.insert(conversation_id.clone(), conversation);
        Ok(conversation_id)
    }
    
    /// Procesar mensaje del usuario
    pub fn process_message(
        &mut self,
        conversation_id: &str,
        user_message: String,
    ) -> Result<ConversationalResponse> {
        let start = Instant::now();
        
        // Analyze sentiment (before mutable borrow)
        let sentiment = self.analyze_sentiment(&user_message);
        
        // Detect topics (before mutable borrow)
        let detected_topics = self.detect_topics(&user_message);
        
        // Adapt tone based on sentiment (before mutable borrow)
        let suggested_tone = self.suggest_tone(&sentiment);
        
        // Get conversation and user name
        let conversation = self.conversations.get_mut(conversation_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Conversation {} not found", conversation_id)))?;
        
        let user_name = conversation.user_name.clone();
        
        // Add to conversation history
        conversation.messages.push(ConversationMessage {
            speaker: Speaker::User,
            content: user_message.clone(),
            timestamp: Utc::now(),
            sentiment: sentiment.clone(),
        });
        
        conversation.sentiment_history.push(sentiment.clone());
        conversation.topics_discussed.extend(detected_topics.clone());
        conversation.total_exchanges += 1;
        conversation.last_updated = Utc::now();
        conversation.current_tone = suggested_tone.clone();
        
        // Generate response content (template-based in v1.1)
        let response_content = self.generate_response_static(user_name, &user_message, &detected_topics);
        
        Ok(ConversationalResponse {
            conversation_id: conversation_id.to_string(),
            response_content,
            suggested_tone,
            detected_topics,
            sentiment,
            processing_time_ms: start.elapsed().as_secs_f64() * 1000.0,
        })
    }
    
    /// Analizar sentiment
    fn analyze_sentiment(&self, text: &str) -> SentimentScore {
        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower.split_whitespace().collect();
        
        let positive_count = self.positive_keywords.iter()
            .filter(|keyword| words.contains(&keyword.as_str()))
            .count() as f32;
        
        let negative_count = self.negative_keywords.iter()
            .filter(|keyword| words.contains(&keyword.as_str()))
            .count() as f32;
        
        // Simple scoring: positive - negative, normalized
        let score = if positive_count + negative_count > 0.0 {
            (positive_count - negative_count) / (positive_count + negative_count)
        } else {
            0.0 // Neutral if no keywords detected
        };
        
        SentimentScore {
            score: score.max(-1.0).min(1.0), // Clamp to [-1, 1]
            detected_at: Utc::now(),
        }
    }
    
    /// Detectar topics
    fn detect_topics(&self, text: &str) -> Vec<String> {
        let mut topics = Vec::new();
        let text_lower = text.to_lowercase();
        
        // Simple keyword-based topic detection (v1.0)
        let topic_keywords = vec![
            ("trabajo", "Work"),
            ("familia", "Family"),
            ("estudio", "Study"),
            ("proyecto", "Project"),
            ("viaje", "Travel"),
            ("salud", "Health"),
            ("hobby", "Hobby"),
        ];
        
        for (keyword, topic) in topic_keywords {
            if text_lower.contains(keyword) {
                topics.push(topic.to_string());
            }
        }
        
        if topics.is_empty() {
            topics.push("General".to_string());
        }
        
        topics
    }
    
    /// Sugerir tono según sentiment
    fn suggest_tone(&self, sentiment: &SentimentScore) -> ConversationalTone {
        if sentiment.score > 0.5 {
            ConversationalTone::Casual
        } else if sentiment.score < -0.5 {
            ConversationalTone::Supportive
        } else if sentiment.score < 0.0 {
            ConversationalTone::Empathetic
        } else {
            ConversationalTone::Curious
        }
    }
    
    /// Generar respuesta (placeholder - templates en v1.1)
    fn generate_response_static(
        &self,
        user_name: Option<String>,
        _user_message: &str,
        topics: &[String],
    ) -> String {
        // Simple response generation (v1.0)
        let greeting = if let Some(name) = user_name {
            format!("Hola {}, ", name)
        } else {
            "Hola, ".to_string()
        };
        
        let topic_mention = if !topics.is_empty() && topics[0] != "General" {
            format!("entiendo que estás hablando sobre {}. ", topics[0])
        } else {
            String::new()
        };
        
        format!(
            "{}{}¿Te gustaría contarme más sobre eso?",
            greeting,
            topic_mention
        )
    }
    
    /// Obtener conversación
    pub fn get_conversation(&self, conversation_id: &str) -> Result<Conversation> {
        self.conversations.get(conversation_id)
            .cloned()
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Conversation {} not found", conversation_id)))
    }
    
    /// Finalizar conversación
    pub fn end_conversation(&mut self, conversation_id: &str) -> Result<()> {
        let conversation = self.conversations.get_mut(conversation_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Conversation {} not found", conversation_id)))?;
        
        conversation.status = ConversationStatus::Ended;
        conversation.last_updated = Utc::now();
        Ok(())
    }
    
    /// Listar conversaciones activas
    pub fn list_active_conversations(&self) -> Vec<Conversation> {
        self.conversations.values()
            .filter(|c| c.status == ConversationStatus::Active)
            .cloned()
            .collect()
    }
}

impl Default for ConversationalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversational_engine_creation() {
        let engine = ConversationalEngine::new();
        assert_eq!(engine.conversations.len(), 0);
        assert!(engine.positive_keywords.len() > 0);
        assert!(engine.negative_keywords.len() > 0);
    }
    
    #[test]
    fn test_start_conversation() {
        let mut engine = ConversationalEngine::new();
        let conversation_id = engine.start_conversation(Some("Eduardo".to_string())).unwrap();
        
        let conversation = engine.get_conversation(&conversation_id).unwrap();
        assert_eq!(conversation.user_name, Some("Eduardo".to_string()));
        assert_eq!(conversation.status, ConversationStatus::Active);
        assert_eq!(conversation.total_exchanges, 0);
    }
    
    #[test]
    fn test_process_message() {
        let mut engine = ConversationalEngine::new();
        let conversation_id = engine.start_conversation(Some("Eduardo".to_string())).unwrap();
        
        let response = engine.process_message(
            &conversation_id,
            "Hoy tuve un día genial en el trabajo".to_string(),
        ).unwrap();
        
        assert!(response.sentiment.score > 0.0); // Positive sentiment
        assert!(response.detected_topics.contains(&"Work".to_string()));
        assert!(response.processing_time_ms < 180.0);
    }
    
    #[test]
    fn test_sentiment_analysis_positive() {
        let engine = ConversationalEngine::new();
        let sentiment = engine.analyze_sentiment("Estoy muy feliz y contento hoy");
        
        assert!(sentiment.score > 0.0);
    }
    
    #[test]
    fn test_sentiment_analysis_negative() {
        let engine = ConversationalEngine::new();
        let sentiment = engine.analyze_sentiment("Me siento triste y frustrado");
        
        assert!(sentiment.score < 0.0);
    }
    
    #[test]
    fn test_sentiment_analysis_neutral() {
        let engine = ConversationalEngine::new();
        let sentiment = engine.analyze_sentiment("Hoy es un día normal");
        
        assert_eq!(sentiment.score, 0.0);
    }
    
    #[test]
    fn test_topic_detection() {
        let engine = ConversationalEngine::new();
        let topics = engine.detect_topics("Tuve problemas en el trabajo con mi proyecto");
        
        assert!(topics.contains(&"Work".to_string()));
        assert!(topics.contains(&"Project".to_string()));
    }
    
    #[test]
    fn test_tone_adaptation() {
        let engine = ConversationalEngine::new();
        
        // Positive sentiment → Casual tone
        let positive_sentiment = SentimentScore {
            score: 0.8,
            detected_at: Utc::now(),
        };
        assert_eq!(engine.suggest_tone(&positive_sentiment), ConversationalTone::Casual);
        
        // Negative sentiment → Supportive tone
        let negative_sentiment = SentimentScore {
            score: -0.8,
            detected_at: Utc::now(),
        };
        assert_eq!(engine.suggest_tone(&negative_sentiment), ConversationalTone::Supportive);
        
        // Mild negative → Empathetic tone
        let mild_negative = SentimentScore {
            score: -0.3,
            detected_at: Utc::now(),
        };
        assert_eq!(engine.suggest_tone(&mild_negative), ConversationalTone::Empathetic);
    }
    
    #[test]
    fn test_conversation_history() {
        let mut engine = ConversationalEngine::new();
        let conversation_id = engine.start_conversation(Some("Eduardo".to_string())).unwrap();
        
        engine.process_message(&conversation_id, "Primer mensaje".to_string()).unwrap();
        engine.process_message(&conversation_id, "Segundo mensaje".to_string()).unwrap();
        
        let conversation = engine.get_conversation(&conversation_id).unwrap();
        assert_eq!(conversation.total_exchanges, 2);
        assert_eq!(conversation.messages.len(), 2);
    }
    
    #[test]
    fn test_end_conversation() {
        let mut engine = ConversationalEngine::new();
        let conversation_id = engine.start_conversation(None).unwrap();
        
        engine.end_conversation(&conversation_id).unwrap();
        
        let conversation = engine.get_conversation(&conversation_id).unwrap();
        assert_eq!(conversation.status, ConversationStatus::Ended);
    }
    
    #[test]
    fn test_list_active_conversations() {
        let mut engine = ConversationalEngine::new();
        
        let conv1 = engine.start_conversation(Some("User1".to_string())).unwrap();
        let conv2 = engine.start_conversation(Some("User2".to_string())).unwrap();
        engine.end_conversation(&conv1).unwrap();
        
        let active = engine.list_active_conversations();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, conv2);
    }
    
    #[test]
    fn test_performance_target() {
        let mut engine = ConversationalEngine::new();
        let conversation_id = engine.start_conversation(Some("Eduardo".to_string())).unwrap();
        
        let response = engine.process_message(
            &conversation_id,
            "Test message".to_string(),
        ).unwrap();
        
        // Target: <180ms
        assert!(
            response.processing_time_ms < 180.0,
            "ConversationalEngine took {:.2}ms (target <180ms)",
            response.processing_time_ms
        );
    }
}
