//! Basic Spark Handler demonstrating TRANSVERSAL service

use std::sync::Arc;
use async_trait::async_trait;
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::ParsedCommand;

/// Simple spark handler for demonstration (TRANSVERSAL service)
pub struct SparkHandler;

impl SparkHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandHandler for SparkHandler {
    fn command_name(&self) -> &'static str {
        "spark"
    }

    fn description(&self) -> &'static str {
        "Captura insights (servicio TRANSVERSAL a PROJECT → TOPIC → ACTION)"
    }

    async fn handle(&self, _context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        if command.command != "spark" {
            return ExecutionResult::error("Command not supported by SparkHandler");
        }
        
        let subcommand = command.subcommand.as_deref().unwrap_or("help");
        
        match subcommand {
            "capture" => ExecutionResult::success(
                "✨ SPARK capturado exitosamente!\n\
                 🔄 SERVICIO TRANSVERSAL activado:\n\
                 PROJECT → TOPIC → ACTION\n\
                     ✨ SPARK puede activarse en cualquier momento\n\
                 💡 Insight registrado para análisis futuro".to_string()
            ),
            "list" => ExecutionResult::success(
                "✨ SPARKs (SERVICIO TRANSVERSAL):\n\
                 • insight-arquitectura (capturado)\n\
                 • aprendizaje-performance (aplicado)\n\
                 • idea-mejora-ux (revisado)\n\
                 💡 Usa 'spark show <id>' para detalles".to_string()
            ),
            "apply" => ExecutionResult::success(
                "🎯 SPARK aplicado exitosamente!\n\
                 ✨ Insight integrado en tu flujo de trabajo\n\
                 📈 Conocimiento transversal activado".to_string()
            ),
            "help" | _ => ExecutionResult::success(
                "✨ SPARK Handler - SERVICIO TRANSVERSAL\n\
                 \n\
                 🔄 Funciona transversalmente en:\n\
                 PROJECT → TOPIC → ACTION\n\
                     ✨ Puede activarse en cualquier momento\n\
                 \n\
                 Comandos disponibles:\n\
                 • spark capture <contenido> - Capturar insight\n\
                 • spark list - Listar insights capturados\n\
                 • spark apply <id> - Aplicar insight\n\
                 • spark show <id> - Ver detalles\n\
                 \n\
                 💡 SPARK no es secuencial - es un servicio que apoya todo el flujo".to_string()
            ),
        }
    }
}
