//! FlowQuery - Sistema de consultas multi-documento con contexto relacional
//!
//! **Filosofía**: No documentamos archivos aislados - documentamos SISTEMAS.
//! FlowQuery permite hacer preguntas que atraviesan múltiples documentos:
//! - "¿Qué archivos dependen de este módulo?"
//! - "¿Cuál es el contexto completo para entender esta función?"
//! - "¿Qué documentos están relacionados con este issue?"
//!
//! # Arquitectura
//!
//! FlowQuery = DocumentGraph + Git History + Template Context
//! 
//! ```text
//! FlowQuery
//!   ├─ find_related() → Vec<DocumentNode> (usando DocumentGraph)
//!   ├─ get_context() → DocumentContext (contenido + dependencies + commits)
//!   ├─ trace_dependencies() → DependencyChain (import tree)
//!   └─ build_multi_doc_context() → MultiDocContext (para narrative builder)
//! ```

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use regex::Regex;

use super::document_graph::{DocumentGraph, DocumentNode, DocumentRelation, DocumentCategory, RelationType};
use super::git_integration::{GitIntegration, CommitInfo};

/// Motor de consultas multi-documento
pub struct FlowQuery {
    graph: DocumentGraph,
    root_path: PathBuf,
    
    /// Cache de imports para evitar re-parsing
    import_cache: HashMap<PathBuf, Vec<String>>,
    
    /// Integración con Git (opcional)
    git: Option<GitIntegration>,
}

impl FlowQuery {
    pub fn new(root_path: PathBuf) -> Self {
        let git = GitIntegration::new(root_path.clone()).ok();
        
        Self {
            graph: DocumentGraph::new(),
            root_path,
            import_cache: HashMap::new(),
            git,
        }
    }
    
    /// Indexar un archivo en el grafo
    pub fn index_file(&mut self, file_path: PathBuf, category: DocumentCategory) -> Result<()> {
        let content = fs::read_to_string(&file_path)
            .context(format!("Failed to read file: {:?}", file_path))?;
        
        let content_hash = format!("{:x}", md5::compute(&content));
        
        // Obtener commits de Git si está disponible
        let git_commits = if let Some(ref git) = self.git {
            git.get_file_commits(&file_path)
                .ok()
                .map(|commits| {
                    commits.into_iter()
                        .take(5) // Top 5 commits más recientes
                        .map(|c| c.short_hash)
                        .collect()
                })
                .unwrap_or_default()
        } else {
            vec![]
        };
        
        let node = DocumentNode {
            path: file_path.clone(),
            content_hash,
            git_commits,
            related: vec![],
            category,
            template_used: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.graph.add_node(node);
        
        // Detectar y agregar relaciones
        self.detect_relations(&file_path, &content)?;
        
        Ok(())
    }
    
    /// Detectar relaciones automáticamente desde el contenido
    fn detect_relations(&mut self, file_path: &PathBuf, content: &str) -> Result<()> {
        // Detectar imports en Rust
        if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let imports = self.extract_rust_imports(content);
            self.import_cache.insert(file_path.clone(), imports.clone());
            
            for import in imports {
                // Intentar resolver import a path real
                if let Some(target_path) = self.resolve_rust_import(&import) {
                    let relation = DocumentRelation::new(
                        file_path.clone(),
                        target_path,
                        RelationType::Imports,
                        format!("Imports: {}", import),
                    );
                    self.graph.add_relation(relation);
                }
            }
        }
        
        // Detectar referencias a otros archivos (markdown links, etc.)
        let references = self.extract_file_references(content);
        for ref_path in references {
            let target_path = self.root_path.join(&ref_path);
            if target_path.exists() {
                let relation = DocumentRelation::new(
                    file_path.clone(),
                    target_path,
                    RelationType::References,
                    format!("References: {}", ref_path),
                );
                self.graph.add_relation(relation);
            }
        }
        
        Ok(())
    }
    
    /// Extraer imports de Rust (use statements)
    fn extract_rust_imports(&self, content: &str) -> Vec<String> {
        let use_regex = Regex::new(r"use\s+([\w:]+)").unwrap();
        
        use_regex.captures_iter(content)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }
    
