//! # Pixel Storage - Almacenamiento Visual de Información
//!
//! Implementa el sistema de encoding de información multidimensional en píxeles RGB.
//! Basado en la especificación de PIXEL_STORAGE_DEEP_DIVE.md

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::telescopedb::{Result, TelescopeDBError, ContextTensor7D, SphericalCoords};

/// Representa un píxel RGB individual
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    /// Crea un nuevo píxel
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Crea un píxel desde valores flotantes normalizados [0.0, 1.0]
    pub fn from_normalized(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    /// Convierte a valores flotantes normalizados [0.0, 1.0]
    pub fn to_normalized(&self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        )
    }

    /// Calcula la distancia euclidiana con otro píxel
    pub fn distance(&self, other: &Pixel) -> f64 {
        let dr = (self.r as f64 - other.r as f64).powi(2);
        let dg = (self.g as f64 - other.g as f64).powi(2);
        let db = (self.b as f64 - other.b as f64).powi(2);

        (dr + dg + db).sqrt()
    }
}

/// Datos de píxel con metadata temporal, espacial y semántica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelData {
    /// Píxel RGB
    pub pixel: Pixel,

    /// Coordenadas esféricas (posición en espacio 7D)
    pub coords: SphericalCoords,

    /// Timestamp de creación
    pub timestamp: DateTime<Utc>,

    /// Dimensiones 7D originales (antes de encoding)
    pub dimensions: ContextTensor7D,

    /// Metadata adicional
    pub metadata: PixelMetadata,
}

/// Metadata del píxel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelMetadata {
    /// Anchors semánticos (palabras clave)
    pub anchors: Vec<String>,

    /// Intensidad emocional [0.0, 1.0]
    pub emotional_intensity: f64,

    /// Categoría temática (técnico, personal, social, etc.)
    pub category: String,

    /// Valencia emocional (-1.0 = negativo, 0.0 = neutral, 1.0 = positivo)
    pub valence: f64,
}

impl Default for PixelMetadata {
    fn default() -> Self {
        Self {
            anchors: Vec::new(),
            emotional_intensity: 0.0,
            category: "unknown".to_string(),
            valence: 0.0,
        }
    }
}

/// Trait para almacenamiento de píxeles
pub trait PixelStore {
    /// Inserta un píxel con metadata
    fn insert(&mut self, pixel_data: PixelData) -> Result<String>;

    /// Recupera un píxel por ID
    fn get(&self, id: &str) -> Result<Option<PixelData>>;

    /// Busca píxeles por similitud visual (distancia RGB)
    fn query_by_similarity(&self, pixel: Pixel, threshold: f64) -> Result<Vec<PixelData>>;

    /// Busca píxeles en un radio esférico
    fn query_by_coords(&self, coords: SphericalCoords, radius: f64) -> Result<Vec<PixelData>>;

    /// Elimina un píxel
    fn delete(&mut self, id: &str) -> Result<()>;

    /// Obtiene el número total de píxeles
    fn len(&self) -> usize;

