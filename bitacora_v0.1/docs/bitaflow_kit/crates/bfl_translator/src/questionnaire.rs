#[derive(Debug, Clone)]
pub struct Score {
    pub market: u8,
    pub product: u8,
    pub model: u8,
    pub execution: u8,
}

impl Score {
    pub fn total(&self) -> u16 {
        self.market as u16 + self.product as u16 + self.model as u16 + self.execution as u16
    }
    pub fn viable(&self) -> bool {
        self.total() >= 65
    }
}

pub const PROMPT_SYSTEM: &str = r#"
Eres un entrevistador técnico/negocio. Haz preguntas claras, una por vez.
Resume cada bloque, puntúa 0–25 en cuatro dimensiones (Mercado/Producto/Modelo/Ejecución)
y al final calcula viabilidad. Si pasa el corte (≥65), genera archivos .bt (BitaFlow)
con las plantillas apropiadas. Si no, devuelve diagnóstico y próximos pasos.
"#;
