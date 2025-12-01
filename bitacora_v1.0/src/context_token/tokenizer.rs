use crate::Result;
use super::token_7d::*;

/// Tokenizador de contexto con capacidades bidireccionales
pub struct ContextTokenizer {
    bidirectional_enabled: bool,
    processing_cache: std::collections::HashMap<String, Vec<ProcessedToken>>,
}

/// Token procesado con metadatos
#[derive(Debug, Clone)]
pub struct ProcessedToken {
    pub content: String,
    pub token_type: TokenType,
    pub position: usize,
    pub bidirectional_links: Vec<usize>,
    pub semantic_weight: f64,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Concept,
    Relation,
    Context,
    Emergent,
    Bidirectional,
}

/// Analizador de contexto 7D
pub struct ContextAnalyzer {
    analyzer_7d: ContextAnalyzer7D,
    breakthrough_mode: bool,
}

impl ContextTokenizer {
    pub fn new() -> Self {
        Self {
            bidirectional_enabled: false,
            processing_cache: std::collections::HashMap::new(),
        }
    }

    /// Configurar procesamiento bidireccional
    pub async fn configure_bidirectional_processing(&mut self) -> Result<()> {
        self.bidirectional_enabled = true;
        tracing::info!("Procesamiento bidireccional ContextToken7D activado");
        Ok(())
    }

    /// Tokenizar texto con metodología bidireccional
    pub async fn tokenize_bidirectional(&mut self, input: &str) -> Result<Vec<ContextToken7D>> {
        // Verificar cache
        if let Some(cached_tokens) = self.processing_cache.get(input) {
            return self.convert_processed_to_context_tokens(cached_tokens.clone()).await;
        }

        // Tokenización inicial
        let processed_tokens = self.process_input_text(input).await?;
        
        // Aplicar procesamiento bidireccional si está habilitado
        let bidirectional_tokens = if self.bidirectional_enabled {
            self.apply_bidirectional_processing(&processed_tokens).await?
        } else {
            processed_tokens
        };

        // Cachear resultado
        self.processing_cache.insert(input.to_string(), bidirectional_tokens.clone());

        // Convertir a tokens de contexto 7D
        self.convert_processed_to_context_tokens(bidirectional_tokens).await
    }

    /// Procesar texto de entrada
    async fn process_input_text(&self, input: &str) -> Result<Vec<ProcessedToken>> {
        let mut tokens = Vec::new();
        let sentences = input.split('.').filter(|s| !s.trim().is_empty());

        for (pos, sentence) in sentences.enumerate() {
            let token_type = self.classify_token_type(sentence)?;
            let semantic_weight = self.calculate_semantic_weight(sentence)?;

            tokens.push(ProcessedToken {
                content: sentence.trim().to_string(),
                token_type,
                position: pos,
                bidirectional_links: Vec::new(),
                semantic_weight,
            });
        }

        Ok(tokens)
    }

    /// Aplicar procesamiento bidireccional
    async fn apply_bidirectional_processing(&self, tokens: &[ProcessedToken]) -> Result<Vec<ProcessedToken>> {
        let mut bidirectional_tokens = tokens.to_vec();

        // Establecer enlaces bidireccionales basados en similitud semántica
        for i in 0..bidirectional_tokens.len() {
            for j in (i + 1)..bidirectional_tokens.len() {
                let similarity = self.calculate_token_similarity(&tokens[i], &tokens[j])?;
                
                if similarity > super::BIDIRECTIONAL_THRESHOLD {
                    bidirectional_tokens[i].bidirectional_links.push(j);
                    bidirectional_tokens[j].bidirectional_links.push(i);
                    
                    // Marcar como bidireccional si tiene suficientes enlaces
                    if bidirectional_tokens[i].bidirectional_links.len() >= 2 {
                        bidirectional_tokens[i].token_type = TokenType::Bidirectional;
                    }
                    if bidirectional_tokens[j].bidirectional_links.len() >= 2 {
                        bidirectional_tokens[j].token_type = TokenType::Bidirectional;
                    }
                }
            }
        }

        Ok(bidirectional_tokens)
    }

    /// Convertir tokens procesados a tokens de contexto 7D
    async fn convert_processed_to_context_tokens(&mut self, processed: Vec<ProcessedToken>) -> Result<Vec<ContextToken7D>> {
        let mut context_tokens = Vec::new();
        let mut analyzer = ContextAnalyzer7D::new();

        for processed_token in &processed {
            let mut context_token = ContextToken7D::new(processed_token.content.clone(), 7); // 7 días por defecto
            
            // Calcular dimensiones 7D
            context_token.calculate_dimensions(&mut analyzer)?;
            
            // Aplicar peso semántico del procesamiento ajustando cada dimensión
            context_token.temporal.score *= processed_token.semantic_weight;
            context_token.semantic.score *= processed_token.semantic_weight;
            context_token.contextual.score *= processed_token.semantic_weight;
            context_token.relational.score *= processed_token.semantic_weight;
            context_token.emotional.score *= processed_token.semantic_weight;
            context_token.intentional.score *= processed_token.semantic_weight;
            context_token.biographical.score *= processed_token.semantic_weight;
            
            // Establecer relaciones basadas en enlaces bidireccionales
            for &linked_pos in &processed_token.bidirectional_links {
                if linked_pos < processed.len() {
                    let relationship_strength = processed_token.semantic_weight * 0.8;
                    context_token.add_relationship(
                        format!("token_{}", linked_pos),
                        RelationshipType::Bidirectional,
                        relationship_strength,
                    );
                }
            }

            context_tokens.push(context_token);
        }

        Ok(context_tokens)
    }

