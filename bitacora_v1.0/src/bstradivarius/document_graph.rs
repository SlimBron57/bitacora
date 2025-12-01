//! Document Graph - Relaciones entre documentos del proyecto
//!
//! Mantiene un grafo de documentos con:
//! - Nodos: Archivos del proyecto (.md, .rs, .toml)
//! - Edges: Relaciones (imports, references, decisions)
//! - Metadata: Git commits, categorías, templates usados
//!
//! # Filosofía
//!
//! El DocumentGraph no es solo un índice de archivos.
//! Es un **mapa de conocimiento** que entiende cómo los documentos
//! se relacionan entre sí. Permite consultas como:
//!
//! - "¿Qué documentos implementan este diseño arquitectónico?"
//! - "¿Qué sesiones tomaron decisiones sobre este módulo?"
//! - "¿Cuál es la historia evolutiva de este componente?"
//!
//! # Ejemplo
//!
//! ```rust,ignore
//! use bstradivarius::document_graph::{DocumentGraph, DocumentNode, DocumentCategory};
//! use std::path::PathBuf;
//!
//! let mut graph = DocumentGraph::new();
//!
//! let node = DocumentNode {
//!     path: PathBuf::from("src/fbcu/mod.rs"),
//!     content_hash: "abc123...".to_string(),
//!     git_commits: vec!["a1b2c3d".to_string()],
//!     related: vec![PathBuf::from("docs/compression_architecture.md")],
//!     category: DocumentCategory::Code,
//!     template_used: Some("code_documentation".to_string()),
//!     created_at: chrono::Utc::now(),
//!     updated_at: chrono::Utc::now(),
//! };
//!
//! graph.add_node(node);
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Categoría de documento en el proyecto
///
/// Clasifica los documentos según su propósito en el proyecto.
/// Esto ayuda a filtrar consultas y entender el tipo de contenido.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentCategory {
    /// Documentación arquitectónica (diseño, ADRs)
    Architecture,
    
    /// Código fuente (módulos, tests)
    Code,
    
    /// Sesiones de trabajo y bitácoras
    Session,
    
    /// Resultados de tests y reportes
    Test,
    
    /// Decisiones arquitectónicas registradas (ADRs)
    Decision,
    
    /// Configuración (Cargo.toml, .yaml, etc.)
    Config,
    
    /// Documentación de usuario/API
    Documentation,
}

/// Tipo de relación entre documentos
///
/// Define la naturaleza de la conexión entre dos documentos.
/// Las relaciones son direccionales (from → to).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    /// Código A importa módulo B (use statement)
    Imports,
    
    /// Documento A referencia concepto de documento B
    References,
    
    /// Código A implementa diseño descrito en documento B
    Implements,
    
    /// Sesión/ADR A toma decisión que afecta módulo B
    Decides,
    
    /// Test A valida funcionalidad de código B
    Tests,
    
    /// Documento A es una versión actualizada de B
    Supersedes,
    
    /// Documento A depende de B para contexto
    DependsOn,
}

/// Nodo del grafo de documentos
///
/// Representa un documento individual con toda su metadata.
/// Incluye información de contenido, versionado Git, y relaciones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNode {
    /// Ruta absoluta o relativa al archivo
    pub path: PathBuf,
    
    /// Hash SHA256 del contenido (para detectar cambios)
    pub content_hash: String,
    
    /// Lista de commits Git relacionados (short hashes)
    pub git_commits: Vec<String>,
    
    /// Rutas a documentos relacionados (sin tipificar)
    pub related: Vec<PathBuf>,
    
    /// Categoría del documento
    pub category: DocumentCategory,
    
    /// Template MTT usado para generar/documentar (si aplica)
    pub template_used: Option<String>,
    
    /// Fecha de creación del nodo (no del archivo)
    pub created_at: DateTime<Utc>,
    
    /// Fecha de última actualización del nodo
    pub updated_at: DateTime<Utc>,
}

