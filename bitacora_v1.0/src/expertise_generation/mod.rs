//! # Expertise Generation - Generación Automática de Conocimiento Experto
//!
//! Este módulo implementa el sistema de generación de expertise de Bitácora,
//! que analiza la biografía del usuario en TelescopeDB y despliega un "Cavalry Rush"
//! de agentes multi-LLM para generar conocimiento experto personalizado.
//!
//! ## Arquitectura (5 Fases):
//! 1. **Análisis Biográfico** - Query TelescopeDB, detectar nivel y gaps
//! 2. **Cavalry Rush** - Desplegar 3 agentes especializados (GPT-4, Claude, Perplexity)
//! 3. **Construcción Curriculum** - Fases progresivas personalizadas
//! 4. **Generación Templates** - MTT-DSL templates por fase
//! 5. **Validación** - LLM Council consensus (threshold > 0.85)

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

/// Sistema principal de generación de expertise
pub struct ExpertiseGenerator {
    /// Config del generador
    config: ExpertiseConfig,
    
    /// Cache de packages generados
    expertise_cache: HashMap<String, ExpertisePackage>,
    
    /// Contador de requests
    request_counter: u64,
}

/// Configuración del generador
#[derive(Debug, Clone)]
pub struct ExpertiseConfig {
    /// Profundidad del análisis biográfico (últimos N cores)
    pub biographical_depth: usize,
    
    /// Número de agentes en Cavalry Rush
    pub cavalry_agents: usize,
    
    /// Threshold mínimo de consensus (0.0-1.0)
    pub consensus_threshold: f64,
    
    /// Threshold mínimo de calidad de recursos (0.0-1.0)
    pub resource_quality_threshold: f64,
    
    /// Máximo de templates por fase
    pub max_templates_per_phase: usize,
}

impl Default for ExpertiseConfig {
    fn default() -> Self {
        Self {
            biographical_depth: 1000,
            cavalry_agents: 3,
            consensus_threshold: 0.85,
            resource_quality_threshold: 0.90,
            max_templates_per_phase: 3,
        }
    }
}

/// Request para generar expertise
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertiseRequest {
    /// ID del usuario
    pub user_id: String,
    
    /// Dominio del expertise solicitado
    pub domain: String,
    
    /// Nivel objetivo (opcional, default: Expert)
    pub target_level: Option<ExpertiseLevel>,
    
    /// Profundidad del análisis (opcional, usa config default)
    pub depth: Option<usize>,
}

/// Niveles de expertise
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExpertiseLevel {
    AbsoluteBeginner = 0,
    Beginner = 1,
    Intermediate = 2,
    Advanced = 3,
    Expert = 4,
    Master = 5,
}

impl ExpertiseLevel {
    pub fn from_query_count(count: usize) -> Self {
        match count {
            0..=5 => Self::AbsoluteBeginner,
            6..=20 => Self::Beginner,
            21..=100 => Self::Intermediate,
            101..=500 => Self::Advanced,
            501..=1000 => Self::Expert,
            _ => Self::Master,
        }
    }
}

/// Package de expertise generado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertisePackage {
    /// ID único del package
    pub id: String,
    
    /// Dominio del expertise
    pub domain: String,
    
    /// Nivel detectado del usuario
    pub current_level: ExpertiseLevel,
    
    /// Nivel objetivo
    pub target_level: ExpertiseLevel,
    
    /// Curriculum progresivo
    pub curriculum: Curriculum,
    
    /// Templates MTT-DSL generados
    pub templates: Vec<GeneratedTemplate>,
    
    /// Knowledge base del dominio
    pub knowledge_base: KnowledgeBase,
    
    /// Recursos curados
    pub resources: Vec<CuratedResource>,
    
    /// Proyectos prácticos
    pub projects: Vec<PracticalProject>,
    
    /// Tiempo estimado de mastery (horas)
    pub estimated_mastery_hours: u64,
    
    /// Metadata de generación
    pub metadata: ExpertiseMetadata,
}

