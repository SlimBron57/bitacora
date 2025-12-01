//! Git Integration - Trazabilidad automática de commits y contexto de evolución
//!
//! **Filosofía**: El código no nace - **evoluciona**.
//! Cada archivo tiene una historia: por qué se creó, cómo ha cambiado, qué decisiones se tomaron.
//! Git no es solo control de versiones - es la **narrativa evolutiva** del proyecto.
//!
//! # Capacidades
//!
//! - `get_file_commits()`: Obtener todos los commits que modificaron un archivo
//! - `get_commit_info()`: Detalles de un commit (autor, fecha, mensaje, diff)
//! - `get_file_creation_context()`: Cuándo y por qué se creó un archivo
//! - `find_related_commits()`: Commits que mencionan un término (para link to issues)
//! - `get_blame()`: Quién escribió cada línea (para atribución)
//!
//! # Integración con FlowQuery
//!
//! FlowQuery usa GitIntegration para enriquecer DocumentContext con:
//! - Commits relevantes
//! - Contexto de creación
//! - Enlaces a issues/PRs mencionados en mensajes de commit

use std::path::{Path, PathBuf};
use git2::{Repository, Commit, Oid, DiffOptions, Blame};
use anyhow::{Result, Context as AnyhowContext};
use chrono::{DateTime, Utc, TimeZone};

/// Motor de integración con Git
pub struct GitIntegration {
    repo: Repository,
    repo_path: PathBuf,
}

impl GitIntegration {
    /// Crear nueva integración con Git
    ///
    /// Busca el repositorio Git en el directorio especificado o en sus padres.
    pub fn new(path: PathBuf) -> Result<Self> {
        let repo = Repository::discover(&path)
            .context("Failed to find Git repository")?;
        
        let repo_path = repo.workdir()
            .context("Repository has no working directory")?
            .to_path_buf();
        
        Ok(Self {
            repo,
            repo_path,
        })
    }
    
    /// Obtener todos los commits que modificaron un archivo
    ///
    /// Retorna lista de CommitInfo ordenada por fecha (más reciente primero)
    pub fn get_file_commits(&self, file_path: &Path) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        
        let relative_path = self.make_relative(file_path)?;
        let mut commits = Vec::new();
        
        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.repo.find_commit(oid)?;
            
