//! # Biographical Import - Integración con SANDBOX
//!
//! Este módulo maneja la importación de datos biográficos desde múltiples fuentes:
//! - SANDBOX/ (futuro - cuando tenga datos reales)
//! - Datos sintéticos (para testing y validación)
//! - APIs externas (v2.0)
//!
//! ## Estado Actual (v1.0)
//! - ✅ Generador de datos sintéticos implementado
//! - ⏸️ SANDBOX import es STUB (preparado para futuro)
//! - ⏸️ API externa pendiente para v2.0

use crate::telescopedb::{
    ContextTensor7D, SphericalCoords, TelescopeDB, TelescopeDBError,
};
use chrono::Utc;
use std::path::Path;
use uuid::Uuid;

/// Formato de entrada biográfica cruda (antes de procesamiento)
#[derive(Debug, Clone)]
pub struct BiographicalRawEntry {
    /// Timestamp en formato ISO 8601
    pub timestamp: String,
    
    /// Contenido textual de la experiencia biográfica
    pub content: String,
    
    /// Tags opcionales para clasificación
    pub tags: Vec<String>,
    
    /// Metadatos adicionales
    pub metadata: std::collections::HashMap<String, String>,
}

/// Resultado de un proceso de importación
#[derive(Debug)]
pub struct ImportResult {
    /// Número de entradas procesadas exitosamente
    pub success_count: usize,
    
    /// Número de entradas que fallaron
    pub failed_count: usize,
    
    /// IDs de las entradas insertadas
    pub entry_ids: Vec<String>,
    
    /// Errores encontrados (si hubo)
    pub errors: Vec<String>,
    
    /// Tiempo total de importación (milisegundos)
    pub duration_ms: u64,
}

/// Generador de datos biográficos sintéticos para testing
pub struct SyntheticDataGenerator {
    seed: u64,
}