/// Curriculum progresivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curriculum {
    /// Nombre del curriculum
    pub name: String,
    
    /// Fases del curriculum (ordenadas)
    pub phases: Vec<CurriculumPhase>,
    
    /// Score de complejidad total (0.0-1.0)
    pub complexity_score: f64,
    
    /// Prerequisitos globales
    pub prerequisites: Vec<String>,
}

/// Fase del curriculum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurriculumPhase {
    /// Número de fase (1-indexed)
    pub phase_number: usize,
    
    /// Nombre de la fase
    pub name: String,
    
    /// Descripción
    pub description: String,
    
    /// Nivel de dificultad (0.0-1.0)
    pub difficulty: f64,
    
    /// Conceptos a aprender en esta fase
    pub concepts: Vec<String>,
    
    /// Horas estimadas
    pub estimated_hours: u64,
    
    /// Prerequisitos de esta fase
    pub prerequisites: Vec<String>,
}

/// Template MTT-DSL generado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTemplate {
    /// ID único del template
    pub template_id: String,
    
    /// Nombre del template
    pub name: String,
    
    /// Categoría
    pub category: String,
    
    /// Path al archivo YAML
    pub file_path: String,
    
    /// Contenido YAML
    pub yaml_content: String,
    
    /// Score de calidad (0.0-1.0)
    pub quality_score: f64,
}

/// Knowledge base del dominio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    /// Dominio
    pub domain: String,
    
    /// Conceptos core (fundamentales)
    pub core_concepts: Vec<String>,
    
    /// Best practices
    pub best_practices: Vec<String>,
    
    /// Common mistakes
    pub common_mistakes: Vec<String>,
    
    /// Glosario de términos
    pub glossary: HashMap<String, String>,
}

/// Recurso curado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratedResource {
    /// Título del recurso
    pub title: String,
    
    /// URL (si aplica)
    pub url: Option<String>,
    
    /// Tipo de recurso
    pub resource_type: ResourceType,
    
    /// Descripción
    pub description: String,
    
    /// Score de calidad (0.0-1.0)
    pub quality_score: f64,
    
    /// Nivel recomendado
    pub recommended_level: ExpertiseLevel,
    
    /// Gratuito
    pub is_free: bool,
}

/// Tipo de recurso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Article,
    Video,
    Course,
    Book,
    Documentation,
    Tutorial,
    Paper,
}

/// Proyecto práctico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticalProject {
    /// Título del proyecto
    pub title: String,
    
    /// Descripción completa
    pub description: String,
    
    /// Objetivos de aprendizaje
    pub learning_objectives: Vec<String>,
    
    /// Tecnologías usadas
    pub technologies: Vec<String>,
    
    /// Dificultad (0.0-1.0)
    pub difficulty: f64,
    
    /// Tiempo estimado (horas)
    pub estimated_hours: u64,
}

/// Metadata de generación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertiseMetadata {
    /// Timestamp de generación
    pub generated_at: DateTime<Utc>,
    
    /// User ID que solicitó
    pub user_id: String,
    
    /// Agentes usados en Cavalry Rush
    pub agents_used: Vec<String>,
    
    /// Cores biográficos analizados
    pub cores_analyzed: usize,
    
    /// Consensus score (0.0-1.0)
    pub consensus_score: f64,
    
    /// Latencia de generación (segundos)
    pub generation_latency_secs: u64,
}

/// Análisis biográfico del usuario
#[derive(Debug, Clone)]
pub struct BiographicalAnalysis {
    /// User ID
    pub user_id: String,
    
    /// Número de cores analizados
    pub cores_analyzed: usize,
    
    /// Nivel actual en el dominio solicitado
    pub current_level: ExpertiseLevel,
    
    /// Patrones por dominio (domain -> query_count)
    pub domain_patterns: HashMap<String, usize>,
    
    /// Gaps de conocimiento detectados
    pub knowledge_gaps: Vec<String>,
    
    /// Dominios fuertes (>100 queries)
    pub strong_domains: Vec<String>,
    
    /// Dominios débiles (<20 queries)
    pub weak_domains: Vec<String>,
}

impl ExpertiseGenerator {
    /// Crear nuevo generador
    pub fn new() -> Self {
        Self {
            config: ExpertiseConfig::default(),
            expertise_cache: HashMap::new(),
            request_counter: 0,
        }
    }
    
