//! # Variable Substitution System 🔄
//!
//! Sistema dinámico para reemplazar variables en templates BitaFlow
//! Soporte para contexto dinámico, funciones built-in y transformaciones

use std::collections::HashMap;
use regex::Regex;
use crate::errors::NavigatorError;

/// Contexto de variables para substitución
#[derive(Debug, Clone)]
pub struct VariableContext {
    /// Variables definidas en el template YAML
    pub template_variables: HashMap<String, String>,
    /// Variables del contexto dinámico (runtime)
    pub runtime_variables: HashMap<String, String>,
    /// Variables del entorno/sistema
    pub system_variables: HashMap<String, String>,
}

impl VariableContext {
    /// Crear nuevo contexto vacío
    pub fn new() -> Self {
        Self {
            template_variables: HashMap::new(),
            runtime_variables: HashMap::new(),
            system_variables: Self::initialize_system_variables(),
        }
    }

    /// Crear contexto desde variables del template
    pub fn from_template_variables(template_vars: HashMap<String, String>) -> Self {
        Self {
            template_variables: template_vars,
            runtime_variables: HashMap::new(),
            system_variables: Self::initialize_system_variables(),
        }
    }

    /// Agregar variable de runtime
    pub fn set_runtime_variable(&mut self, key: String, value: String) {
        self.runtime_variables.insert(key, value);
    }

    /// Obtener valor de variable con prioridades
    /// Orden de prioridad: runtime > template > system
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.runtime_variables.get(key)
            .or_else(|| self.template_variables.get(key))
            .or_else(|| self.system_variables.get(key))
    }

    /// Inicializar variables del sistema
    fn initialize_system_variables() -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        // Variables de tiempo
        vars.insert("current_timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        vars.insert("current_date".to_string(), chrono::Utc::now().format("%Y-%m-%d").to_string());
        vars.insert("current_time".to_string(), chrono::Utc::now().format("%H:%M:%S").to_string());
        
        // Variables de usuario/sistema
        if let Ok(user) = std::env::var("USER") {
            vars.insert("current_user".to_string(), user);
        }
        
        if let Ok(pwd) = std::env::var("PWD") {
            vars.insert("current_directory".to_string(), pwd);
        }

        // Variables del proyecto BitaFlow
        vars.insert("bitaflow_version".to_string(), "1.0.0".to_string());
        vars.insert("navigator_engine".to_string(), "HybridNavigator".to_string());
        
        vars
    }

    /// Mergear otro contexto en este
    pub fn merge(&mut self, other: &VariableContext) {
        for (k, v) in &other.template_variables {
            self.template_variables.insert(k.clone(), v.clone());
        }
        for (k, v) in &other.runtime_variables {
            self.runtime_variables.insert(k.clone(), v.clone());
        }
    }
}

/// Motor de substitución de variables
pub struct VariableSubstitutor {
    /// Regex para detectar variables {{variable}}
    variable_regex: Regex,
    /// Regex para detectar funciones {{function(args)}}
    function_regex: Regex,
}

impl VariableSubstitutor {
    /// Crear nuevo substitutor
    pub fn new() -> Result<Self, NavigatorError> {
        let variable_regex = Regex::new(r"\{\{([^}]+)\}\}")
            .map_err(|e| NavigatorError::configuration(format!("Invalid variable regex: {}", e)))?;
        
        let function_regex = Regex::new(r"\{\{(\w+)\((.*?)\)\}\}")
            .map_err(|e| NavigatorError::configuration(format!("Invalid function regex: {}", e)))?;

        Ok(Self {
            variable_regex,
            function_regex,
        })
    }

    /// Substituir variables en texto
    pub fn substitute(&self, content: &str, context: &VariableContext) -> Result<String, NavigatorError> {
        let mut result = content.to_string();

        // Primero procesar funciones {{function(args)}}
        result = self.substitute_functions(&result, context)?;

        // Luego procesar variables simples {{variable}}
        result = self.substitute_variables(&result, context)?;

        Ok(result)
    }

    /// Substituir variables simples {{variable}}
    fn substitute_variables(&self, content: &str, context: &VariableContext) -> Result<String, NavigatorError> {
        let result = self.variable_regex.replace_all(content, |caps: &regex::Captures| {
            let var_name = caps.get(1).unwrap().as_str().trim();
            
            // Buscar valor en contexto
            match context.get_variable(var_name) {
                Some(value) => value.clone(),
                None => {
                    // Si no encuentra la variable, la deja como está pero con indicador
                    format!("{{{{{}:NOT_FOUND}}}}", var_name)
                }
            }
        });

        Ok(result.to_string())
    }

    /// Substituir funciones {{function(args)}}
    fn substitute_functions(&self, content: &str, context: &VariableContext) -> Result<String, NavigatorError> {
        let result = self.function_regex.replace_all(content, |caps: &regex::Captures| {
            let function_name = caps.get(1).unwrap().as_str().trim();
            let args_str = caps.get(2).unwrap().as_str();
            
            match self.execute_builtin_function(function_name, args_str, context) {
                Ok(value) => value,
                Err(_) => {
                    // Si la función falla, devolver indicador de error
                    format!("{{{{{}({}):ERROR}}}}", function_name, args_str)
                }
            }
        });

        Ok(result.to_string())
    }

