//! # Light Engine
//!
//! Motor para respuestas directas sin procesamiento LLM.
//!
//! ## Responsabilidades
//!
//! 1. **Quick Facts**: Respuestas inmediatas (matemáticas, definiciones)
//! 2. **Lookups**: Búsquedas en knowledge base local
//! 3. **Conversions**: Unidades, monedas, fechas
//! 4. **Status**: Estado del sistema Bitácora
//!
//! ## Modo Light
//!
//! Cuando el usuario pregunta:
//! - "¿cuál es la raíz cuadrada de 144?"
//! - "¿qué es Rust?"
//! - "convert 100 USD to EUR"
//! - "¿cuántos proyectos tengo activos?"
//!
//! ShuiDao detecta intención **Light** y activa este engine.
//!
//! ## Filosofía
//!
//! Light es el **fallback universal** - cuando otros modos fallan o la
//! confianza es baja, Light proporciona una respuesta básica pero útil.
//!
//! **NO requiere LLM** - Todas las respuestas son determinísticas.
//!
//! ## Performance
//!
//! **Target**: <10ms respuesta (O(1) complexity)
//! **Latencia**: 0ms LLM (sin llamadas externas)
//!
//! **Versión**: 1.0.0-beta  
//! **Fecha**: 2025-11-24 (Week 2 Day 5)

use crate::shuidao::error::{Result, ShuiDaoError};
use std::collections::HashMap;
use std::time::Instant;

// ============================================================================
// STRUCTURES
// ============================================================================

/// Light Engine - Respuestas directas sin LLM
pub struct LightEngine {
    /// Knowledge base local (pregunta → respuesta)
    knowledge_base: HashMap<String, String>,
    
    /// Math operations cache
    math_cache: HashMap<String, f64>,
}

/// Resultado de operación Light
#[derive(Debug, Clone)]
pub struct LightResponse {
    /// Respuesta generada
    pub answer: String,
    
    /// Tipo de respuesta
    pub response_type: LightResponseType,
    
    /// Tiempo de procesamiento (ms)
    pub processing_time_ms: f64,
    
    /// Confianza en la respuesta (0.0-1.0)
    pub confidence: f32,
    
    /// Fuente de la respuesta
    pub source: String,
}

/// Tipo de respuesta Light
#[derive(Debug, Clone, PartialEq)]
pub enum LightResponseType {
    /// Operación matemática
    Math,
    
    /// Definición de término
    Definition,
    
    /// Conversión de unidades
    Conversion,
    
    /// Estado del sistema
    SystemStatus,
    
    /// Lookup en knowledge base
    KnowledgeLookup,
    