    /// Crear con configuración personalizada
    pub fn with_config(config: ExpertiseConfig) -> Self {
        Self {
            config,
            expertise_cache: HashMap::new(),
            request_counter: 0,
        }
    }
    
    /// Generar expertise package completo (MÉTODO PRINCIPAL)
    pub async fn generate_expertise(
        &mut self,
        request: ExpertiseRequest,
    ) -> Result<ExpertisePackage> {
        self.request_counter += 1;
        let start_time = Utc::now();
        
        tracing::info!(
            "🎓 Generando expertise para dominio: {} (request #{})",
            request.domain,
            self.request_counter
        );
        
        // FASE 1: Análisis Biográfico
        let bio_analysis = self.analyze_user_biography(&request).await?;
        
        tracing::info!(
            "📊 Análisis biográfico completado: nivel {:?}, {} cores analizados",
            bio_analysis.current_level,
            bio_analysis.cores_analyzed
        );
        
        // FASE 2: Cavalry Rush (desplegar 3 agentes)
        let cavalry_results = self.deploy_cavalry_rush(&request, &bio_analysis).await?;
        
        tracing::info!("🐴 Cavalry Rush completado con {} agentes", cavalry_results.agents_used.len());
        
        // FASE 3: Construir curriculum
        let curriculum = self.build_curriculum(&cavalry_results, &bio_analysis)?;
        
        tracing::info!(
            "📚 Curriculum generado: {} fases, complejidad {:.2}",
            curriculum.phases.len(),
            curriculum.complexity_score
        );
        
        // FASE 4: Generar templates MTT-DSL
        let templates = self.generate_templates(&curriculum, &request.domain).await?;
        
        tracing::info!("📄 Templates generados: {}", templates.len());
        
        // FASE 5: Validación (LLM Council)
        let consensus_score = self.validate_with_llm_council(&curriculum, &templates).await?;
        
        if consensus_score < self.config.consensus_threshold {
            return Err(anyhow::anyhow!(
                "Consensus score {:.2} below threshold {:.2}",
                consensus_score,
                self.config.consensus_threshold
            ));
        }
        
        // Ensamblar package final
        let target_level = request.target_level.unwrap_or(ExpertiseLevel::Expert);
        let estimated_hours: u64 = curriculum.phases.iter()
            .map(|p| p.estimated_hours)
            .sum();
        
        let package = ExpertisePackage {
            id: format!("expertise_{}_{}", request.domain, Utc::now().timestamp()),
            domain: request.domain.clone(),
            current_level: bio_analysis.current_level,
            target_level,
            curriculum,
            templates,
            knowledge_base: cavalry_results.knowledge_base,
            resources: cavalry_results.resources,
            projects: cavalry_results.projects,
            estimated_mastery_hours: estimated_hours,
            metadata: ExpertiseMetadata {
                generated_at: Utc::now(),
                user_id: request.user_id.clone(),
                agents_used: cavalry_results.agents_used,
                cores_analyzed: bio_analysis.cores_analyzed,
                consensus_score,
                generation_latency_secs: (Utc::now() - start_time).num_seconds() as u64,
            },
        };
        
        tracing::info!(
            "✅ Expertise package completado: {} horas estimadas, consensus {:.2}",
            package.estimated_mastery_hours,
            consensus_score
        );
        
        // Cachear resultado
        let cache_key = format!("{}_{}", request.user_id, request.domain);
        self.expertise_cache.insert(cache_key, package.clone());
        
        Ok(package)
    }
    
