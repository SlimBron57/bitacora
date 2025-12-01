//! # Memory Forensics - Análisis Temporal de Memoria
//!
//! Sistema de análisis temporal: snapshots, diffs, pattern detection.
//! Timeline reconstruction y state verification.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::telescopedb::{Result, TelescopeDBError, FBCUCore, SphericalCoords};

/// Evento en la línea temporal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// ID único del evento
    pub id: String,

    /// Timestamp del evento
    pub timestamp: DateTime<Utc>,

    /// Tipo de evento
    pub event_type: EventType,

    /// FBCU Core asociado (si aplica)
    pub core_id: Option<String>,

    /// Coordenadas esféricas en el momento del evento
    pub coords: SphericalCoords,

    /// Metadata del evento
    pub metadata: HashMap<String, String>,
}

/// Tipos de eventos en la timeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// Inserción de nuevo core
    Insert,
    /// Modificación de core existente
    Update,
    /// Eliminación de core
    Delete,
    /// Snapshot creado
    Snapshot,
    /// Query ejecutado
    Query,
    /// Pattern detectado
    PatternDetected,
}

/// Diferencia entre dos FBCU Cores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDiff {
    /// ID del core original
    pub old_id: String,

    /// ID del core nuevo
    pub new_id: String,

    /// Timestamp de la comparación
    pub timestamp: DateTime<Utc>,

    /// Diferencias en coordenadas
    pub coord_diff: CoordDiff,

    /// Diferencias en dimensiones
    pub dimension_diff: Vec<DimensionDiff>,

    /// Distancia total (métrica combinada)
    pub total_distance: f64,
}

/// Diferencia en coordenadas esféricas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordDiff {
    pub delta_r: f64,
    pub delta_theta: f64,
    pub delta_phi: f64,
    pub euclidean_distance: f64,
}

/// Diferencia en una dimensión específica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionDiff {
    pub dimension_name: String,
    pub old_value: f64,
    pub new_value: f64,
    pub delta: f64,
}

/// Patrón detectado en la memoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPattern {
    /// ID del patrón
    pub id: String,

    /// Tipo de patrón
    pub pattern_type: PatternType,

    /// Cores involucrados
    pub core_ids: Vec<String>,

    /// Timestamp de detección
    pub detected_at: DateTime<Utc>,

    /// Confianza del patrón [0.0, 1.0]
    pub confidence: f64,

    /// Descripción del patrón
    pub description: String,

    /// Metadata adicional
    pub metadata: HashMap<String, String>,
}

/// Tipos de patrones detectables
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    /// Clustering espacial (cores cercanos en espacio esférico)
    SpatialClustering,

    /// Secuencia temporal (eventos consecutivos relacionados)
    TemporalSequence,

    /// Oscilación (cambios cíclicos en dimensiones)
    Oscillation,

    /// Tendencia (cambio gradual en una dirección)
    Trend,

    /// Anomalía (outlier en el espacio)
    Anomaly,
}

/// Motor de análisis forense de memoria
pub struct MemoryForensics {
    /// Timeline de eventos
    timeline: Vec<TimelineEvent>,

    /// Patrones detectados
    patterns: Vec<MemoryPattern>,

    /// Configuración de detección
    config: ForensicsConfig,
}

/// Configuración de análisis forense
#[derive(Debug, Clone)]
pub struct ForensicsConfig {
    /// Umbral para clustering espacial
    pub spatial_cluster_threshold: f64,

    /// Ventana temporal para secuencias (en segundos)
    pub temporal_window_secs: i64,

    /// Umbral de confianza mínima para patrones
    pub min_pattern_confidence: f64,
}

impl Default for ForensicsConfig {
    fn default() -> Self {
        Self {
            spatial_cluster_threshold: 0.3,
            temporal_window_secs: 3600, // 1 hora
            min_pattern_confidence: 0.7,
        }
    }
}

impl MemoryForensics {
    /// Crea un nuevo analizador forense
    pub fn new(config: ForensicsConfig) -> Self {
        Self {
            timeline: Vec::new(),
            patterns: Vec::new(),
            config,
        }
    }