    /// Clasificar tipo de token
    fn classify_token_type(&self, content: &str) -> Result<TokenType> {
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("porque") || content_lower.contains("debido") || content_lower.contains("resulta") {
            Ok(TokenType::Relation)
        } else if content_lower.contains("emerge") || content_lower.contains("surge") || content_lower.contains("aparece") {
            Ok(TokenType::Emergent)
        } else if content_lower.contains("contexto") || content_lower.contains("situación") || content_lower.contains("ambiente") {
            Ok(TokenType::Context)
        } else {
            Ok(TokenType::Concept)
        }
    }

    /// Calcular peso semántico
    fn calculate_semantic_weight(&self, content: &str) -> Result<f64> {
        let word_count = content.split_whitespace().count();
        let complexity_indicators = ["complejo", "sofisticado", "avanzado", "integrado"];
        let complexity_score = complexity_indicators.iter()
            .map(|&indicator| content.to_lowercase().matches(indicator).count())
            .sum::<usize>();

        let base_weight = (word_count as f64 * 0.1).min(1.0);
        let complexity_bonus = (complexity_score as f64 * 0.2).min(0.5);
        
        Ok((base_weight + complexity_bonus).min(1.0))
    }

    /// Calcular similitud entre tokens procesados
    fn calculate_token_similarity(&self, token1: &ProcessedToken, token2: &ProcessedToken) -> Result<f64> {
        // Similitud basada en palabras compartidas
        let content1_lower = token1.content.to_lowercase();
        let content2_lower = token2.content.to_lowercase();
        
        let words1: std::collections::HashSet<_> = content1_lower
            .split_whitespace()
            .collect();
        let words2: std::collections::HashSet<_> = content2_lower
            .split_whitespace()
            .collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        let word_similarity = if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        };

        // Factor de tipo de token
        let type_similarity = match (&token1.token_type, &token2.token_type) {
            (TokenType::Concept, TokenType::Concept) => 1.0,
            (TokenType::Relation, TokenType::Relation) => 1.0,
            (TokenType::Emergent, TokenType::Emergent) => 1.2, // Bonus para tokens emergentes
            _ => 0.7,
        };

        // Factor de peso semántico
        let weight_factor = (token1.semantic_weight + token2.semantic_weight) / 2.0;

        Ok(word_similarity * type_similarity * weight_factor)
    }
}

impl ContextAnalyzer {
    pub fn new() -> Self {
        Self {
            analyzer_7d: ContextAnalyzer7D::new(),
            breakthrough_mode: false,
        }
    }

    /// Configurar parámetros breakthrough
    pub async fn configure_breakthrough_parameters(&mut self) -> Result<()> {
        self.breakthrough_mode = true;
        tracing::info!("Modo breakthrough ContextToken7D activado - Target: 133.8/100");
        Ok(())
    }

    /// Analizar 7 dimensiones con modo breakthrough
    pub async fn analyze_7_dimensions(&mut self, tokens: &[ContextToken7D]) -> Result<Vec<ContextToken7D>> {
        if tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut analyzed_tokens = tokens.to_vec();

        for token in &mut analyzed_tokens {
            if self.breakthrough_mode {
                // Aplicar optimizaciones breakthrough
                self.apply_breakthrough_optimizations(token).await?;
            }

            // Recalcular puntuación de relevancia con nuevos parámetros
            token.update_lifecycle_stage()?;
        }

        Ok(analyzed_tokens)
    }

    /// Aplicar optimizaciones breakthrough
    async fn apply_breakthrough_optimizations(&mut self, token: &mut ContextToken7D) -> Result<()> {
        // Amplificar dimensiones con alto potencial
        token.intentional.score *= 1.2;     // Propiedades emergentes (índice 5)
        token.biographical.score *= 1.15;   // Potencial bidireccional (índice 6) 
        token.semantic.score *= 1.1;        // Profundidad conceptual (índice 1)

        // Recalcular peso bidireccional con optimizaciones
        token.bidirectional_weight = self.analyzer_7d.calculate_bidirectional_weight(&token)?;
        
        // Aplicar factor breakthrough
        if token.bidirectional_weight > 0.8 {
            token.bidirectional_weight *= 1.338; // Factor breakthrough
            token.bidirectional_weight = token.bidirectional_weight.min(1.0);
        }

        Ok(())
    }
}

impl Default for ContextTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ContextAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}