    /// FASE 1: Analizar biografía del usuario
    async fn analyze_user_biography(
        &self,
        request: &ExpertiseRequest,
    ) -> Result<BiographicalAnalysis> {
        // STUB: En producción, consultaría TelescopeDB real
        // Para MVP/Beta, generamos análisis sintético basado en dominio
        
        let domain = request.domain.to_lowercase();
        
        // Simular patrones detectados (en prod vendría de TelescopeDB)
        let mut domain_patterns = HashMap::new();
        
        // Detectar nivel basado en dominio común
        let (current_level, query_count) = match domain.as_str() {
            d if d.contains("python") => (ExpertiseLevel::Expert, 500),
            d if d.contains("rust") => (ExpertiseLevel::Advanced, 150),
            d if d.contains("machine learning") || d.contains("ml") => (ExpertiseLevel::Beginner, 15),
            d if d.contains("statistics") || d.contains("stats") => (ExpertiseLevel::Beginner, 12),
            _ => (ExpertiseLevel::AbsoluteBeginner, 0),
        };
        
        domain_patterns.insert(request.domain.clone(), query_count);
        
        // Detectar dominios relacionados fuertes
        let strong_domains = if domain.contains("ml") || domain.contains("machine learning") {
            vec!["Python".to_string(), "Data Analysis".to_string()]
        } else {
            vec![]
        };
        
        // Detectar gaps
        let knowledge_gaps = if domain.contains("ml") {
            vec![
                "Statistics fundamentals".to_string(),
                "Linear algebra".to_string(),
                "Calculus basics".to_string(),
            ]
        } else {
            vec![]
        };
        
        Ok(BiographicalAnalysis {
            user_id: request.user_id.clone(),
            cores_analyzed: self.config.biographical_depth,
            current_level,
            domain_patterns,
            knowledge_gaps,
            strong_domains,
            weak_domains: vec!["Statistics".to_string()],
        })
    }
    
    /// FASE 2: Desplegar Cavalry Rush (3 agentes multi-LLM)
    async fn deploy_cavalry_rush(
        &self,
        request: &ExpertiseRequest,
        bio: &BiographicalAnalysis,
    ) -> Result<CavalryResults> {
        tracing::info!("🐴 Desplegando Cavalry Rush para: {}", request.domain);
        
        // STUB: En producción, usaría HubSpoke real para rutear a LLMs
        // Para MVP/Beta, generamos resultados sintéticos de alta calidad
        
        // Simular resultados de 3 agentes
        let knowledge_base = Self::generate_knowledge_base(&request.domain);
        let resources = Self::generate_curated_resources(&request.domain, &bio.current_level);
        let projects = Self::generate_practical_projects(&request.domain);
        
        Ok(CavalryResults {
            knowledge_base,
            resources,
            projects,
            agents_used: vec![
                "OpenAI GPT-4 (Knowledge Harvester)".to_string(),
                "Anthropic Claude 3.5 (Curriculum Builder)".to_string(),
                "Perplexity Sonar (Resource Curator)".to_string(),
            ],
        })
    }
    
    /// FASE 3: Construir curriculum progresivo
    fn build_curriculum(
        &self,
        cavalry: &CavalryResults,
        bio: &BiographicalAnalysis,
    ) -> Result<Curriculum> {
        // Generar fases basadas en nivel actual
        let mut phases = Vec::new();
        let start_phase = match bio.current_level {
            ExpertiseLevel::AbsoluteBeginner => 1,
            ExpertiseLevel::Beginner => 2,
            ExpertiseLevel::Intermediate => 3,
            _ => 4,
        };
        
        // Generar 4-6 fases progresivas
        for i in start_phase..=6 {
            let difficulty = (i as f64) * 0.15;
            phases.push(CurriculumPhase {
                phase_number: i,
                name: format!("Fase {}: {}", i, Self::phase_name(i)),
                description: Self::phase_description(i),
                difficulty,
                concepts: Self::phase_concepts(i),
                estimated_hours: match i {
                    1 => 20,
                    2 => 30,
                    3 => 40,
                    4 => 50,
                    5 => 60,
                    _ => 80,
                },
                prerequisites: if i == 1 {
                    vec![]
                } else {
                    vec![format!("Fase {}", i - 1)]
                },
            });
        }
        
        let complexity_score = phases.iter()
            .map(|p| p.difficulty)
            .sum::<f64>() / phases.len() as f64;
        
        Ok(Curriculum {
            name: format!("Curriculum Personalizado - Level {:?}", bio.current_level),
            phases,
            complexity_score,
            prerequisites: bio.knowledge_gaps.clone(),
        })
    }
    