impl DocumentNode {
    /// Crea un nuevo nodo de documento
    pub fn new(
        path: PathBuf,
        content_hash: String,
        category: DocumentCategory,
    ) -> Self {
        let now = Utc::now();
        Self {
            path,
            content_hash,
            git_commits: Vec::new(),
            related: Vec::new(),
            category,
            template_used: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Actualiza el hash de contenido y la fecha de actualización
    pub fn update_content(&mut self, new_hash: String) {
        self.content_hash = new_hash;
        self.updated_at = Utc::now();
    }
    
    /// Agrega un commit Git a la historia del documento
    pub fn add_commit(&mut self, commit_hash: String) {
        if !self.git_commits.contains(&commit_hash) {
            self.git_commits.push(commit_hash);
            self.updated_at = Utc::now();
        }
    }
    
    /// Agrega un documento relacionado
    pub fn add_related(&mut self, path: PathBuf) {
        if !self.related.contains(&path) {
            self.related.push(path);
            self.updated_at = Utc::now();
        }
    }
}

/// Relación direccional entre dos documentos
///
/// Representa una conexión tipificada: from → to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRelation {
    /// Documento origen
    pub from: PathBuf,
    
    /// Documento destino
    pub to: PathBuf,
    
    /// Tipo de relación
    pub relation_type: RelationType,
    
    /// Descripción adicional de la relación
    pub description: String,
    
    /// Fecha en que se estableció la relación
    pub created_at: DateTime<Utc>,
}

impl DocumentRelation {
    /// Crea una nueva relación entre documentos
    pub fn new(
        from: PathBuf,
        to: PathBuf,
        relation_type: RelationType,
        description: String,
    ) -> Self {
        Self {
            from,
            to,
            relation_type,
            description,
            created_at: Utc::now(),
        }
    }
}

/// Grafo de documentos del proyecto
///
/// Estructura principal que mantiene todos los nodos y relaciones.
/// Permite consultas eficientes sobre la estructura del proyecto.
///
/// # Ejemplo de Uso
///
/// ```rust,ignore
/// let mut graph = DocumentGraph::new();
///
/// // Agregar documento
/// let node = DocumentNode::new(
///     PathBuf::from("src/fbcu/mod.rs"),
///     "abc123...".to_string(),
///     DocumentCategory::Code,
/// );
/// graph.add_node(node);
///
/// // Agregar relación
/// let relation = DocumentRelation::new(
///     PathBuf::from("src/fbcu/mod.rs"),
///     PathBuf::from("docs/compression.md"),
///     RelationType::Implements,
///     "Implementa la arquitectura de compresión".to_string(),
/// );
/// graph.add_relation(relation);
///
/// // Consultar relaciones
/// let related = graph.get_related(&PathBuf::from("src/fbcu/mod.rs"));
/// ```
pub struct DocumentGraph {
    /// Mapa de path → DocumentNode
    nodes: HashMap<PathBuf, DocumentNode>,
    
    /// Lista de relaciones (edges del grafo)
    edges: Vec<DocumentRelation>,
}

impl DocumentGraph {
    /// Crea un nuevo grafo vacío
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }
    
    /// Agrega o actualiza un nodo en el grafo
    ///
    /// Si el nodo ya existe (mismo path), se actualiza.
    pub fn add_node(&mut self, node: DocumentNode) {
        self.nodes.insert(node.path.clone(), node);
    }
    
    /// Obtiene un nodo por su path
    pub fn get_node(&self, path: &PathBuf) -> Option<&DocumentNode> {
        self.nodes.get(path)
    }
    
    /// Obtiene un nodo mutable por su path
    pub fn get_node_mut(&mut self, path: &PathBuf) -> Option<&mut DocumentNode> {
        self.nodes.get_mut(path)
    }
    
    /// Agrega una relación entre documentos
    pub fn add_relation(&mut self, relation: DocumentRelation) {
        self.edges.push(relation);
    }
    
    /// Obtiene todos los documentos relacionados desde un nodo
    ///
    /// Retorna los nodos destino de todas las relaciones que parten
    /// del path especificado.
    pub fn get_related(&self, path: &PathBuf) -> Vec<&DocumentNode> {
        self.edges.iter()
            .filter(|e| &e.from == path)
            .filter_map(|e| self.nodes.get(&e.to))
            .collect()
    }
    
    /// Obtiene todas las relaciones de un tipo específico desde un nodo
    pub fn get_related_by_type(
        &self,
        path: &PathBuf,
        relation_type: RelationType,
    ) -> Vec<&DocumentNode> {
        self.edges.iter()
            .filter(|e| &e.from == path && e.relation_type == relation_type)
            .filter_map(|e| self.nodes.get(&e.to))
            .collect()
    }
    