    /// Registra un evento en la timeline
    pub fn record_event(&mut self, event: TimelineEvent) {
        self.timeline.push(event);

        // Ordenar por timestamp
        self.timeline
            .sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    /// Calcula la diferencia entre dos cores
    pub fn compute_diff(&self, old_core: &FBCUCore, new_core: &FBCUCore) -> CoreDiff {
        // Diferencia en coordenadas
        let delta_r = new_core.coords.r - old_core.coords.r;
        let delta_theta = new_core.coords.theta - old_core.coords.theta;
        let delta_phi = new_core.coords.phi - old_core.coords.phi;
        let euclidean_distance = old_core.coords.distance(&new_core.coords);

        let coord_diff = CoordDiff {
            delta_r,
            delta_theta,
            delta_phi,
            euclidean_distance,
        };

        // Diferencias en dimensiones 7D
        let old_dims = old_core.context_tensor.to_vec();
        let new_dims = new_core.context_tensor.to_vec();

        let dimension_names = vec![
            "semantic",
            "syntactic",
            "emotional",
            "intentional",
            "contextual",
            "biographical",
            "relational",
        ];

        let dimension_diff: Vec<DimensionDiff> = old_dims
            .iter()
            .zip(new_dims.iter())
            .zip(dimension_names.iter())
            .map(|((old_val, new_val), name)| DimensionDiff {
                dimension_name: name.to_string(),
                old_value: *old_val,
                new_value: *new_val,
                delta: new_val - old_val,
            })
            .collect();

        // Distancia total (combinación de coordenadas y dimensiones)
        let dimension_distance: f64 = dimension_diff.iter().map(|d| d.delta.abs()).sum();
        let total_distance = euclidean_distance + dimension_distance;

        CoreDiff {
            old_id: old_core.id.clone(),
            new_id: new_core.id.clone(),
            timestamp: Utc::now(),
            coord_diff,
            dimension_diff,
            total_distance,
        }
    }

    /// Detecta clustering espacial en cores
    pub fn detect_spatial_clustering(&mut self, cores: &[FBCUCore]) -> Vec<MemoryPattern> {
        let mut clusters = Vec::new();

        // Algoritmo simple de clustering por umbral de distancia
        for i in 0..cores.len() {
            let mut cluster_cores = vec![cores[i].id.clone()];

            for j in (i + 1)..cores.len() {
                let distance = cores[i].coords.distance(&cores[j].coords);

                if distance <= self.config.spatial_cluster_threshold {
                    cluster_cores.push(cores[j].id.clone());
                }
            }

            // Solo considerar clusters con al menos 2 cores
            if cluster_cores.len() >= 2 {
                let pattern = MemoryPattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::SpatialClustering,
                    core_ids: cluster_cores.clone(),
                    detected_at: Utc::now(),
                    confidence: 0.85, // TODO: Calcular confianza real
                    description: format!("Spatial cluster with {} cores", cluster_cores.len()),
                    metadata: HashMap::new(),
                };

                clusters.push(pattern.clone());
                self.patterns.push(pattern);
            }
        }

        clusters
    }

    /// Detecta secuencias temporales
    pub fn detect_temporal_sequences(&mut self) -> Vec<MemoryPattern> {
        let mut sequences = Vec::new();

        // Agrupar eventos por ventana temporal
        let mut current_sequence = Vec::new();

        for i in 0..self.timeline.len() {
            if current_sequence.is_empty() {
                current_sequence.push(self.timeline[i].id.clone());
                continue;
            }

            // Verificar si está dentro de la ventana temporal
            let last_idx = current_sequence.len() - 1;
            let last_event = &self.timeline
                .iter()
                .find(|e| e.id == current_sequence[last_idx])
                .unwrap();

            let time_diff = self.timeline[i]
                .timestamp
                .signed_duration_since(last_event.timestamp)
                .num_seconds();

            if time_diff.abs() <= self.config.temporal_window_secs {
                current_sequence.push(self.timeline[i].id.clone());
            } else {
                // Nueva secuencia
                if current_sequence.len() >= 2 {
                    let pattern = MemoryPattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::TemporalSequence,
                        core_ids: current_sequence.clone(),
                        detected_at: Utc::now(),
                        confidence: 0.80,
                        description: format!(
                            "Temporal sequence with {} events",
                            current_sequence.len()
                        ),
                        metadata: HashMap::new(),
                    };

                    sequences.push(pattern.clone());
                    self.patterns.push(pattern);
                }

                current_sequence.clear();
                current_sequence.push(self.timeline[i].id.clone());
            }
        }