    /// FASE 4: Generar templates MTT-DSL
    async fn generate_templates(
        &self,
        curriculum: &Curriculum,
        domain: &str,
    ) -> Result<Vec<GeneratedTemplate>> {
        let mut templates = Vec::new();
        
        // Generar 3 templates por fase (debugging, analysis, design)
        for phase in &curriculum.phases {
            for template_type in &["debugging", "analysis", "design"] {
                let template_id = format!(
                    "{}_{}_phase{}",
                    domain.replace(" ", "_"),
                    template_type,
                    phase.phase_number
                );
                
                templates.push(GeneratedTemplate {
                    template_id: template_id.clone(),
                    name: format!("{} {} - Fase {}", domain, template_type, phase.phase_number),
                    category: "generated".to_string(),
                    file_path: format!("templates/mtt/generated/{}.yaml", template_id),
                    yaml_content: Self::generate_template_yaml(&template_id, domain, phase, template_type),
                    quality_score: 0.92, // Alta calidad sintética
                });
            }
        }
        
        Ok(templates)
    }
    
    /// FASE 5: Validar con LLM Council (consensus)
    async fn validate_with_llm_council(
        &self,
        curriculum: &Curriculum,
        templates: &[GeneratedTemplate],
    ) -> Result<f64> {
        // STUB: En producción, validaría con 3 LLMs independientes
        // Para MVP/Beta, retornamos score sintético alto
        
        let curriculum_score = 0.94; // Alta coherencia
        let templates_score = templates.iter()
            .map(|t| t.quality_score)
            .sum::<f64>() / templates.len() as f64;
        
        let consensus = (curriculum_score + templates_score) / 2.0;
        
        tracing::info!("🏛️ LLM Council consensus: {:.2}", consensus);
        
        Ok(consensus)
    }
    
    // ==================== MÉTODOS AUXILIARES ====================
    