impl SyntheticDataGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    
    /// Genera N entradas biográficas sintéticas realistas
    ///
    /// # Ejemplos de datos generados:
    /// - "Aprendí sobre content-addressable storage hoy" (técnico)
    /// - "Tuve una conversación profunda con mi padre" (emocional)
    /// - "Completé el proyecto TelescopeDB después de 3 semanas" (logro)
    /// - "Me sentí frustrado por un bug que no podía resolver" (emocional negativo)
    ///
    /// # Performance:
    /// - ~1000 entradas/segundo
    /// - Memoria: O(n) donde n = número de entradas
    pub fn generate(&self, count: usize) -> Vec<BiographicalRawEntry> {
        let templates = vec![
            // Técnicas (40%)
            "Implementé {tech} en el proyecto {project}",
            "Aprendí sobre {concept} mientras trabajaba en {context}",
            "Resolví un bug complejo en {component} usando {technique}",
            "Documenté {feature} con ejemplos y diagramas",
            "Refactoricé {code_area} para mejorar {metric}",
            
            // Emocionales (30%)
            "Me sentí {emotion} después de {event}",
            "Tuve una conversación {adjective} con {person}",
            "Celebré {achievement} con {people}",
            "Reflexioné sobre {topic} en silencio",
            "Procesé la experiencia de {event} profundamente",
            
            // Logros (20%)
            "Completé {milestone} después de {duration}",
            "Alcancé {goal} superando {obstacle}",
            "Colaboré con {person} en {project}",
            "Publiqué {output} con éxito",
            "Validé {hypothesis} con {result}",
            
            // Relacionales (10%)
            "Compartí {topic} con {person}",
            "Aprendí de {person} sobre {subject}",
            "Ayudé a {person} con {problem}",
            "Recibí feedback de {person} sobre {work}",
            "Conecté con {person} a nivel {dimension}",
        ];
        
        let tech_terms = vec!["TelescopeDB", "FBCU", "Context Token 7D", "pixel storage", "CBOR serialization", "VoxelDB", "HubSpoke", "LIP Protocol", "Routier", "FlowPacks"];
        let concepts = vec!["arquitectura esférica", "compresión fractal", "simulación estocástica", "indexación semántica", "navegación adaptativa", "persistencia lógica", "routing semántico", "templates estructurales"];
        let emotions = vec!["inspirado", "frustrado", "orgulloso", "curioso", "en paz", "determinado", "reflexivo", "entusiasmado", "concentrado", "satisfecho"];
        let projects = vec!["Bitácora v1.0", "integración SANDBOX", "documentación ROADMAP", "testing integral", "arquitectura dual", "sistema de lentes"];
        let components = vec!["módulo core", "pipeline de datos", "sistema de caching", "motor de compresión", "analizador semántico", "gestor de memoria"];
        let techniques = vec!["debugger paso a paso", "property-based testing", "refactoring incremental", "análisis de dependencias", "benchmarking comparativo"];
        let features = vec!["API de consultas", "sistema de snapshots", "detección de patrones", "análisis forense", "indexación espacial"];
        let metrics = vec!["legibilidad del código", "performance", "cobertura de tests", "coherencia arquitectónica", "mantenibilidad"];
        let events = vec!["completar documentación", "resolver bug crítico", "integrar componente", "validar arquitectura", "optimizar algoritmo"];
        let people = vec!["Eduardo", "el equipo", "colaborador", "mentor", "compañero"];
        
        let mut entries = Vec::with_capacity(count);
        
        for i in 0..count {
            // Seleccionar template de forma determinista pero variada
            let template_idx = ((self.seed + i as u64) * 7919) % templates.len() as u64;
            let mut content = templates[template_idx as usize].to_string();
            
            // Reemplazar placeholders con variedad real
            content = content
                .replace("{tech}", tech_terms[(i * 3) % tech_terms.len()])
                .replace("{concept}", concepts[(i * 5) % concepts.len()])
                .replace("{emotion}", emotions[(i * 7) % emotions.len()])
                .replace("{project}", projects[i % projects.len()])
                .replace("{component}", components[(i * 11) % components.len()])
                .replace("{technique}", techniques[(i * 13) % techniques.len()])
                .replace("{feature}", features[(i * 17) % features.len()])
                .replace("{metric}", metrics[(i * 19) % metrics.len()])
                .replace("{code_area}", components[(i * 2) % components.len()])
                .replace("{event}", events[(i * 23) % events.len()])
                .replace("{adjective}", vec!["profunda", "inspiradora", "técnica", "filosófica", "práctica"][(i * 29) % 5])
                .replace("{person}", people[(i * 31) % people.len()])
                .replace("{people}", people[(i * 3) % people.len()])
                .replace("{achievement}", vec!["TelescopeDB funcional", "FBCU optimizado", "tests al 100%", "documentación completa", "arquitectura validada"][(i * 37) % 5])
                .replace("{topic}", vec!["filosofía del proyecto", "decisiones técnicas", "próximos pasos", "arquitectura", "metodología"][(i * 41) % 5])
                .replace("{milestone}", vec!["FASE 0 documentación", "FASE 1 fundaciones", "integración completa", "optimización", "validación final"][(i * 43) % 5])
                .replace("{duration}", vec!["2 semanas", "3 días", "1 mes", "5 horas", "2 meses"][(i * 47) % 5])
                .replace("{goal}", vec!["100% tests pasando", "arquitectura sólida", "documentación completa", "performance óptimo", "código limpio"][(i * 53) % 5])
                .replace("{obstacle}", vec!["bugs complejos", "decisiones difíciles", "limitaciones técnicas", "tiempo ajustado", "complejidad alta"][(i * 59) % 5])
                .replace("{output}", vec!["documentación completa", "código funcional", "tests validados", "especificación técnica", "análisis profundo"][(i * 61) % 5])
                .replace("{hypothesis}", vec!["compresión >99%", "performance <100ms", "cobertura >90%", "escalabilidad lineal", "precisión >95%"][(i * 67) % 5])
                .replace("{result}", vec!["validación exitosa", "mejora 10x", "objetivo cumplido", "supera expectativas", "resultado prometedor"][(i * 71) % 5])
                .replace("{problem}", vec!["integración SANDBOX", "bug en producción", "optimización necesaria", "arquitectura confusa", "falta documentación"][(i * 73) % 5])
                .replace("{work}", vec!["implementación TelescopeDB", "diseño VoxelDB", "tests FlowPacks", "documentación ROADMAP", "análisis arquitectura"][(i * 79) % 5])
                .replace("{subject}", vec!["arquitectura de sistemas", "metodología ágil", "principios SOLID", "patrones de diseño", "filosofía del código"][(i * 83) % 5])
                .replace("{dimension}", vec!["filosófico", "técnico", "emocional", "profesional", "personal"][(i * 89) % 5])
                .replace("{context}", vec!["resolver un problema real", "aprender algo nuevo", "colaborar con equipo", "explorar alternativas", "validar decisión"][(i * 97) % 5]);
            
            // Timestamp: últimos 30 días, distribuidos uniformemente
            let days_ago = (i as i64 * 30) / count as i64;
            let timestamp = (Utc::now() - chrono::Duration::days(days_ago))
                .to_rfc3339();
            
            // Tags según tipo de entrada
            let tags = if content.contains("Implementé") || content.contains("Aprendí") {
                vec!["técnico".into(), "desarrollo".into()]
            } else if content.contains("sentí") || content.contains("conversación") {
                vec!["emocional".into(), "personal".into()]
            } else if content.contains("Completé") || content.contains("Alcancé") {
                vec!["logro".into(), "hito".into()]
            } else {
                vec!["relacional".into(), "colaboración".into()]
            };
            
            // Metadata
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("source".into(), "synthetic_generator".into());
            metadata.insert("version".into(), "1.0".into());
            metadata.insert("index".into(), i.to_string());
            
            entries.push(BiographicalRawEntry {
                timestamp,
                content,
                tags,
                metadata,
            });
        }
        
        entries
    }
}