            // Verificar si este commit modificó el archivo
            if self.commit_touches_file(&commit, &relative_path)? {
                commits.push(self.commit_to_info(&commit)?);
            }
        }
        
        // Ordenar por fecha (más reciente primero)
        commits.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(commits)
    }
    
    /// Obtener información detallada de un commit por hash
    pub fn get_commit_info(&self, hash: &str) -> Result<CommitInfo> {
        let oid = Oid::from_str(hash)
            .context(format!("Invalid commit hash: {}", hash))?;
        let commit = self.repo.find_commit(oid)?;
        self.commit_to_info(&commit)
    }
    
    /// Obtener contexto de creación de un archivo
    ///
    /// Retorna el primer commit que introdujo el archivo.
    pub fn get_file_creation_context(&self, file_path: &Path) -> Result<CommitInfo> {
        let commits = self.get_file_commits(file_path)?;
        
        commits.into_iter()
            .last() // El último en la lista (más antiguo)
            .context(format!("No commits found for file: {:?}", file_path))
    }
    
    /// Buscar commits que mencionen un término (para link to issues)
    ///
    /// Útil para encontrar commits relacionados con un issue: "Fix #123", "Closes #456"
    pub fn find_related_commits(&self, query: &str) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        
        let query_lower = query.to_lowercase();
        let mut matching_commits = Vec::new();
        
        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.repo.find_commit(oid)?;
            
            let message = commit.message().unwrap_or("");
            if message.to_lowercase().contains(&query_lower) {
                matching_commits.push(self.commit_to_info(&commit)?);
            }
        }
        
        Ok(matching_commits)
    }
    
    /// Obtener blame (quién escribió cada línea)
    ///
    /// Útil para atribución de código en documentación
    pub fn get_blame(&self, file_path: &Path) -> Result<Vec<BlameHunk>> {
        let relative_path = self.make_relative(file_path)?;
        let blame = self.repo.blame_file(relative_path.as_path(), None)?;
        
        let mut hunks = Vec::new();
        
        for i in 0..blame.len() {
            if let Some(hunk) = blame.get_index(i) {
                let commit = self.repo.find_commit(hunk.final_commit_id())?;
                let author = commit.author();
                
                hunks.push(BlameHunk {
                    start_line: hunk.final_start_line(),
                    line_count: hunk.lines_in_hunk(),
                    commit_hash: format!("{}", hunk.final_commit_id()),
                    author_name: author.name().unwrap_or("Unknown").to_string(),
                    author_email: author.email().unwrap_or("").to_string(),
                });
            }
        }
        
        Ok(hunks)
    }
    
    /// Obtener diff de un archivo en un commit específico
    pub fn get_file_diff(&self, commit_hash: &str, file_path: &Path) -> Result<String> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repo.find_commit(oid)?;
        
        let parent = if commit.parent_count() > 0 {
            Some(commit.parent(0)?)
        } else {
            None
        };
        
        let parent_tree = parent.map(|p| p.tree()).transpose()?;
        let commit_tree = commit.tree()?;
        
        let mut diff_opts = DiffOptions::new();
        let relative_path = self.make_relative(file_path)?;
        diff_opts.pathspec(relative_path);
        
        let diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&commit_tree),
            Some(&mut diff_opts),
        )?;
        
        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let content = std::str::from_utf8(line.content()).unwrap_or("");
            diff_text.push_str(content);
            true
        })?;
        
        Ok(diff_text)
    }
    
    // === Helpers privados ===
    
    /// Convertir commit a CommitInfo
    fn commit_to_info(&self, commit: &Commit) -> Result<CommitInfo> {
        let author = commit.author();
        let timestamp = Utc.timestamp_opt(commit.time().seconds(), 0)
            .single()
            .unwrap_or_else(Utc::now);
        
        Ok(CommitInfo {
            hash: format!("{}", commit.id()),
            short_hash: format!("{:.7}", commit.id()),
            author_name: author.name().unwrap_or("Unknown").to_string(),
            author_email: author.email().unwrap_or("").to_string(),
            timestamp,
            message: commit.message().unwrap_or("").to_string(),
            summary: commit.summary().unwrap_or("").to_string(),
        })
    }
    
    /// Verificar si un commit modificó un archivo
    fn commit_touches_file(&self, commit: &Commit, file_path: &Path) -> Result<bool> {
        let parent = if commit.parent_count() > 0 {
            Some(commit.parent(0)?)
        } else {
            return Ok(true); // Primer commit - asumimos que lo toca
        };
        
        let parent_tree = parent.map(|p| p.tree()).transpose()?;
        let commit_tree = commit.tree()?;
        
        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(file_path);
        
        let diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&commit_tree),
            Some(&mut diff_opts),
        )?;
        
        Ok(diff.deltas().len() > 0)
    }
    
    /// Hacer path relativo al repositorio
    fn make_relative(&self, file_path: &Path) -> Result<PathBuf> {
        if file_path.is_relative() {
            Ok(file_path.to_path_buf())
        } else {
            file_path.strip_prefix(&self.repo_path)
                .map(|p| p.to_path_buf())
                .context(format!("File {:?} is not inside repository {:?}", file_path, self.repo_path))
        }
    }
}

/// Información de un commit
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub summary: String, // Primera línea del mensaje
}

/// Hunk de blame (rango de líneas con mismo autor)
#[derive(Debug, Clone)]
pub struct BlameHunk {
    pub start_line: usize,
    pub line_count: usize,
    pub commit_hash: String,
    pub author_name: String,
    pub author_email: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_create_git_integration() {
        let root = env::current_dir().unwrap();
        let git = GitIntegration::new(root);
        assert!(git.is_ok(), "Failed to create GitIntegration");
    }
    
    #[test]
    fn test_get_file_commits() {
        let root = env::current_dir().unwrap();
        let git = GitIntegration::new(root).unwrap();
        
        // Probar con este mismo archivo
        let this_file = root.join("src/bstradivarius/git_integration.rs");
        if this_file.exists() {
            let commits = git.get_file_commits(&this_file);
            assert!(commits.is_ok(), "Failed to get commits");
            
            let commits = commits.unwrap();
            assert!(!commits.is_empty(), "No commits found for git_integration.rs");
        }
    }
}
