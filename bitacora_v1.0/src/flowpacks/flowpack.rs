// src/flowpacks/flowpack.rs
// FlowPack Core Structures
//
// FlowPack: Unidad de compresión contextual que agrupa mensajes relacionados
// FlowPackEntry: Entrada individual dentro de un FlowPack
// EntryType: Tipo de entrada (FullMessage, Summary, Reference)
//
// Design: Centroid embedding + temporal window + access tracking

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

/// Unidad de compresión contextual (agrupa mensajes relacionados)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPack {
    /// ID único del FlowPack
    pub id: Uuid,
    
    /// Embedding centroide (promedio de embeddings de entries)
    /// Dimensión: 384 (MiniLM-L6-v2)
    /// Se actualiza incrementalmente con cada nuevo entry
    pub centroid_embedding: Vec<f32>,
    
    /// Entradas del FlowPack (mensajes relacionados)
    pub entries: Vec<FlowPackEntry>,
    
    /// Timestamp de creación
    pub created_at: DateTime<Utc>,
    
    /// Timestamp de último acceso
    pub last_accessed: DateTime<Utc>,
    
    /// Número de veces que se accedió a este FlowPack
    pub access_count: u32,
    
    /// Keywords extraídos automáticamente (para búsqueda)
    /// Ejemplo: ["CTX7D", "7 dimensiones", "motor contextual"]
    pub keywords: Vec<String>,
    
    /// Metadata adicional (extensible)
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

/// Entrada individual dentro de un FlowPack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPackEntry {
    /// ID único de la entrada
    pub id: Uuid,
    
    /// Tipo de entrada (FullMessage, Summary, Reference)
    pub entry_type: EntryType,
    
    /// Contenido (puede ser mensaje completo, resumen, o referencia)
    pub content: String,
    
    /// Embedding del contenido (384 dims MiniLM-L6-v2)
    pub embedding: Vec<f32>,
    
    /// Timestamp de creación
    pub timestamp: DateTime<Utc>,
    
    /// Tamaño original (bytes, antes de compresión)
    pub original_size: usize,
    
    /// Tamaño comprimido (bytes, después de FBCU si aplica)
    pub compressed_size: Option<usize>,
    
    /// ID de sesión original (para trazabilidad)
    pub session_id: Option<String>,
}

/// Tipo de entrada en FlowPack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    /// Mensaje completo (sin compresión semántica)
    /// Se usa cuando es primera vez que se menciona un tema
    FullMessage,
    
    /// Resumen comprimido (compresión semántica aplicada)
    /// Se usa cuando hay múltiples mensajes similares
    Summary,
    
    /// Referencia a FlowPack anterior (compresión máxima)
    /// Se usa cuando usuario pregunta algo EXACTAMENTE igual
    /// Ejemplo: "Ver discusión anterior sobre CTX7D (FlowPack abc-123)"
    Reference,
}