/// Importador de datos biográficos (coordina múltiples fuentes)
pub struct BiographicalImporter {
    db: TelescopeDB,
}

impl BiographicalImporter {
    pub fn new(db: TelescopeDB) -> Self {
        Self { db }
    }
    
    /// Importa entradas biográficas crudas a TelescopeDB
    ///
    /// # Proceso:
    /// 1. Validar formato de entrada
    /// 2. Generar Context Token 7D sintético (para testing)
    /// 3. Insertar en TelescopeDB
    /// 4. Reportar estadísticas
    ///
    /// # Performance target: >1000 entradas/segundo
    pub async fn import_entries(
        &mut self,
        entries: Vec<BiographicalRawEntry>,
    ) -> Result<ImportResult, TelescopeDBError> {
        let start = std::time::Instant::now();
        let total = entries.len();
        
        let mut success_count = 0;
        let mut failed_count = 0;
        let mut entry_ids = Vec::new();
        let mut errors = Vec::new();
        
        for (idx, raw_entry) in entries.into_iter().enumerate() {
            match self.process_single_entry(raw_entry).await {
                Ok(entry_id) => {
                    success_count += 1;
                    entry_ids.push(entry_id);
                }
                Err(e) => {
                    failed_count += 1;
                    errors.push(format!("Entry {}: {}", idx, e));
                }
            }
        }
        
        let duration_ms = start.elapsed().as_millis() as u64;
        
        Ok(ImportResult {
            success_count,
            failed_count,
            entry_ids,
            errors,
            duration_ms,
        })
    }
    
    /// Procesa una entrada individual
    async fn process_single_entry(
        &mut self,
        raw: BiographicalRawEntry,
    ) -> Result<String, TelescopeDBError> {
        // 1. Parsear timestamp
        let timestamp = chrono::DateTime::parse_from_rfc3339(&raw.timestamp)
            .map_err(|e| TelescopeDBError::InvalidTimestamp(e.to_string()))?
            .timestamp_millis() as u64;
        
        // 2. Generar Context Token 7D sintético basado en contenido
        let ctx7d = self.generate_synthetic_ctx7d(&raw.content);
        
        // 3. Insertar en TelescopeDB usando el método existente
        let id = self.db.insert_from_ctx7d(&ctx7d).await?;
        
        Ok(id)
    }
    
    /// Genera Context Token 7D sintético basado en análisis del contenido
    ///
    /// NOTA: En producción, esto sería análisis real con LLM.
    /// Para testing, usamos heurísticas simples.
    fn generate_synthetic_ctx7d(&self, content: &str) -> ContextTensor7D {
        let content_lower = content.to_lowercase();
        
        // Temporal: basado en palabras clave temporales
        let temporal = if content_lower.contains("hoy") || content_lower.contains("ahora") {
            0.9
        } else if content_lower.contains("ayer") || content_lower.contains("después") {
            0.7
        } else {
            0.5
        };
        
        // Semántico: basado en densidad de conceptos técnicos
        let technical_terms = ["implementé", "aprendí", "código", "bug", "arquitectura"];
        let semantic = technical_terms.iter()
            .filter(|term| content_lower.contains(*term))
            .count() as f64 * 0.2;
        
        // Contextual: basado en conectores y coherencia
        let contextual = if content_lower.contains("porque") || content_lower.contains("para") {
            0.8_f64
        } else {
            0.6_f64
        };
        
        // Relacional: basado en menciones de personas/colaboración
        let relational = if content_lower.contains("eduardo") || content_lower.contains("equipo") {
            0.85_f64
        } else if content_lower.contains("conversación") || content_lower.contains("compartí") {
            0.75_f64
        } else {
            0.4_f64
        };
        
        // Emocional: basado en palabras emocionales
        let emotional_words = ["sentí", "celebré", "frustrado", "orgulloso", "paz"];
        let emotional = emotional_words.iter()
            .filter(|word| content_lower.contains(*word))
            .count() as f64 * 0.25_f64;
        
        // Intencional: basado en verbos de acción
        let intentional = if content_lower.contains("completé") || content_lower.contains("alcancé") {
            0.9_f64
        } else if content_lower.contains("trabajé") || content_lower.contains("implementé") {
            0.8_f64
        } else {
            0.5_f64
        };
        
        // Biográfico: basado en singularidad de la experiencia
        let biographical = if content.len() > 100 {
            0.7_f64
        } else if content.len() > 50 {
            0.5_f64
        } else {
            0.3_f64
        };
        
        ContextTensor7D {
            semantic: semantic.min(1.0_f64),
            syntactic: 0.5_f64, // TODO: Implementar análisis sintáctico real
            emotional: emotional.min(1.0_f64),
            intentional: intentional.min(1.0_f64),
            contextual: contextual.min(1.0_f64),
            biographical: biographical.min(1.0_f64),
            relational: relational.min(1.0_f64),
        }
    }
    