    /// Ejecutar funciones built-in
    fn execute_builtin_function(&self, function: &str, args: &str, context: &VariableContext) -> Result<String, NavigatorError> {
        match function {
            // Funciones de string
            "upper" => {
                let var_name = args.trim();
                match context.get_variable(var_name) {
                    Some(value) => Ok(value.to_uppercase()),
                    None => Ok(format!("{{{}:NOT_FOUND}}", var_name))
                }
            },
            "lower" => {
                let var_name = args.trim();
                match context.get_variable(var_name) {
                    Some(value) => Ok(value.to_lowercase()),
                    None => Ok(format!("{{{}:NOT_FOUND}}", var_name))
                }
            },
            "capitalize" => {
                let var_name = args.trim();
                match context.get_variable(var_name) {
                    Some(value) => {
                        let mut chars = value.chars();
                        match chars.next() {
                            None => Ok(String::new()),
                            Some(first) => Ok(first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase())
                        }
                    },
                    None => Ok(format!("{{{}:NOT_FOUND}}", var_name))
                }
            },

            // Funciones de formato
            "format_date" => {
                let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 2 {
                    let var_name = parts[0];
                    let format_str = parts[1];
                    // Implementación simplificada - en producción usar chrono para parsing
                    match context.get_variable(var_name) {
                        Some(value) => Ok(format!("{}(formatted:{})", value, format_str)),
                        None => Ok(format!("{{{}:NOT_FOUND}}", var_name))
                    }
                } else {
                    Err(NavigatorError::configuration("format_date requires variable and format".to_string()))
                }
            },

            // Funciones de contexto
            "if_set" => {
                let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    let var_name = parts[0];
                    let if_true = parts[1];
                    let if_false = parts[2];
                    
                    match context.get_variable(var_name) {
                        Some(_) => Ok(if_true.to_string()),
                        None => Ok(if_false.to_string())
                    }
                } else {
                    Err(NavigatorError::configuration("if_set requires variable, true_value, false_value".to_string()))
                }
            },

            // Función default
            "default" => {
                let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 2 {
                    let var_name = parts[0];
                    let default_value = parts[1];
                    
                    match context.get_variable(var_name) {
                        Some(value) => Ok(value.clone()),
                        None => Ok(default_value.to_string())
                    }
                } else {
                    Err(NavigatorError::configuration("default requires variable and default_value".to_string()))
                }
            },

            _ => Err(NavigatorError::configuration(format!("Unknown function: {}", function)))
        }
    }

    /// Validar que todas las variables en el contenido están disponibles
    pub fn validate_variables(&self, content: &str, context: &VariableContext) -> Result<Vec<String>, NavigatorError> {
        let mut missing_variables = Vec::new();

        // Buscar variables faltantes
        for caps in self.variable_regex.captures_iter(content) {
            let var_name = caps.get(1).unwrap().as_str().trim();
            if context.get_variable(var_name).is_none() {
                missing_variables.push(var_name.to_string());
            }
        }

        Ok(missing_variables)
    }

    /// Extraer todas las variables de un contenido
    pub fn extract_variables(&self, content: &str) -> Vec<String> {
        self.variable_regex
            .captures_iter(content)
            .map(|caps| caps.get(1).unwrap().as_str().trim().to_string())
            .collect::<std::collections::HashSet<_>>()  // Eliminar duplicados
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_context_creation() {
        let context = VariableContext::new();
        
        // Verificar que las variables del sistema están inicializadas
        assert!(context.get_variable("current_date").is_some());
        assert!(context.get_variable("bitaflow_version").is_some());
    }

    #[test]
    fn test_variable_context_priorities() {
        let mut context = VariableContext::new();
        
        // Agregar en template
        context.template_variables.insert("test_var".to_string(), "template_value".to_string());
        
        // Agregar en runtime (debe tener prioridad)
        context.set_runtime_variable("test_var".to_string(), "runtime_value".to_string());
        
        // Runtime debe ganar
        assert_eq!(context.get_variable("test_var"), Some(&"runtime_value".to_string()));
    }

    #[test]
    fn test_simple_variable_substitution() {
        let substitutor = VariableSubstitutor::new().unwrap();
        let mut context = VariableContext::new();
        
        context.set_runtime_variable("error_type".to_string(), "RuntimeError".to_string());
        
        let content = "Error detected: {{error_type}} in file {{error_file}}";
        let result = substitutor.substitute(content, &context).unwrap();
        
        assert!(result.contains("Error detected: RuntimeError"));
        assert!(result.contains("{{error_file:NOT_FOUND}}"));
    }

    #[test]
    fn test_function_substitution() {
        let substitutor = VariableSubstitutor::new().unwrap();
        let mut context = VariableContext::new();
        
        context.set_runtime_variable("error_type".to_string(), "runtimeerror".to_string());
        
        let content = "Error: {{upper(error_type)}} - {{capitalize(error_type)}}";
        let result = substitutor.substitute(content, &context).unwrap();
        
        assert!(result.contains("Error: RUNTIMEERROR - Runtimeerror"));
    }

    #[test]
    fn test_default_function() {
        let substitutor = VariableSubstitutor::new().unwrap();
        let context = VariableContext::new();
        
        let content = "Severity: {{default(severity, medium)}}";
        let result = substitutor.substitute(content, &context).unwrap();
        
        assert!(result.contains("Severity: medium"));
    }

    #[test]
    fn test_variable_extraction() {
        let substitutor = VariableSubstitutor::new().unwrap();
        let content = "Error {{error_type}} in {{file_name}} at {{line_number}}";
        
        let variables = substitutor.extract_variables(content);
        
        assert_eq!(variables.len(), 3);
        assert!(variables.contains(&"error_type".to_string()));
        assert!(variables.contains(&"file_name".to_string()));
        assert!(variables.contains(&"line_number".to_string()));
    }

    #[test]
    fn test_variable_validation() {
        let substitutor = VariableSubstitutor::new().unwrap();
        let mut context = VariableContext::new();
        
        context.set_runtime_variable("error_type".to_string(), "RuntimeError".to_string());
        
        let content = "Error {{error_type}} in {{missing_var}}";
        let missing = substitutor.validate_variables(content, &context).unwrap();
        
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], "missing_var");
    }
}