        sequences
    }

    /// Reconstruye la timeline entre dos timestamps
    pub fn reconstruct_timeline(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<TimelineEvent> {
        self.timeline
            .iter()
            .filter(|event| event.timestamp >= start && event.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Obtiene estadísticas de la timeline
    pub fn timeline_stats(&self) -> TimelineStats {
        let total_events = self.timeline.len();

        let mut event_type_counts: HashMap<EventType, usize> = HashMap::new();
        for event in &self.timeline {
            *event_type_counts.entry(event.event_type).or_insert(0) += 1;
        }

        let first_event = self.timeline.first().map(|e| e.timestamp);
        let last_event = self.timeline.last().map(|e| e.timestamp);

        TimelineStats {
            total_events,
            event_type_counts,
            first_event,
            last_event,
            total_patterns: self.patterns.len(),
        }
    }

    /// Obtiene todos los patrones detectados
    pub fn patterns(&self) -> &[MemoryPattern] {
        &self.patterns
    }

    /// Obtiene la timeline completa
    pub fn timeline(&self) -> &[TimelineEvent] {
        &self.timeline
    }
}

/// Estadísticas de la timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineStats {
    pub total_events: usize,
    pub event_type_counts: HashMap<EventType, usize>,
    pub first_event: Option<DateTime<Utc>>,
    pub last_event: Option<DateTime<Utc>>,
    pub total_patterns: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescopedb::ContextTensor7D;

    fn create_test_core(id: String, coords: SphericalCoords) -> FBCUCore {
        FBCUCore {
            id,
            atomic_core: crate::telescopedb::AtomicCore {
                embedding: crate::telescopedb::Embedding::empty("test".to_string()),
                anchors: Vec::new(),
                timestamp: Utc::now(),
                content: Vec::new(),
            },
            context_tensor: ContextTensor7D {
                semantic: 0.8,
                syntactic: 0.6,
                emotional: 0.9,
                intentional: 0.7,
                contextual: 0.5,
                biographical: 0.4,
                relational: 0.3,
            },
            coords,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_memory_forensics_creation() {
        let forensics = MemoryForensics::new(ForensicsConfig::default());
        assert_eq!(forensics.timeline().len(), 0);
        assert_eq!(forensics.patterns().len(), 0);
    }

    #[test]
    fn test_record_event() {
        let mut forensics = MemoryForensics::new(ForensicsConfig::default());

        let event = TimelineEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: EventType::Insert,
            core_id: Some("core-1".to_string()),
            coords: SphericalCoords::new(1.0, 0.5, 1.0).unwrap(),
            metadata: HashMap::new(),
        };

        forensics.record_event(event);
        assert_eq!(forensics.timeline().len(), 1);
    }

    #[test]
    fn test_compute_diff() {
        let forensics = MemoryForensics::new(ForensicsConfig::default());

        let coords1 = SphericalCoords::new(1.0, 0.5, 1.0).unwrap();
        let coords2 = SphericalCoords::new(1.2, 0.6, 1.1).unwrap();

        let core1 = create_test_core("core-1".to_string(), coords1);
        let core2 = create_test_core("core-2".to_string(), coords2);

        let diff = forensics.compute_diff(&core1, &core2);

        assert_eq!(diff.old_id, "core-1");
        assert_eq!(diff.new_id, "core-2");
        assert!((diff.coord_diff.delta_r - 0.2).abs() < 1e-10);
        assert_eq!(diff.dimension_diff.len(), 7);
        assert!(diff.total_distance > 0.0);
    }

    #[test]
    fn test_spatial_clustering() {
        let mut forensics = MemoryForensics::new(ForensicsConfig::default());

        // Crear cores cercanos entre sí
        let coords1 = SphericalCoords::new(1.0, 0.5, 1.0).unwrap();
        let coords2 = SphericalCoords::new(1.1, 0.52, 1.02).unwrap(); // Muy cerca de coords1
        let coords3 = SphericalCoords::new(5.0, 2.0, 2.0).unwrap(); // Lejos

        let cores = vec![
            create_test_core("core-1".to_string(), coords1),
            create_test_core("core-2".to_string(), coords2),
            create_test_core("core-3".to_string(), coords3),
        ];

        let clusters = forensics.detect_spatial_clustering(&cores);

        // Debería detectar al menos un cluster (core-1 y core-2)
        assert!(!clusters.is_empty());
        assert_eq!(clusters[0].pattern_type, PatternType::SpatialClustering);
    }

    #[test]
    fn test_timeline_reconstruction() {
        let mut forensics = MemoryForensics::new(ForensicsConfig::default());

        let now = Utc::now();
        let hour_ago = now - chrono::Duration::hours(1);
        let two_hours_ago = now - chrono::Duration::hours(2);

        let event1 = TimelineEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: two_hours_ago,
            event_type: EventType::Insert,
            core_id: None,
            coords: SphericalCoords::new(1.0, 0.5, 1.0).unwrap(),
            metadata: HashMap::new(),
        };

        let event2 = TimelineEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: hour_ago,
            event_type: EventType::Query,
            core_id: None,
            coords: SphericalCoords::new(1.0, 0.5, 1.0).unwrap(),
            metadata: HashMap::new(),
        };

        forensics.record_event(event1);
        forensics.record_event(event2);

        let reconstructed = forensics.reconstruct_timeline(two_hours_ago, now);
        assert_eq!(reconstructed.len(), 2);
    }

    #[test]
    fn test_timeline_stats() {
        let mut forensics = MemoryForensics::new(ForensicsConfig::default());

        let event1 = TimelineEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: EventType::Insert,
            core_id: None,
            coords: SphericalCoords::new(1.0, 0.5, 1.0).unwrap(),
            metadata: HashMap::new(),
        };

        let event2 = TimelineEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: EventType::Query,
            core_id: None,
            coords: SphericalCoords::new(1.0, 0.5, 1.0).unwrap(),
            metadata: HashMap::new(),
        };

        forensics.record_event(event1);
        forensics.record_event(event2);

        let stats = forensics.timeline_stats();
        assert_eq!(stats.total_events, 2);
        assert!(stats.first_event.is_some());
        assert!(stats.last_event.is_some());
    }
}