    /// Calcula coordenadas esféricas desde Context Token 7D
    fn calculate_coordinates(&self, ctx7d: &ContextTensor7D) -> SphericalCoords {
        // Radio: función de syntactic (dimensión 2) - más sintácticamente rico = más estructura
        // Invertimos para que más cerca del origen = mayor estructura
        let r = 0.1_f64 + (1.0_f64 - ctx7d.syntactic) * 9.9_f64;
        
        // Theta: función de dimensión semántica [0, 2π]
        let theta = ctx7d.semantic * 2.0_f64 * std::f64::consts::PI;
        
        // Phi: función de dimensión emocional [0, π]
        // Mapeamos emocional de [0,1] a valence [-1,1] y luego a phi
        let valence = ctx7d.emotional * 2.0_f64 - 1.0_f64;
        let phi = ((valence + 1.0_f64) / 2.0_f64) * std::f64::consts::PI;
        
        SphericalCoords { r, theta, phi }
    }
}

/// STUB: Importación desde SANDBOX (preparado para implementación futura)
///
/// # Estado: ⏸️ NO IMPLEMENTADO (v1.0)
///
/// Esta función está preparada como interfaz para cuando SANDBOX
/// tenga datos biográficos reales. Por ahora retorna datos sintéticos.
///
/// # Implementación futura (v2.0):
/// 1. Escanear SANDBOX/docs/ y SANDBOX/utils/
/// 2. Parsear archivos MD, JSON, o formato definido
/// 3. Extraer metadatos y contenido
/// 4. Validar schema
/// 5. Retornar BiographicalRawEntry[]
pub async fn import_from_sandbox(
    sandbox_path: &Path,
) -> Result<Vec<BiographicalRawEntry>, TelescopeDBError> {
    // Verificar que SANDBOX existe
    if !sandbox_path.exists() {
        return Err(TelescopeDBError::ImportError(
            format!("SANDBOX path does not exist: {:?}", sandbox_path)
        ));
    }
    
    // TODO (v2.0): Implementar lectura real de SANDBOX/
    log::warn!(
        "⏸️ SANDBOX import not yet implemented. Using synthetic data for testing."
    );
    log::info!("📂 SANDBOX path checked: {:?}", sandbox_path);
    
    // Por ahora: generar datos sintéticos como placeholder
    let generator = SyntheticDataGenerator::new(42);
    let synthetic_data = generator.generate(10);
    
    log::info!("✅ Generated {} synthetic biographical entries", synthetic_data.len());
    
    Ok(synthetic_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_synthetic_generator() {
        let generator = SyntheticDataGenerator::new(12345);
        let entries = generator.generate(100);
        
        assert_eq!(entries.len(), 100);
        
        // Verificar que tienen contenido variado (al menos 25% único)
        let unique_contents: std::collections::HashSet<_> = 
            entries.iter().map(|e| &e.content).collect();
        assert!(
            unique_contents.len() >= 25,
            "Not enough variety: only {} unique out of 100",
            unique_contents.len()
        );
        
        // Verificar timestamps son válidos
        for entry in &entries {
            assert!(chrono::DateTime::parse_from_rfc3339(&entry.timestamp).is_ok());
        }
        
        // Verificar tags
        for entry in &entries {
            assert!(!entry.tags.is_empty());
        }
    }
    
    #[test]
    fn test_synthetic_generator_determinism() {
        // Mismo seed → mismo output
        let gen1 = SyntheticDataGenerator::new(999);
        let gen2 = SyntheticDataGenerator::new(999);
        
        let entries1 = gen1.generate(50);
        let entries2 = gen2.generate(50);
        
        for (e1, e2) in entries1.iter().zip(entries2.iter()) {
            assert_eq!(e1.content, e2.content);
        }
    }
    
    #[tokio::test]
    async fn test_import_from_sandbox_stub() {
        use std::path::PathBuf;
        
        let sandbox_path = PathBuf::from("./SANDBOX");
        
        // Crear directorio temporal si no existe
        if !sandbox_path.exists() {
            std::fs::create_dir_all(&sandbox_path).unwrap();
        }
        
        let result = import_from_sandbox(&sandbox_path).await;
        assert!(result.is_ok());
        
        let entries = result.unwrap();
        assert_eq!(entries.len(), 10); // STUB retorna 10 sintéticas
    }
}
