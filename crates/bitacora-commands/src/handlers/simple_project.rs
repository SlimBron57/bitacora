//! Basic Project Handler demonstrating PROJECT → TOPIC → ACTION architecture

use std::sync::Arc;
use async_trait::async_trait;
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::ParsedCommand;

/// Simple project handler for demonstration
pub struct ProjectHandler;

impl ProjectHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandHandler for ProjectHandler {
    fn command_name(&self) -> &'static str {
        "project"
    }

    fn description(&self) -> &'static str {
        "Gestiona proyectos (nivel 1 en PROJECT → TOPIC → ACTION)"
    }

    async fn handle(&self, _context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        if command.command != "project" {
            return ExecutionResult::error("Command not supported by ProjectHandler");
        }
        
        let subcommand = command.subcommand.as_deref().unwrap_or("help");
        
        match subcommand {
            "create" => ExecutionResult::success(
                "✅ PROJECT creado exitosamente!\n\
                 🔄 Flujo: PROJECT → TOPIC → ACTION\n\
                          ^^^^^^^ Estás aquí\n\
                 💡 Próximo: 'topic create' para añadir temas".to_string()
            ),
            "list" => ExecutionResult::success(
                "📁 PROYECTOS (PROJECT → TOPIC → ACTION):\n\
                 • proyecto-1 (activo)\n\
                 • proyecto-2 (completado)\n\
                 💡 Usa 'project show <nombre>' para detalles".to_string()
            ),
            "help" | _ => ExecutionResult::success(
                "📁 PROJECT Handler - Nivel 1 de la secuencia\n\
                 \n\
                 🔄 Arquitectura: PROJECT → TOPIC → ACTION\n\
                                  ^^^^^^^ Gestiona contenedores de alto nivel\n\
                 \n\
                 Comandos disponibles:\n\
                 • project create <nombre> - Crear nuevo proyecto\n\
                 • project list - Listar proyectos\n\
                 • project show <nombre> - Ver detalles\n\
                 \n\
                 💡 Después de crear un PROJECT, usa 'topic create' para el siguiente nivel".to_string()
            ),
        }
    }
}
