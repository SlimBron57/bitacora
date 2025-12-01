// src/flowpacks/response.rs
// Adaptive Response Generation
//
// AdaptiveResponse: Genera respuestas adaptadas según nivel de similitud
// 3 niveles: Reference (>0.95), PartialReference (0.85-0.95), Full (<0.85)
//
// Design: Compresión progresiva basada en similitud detectada

use serde::{Deserialize, Serialize};
use crate::flowpacks::flowpack::{FlowPack, EntryType};
use crate::flowpacks::config::FlowPackConfig;

/// Nivel de respuesta adaptativa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseLevel {
    /// Referencia a conversación anterior (similitud >0.95)
    /// Respuesta MUY corta: "Ver discusión anterior sobre X"
    /// Target: <50 tokens
    Reference,
    
    /// Referencia parcial + resumen (similitud 0.85-0.95)
    /// Respuesta corta: resumen + link a conversación anterior
    /// Target: <200 tokens
    PartialReference,
    
    /// Respuesta completa (similitud <0.85)
    /// Respuesta normal: explicación completa como si fuera primera vez
    /// Sin límite de tokens
    Full,
}

/// Respuesta adaptativa generada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveResponse {
    /// Nivel de respuesta
    pub level: ResponseLevel,
    
    /// Contenido de la respuesta
    pub content: String,
    
    /// FlowPack al que se refiere (si aplica)
    pub referenced_flowpack_id: Option<uuid::Uuid>,
    
    /// Similitud detectada (0.0-1.0)
    pub similarity: f32,
    
    /// Estimación de tokens ahorrados vs respuesta Full
    pub tokens_saved: usize,
}

impl AdaptiveResponse {
    /// Generar respuesta adaptativa basada en similitud
    pub fn generate(
        query: &str,
        similar_flowpacks: &[(FlowPack, f32)],
        config: &FlowPackConfig,
    ) -> Self {
        if similar_flowpacks.is_empty() {
            // Sin FlowPacks similares → respuesta Full
            return Self::full_response(query);
        }
        
        // Tomar FlowPack más similar
        let (most_similar_fp, similarity) = &similar_flowpacks[0];
        
        // Determinar nivel basado en similitud y thresholds
        if *similarity >= config.exact_threshold as f32 {
            Self::reference_response(query, most_similar_fp, *similarity)
        } else if *similarity >= config.similarity_threshold as f32 {
            Self::partial_reference_response(query, most_similar_fp, *similarity)
        } else {
            Self::full_response(query)
        }
    }
    
    /// Respuesta tipo Reference (similitud >0.95)
    fn reference_response(
        _query: &str,
        flowpack: &FlowPack,
        similarity: f32,
    ) -> Self {
        // Contenido: Referencia MUY corta
        let content = format!(
            "📌 Ya discutimos esto antes (similitud: {:.1}%).\n\
             Ver conversación anterior: FlowPack {} ({})\n\
             Keywords: {}",
            similarity * 100.0,
            flowpack.id,
            flowpack.created_at.format("%Y-%m-%d %H:%M"),
            flowpack.keywords.join(", "),
        );
        
        Self {
            level: ResponseLevel::Reference,
            content,
            referenced_flowpack_id: Some(flowpack.id),
            similarity,
            tokens_saved: 450, // Estimación: ~500 tokens Full - ~50 tokens Reference
        }
    }
    
    /// Respuesta tipo PartialReference (similitud 0.85-0.95)
    fn partial_reference_response(
        _query: &str,
        flowpack: &FlowPack,
        similarity: f32,
    ) -> Self {
        // Contenido: Resumen + referencia
        let summary = Self::generate_summary(flowpack);
        
        let content = format!(
            "📝 Tema relacionado con conversación anterior (similitud: {:.1}%).\n\n\
             **Resumen rápido:**\n{}\n\n\
             💡 Para más detalles ver: FlowPack {} ({})\n\
             Keywords: {}",
            similarity * 100.0,
            summary,
            flowpack.id,
            flowpack.created_at.format("%Y-%m-%d %H:%M"),
            flowpack.keywords.join(", "),
        );
        
        Self {
            level: ResponseLevel::PartialReference,
            content,
            referenced_flowpack_id: Some(flowpack.id),
            similarity,
            tokens_saved: 300, // Estimación: ~500 tokens Full - ~200 tokens Partial
        }
    }
    
    /// Respuesta tipo Full (similitud <0.85 o sin FlowPacks)
    fn full_response(query: &str) -> Self {
        let content = format!(
            "📋 Generando respuesta completa para: \"{}\"\n\n\
             [PLACEHOLDER: Aquí iría la respuesta completa generada por LLM]\n\
             \n\
             💡 Esta es una conversación nueva, no hay referencias previas.",
            query
        );
        
        Self {
            level: ResponseLevel::Full,
            content,
            referenced_flowpack_id: None,
            similarity: 0.0,
            tokens_saved: 0,
        }
    }
    