    /// Extraer referencias a otros archivos (markdown links, paths, etc.)
    fn extract_file_references(&self, content: &str) -> Vec<String> {
        let mut references = Vec::new();
        
        // Markdown links: [text](path/to/file.md)
        let md_link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        for cap in md_link_regex.captures_iter(content) {
            if let Some(path) = cap.get(2) {
                let path_str = path.as_str();
                if !path_str.starts_with("http") {
                    references.push(path_str.to_string());
                }
            }
        }
        
        references
    }
    
    /// Resolver import de Rust a path real en el filesystem
    fn resolve_rust_import(&self, import: &str) -> Option<PathBuf> {
        // Simplificado: solo resuelve imports del proyecto (crate::*)
        if import.starts_with("crate::") || import.starts_with("super::") {
            let parts: Vec<&str> = import.split("::").collect();
            
            // crate::fbcu::engine → src/fbcu/engine.rs
            if parts[0] == "crate" {
                let mut path = self.root_path.join("src");
                for part in &parts[1..] {
                    path = path.join(part);
                }
                
                // Intentar .rs o mod.rs
                let rs_path = path.with_extension("rs");
                if rs_path.exists() {
                    return Some(rs_path);
                }
                
                let mod_path = path.join("mod.rs");
                if mod_path.exists() {
                    return Some(mod_path);
                }
            }
        }
        
        None
    }
    
    /// Encontrar archivos relacionados con un path dado
    pub fn find_related(&self, file_path: &Path) -> Vec<&DocumentNode> {
        self.graph.get_related(&file_path.to_path_buf())
    }
    
    /// Obtener contexto completo para un archivo (contenido + dependencias + relacionados)
    pub fn get_context(&self, file_path: &Path) -> Result<DocumentContext> {
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file: {:?}", file_path))?;
        
        let related = self.find_related(file_path);
        let dependencies = self.get_dependencies(file_path);
        let dependents = self.get_dependents(file_path);
        
        // Obtener commits de Git
        let git_commits = if let Some(ref git) = self.git {
            git.get_file_commits(file_path)
                .ok()
                .unwrap_or_default()
        } else {
            vec![]
        };
        
        Ok(DocumentContext {
            file_path: file_path.to_path_buf(),
            content,
            related_docs: related.iter().map(|n| n.path.clone()).collect(),
            dependencies,
            dependents,
            git_commits,
        })
    }
    
    /// Obtener archivos que este archivo importa (dependencies)
    fn get_dependencies(&self, file_path: &Path) -> Vec<PathBuf> {
        self.graph.get_relations_from(&file_path.to_path_buf())
            .iter()
            .filter(|r| matches!(r.relation_type, RelationType::Imports))
            .map(|r| r.to.clone())
            .collect()
    }
    
    /// Obtener archivos que importan este archivo (dependents)
    fn get_dependents(&self, file_path: &Path) -> Vec<PathBuf> {
        self.graph.get_relations_to(&file_path.to_path_buf())
            .iter()
            .filter(|r| matches!(r.relation_type, RelationType::Imports))
            .map(|r| r.from.clone())
            .collect()
    }
    
    /// Trazar cadena de dependencias completa (dependency tree)
    pub fn trace_dependencies(&self, file_path: &Path) -> DependencyChain {
        let mut chain = DependencyChain {
            root: file_path.to_path_buf(),
            direct: Vec::new(),
            transitive: Vec::new(),
            cycle_detected: false,
        };
        
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        
        self.trace_dependencies_recursive(
            file_path,
            &mut visited,
            &mut stack,
            &mut chain,
        );
        
        chain
    }
    