    /// Obtiene todos los documentos que apuntan a un nodo (relaciones inversas)
    pub fn get_referrers(&self, path: &PathBuf) -> Vec<&DocumentNode> {
        self.edges.iter()
            .filter(|e| &e.to == path)
            .filter_map(|e| self.nodes.get(&e.from))
            .collect()
    }
    
    /// Obtiene todos los nodos de una categoría específica
    pub fn get_by_category(&self, category: &DocumentCategory) -> Vec<&DocumentNode> {
        self.nodes.values()
            .filter(|n| &n.category == category)
            .collect()
    }
    
    /// Obtiene el número total de nodos en el grafo
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Obtiene el número total de relaciones en el grafo
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    
    /// Verifica si existe un nodo con el path especificado
    pub fn contains(&self, path: &PathBuf) -> bool {
        self.nodes.contains_key(path)
    }
    
    /// Obtiene todas las relaciones que parten de un nodo
    ///
    /// Usado por FlowQuery para trazar dependencias
    pub fn get_relations_from(&self, path: &PathBuf) -> Vec<&DocumentRelation> {
        self.edges.iter()
            .filter(|e| &e.from == path)
            .collect()
    }
    
    /// Obtiene todas las relaciones que apuntan a un nodo
    ///
    /// Usado por FlowQuery para encontrar dependents
    pub fn get_relations_to(&self, path: &PathBuf) -> Vec<&DocumentRelation> {
        self.edges.iter()
            .filter(|e| &e.to == path)
            .collect()
    }
}

impl Default for DocumentGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_graph() {
        let graph = DocumentGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }
    
    #[test]
    fn test_add_node() {
        let mut graph = DocumentGraph::new();
        let node = DocumentNode::new(
            PathBuf::from("test.rs"),
            "abc123".to_string(),
            DocumentCategory::Code,
        );
        
        graph.add_node(node);
        assert_eq!(graph.node_count(), 1);
        assert!(graph.contains(&PathBuf::from("test.rs")));
    }
    
    #[test]
    fn test_add_relation() {
        let mut graph = DocumentGraph::new();
        
        let node1 = DocumentNode::new(
            PathBuf::from("code.rs"),
            "hash1".to_string(),
            DocumentCategory::Code,
        );
        let node2 = DocumentNode::new(
            PathBuf::from("doc.md"),
            "hash2".to_string(),
            DocumentCategory::Documentation,
        );
        
        graph.add_node(node1);
        graph.add_node(node2);
        
        let relation = DocumentRelation::new(
            PathBuf::from("code.rs"),
            PathBuf::from("doc.md"),
            RelationType::Implements,
            "Test relation".to_string(),
        );
        
        graph.add_relation(relation);
        assert_eq!(graph.edge_count(), 1);
        
        let related = graph.get_related(&PathBuf::from("code.rs"));
        assert_eq!(related.len(), 1);
        assert_eq!(related[0].path, PathBuf::from("doc.md"));
    }
    
    #[test]
    fn test_get_by_category() {
        let mut graph = DocumentGraph::new();
        
        graph.add_node(DocumentNode::new(
            PathBuf::from("code1.rs"),
            "h1".to_string(),
            DocumentCategory::Code,
        ));
        graph.add_node(DocumentNode::new(
            PathBuf::from("code2.rs"),
            "h2".to_string(),
            DocumentCategory::Code,
        ));
        graph.add_node(DocumentNode::new(
            PathBuf::from("doc.md"),
            "h3".to_string(),
            DocumentCategory::Documentation,
        ));
        
        let code_nodes = graph.get_by_category(&DocumentCategory::Code);
        assert_eq!(code_nodes.len(), 2);
    }
    
    #[test]
    fn test_node_update_content() {
        let mut node = DocumentNode::new(
            PathBuf::from("test.rs"),
            "old_hash".to_string(),
            DocumentCategory::Code,
        );
        
        let old_time = node.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        node.update_content("new_hash".to_string());
        
        assert_eq!(node.content_hash, "new_hash");
        assert!(node.updated_at > old_time);
    }
}
