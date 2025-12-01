//! Narrative Builder - Construcción de narrativas coherentes multi-documento
//!
//! **Filosofía**: Un sistema no es una colección de archivos - es una HISTORIA.
//! El Narrative Builder conecta múltiples documentos en una narrativa coherente
//! que explica no solo QUÉ hace el sistema, sino POR QUÉ evolucionó así.
//!
//! # Concepto
//!
//! ```text
//! Archivo A + Archivo B + Archivo C
//!     ↓ (no es suma, es narrativa)
//! "Historia coherente que conecta propósito, arquitectura, y decisiones"
//! ```
//!
//! # Capacidades
//!
//! - `build_narrative()`: Generar historia multi-documento
//! - `connect_docs()`: Identificar puntos de conexión entre documentos
//! - `build_timeline()`: Línea temporal de evolución (vía Git)
//! - `extract_decisions()`: Decisiones arquitectónicas del contexto
//! - `generate_overview()`: Vista panorámica del sistema
//!
//! # Integración
//!
//! NarrativeBuilder = FlowQuery + GitIntegration + TemplateEngine
//!
//! Usa templates para estructura, FlowQuery para relaciones, Git para evolución.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context as AnyhowContext};
use chrono::{DateTime, Utc};

use super::flow_query::{FlowQuery, DocumentContext, MultiDocContext};
use super::template_engine::{TemplateEngine, MTTTemplate};
use super::git_integration::{GitIntegration, CommitInfo};

/// Constructor de narrativas multi-documento
pub struct NarrativeBuilder {
    flow_query: FlowQuery,
    template_engine: Option<TemplateEngine>,
}

impl NarrativeBuilder {
    /// Crear nuevo narrative builder
    pub fn new(root_path: PathBuf, templates_dir: Option<PathBuf>) -> Result<Self> {
        let flow_query = FlowQuery::new(root_path);
        
        let template_engine = if let Some(dir) = templates_dir {
            Some(TemplateEngine::new(dir)?)
        } else {
            None
        };
        
        Ok(Self {
            flow_query,
            template_engine,
        })
    }
    
    /// Construir narrativa coherente para múltiples documentos
    ///
    /// Retorna markdown con:
    /// - Vista panorámica del sistema
    /// - Línea temporal de evolución
    /// - Conexiones entre documentos
    /// - Decisiones arquitectónicas clave
    pub fn build_narrative(&mut self, files: &[PathBuf]) -> Result<String> {
        let multi_ctx = self.flow_query.build_multi_doc_context(files)?;
        
        let mut narrative = String::new();
        
        // Header
        narrative.push_str("# Narrativa del Sistema\n\n");
        narrative.push_str(&format!("> Documentos analizados: {}\n", files.len()));
        narrative.push_str(&format!("> Fecha de generación: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M UTC")));
        narrative.push_str("---\n\n");
        
        // Sección 1: Vista Panorámica
        narrative.push_str(&self.generate_overview(&multi_ctx)?);
        narrative.push_str("\n---\n\n");
        
        // Sección 2: Línea Temporal
        narrative.push_str(&self.build_timeline(&multi_ctx)?);
        narrative.push_str("\n---\n\n");
        
        // Sección 3: Conexiones entre Documentos
        narrative.push_str(&self.connect_docs(&multi_ctx)?);
        narrative.push_str("\n---\n\n");
        
        // Sección 4: Decisiones Arquitectónicas
        narrative.push_str(&self.extract_decisions(&multi_ctx)?);
        narrative.push_str("\n---\n\n");
        
        // Sección 5: Dependencias Compartidas
        narrative.push_str(&self.analyze_shared_dependencies(&multi_ctx)?);
        