    fn trace_dependencies_recursive(
        &self,
        current: &Path,
        visited: &mut HashSet<PathBuf>,
        stack: &mut Vec<PathBuf>,
        chain: &mut DependencyChain,
    ) {
        let current_path = current.to_path_buf();
        
        if stack.contains(&current_path) {
            chain.cycle_detected = true;
            return;
        }
        
        if visited.contains(&current_path) {
            return;
        }
        
        visited.insert(current_path.clone());
        stack.push(current_path.clone());
        
        let deps = self.get_dependencies(current);
        
        if current == chain.root.as_path() {
            chain.direct = deps.clone();
        } else {
            chain.transitive.extend(deps.clone());
        }
        
        for dep in deps {
            self.trace_dependencies_recursive(&dep, visited, stack, chain);
        }
        
        stack.pop();
    }
    
    /// Construir contexto multi-documento para narrative builder
    pub fn build_multi_doc_context(&self, files: &[PathBuf]) -> Result<MultiDocContext> {
        let mut contexts = Vec::new();
        
        for file in files {
            let ctx = self.get_context(file)?;
            contexts.push(ctx);
        }
        
        Ok(MultiDocContext {
            documents: contexts,
            shared_dependencies: self.find_shared_dependencies(files),
            relationship_map: self.build_relationship_map(files),
        })
    }
    
    /// Encontrar dependencias compartidas entre múltiples archivos
    fn find_shared_dependencies(&self, files: &[PathBuf]) -> Vec<PathBuf> {
        if files.is_empty() {
            return Vec::new();
        }
        
        let mut dep_counts: HashMap<PathBuf, usize> = HashMap::new();
        
        for file in files {
            let deps = self.get_dependencies(file);
            for dep in deps {
                *dep_counts.entry(dep).or_insert(0) += 1;
            }
        }
        
        // Dependencias compartidas = usadas por más de un archivo
        dep_counts.into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(path, _)| path)
            .collect()
    }
    
    /// Construir mapa de relaciones entre archivos
    fn build_relationship_map(&self, files: &[PathBuf]) -> HashMap<PathBuf, Vec<PathBuf>> {
        let mut map = HashMap::new();
        
        for file in files {
            let related = self.find_related(file)
                .iter()
                .map(|n| n.path.clone())
                .collect();
            map.insert(file.clone(), related);
        }
        
        map
    }
    
    /// Obtener el grafo completo (para debugging/visualization)
    pub fn graph(&self) -> &DocumentGraph {
        &self.graph
    }
}

/// Contexto completo de un documento
#[derive(Debug, Clone)]
pub struct DocumentContext {
    pub file_path: PathBuf,
    pub content: String,
    pub related_docs: Vec<PathBuf>,
    pub dependencies: Vec<PathBuf>,
    pub dependents: Vec<PathBuf>,
    pub git_commits: Vec<CommitInfo>,
}

/// Cadena de dependencias completa
#[derive(Debug, Clone)]
pub struct DependencyChain {
    pub root: PathBuf,
    pub direct: Vec<PathBuf>,
    pub transitive: Vec<PathBuf>,
    pub cycle_detected: bool,
}

/// Contexto multi-documento para narrative builder
#[derive(Debug, Clone)]
pub struct MultiDocContext {
    pub documents: Vec<DocumentContext>,
    pub shared_dependencies: Vec<PathBuf>,
    pub relationship_map: HashMap<PathBuf, Vec<PathBuf>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_create_flow_query() {
        let root = env::current_dir().unwrap();
        let fq = FlowQuery::new(root);
        assert_eq!(fq.import_cache.len(), 0);
    }
    
    #[test]
    fn test_extract_rust_imports() {
        let root = env::current_dir().unwrap();
        let fq = FlowQuery::new(root);
        
        let code = r#"
            use std::path::PathBuf;
            use crate::fbcu::engine;
            use super::types;
        "#;
        
        let imports = fq.extract_rust_imports(code);
        assert!(imports.contains(&"std::path::PathBuf".to_string()));
        assert!(imports.contains(&"crate::fbcu::engine".to_string()));
    }
    
    #[test]
    fn test_extract_file_references() {
        let root = env::current_dir().unwrap();
        let fq = FlowQuery::new(root);
        
        let markdown = r#"
            See [architecture](docs/architecture.md) for details.
            External link: [Google](https://google.com)
        "#;
        
        let refs = fq.extract_file_references(markdown);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0], "docs/architecture.md");
    }
}
