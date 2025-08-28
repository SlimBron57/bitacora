//! API Documentation utilities and extensions

// Documentation helpers removed - examples and OpenAPI customization were previously provided via utoipa
pub fn generate_examples_json() -> serde_json::Value {
    serde_json::json!({
        "note": "Documentation generation removed. Use project docs in /docs or re-enable utoipa if needed."
    })
}

#[cfg(test)]
mod tests {
    use super::generate_examples_json;

    #[test]
    fn test_examples_generation() {
        let examples = generate_examples_json();
        assert!(examples.is_object());
    }
}
