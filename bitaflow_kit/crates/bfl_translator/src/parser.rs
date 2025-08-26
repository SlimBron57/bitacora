use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub alias: String,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub kind: Option<String>,
    pub version: Option<u32>,
    pub locale: Option<String>,
    pub requires: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct BflDoc {
    pub front: FrontMatter,
    pub body: String,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("front-matter missing")]
    MissingFrontMatter,
    #[error("yaml parse error: {0}")]
    Yaml(String),
}

pub fn parse_bfl(input: &str) -> Result<BflDoc, ParseError> {
    let parts: Vec<&str> = input.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(ParseError::MissingFrontMatter);
    }
    let yaml = parts[1];
    let body = parts[2].strip_prefix('\n').unwrap_or(parts[2]).to_string();
    let front: FrontMatter = serde_yaml::from_str(yaml).map_err(|e| ParseError::Yaml(e.to_string()))?;
    Ok(BflDoc { front, body })
}
