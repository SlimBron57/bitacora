//! API Documentation utilities and extensions

use utoipa::{openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}, Modify, OpenApi};
use utoipa::openapi::example::Example;
use std::collections::BTreeMap;

/// OpenAPI documentation customization
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Add security schemes for future authentication
        let mut security_schemes = BTreeMap::new();
        
        // API Key authentication
        security_schemes.insert(
            "ApiKeyAuth".to_string(),
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
        );
        
        // JWT Bearer authentication  
        security_schemes.insert(
            "BearerAuth".to_string(),
            SecurityScheme::Http(
                utoipa::openapi::security::Http::new(
                    utoipa::openapi::security::HttpAuthScheme::Bearer
                )
                .bearer_format("JWT")
            ),
        );
        
        if openapi.components.is_none() {
            openapi.components = Some(utoipa::openapi::Components::default());
        }
        
        if let Some(components) = &mut openapi.components {
            components.security_schemes = security_schemes;
        }
        
        // Add global tags with descriptions
        openapi.tags = Some(vec![
            utoipa::openapi::Tag::new("projects"),
            utoipa::openapi::Tag::new("topics"),
            utoipa::openapi::Tag::new("actions"),
            utoipa::openapi::Tag::new("sparks"),
            utoipa::openapi::Tag::new("health"),
        ]);
    }
}

/// Generate example requests and responses for documentation
pub fn generate_examples() -> BTreeMap<String, Example> {
    let mut examples = BTreeMap::new();
    
    // Project creation example
    examples.insert(
        "project_example".to_string(),
        Example::new(
            serde_json::json!({
                "name": "E-commerce Platform",
                "description": "Building a modern e-commerce platform with microservices architecture",
                "status": "planning",
                "tags": ["ecommerce", "microservices", "web"],
                "metadata": {
                    "technology_stack": "rust, react, postgresql",
                    "target_launch": "2025-12-01"
                }
            })
        )
        .summary(Some("Create E-commerce Project".to_string()))
        .description(Some("Example of creating a new e-commerce platform project".to_string()))
    );
    
    // Topic creation example
    examples.insert(
        "CreateTopicExample".to_string(),
        Example::new(
            serde_json::json!({
                "project_id": "550e8400-e29b-41d4-a716-446655440000",
                "title": "User Authentication System",
                "description": "Implement secure user authentication with JWT tokens and OAuth integration",
                "priority": "high",
                "tags": ["authentication", "security", "jwt"],
                "estimated_hours": 16.0
            })
        )
        .summary(Some("Create Authentication Topic".to_string()))
        .description(Some("Example of creating a user authentication topic within a project".to_string()))
    );
    
    // Action creation example
    examples.insert(
        "CreateActionExample".to_string(),
        Example::new(
            serde_json::json!({
                "topic_id": "550e8400-e29b-41d4-a716-446655440001",
                "title": "Implement JWT token generation",
                "description": "Create service for generating and validating JWT tokens with proper expiration",
                "action_type": "development",
                "priority": "high",
                "estimated_hours": 4.0,
                "tags": ["jwt", "security", "backend"]
            })
        )
        .summary(Some("Create JWT Implementation Action".to_string()))
        .description(Some("Example of creating an action for JWT token implementation".to_string()))
    );
    
    // Spark creation example
    examples.insert(
        "CreateSparkExample".to_string(),
        Example::new(
            serde_json::json!({
                "context": {
                    "level": "action",
                    "entity_id": "550e8400-e29b-41d4-a716-446655440002",
                    "description": "While implementing JWT authentication"
                },
                "spark_type": "learning",
                "title": "JWT token size optimization",
                "content": "Discovered that using short claim names can reduce JWT token size by 20-30%",
                "insights": "Smaller tokens mean less bandwidth usage and faster authentication checks",
                "impact": "medium",
                "tags": ["jwt", "optimization", "performance"],
                "action_items": [
                    {
                        "description": "Update JWT service to use abbreviated claim names",
                        "priority": "medium",
                        "converted": false
                    }
                ]
            })
        )
        .summary(Some("JWT Optimization Spark".to_string()))
        .description(Some("Example of capturing a learning insight about JWT optimization".to_string()))
    );
    
    examples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_generation() {
        let examples = generate_examples();
        assert!(!examples.is_empty());
        assert!(examples.contains_key("CreateProjectExample"));
        assert!(examples.contains_key("CreateTopicExample"));
        assert!(examples.contains_key("CreateActionExample"));
        assert!(examples.contains_key("CreateSparkExample"));
    }
}