    /// Verifica si está vacío
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Implementación in-memory de PixelStore
pub struct InMemoryPixelStore {
    pixels: std::collections::HashMap<String, PixelData>,
}

impl InMemoryPixelStore {
    /// Crea un nuevo almacén de píxeles en memoria
    pub fn new() -> Self {
        Self {
            pixels: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryPixelStore {
    fn default() -> Self {
        Self::new()
    }
}

impl PixelStore for InMemoryPixelStore {
    fn insert(&mut self, pixel_data: PixelData) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        self.pixels.insert(id.clone(), pixel_data);
        Ok(id)
    }

    fn get(&self, id: &str) -> Result<Option<PixelData>> {
        Ok(self.pixels.get(id).cloned())
    }

    fn query_by_similarity(&self, pixel: Pixel, threshold: f64) -> Result<Vec<PixelData>> {
        let mut results: Vec<(PixelData, f64)> = self
            .pixels
            .values()
            .filter_map(|data| {
                let distance = pixel.distance(&data.pixel);
                if distance <= threshold {
                    Some((data.clone(), distance))
                } else {
                    None
                }
            })
            .collect();

        // Ordenar por distancia (más cercano primero)
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        Ok(results.into_iter().map(|(data, _)| data).collect())
    }

    fn query_by_coords(&self, coords: SphericalCoords, radius: f64) -> Result<Vec<PixelData>> {
        let mut results: Vec<(PixelData, f64)> = self
            .pixels
            .values()
            .filter_map(|data| {
                let distance = coords.distance(&data.coords);
                if distance <= radius {
                    Some((data.clone(), distance))
                } else {
                    None
                }
            })
            .collect();

        // Ordenar por distancia (más cercano primero)
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        Ok(results.into_iter().map(|(data, _)| data).collect())
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        self.pixels.remove(id);
        Ok(())
    }

    fn len(&self) -> usize {
        self.pixels.len()
    }
}

/// Conversor de dimensiones 7D a píxeles
pub struct DimensionToPixelConverter;

impl DimensionToPixelConverter {
    /// Convierte un tensor 7D a array de píxeles
    ///
    /// Estrategia: Grupos de 3 dimensiones = 1 píxel
    /// - Pixel 1: (semantic, syntactic, emotional) → RGB
    /// - Pixel 2: (intentional, contextual, biographical) → RGB
    /// - Pixel 3: (relational, padding, padding) → RGB
    pub fn convert(tensor: &ContextTensor7D) -> Vec<Pixel> {
        let dims = tensor.to_vec();
        let mut pixels = Vec::new();

        // Agrupar de 3 en 3
        for chunk in dims.chunks(3) {
            let r = chunk.get(0).copied().unwrap_or(0.0);
            let g = chunk.get(1).copied().unwrap_or(0.0);
            let b = chunk.get(2).copied().unwrap_or(0.0);

            pixels.push(Pixel::from_normalized(r, g, b));
        }

        pixels
    }

    /// Convierte un array de píxeles de vuelta a tensor 7D
    pub fn unconvert(pixels: &[Pixel]) -> Option<ContextTensor7D> {
        if pixels.is_empty() {
            return None;
        }

        let mut dims = Vec::new();

        for pixel in pixels {
            let (r, g, b) = pixel.to_normalized();
            dims.push(r);
            dims.push(g);
            dims.push(b);
        }

        // Truncar a 7 dimensiones
        dims.truncate(7);

        ContextTensor7D::from_vec(&dims)
    }

    /// Calcula la distancia promedio entre arrays de píxeles
    pub fn pixel_array_distance(a: &[Pixel], b: &[Pixel]) -> f64 {
        if a.is_empty() || b.is_empty() || a.len() != b.len() {
            return f64::MAX;
        }

        let sum: f64 = a
            .iter()
            .zip(b.iter())
            .map(|(px_a, px_b)| px_a.distance(px_b))
            .sum();

        sum / a.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_creation() {
        let pixel = Pixel::new(255, 128, 0);
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 128);
        assert_eq!(pixel.b, 0);
    }

    #[test]
    fn test_pixel_normalized() {
        let pixel = Pixel::from_normalized(1.0, 0.5, 0.0);
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 127);
        assert_eq!(pixel.b, 0);

        let (r, g, b) = pixel.to_normalized();
        assert!((r - 1.0).abs() < 0.01);
        assert!((g - 0.5).abs() < 0.01);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn test_pixel_distance() {
        let p1 = Pixel::new(255, 0, 0);
        let p2 = Pixel::new(0, 255, 0);

        let distance = p1.distance(&p2);
        assert!(distance > 0.0);

        // Distance to self should be 0
        assert_eq!(p1.distance(&p1), 0.0);
    }

    #[test]
    fn test_dimension_to_pixel_conversion() {
        let tensor = ContextTensor7D {
            semantic: 0.8,
            syntactic: 0.6,
            emotional: 0.9,
            intentional: 0.7,
            contextual: 0.5,
            biographical: 0.4,
            relational: 0.3,
        };

        let pixels = DimensionToPixelConverter::convert(&tensor);
        assert_eq!(pixels.len(), 3); // 7 dims / 3 = 3 pixels (with padding)

        // Pixel 1: semantic, syntactic, emotional
        let (r, g, b) = pixels[0].to_normalized();
        assert!((r - 0.8).abs() < 0.01);
        assert!((g - 0.6).abs() < 0.01);
        assert!((b - 0.9).abs() < 0.01);

        // Pixel 2: intentional, contextual, biographical
        let (r, g, b) = pixels[1].to_normalized();
        assert!((r - 0.7).abs() < 0.01);
        assert!((g - 0.5).abs() < 0.01);
        assert!((b - 0.4).abs() < 0.01);

        // Pixel 3: relational, padding, padding
        let (r, g, _b) = pixels[2].to_normalized();
        assert!((r - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_pixel_to_dimension_roundtrip() {
        let original = ContextTensor7D {
            semantic: 0.8,
            syntactic: 0.6,
            emotional: 0.9,
            intentional: 0.7,
            contextual: 0.5,
            biographical: 0.4,
            relational: 0.3,
        };

        let pixels = DimensionToPixelConverter::convert(&original);
        let recovered = DimensionToPixelConverter::unconvert(&pixels).unwrap();

        // Should be approximately equal (within quantization error)
        assert!((original.semantic - recovered.semantic).abs() < 0.01);
        assert!((original.syntactic - recovered.syntactic).abs() < 0.01);
        assert!((original.emotional - recovered.emotional).abs() < 0.01);
    }

    #[test]
    fn test_in_memory_pixel_store() {
        let mut store = InMemoryPixelStore::new();

        let coords = SphericalCoords::new(1.0, 0.5, 1.0).unwrap();
        let pixel_data = PixelData {
            pixel: Pixel::new(255, 128, 0),
            coords,
            timestamp: Utc::now(),
            dimensions: ContextTensor7D {
                semantic: 0.8,
                syntactic: 0.6,
                emotional: 0.9,
                intentional: 0.7,
                contextual: 0.5,
                biographical: 0.4,
                relational: 0.3,
            },
            metadata: PixelMetadata::default(),
        };

        let id = store.insert(pixel_data.clone()).unwrap();
        assert_eq!(store.len(), 1);

        let retrieved = store.get(&id).unwrap();
        assert!(retrieved.is_some());

        store.delete(&id).unwrap();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_query_by_similarity() {
        let mut store = InMemoryPixelStore::new();

        let coords = SphericalCoords::new(1.0, 0.5, 1.0).unwrap();

        // Insert similar pixels
        for i in 0..5 {
            let pixel_data = PixelData {
                pixel: Pixel::new(250 + i, 128, 0),
                coords,
                timestamp: Utc::now(),
                dimensions: ContextTensor7D {
                    semantic: 0.8,
                    syntactic: 0.6,
                    emotional: 0.9,
                    intentional: 0.7,
                    contextual: 0.5,
                    biographical: 0.4,
                    relational: 0.3,
                },
                metadata: PixelMetadata::default(),
            };
            store.insert(pixel_data).unwrap();
        }

        let query_pixel = Pixel::new(252, 128, 0);
        let results = store.query_by_similarity(query_pixel, 10.0).unwrap();

        assert!(!results.is_empty());
        assert!(results.len() <= 5);
    }
}