    /// Generar resumen de FlowPack (heurística simple)
    fn generate_summary(flowpack: &FlowPack) -> String {
        // Estrategia: Tomar contenido de entries con más información
        let mut summary_parts = Vec::new();
        
        for entry in &flowpack.entries {
            match entry.entry_type {
                EntryType::FullMessage => {
                    // Tomar primeras 2 líneas del mensaje completo
                    let lines: Vec<&str> = entry.content.lines().take(2).collect();
                    summary_parts.push(lines.join("\n"));
                }
                EntryType::Summary => {
                    // Ya es un resumen, usarlo completo
                    summary_parts.push(entry.content.clone());
                }
                EntryType::Reference => {
                    // Skip referencias (muy cortas)
                }
            }
        }
        
        // Limitar a 150 palabras (~200 tokens)
        let full_summary = summary_parts.join("\n\n");
        let words: Vec<&str> = full_summary.split_whitespace().collect();
        
        if words.len() <= 150 {
            full_summary
        } else {
            format!("{}...", words[..150].join(" "))
        }
    }
    
    /// Calcular ratio de compresión vs respuesta Full
    pub fn compression_ratio(&self) -> f32 {
        let full_tokens = 500.0; // Estimación de respuesta Full típica
        let actual_tokens = full_tokens - self.tokens_saved as f32;
        
        if actual_tokens > 0.0 {
            full_tokens / actual_tokens
        } else {
            1.0
        }
    }
    
    /// Verificar si respuesta es adaptativa (no Full)
    pub fn is_adaptive(&self) -> bool {
        self.level != ResponseLevel::Full
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flowpacks::flowpack::{FlowPackEntry, EntryType};
    
    fn create_test_flowpack() -> FlowPack {
        let entry = FlowPackEntry::new(
            "CTX7D es un motor multidimensional con 7 dimensiones.".to_string(),
            vec![0.5; 384],
            EntryType::FullMessage,
            None,
        );
        FlowPack::new(entry)
    }
    
    #[test]
    fn test_full_response() {
        let response = AdaptiveResponse::full_response("¿Qué es CTX7D?");
        
        assert_eq!(response.level, ResponseLevel::Full);
        assert_eq!(response.tokens_saved, 0);
        assert!(!response.is_adaptive());
    }
    
    #[test]
    fn test_reference_response() {
        let fp = create_test_flowpack();
        let response = AdaptiveResponse::reference_response(
            "¿Qué es CTX7D?",
            &fp,
            0.96,
        );
        
        assert_eq!(response.level, ResponseLevel::Reference);
        assert!(response.tokens_saved > 0);
        assert!(response.is_adaptive());
        assert!(response.content.contains("Ya discutimos esto antes"));
    }
    
    #[test]
    fn test_partial_reference_response() {
        let fp = create_test_flowpack();
        let response = AdaptiveResponse::partial_reference_response(
            "¿Qué es CTX7D?",
            &fp,
            0.88,
        );
        
        assert_eq!(response.level, ResponseLevel::PartialReference);
        assert!(response.tokens_saved > 0);
        assert!(response.is_adaptive());
        assert!(response.content.contains("Resumen rápido"));
    }
    
    #[test]
    fn test_generate_with_high_similarity() {
        let config = FlowPackConfig::default();
        let fp = create_test_flowpack();
        let similar_fps = vec![(fp, 0.96)];
        
        let response = AdaptiveResponse::generate(
            "¿Qué es CTX7D?",
            &similar_fps,
            &config,
        );
        
        assert_eq!(response.level, ResponseLevel::Reference);
    }
    
    #[test]
    fn test_generate_with_medium_similarity() {
        let config = FlowPackConfig::default();
        let fp = create_test_flowpack();
        let similar_fps = vec![(fp, 0.88)];
        
        let response = AdaptiveResponse::generate(
            "¿Qué es CTX7D?",
            &similar_fps,
            &config,
        );
        
        assert_eq!(response.level, ResponseLevel::PartialReference);
    }
    
    #[test]
    fn test_generate_with_low_similarity() {
        let config = FlowPackConfig::default();
        let fp = create_test_flowpack();
        let similar_fps = vec![(fp, 0.75)];
        
        let response = AdaptiveResponse::generate(
            "¿Qué es CTX7D?",
            &similar_fps,
            &config,
        );
        
        assert_eq!(response.level, ResponseLevel::Full);
    }
    
    #[test]
    fn test_generate_no_similar() {
        let config = FlowPackConfig::default();
        let similar_fps = vec![];
        
        let response = AdaptiveResponse::generate(
            "¿Qué es CTX7D?",
            &similar_fps,
            &config,
        );
        
        assert_eq!(response.level, ResponseLevel::Full);
        assert!(!response.is_adaptive());
    }
    
    #[test]
    fn test_compression_ratio() {
        let fp = create_test_flowpack();
        let response = AdaptiveResponse::reference_response(
            "test",
            &fp,
            0.96,
        );
        
        // Reference ahorra ~450 tokens de 500 → ratio ≈ 10x
        let ratio = response.compression_ratio();
        assert!(ratio > 5.0);
    }
    
    #[test]
    fn test_generate_summary() {
        let mut fp = create_test_flowpack();
        
        // Agregar más entries
        let entry2 = FlowPackEntry::new(
            "Las 7 dimensiones son: temporal, semántica, contextual...".to_string(),
            vec![0.5; 384],
            EntryType::FullMessage,
            None,
        );
        fp.add_entry(entry2);
        
        let summary = AdaptiveResponse::generate_summary(&fp);
        assert!(!summary.is_empty());
        assert!(summary.contains("CTX7D") || summary.contains("dimensiones"));
    }
}