        Ok(narrative)
    }
    
    /// Generar vista panorámica del sistema
    fn generate_overview(&self, ctx: &MultiDocContext) -> Result<String> {
        let mut overview = String::new();
        
        overview.push_str("## 🌍 Vista Panorámica\n\n");
        overview.push_str("### Documentos en el Sistema\n\n");
        
        for (i, doc_ctx) in ctx.documents.iter().enumerate() {
            let file_name = doc_ctx.file_path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            overview.push_str(&format!("{}. **{}**\n", i + 1, file_name));
            overview.push_str(&format!("   - Path: `{}`\n", doc_ctx.file_path.display()));
            overview.push_str(&format!("   - Size: {} bytes\n", doc_ctx.content.len()));
            overview.push_str(&format!("   - Commits: {}\n", doc_ctx.git_commits.len()));
            overview.push_str(&format!("   - Dependencies: {}\n", doc_ctx.dependencies.len()));
            overview.push_str(&format!("   - Dependents: {}\n", doc_ctx.dependents.len()));
            overview.push_str("\n");
        }
        
        Ok(overview)
    }
    
    /// Construir línea temporal de evolución
    fn build_timeline(&self, ctx: &MultiDocContext) -> Result<String> {
        let mut timeline = String::new();
        
        timeline.push_str("## 📅 Línea Temporal de Evolución\n\n");
        
        // Recolectar todos los commits de todos los documentos
        let mut all_commits: Vec<(&PathBuf, &CommitInfo)> = Vec::new();
        
        for doc_ctx in &ctx.documents {
            for commit in &doc_ctx.git_commits {
                all_commits.push((&doc_ctx.file_path, commit));
            }
        }
        
        // Ordenar por timestamp (más antiguo primero para timeline cronológica)
        all_commits.sort_by(|a, b| a.1.timestamp.cmp(&b.1.timestamp));
        
        if all_commits.is_empty() {
            timeline.push_str("*No se encontraron commits Git para estos archivos*\n");
            return Ok(timeline);
        }
        
        // Agrupar por mes
        let mut current_month = String::new();
        
        for (file_path, commit) in all_commits.iter().take(20) { // Top 20 más recientes
            let month = commit.timestamp.format("%Y-%m").to_string();
            
            if month != current_month {
                current_month = month.clone();
                timeline.push_str(&format!("\n### {}\n\n", commit.timestamp.format("%B %Y")));
            }
            
            let file_name = file_path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            timeline.push_str(&format!(
                "- **{}** `{}` — {} ({})\n",
                commit.timestamp.format("%d/%m"),
                commit.short_hash,
                commit.summary,
                file_name
            ));
        }
        
        if all_commits.len() > 20 {
            timeline.push_str(&format!("\n*...y {} commits más*\n", all_commits.len() - 20));
        }
        
        Ok(timeline)
    }
    
    /// Conectar documentos identificando relaciones
    fn connect_docs(&self, ctx: &MultiDocContext) -> Result<String> {
        let mut connections = String::new();
        
        connections.push_str("## 🔗 Conexiones entre Documentos\n\n");
        
        for doc_ctx in &ctx.documents {
            let file_name = doc_ctx.file_path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            if doc_ctx.dependencies.is_empty() && doc_ctx.dependents.is_empty() {
                continue;
            }
            
            connections.push_str(&format!("### {}\n\n", file_name));
            
            if !doc_ctx.dependencies.is_empty() {
                connections.push_str("**Depende de**:\n");
                for dep in &doc_ctx.dependencies {
                    let dep_name = dep.file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");
                    connections.push_str(&format!("- `{}`\n", dep_name));
                }
                connections.push_str("\n");
            }
            
            if !doc_ctx.dependents.is_empty() {
                connections.push_str("**Usado por**:\n");
                for dependent in &doc_ctx.dependents {
                    let dep_name = dependent.file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");
                    connections.push_str(&format!("- `{}`\n", dep_name));
                }
                connections.push_str("\n");
            }
        }
        
        Ok(connections)
    }
    
    /// Extraer decisiones arquitectónicas del contexto
    fn extract_decisions(&self, ctx: &MultiDocContext) -> Result<String> {
        let mut decisions = String::new();
        
        decisions.push_str("## 💡 Decisiones Arquitectónicas\n\n");
        decisions.push_str("*Inferidas de mensajes de commit y estructura del código*\n\n");
        
        // Buscar commits con keywords de decisión: "refactor", "redesign", "breaking", "decision", "architecture"
        let decision_keywords = ["refactor", "redesign", "breaking", "decision", "architecture", "adr"];
        
        let mut decision_commits = Vec::new();
        
        for doc_ctx in &ctx.documents {
            for commit in &doc_ctx.git_commits {
                let message_lower = commit.message.to_lowercase();
                if decision_keywords.iter().any(|kw| message_lower.contains(kw)) {
                    decision_commits.push((doc_ctx, commit));
                }
            }
        }
        
        if decision_commits.is_empty() {
            decisions.push_str("*No se detectaron commits de decisión arquitectónica en este conjunto de documentos*\n");
            return Ok(decisions);
        }
        
        // Ordenar por timestamp (más reciente primero)
        decision_commits.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        
        for (doc_ctx, commit) in decision_commits.iter().take(10) {
            let file_name = doc_ctx.file_path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            decisions.push_str(&format!(
                "### {} — {} ({})\n\n",
                commit.timestamp.format("%Y-%m-%d"),
                commit.summary,
                file_name
            ));
            
            decisions.push_str(&format!("- **Commit**: `{}`\n", commit.short_hash));
            decisions.push_str(&format!("- **Autor**: {}\n", commit.author_name));
            
            // Mostrar mensaje completo si tiene más de una línea
            if commit.message.lines().count() > 1 {
                decisions.push_str(&format!("\n**Descripción**:\n```\n{}\n```\n", commit.message));
            }
            
            decisions.push_str("\n");
        }
        
        Ok(decisions)
    }
    
    /// Analizar dependencias compartidas entre documentos
    fn analyze_shared_dependencies(&self, ctx: &MultiDocContext) -> Result<String> {
        let mut analysis = String::new();
        
        analysis.push_str("## 🎯 Dependencias Compartidas\n\n");
        
        if ctx.shared_dependencies.is_empty() {
            analysis.push_str("*No se detectaron dependencias compartidas entre estos documentos*\n");
            return Ok(analysis);
        }
        
        analysis.push_str("Módulos/archivos usados por múltiples documentos en este conjunto:\n\n");
        
        for shared_dep in &ctx.shared_dependencies {
            let dep_name = shared_dep.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            // Contar cuántos documentos dependen de este
            let dependent_count = ctx.documents.iter()
                .filter(|doc| doc.dependencies.contains(shared_dep))
                .count();
            
            analysis.push_str(&format!("- **{}** (usado por {} documentos)\n", dep_name, dependent_count));
        }
        
        analysis.push_str("\n**Implicación**: Estos módulos son puntos críticos del sistema. Cambios aquí afectan múltiples componentes.\n");
        
        Ok(analysis)
    }
    
    /// Obtener FlowQuery subyacente (para acceso directo)
    pub fn flow_query(&self) -> &FlowQuery {
        &self.flow_query
    }
    
    /// Obtener FlowQuery mutable
    pub fn flow_query_mut(&mut self) -> &mut FlowQuery {
        &mut self.flow_query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use super::super::document_graph::DocumentCategory;
    
    #[test]
    fn test_create_narrative_builder() {
        let root = env::current_dir().unwrap();
        let builder = NarrativeBuilder::new(root, None);
        assert!(builder.is_ok(), "Failed to create NarrativeBuilder");
    }
    
    #[test]
    fn test_build_narrative() {
        let root = env::current_dir().unwrap();
        let mut builder = NarrativeBuilder::new(root.clone(), None).unwrap();
        
        // Indexar algunos archivos
        let file1 = root.join("src/bstradivarius/flow_query.rs");
        let file2 = root.join("src/bstradivarius/document_graph.rs");
        
        if file1.exists() && file2.exists() {
            builder.flow_query_mut().index_file(file1.clone(), DocumentCategory::Code).ok();
            builder.flow_query_mut().index_file(file2.clone(), DocumentCategory::Code).ok();
            
            let narrative = builder.build_narrative(&vec![file1, file2]);
            assert!(narrative.is_ok(), "Failed to build narrative");
            
            let narrative = narrative.unwrap();
            assert!(narrative.contains("Narrativa del Sistema"));
            assert!(narrative.contains("Vista Panorámica"));
        }
    }
}
