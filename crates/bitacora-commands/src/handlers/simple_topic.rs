//! Basic Topic Handler demonstrating PROJECT → TOPIC → ACTION architecture

use std::sync::Arc;
use async_trait::async_trait;
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::ParsedCommand;

/// Simple topic handler for demonstration  
pub struct TopicHandler;

impl TopicHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandHandler for TopicHandler {
    fn command_name(&self) -> &'static str {
        "topic"
    }

    fn description(&self) -> &'static str {
        "Gestiona temas (nivel 2 en PROJECT → TOPIC → ACTION)"
    }

    async fn handle(&self, _context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        if command.command != "topic" {
            return ExecutionResult::error("Command not supported by TopicHandler");
        }
        
        let subcommand = command.subcommand.as_deref().unwrap_or("help");
        
        match subcommand {
            "create" => ExecutionResult::success(
                "✅ TOPIC creado exitosamente!\n\
                 🔄 Flujo: PROJECT → TOPIC → ACTION\n\
                                    ^^^^^^ Estás aquí\n\
                 💡 Próximo: 'action create' para añadir acciones específicas".to_string()
            ),
            "list" => ExecutionResult::success(
                "📋 TOPICs (PROJECT → TOPIC → ACTION):\n\
                 • tema-frontend (activo)\n\
                 • tema-backend (en progreso)\n\
                 • tema-database (completado)\n\
                 💡 Usa 'topic show <nombre>' para detalles".to_string()
            ),
            "help" | _ => ExecutionResult::success(
                "📋 TOPIC Handler - Nivel 2 de la secuencia\n\
                 \n\
                 🔄 Arquitectura: PROJECT → TOPIC → ACTION\n\
                                           ^^^^^^ Conecta proyectos con acciones\n\
                 \n\
                 Comandos disponibles:\n\
                 • topic create <nombre> --project <proyecto> - Crear tema en proyecto\n\
                 • topic list - Listar temas\n\
                 • topic show <nombre> - Ver detalles\n\
                 \n\
                 💡 Después de crear un TOPIC, usa 'action create' para el nivel final".to_string()
            ),
        }
    }
}