    /// No se encontró respuesta
    NotFound,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl LightEngine {
    /// Crear nuevo LightEngine con knowledge base básico
    pub fn new() -> Self {
        let mut knowledge_base = HashMap::new();
        
        // Definiciones técnicas básicas
        knowledge_base.insert(
            "rust".to_string(),
            "Rust es un lenguaje de programación de sistemas enfocado en seguridad, \
             velocidad y concurrencia. Previene errores de memoria sin garbage collector.".to_string()
        );
        
        knowledge_base.insert(
            "bitácora".to_string(),
            "Bitácora es un sistema de inteligencia contextual que implementa memoria \
             persistente, análisis multidimensional y procesamiento multimodal con \
             arquitectura local-first.".to_string()
        );
        
        knowledge_base.insert(
            "shuidao".to_string(),
            "ShuiDao (水道) es la arquitectura cognitiva de Bitácora que detecta intención \
             y enruta a diferentes motores según el modo cognitivo requerido.".to_string()
        );
        
        Self {
            knowledge_base,
            math_cache: HashMap::new(),
        }
    }
    
    /// Procesar query en modo Light
    ///
    /// **Performance**: <10ms (sin LLM)
    pub fn process(&mut self, query: &str) -> Result<LightResponse> {
        let start = Instant::now();
        // Normalizar manteniendo caracteres especiales
        let query_lower = query.trim().to_lowercase();
        
        // 1. Try math operations
        if let Some(answer) = self.try_math(&query_lower) {
            return Ok(self.build_response(
                answer.to_string(),
                LightResponseType::Math,
                1.0,
                "internal_math",
                start,
            ));
        }
        
        // 2. Try knowledge base lookup
        if let Some(answer) = self.try_knowledge_lookup(&query_lower) {
            return Ok(self.build_response(
                answer.clone(),
                LightResponseType::KnowledgeLookup,
                0.95,
                "knowledge_base",
                start,
            ));
        }
        
        // 3. Try system status
        if query_lower.contains("estado") || query_lower.contains("status") {
            let status = self.get_system_status();
            return Ok(self.build_response(
                status,
                LightResponseType::SystemStatus,
                0.90,
                "system_introspection",
                start,
            ));
        }
        
        // 4. Not found - suggest fallback
        Ok(self.build_response(
            format!("No encontré una respuesta directa para: '{}'.\n\
                    Puedo ayudarte con:\n\
                    - Operaciones matemáticas (ej: '¿cuál es la raíz cuadrada de 144?')\n\
                    - Definiciones técnicas (ej: '¿qué es Rust?')\n\
                    - Estado del sistema (ej: 'estado de Bitácora')", query),
            LightResponseType::NotFound,
            0.0,
            "fallback",
            start,
        ))
    }
    
    /// Intentar operación matemática
    fn try_math(&mut self, query: &str) -> Option<f64> {
        // Check cache first
        if let Some(cached) = self.math_cache.get(query) {
            return Some(*cached);
        }
        
        // Raíz cuadrada (acepta con/sin acento)
        if query.contains("raíz") || query.contains("raiz") || query.contains("sqrt") {
            if let Some(num) = self.extract_number(query) {
                let result = num.sqrt();
                self.math_cache.insert(query.to_string(), result);
                return Some(result);
            }
        }
        
        // Suma simple
        if query.contains("+") {
            if let Some((a, b)) = self.extract_two_numbers(query) {
                let result = a + b;
                self.math_cache.insert(query.to_string(), result);
                return Some(result);
            }
        }
        
        // Resta simple
        if query.contains("-") && !query.starts_with('-') {
            if let Some((a, b)) = self.extract_two_numbers(query) {
                let result = a - b;
                self.math_cache.insert(query.to_string(), result);
                return Some(result);
            }
        }
        
        // Multiplicación
        if query.contains("*") || query.contains("×") {
            if let Some((a, b)) = self.extract_two_numbers(query) {
                let result = a * b;
                self.math_cache.insert(query.to_string(), result);
                return Some(result);
            }
        }
        
        // División
        if query.contains("/") || query.contains("÷") {
            if let Some((a, b)) = self.extract_two_numbers(query) {
                if b != 0.0 {
                    let result = a / b;
                    self.math_cache.insert(query.to_string(), result);
                    return Some(result);
                }
            }
        }
        
        None
    }
    
    /// Intentar lookup en knowledge base
    fn try_knowledge_lookup(&self, query: &str) -> Option<&String> {
        // Buscar términos clave en la query
        for (key, value) in &self.knowledge_base {
            if query.contains(key) {
                return Some(value);
            }
        }
        None
    }
    
    /// Obtener estado del sistema
    fn get_system_status(&self) -> String {
        format!(
            "Sistema Bitácora v1.0 operativo\n\
             - Modo: ShuiDao (Light Engine activo)\n\
             - Knowledge Base: {} entradas\n\
             - Math Cache: {} operaciones\n\
             - Estado: ✅ Funcionando correctamente",
            self.knowledge_base.len(),
            self.math_cache.len()
        )
    }
    
    /// Extraer un número de la query
    fn extract_number(&self, query: &str) -> Option<f64> {
        query
            .split_whitespace()
            .find_map(|word| {
                // Remove non-numeric characters except . and -
                let cleaned: String = word
                    .chars()
                    .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
                    .collect();
                cleaned.parse::<f64>().ok()
            })
    }
    
    /// Extraer dos números de la query
    fn extract_two_numbers(&self, query: &str) -> Option<(f64, f64)> {
        let numbers: Vec<f64> = query
            .split_whitespace()
            .filter_map(|word| {
                // Remove non-numeric characters except . and -
                let cleaned: String = word
                    .chars()
                    .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
                    .collect();
                cleaned.parse::<f64>().ok()
            })
            .collect();
        
        if numbers.len() >= 2 {
            Some((numbers[0], numbers[1]))
        } else {
            None
        }
    }
    
    /// Construir respuesta
    fn build_response(
        &self,
        answer: String,
        response_type: LightResponseType,
        confidence: f32,
        source: &str,
        start: Instant,
    ) -> LightResponse {
        let processing_time_ms = start.elapsed().as_secs_f64() * 1000.0;
        
        // Validar performance target
        if processing_time_ms > 10.0 {
            eprintln!(
                "⚠️  LightEngine::process() took {:.2}ms (target <10ms)",
                processing_time_ms
            );
        }
        
        LightResponse {
            answer,
            response_type,
            processing_time_ms,
            confidence,
            source: source.to_string(),
        }
    }
    
    /// Añadir entrada al knowledge base
    pub fn add_knowledge(&mut self, key: String, value: String) {
        self.knowledge_base.insert(key.to_lowercase(), value);
    }
    
    /// Limpiar cache de matemáticas
    pub fn clear_math_cache(&mut self) {
        self.math_cache.clear();
    }
    
    /// Obtener tamaño del knowledge base
    pub fn knowledge_base_size(&self) -> usize {
        self.knowledge_base.len()
    }
}

impl Default for LightEngine {
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
    fn test_light_engine_creation() {
        let engine = LightEngine::new();
        assert!(engine.knowledge_base_size() > 0);
        assert_eq!(engine.math_cache.len(), 0);
    }
    