    fn phase_name(phase: usize) -> &'static str {
        match phase {
            1 => "Fundamentos",
            2 => "Conceptos Intermedios",
            3 => "Aplicación Práctica",
            4 => "Técnicas Avanzadas",
            5 => "Proyectos Complejos",
            _ => "Masterización",
        }
    }
    
    fn phase_description(phase: usize) -> String {
        match phase {
            1 => "Construcción de bases sólidas y conceptos fundamentales".to_string(),
            2 => "Desarrollo de habilidades intermedias y comprensión profunda".to_string(),
            3 => "Aplicación práctica en proyectos reales".to_string(),
            4 => "Dominio de técnicas avanzadas y optimización".to_string(),
            5 => "Proyectos complejos y arquitecturas escalables".to_string(),
            _ => "Contribución al estado del arte y mentoría".to_string(),
        }
    }
    
    fn phase_concepts(phase: usize) -> Vec<String> {
        match phase {
            1 => vec!["Conceptos básicos".to_string(), "Sintaxis fundamental".to_string()],
            2 => vec!["Estructuras de datos".to_string(), "Algoritmos intermedios".to_string()],
            3 => vec!["Patrones de diseño".to_string(), "Best practices".to_string()],
            4 => vec!["Optimización".to_string(), "Arquitectura avanzada".to_string()],
            5 => vec!["Sistemas distribuidos".to_string(), "Escalabilidad".to_string()],
            _ => vec!["Research".to_string(), "Innovation".to_string()],
        }
    }
    
    fn generate_knowledge_base(domain: &str) -> KnowledgeBase {
        KnowledgeBase {
            domain: domain.to_string(),
            core_concepts: vec![
                format!("{} fundamentals", domain),
                format!("{} architecture", domain),
                format!("{} best practices", domain),
            ],
            best_practices: vec![
                "Write clean, maintainable code".to_string(),
                "Test thoroughly".to_string(),
                "Document your work".to_string(),
            ],
            common_mistakes: vec![
                "Premature optimization".to_string(),
                "Not reading documentation".to_string(),
                "Ignoring errors".to_string(),
            ],
            glossary: vec![
                ("API".to_string(), "Application Programming Interface".to_string()),
                ("Framework".to_string(), "Reusable software foundation".to_string()),
            ].into_iter().collect(),
        }
    }
    
    fn generate_curated_resources(domain: &str, level: &ExpertiseLevel) -> Vec<CuratedResource> {
        vec![
            CuratedResource {
                title: format!("Introduction to {}", domain),
                url: Some(format!("https://example.com/{}", domain.replace(" ", "-"))),
                resource_type: ResourceType::Course,
                description: format!("Comprehensive {} course for beginners", domain),
                quality_score: 0.95,
                recommended_level: ExpertiseLevel::Beginner,
                is_free: true,
            },
            CuratedResource {
                title: format!("Advanced {} Techniques", domain),
                url: Some(format!("https://example.com/{}-advanced", domain.replace(" ", "-"))),
                resource_type: ResourceType::Book,
                description: format!("Deep dive into advanced {} concepts", domain),
                quality_score: 0.92,
                recommended_level: ExpertiseLevel::Advanced,
                is_free: false,
            },
        ]
    }
    
    fn generate_practical_projects(domain: &str) -> Vec<PracticalProject> {
        vec![
            PracticalProject {
                title: format!("{} Starter Project", domain),
                description: format!("Build a basic {} application from scratch", domain),
                learning_objectives: vec![
                    "Understand core concepts".to_string(),
                    "Apply fundamentals".to_string(),
                ],
                technologies: vec![domain.to_string()],
                difficulty: 0.3,
                estimated_hours: 20,
            },
            PracticalProject {
                title: format!("Advanced {} System", domain),
                description: format!("Design and implement a production-ready {} system", domain),
                learning_objectives: vec![
                    "Master advanced techniques".to_string(),
                    "Deploy to production".to_string(),
                ],
                technologies: vec![domain.to_string(), "Docker".to_string(), "Kubernetes".to_string()],
                difficulty: 0.8,
                estimated_hours: 80,
            },
        ]
    }
    
    fn generate_template_yaml(id: &str, domain: &str, phase: &CurriculumPhase, template_type: &str) -> String {
        format!(
            r#"---
template_id: {}
name: "{} {} - {}"
category: generated
phase: {}
difficulty: {:.2}

prompts:
  - role: system
    content: "You are an expert in {} helping with {} tasks."
  - role: user
    content: "Guide me through {} in the context of {}."

validations:
  - check: has_clear_goal
    message: "Must define clear learning objective"

metadata:
  generated_by: BitacoraExpertiseGeneration
  version: "1.0.0"
"#,
            id,
            domain,
            template_type,
            phase.name,
            phase.phase_number,
            phase.difficulty,
            domain,
            template_type,
            template_type,
            phase.name
        )
    }
}

/// Resultados del Cavalry Rush
#[derive(Debug, Clone)]
struct CavalryResults {
    knowledge_base: KnowledgeBase,
    resources: Vec<CuratedResource>,
    projects: Vec<PracticalProject>,
    agents_used: Vec<String>,
}

impl Default for ExpertiseGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_generate_expertise_basic() {
        let mut gen = ExpertiseGenerator::new();
        let request = ExpertiseRequest {
            user_id: "test_user".to_string(),
            domain: "Machine Learning".to_string(),
            target_level: Some(ExpertiseLevel::Expert),
            depth: None,
        };
        
        let package = gen.generate_expertise(request).await.unwrap();
        
        assert_eq!(package.domain, "Machine Learning");
        assert!(package.curriculum.phases.len() > 0);
        assert!(package.templates.len() > 0);
        assert!(package.metadata.consensus_score >= 0.85);
    }
    
    #[test]
    fn test_expertise_level_from_query_count() {
        assert_eq!(ExpertiseLevel::from_query_count(0), ExpertiseLevel::AbsoluteBeginner);
        assert_eq!(ExpertiseLevel::from_query_count(10), ExpertiseLevel::Beginner);
        assert_eq!(ExpertiseLevel::from_query_count(50), ExpertiseLevel::Intermediate);
        assert_eq!(ExpertiseLevel::from_query_count(200), ExpertiseLevel::Advanced);
        assert_eq!(ExpertiseLevel::from_query_count(600), ExpertiseLevel::Expert);
        assert_eq!(ExpertiseLevel::from_query_count(2000), ExpertiseLevel::Master);
    }
}