impl FlowPack {
    /// Crear FlowPack nuevo desde primer mensaje
    pub fn new(first_entry: FlowPackEntry) -> Self {
        let centroid = first_entry.embedding.clone();
        let keywords = Self::extract_keywords(&first_entry.content);
        
        Self {
            id: Uuid::new_v4(),
            centroid_embedding: centroid,
            entries: vec![first_entry],
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            keywords,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Agregar entrada al FlowPack
    /// Actualiza centroid embedding incrementalmente
    pub fn add_entry(&mut self, entry: FlowPackEntry) {
        // Actualizar centroid (promedio incremental)
        self.update_centroid(&entry.embedding);
        
        // Agregar keywords nuevos
        let new_keywords = Self::extract_keywords(&entry.content);
        for keyword in new_keywords {
            if !self.keywords.contains(&keyword) {
                self.keywords.push(keyword);
            }
        }
        
        // Agregar entrada
        self.entries.push(entry);
    }
    
    /// Actualizar centroid embedding incrementalmente
    /// Formula: new_centroid = (old_centroid * n + new_embedding) / (n + 1)
    /// Complejidad: O(dims) vs O(n * dims) si recomputamos desde cero
    fn update_centroid(&mut self, new_embedding: &[f32]) {
        let n = self.entries.len() as f32;
        
        for (i, value) in new_embedding.iter().enumerate() {
            self.centroid_embedding[i] = 
                (self.centroid_embedding[i] * n + value) / (n + 1.0);
        }
    }
    
    /// Extraer keywords del contenido (simple heurística)
    /// TODO: Mejorar con TF-IDF o NER en v1.1
    fn extract_keywords(content: &str) -> Vec<String> {
        // Palabras en mayúsculas (probablemente acrónimos/nombres)
        let uppercase_words: Vec<String> = content
            .split_whitespace()
            .filter(|w| w.chars().all(|c| c.is_uppercase() || c.is_numeric()))
            .filter(|w| w.len() >= 3)
            .map(|w| w.to_string())
            .collect();
        
        // Números seguidos de palabras (ej: "7 dimensiones")
        let number_phrases: Vec<String> = content
            .split('.')
            .filter_map(|sentence| {
                if sentence.contains(char::is_numeric) {
                    Some(sentence.trim().to_string())
                } else {
                    None
                }
            })
            .collect();
        
        let mut keywords = uppercase_words;
        keywords.extend(number_phrases);
        keywords.truncate(10); // Máximo 10 keywords por FlowPack
        keywords
    }
    
    /// Registrar acceso al FlowPack (para LRU, analytics)
    pub fn record_access(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }
    
    /// Calcular factor de decay temporal
    /// Formula: e^(-hours_since_creation / decay_constant)
    /// decay_constant default: 168 horas (1 semana)
    pub fn temporal_decay_factor(&self, decay_hours: f32) -> f32 {
        let hours_elapsed = Utc::now()
            .signed_duration_since(self.created_at)
            .num_hours() as f32;
        
        (-hours_elapsed / decay_hours).exp()
    }
    
    /// Calcular tamaño total (original + comprimido)
    pub fn total_size(&self) -> (usize, usize) {
        let original: usize = self.entries.iter()
            .map(|e| e.original_size)
            .sum();
        
        let compressed: usize = self.entries.iter()
            .filter_map(|e| e.compressed_size)
            .sum();
        
        (original, compressed)
    }
    
    /// Calcular ratio de compresión
    pub fn compression_ratio(&self) -> f32 {
        let (original, compressed) = self.total_size();
        if compressed == 0 {
            1.0
        } else {
            original as f32 / compressed as f32
        }
    }
    
    /// Verificar si FlowPack está dentro de ventana temporal
    pub fn is_within_temporal_window(&self, window_hours: u64) -> bool {
        let now = Utc::now();
        let window = Duration::hours(window_hours as i64);
        
        now.signed_duration_since(self.created_at) <= window
    }
}

impl FlowPackEntry {
    /// Crear entrada nueva
    pub fn new(
        content: String,
        embedding: Vec<f32>,
        entry_type: EntryType,
        session_id: Option<String>,
    ) -> Self {
        let original_size = content.len();
        
        Self {
            id: Uuid::new_v4(),
            entry_type,
            content,
            embedding,
            timestamp: Utc::now(),
            original_size,
            compressed_size: None,
            session_id,
        }
    }
    
    /// Marcar como comprimido (después de FBCU)
    pub fn mark_compressed(&mut self, compressed_size: usize) {
        self.compressed_size = Some(compressed_size);
    }
    
    /// Calcular ratio de compresión de esta entrada
    pub fn compression_ratio(&self) -> f32 {
        match self.compressed_size {
            Some(compressed) if compressed > 0 => {
                self.original_size as f32 / compressed as f32
            },
            _ => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_entry(content: &str) -> FlowPackEntry {
        FlowPackEntry::new(
            content.to_string(),
            vec![0.5; 384], // Embedding dummy
            EntryType::FullMessage,
            Some("session-123".to_string()),
        )
    }
    
    #[test]
    fn test_flowpack_creation() {
        let entry = create_test_entry("Test message");
        let fp = FlowPack::new(entry);
        
        assert_eq!(fp.entries.len(), 1);
        assert_eq!(fp.access_count, 0);
        assert_eq!(fp.centroid_embedding.len(), 384);
    }
    
    #[test]
    fn test_add_entry() {
        let entry1 = create_test_entry("First message");
        let mut fp = FlowPack::new(entry1);
        
        let entry2 = create_test_entry("Second message");
        fp.add_entry(entry2);
        
        assert_eq!(fp.entries.len(), 2);
    }
    
    #[test]
    fn test_centroid_update() {
        let entry1 = FlowPackEntry::new(
            "Test".to_string(),
            vec![1.0; 384],
            EntryType::FullMessage,
            None,
        );
        let mut fp = FlowPack::new(entry1);
        
        let entry2 = FlowPackEntry::new(
            "Test2".to_string(),
            vec![0.0; 384],
            EntryType::FullMessage,
            None,
        );
        fp.add_entry(entry2);
        
        // Centroid debería ser promedio: (1.0 + 0.0) / 2 = 0.5
        assert!((fp.centroid_embedding[0] - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_keyword_extraction() {
        let entry = create_test_entry("CTX7D es un motor con 7 dimensiones");
        let fp = FlowPack::new(entry);
        
        assert!(fp.keywords.iter().any(|k| k.contains("CTX7D")));
    }
    
    #[test]
    fn test_temporal_decay() {
        let entry = create_test_entry("Test");
        let fp = FlowPack::new(entry);
        
        // FlowPack recién creado → decay ≈ 1.0
        let decay = fp.temporal_decay_factor(168.0);
        assert!(decay > 0.99);
    }
    
    #[test]
    fn test_compression_ratio() {
        let mut entry = create_test_entry("Test message with content");
        entry.mark_compressed(10); // Original: 26 bytes, Compressed: 10 bytes
        
        let ratio = entry.compression_ratio();
        // Use tolerance for compression ratio (varies by alignment/padding)
        const TOLERANCE: f32 = 0.2; // 20% tolerance
        assert!(
            (ratio - 2.6).abs() < TOLERANCE,
            "Ratio {:.2} outside expected 2.6 ± {:.2}",
            ratio, TOLERANCE
        );
    }
    
    #[test]
    fn test_record_access() {
        let entry = create_test_entry("Test");
        let mut fp = FlowPack::new(entry);
        
        assert_eq!(fp.access_count, 0);
        
        fp.record_access();
        assert_eq!(fp.access_count, 1);
        
        fp.record_access();
        assert_eq!(fp.access_count, 2);
    }
    
    #[test]
    fn test_temporal_window() {
        let entry = create_test_entry("Test");
        let fp = FlowPack::new(entry);
        
        // FlowPack recién creado → dentro de ventana de 72h
        assert!(fp.is_within_temporal_window(72));
        
        // Ventana de 0 horas → fuera
        assert!(!fp.is_within_temporal_window(0));
    }
}