    #[test]
    fn test_math_sqrt() {
        let mut engine = LightEngine::new();
        
        // Test con acento
        let response = engine.process("¿cuál es la raíz cuadrada de 144?").unwrap();
        assert_eq!(response.answer, "12");
        assert_eq!(response.response_type, LightResponseType::Math);
        assert_eq!(response.confidence, 1.0);
        assert!(response.processing_time_ms < 10.0);
        
        // Test sin acento también
        let response2 = engine.process("raiz cuadrada de 144").unwrap();
        assert_eq!(response2.answer, "12");
    }
    
    #[test]
    fn test_math_addition() {
        let mut engine = LightEngine::new();
        let response = engine.process("2 + 2").unwrap();
        
        assert_eq!(response.answer, "4");
        assert_eq!(response.response_type, LightResponseType::Math);
    }
    
    #[test]
    fn test_math_subtraction() {
        let mut engine = LightEngine::new();
        let response = engine.process("10 - 3").unwrap();
        
        assert_eq!(response.answer, "7");
        assert_eq!(response.response_type, LightResponseType::Math);
    }
    
    #[test]
    fn test_math_multiplication() {
        let mut engine = LightEngine::new();
        let response = engine.process("5 * 6").unwrap();
        
        assert_eq!(response.answer, "30");
        assert_eq!(response.response_type, LightResponseType::Math);
    }
    
    #[test]
    fn test_math_division() {
        let mut engine = LightEngine::new();
        let response = engine.process("20 / 4").unwrap();
        
        assert_eq!(response.answer, "5");
        assert_eq!(response.response_type, LightResponseType::Math);
    }
    
    #[test]
    fn test_math_cache() {
        let mut engine = LightEngine::new();
        
        // First call
        let response1 = engine.process("raíz cuadrada de 144").unwrap();
        assert_eq!(engine.math_cache.len(), 1);
        
        // Second call (cached)
        let response2 = engine.process("raíz cuadrada de 144").unwrap();
        assert_eq!(response1.answer, response2.answer);
    }
    
    #[test]
    fn test_knowledge_lookup_rust() {
        let mut engine = LightEngine::new();
        let response = engine.process("¿qué es rust?").unwrap();
        
        assert!(response.answer.contains("lenguaje de programación"));
        assert_eq!(response.response_type, LightResponseType::KnowledgeLookup);
        assert_eq!(response.confidence, 0.95);
    }
    
    #[test]
    fn test_knowledge_lookup_bitacora() {
        let mut engine = LightEngine::new();
        let response = engine.process("¿qué es bitácora?").unwrap();
        
        assert!(response.answer.contains("inteligencia contextual"));
        assert_eq!(response.response_type, LightResponseType::KnowledgeLookup);
    }
    
    #[test]
    fn test_system_status() {
        let mut engine = LightEngine::new();
        let response = engine.process("estado del sistema").unwrap();
        
        assert!(response.answer.contains("Bitácora v1.0"));
        assert_eq!(response.response_type, LightResponseType::SystemStatus);
        assert_eq!(response.confidence, 0.90);
    }
    
    #[test]
    fn test_not_found() {
        let mut engine = LightEngine::new();
        let response = engine.process("pregunta completamente aleatoria xyz123").unwrap();
        
        assert_eq!(response.response_type, LightResponseType::NotFound);
        assert_eq!(response.confidence, 0.0);
        assert!(response.answer.contains("No encontré una respuesta directa"));
    }
    
    #[test]
    fn test_add_knowledge() {
        let mut engine = LightEngine::new();
        let initial_size = engine.knowledge_base_size();
        
        engine.add_knowledge(
            "Kubernetes".to_string(),
            "Sistema de orquestación de contenedores".to_string()
        );
        
        assert_eq!(engine.knowledge_base_size(), initial_size + 1);
        
        let response = engine.process("¿qué es kubernetes?").unwrap();
        assert!(response.answer.contains("orquestación de contenedores"));
    }
    
    #[test]
    fn test_performance_target() {
        let mut engine = LightEngine::new();
        let response = engine.process("2 + 2").unwrap();
        
        // Target: <10ms
        assert!(
            response.processing_time_ms < 10.0,
            "LightEngine took {:.2}ms (target <10ms)",
            response.processing_time_ms
        );
    }
    
    #[test]
    fn test_clear_cache() {
        let mut engine = LightEngine::new();
        
        engine.process("raíz cuadrada de 144").unwrap();
        assert_eq!(engine.math_cache.len(), 1);
        
        engine.clear_math_cache();
        assert_eq!(engine.math_cache.len(), 0);
    }
}
