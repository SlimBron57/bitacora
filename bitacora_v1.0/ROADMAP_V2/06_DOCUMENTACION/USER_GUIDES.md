# 📖 User Guides: Guías de Usuario para Bitácora v1.0

**Archivo:** `ROADMAP_V2/06_DOCUMENTACION/USER_GUIDES.md`  
**Versión:** 1.0  
**Fecha:** 2025-10-26  
**Propósito:** Guías completas para usuarios finales (desarrolladores y end-users)

---

## 🎯 AUDIENCIAS

Este documento contiene guías para 3 tipos de usuarios:

1. **Desarrolladores** que integran Bitácora en sus aplicaciones
2. **End-users** que usan la interfaz de Bitácora
3. **Administradores** que despliegan y mantienen el sistema

---

## 👨‍💻 GUÍA PARA DESARROLLADORES

### **Quickstart: Primera Integración en 5 Minutos**

#### **Paso 1: Instalación**

```bash
# Clonar repositorio
git clone https://github.com/bitacora/bitacora_v1.0.git
cd bitacora_v1.0

# Instalar dependencias
cargo build --release

# Configurar API keys
cp .env.example .env
# Editar .env con tus keys:
# OPENAI_API_KEY=sk-...
# ANTHROPIC_API_KEY=sk-ant-...
```

#### **Paso 2: Primer Query**

```rust
use bitacora::{Bitacora, QueryRequest, QueryMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar Bitácora
    let bitacora = Bitacora::new().await?;
    
    // Crear query
    let request = QueryRequest {
        query: "How do I fix a lifetime error in Rust?".to_string(),
        mode: QueryMode::Auto, // Auto-selecciona local vs LLM
        max_results: 5,
        context: None,
    };
    
    // Ejecutar
    let response = bitacora.query(request).await?;
    
    // Mostrar resultado
    println!("Mode: {:?}", response.execution_mode);
    println!("Response: {}", response.results[0].response);
    println!("Time: {}ms", response.execution_time_ms);
    
    Ok(())
}
```

**Output esperado:**
```
Mode: Local
Response: Let's analyze the lifetime requirements...
Time: 142ms
```

---

### **Uso Avanzado: Modo Local vs LLM**

#### **Forzar Modo Local (máxima velocidad)**

```rust
let request = QueryRequest {
    query: "Common Rust patterns".to_string(),
    mode: QueryMode::Local, // Forzar local
    max_results: 5,
    context: None,
};

let response = bitacora.query(request).await?;

if response.execution_mode == ExecutionMode::Local {
    println!("✓ Respondido localmente en {}ms", response.execution_time_ms);
} else {
    println!("⚠ Fallback a LLM (template no encontrado)");
}
```

#### **Forzar Modo LLM (máxima calidad)**

```rust
let request = QueryRequest {
    query: "Explain the Raft consensus algorithm in detail".to_string(),
    mode: QueryMode::LLM, // Forzar LLM
    max_results: 1,
    context: Some(hashmap! {
        "model_preference" => "gpt-4",
        "max_tokens" => "2000",
    }),
};

let response = bitacora.query(request).await?;
println!("Response from {}: {}", response.model_used, response.results[0].response);
```

---

### **Streaming Responses**

```rust
use futures::StreamExt;

let mut stream = bitacora.query_stream(QueryRequest {
    query: "Explain Rust ownership step by step".to_string(),
    mode: QueryMode::LLM,
    ..Default::default()
}).await?;

print!("Response: ");
while let Some(chunk) = stream.next().await {
    match chunk? {
        StreamChunk::Token(text) => print!("{}", text),
        StreamChunk::Done { total_tokens } => {
            println!("\n\nTotal tokens: {}", total_tokens);
        }
    }
}
```

---

### **Trabajar con TelescopeDB**

#### **Almacenar Frame**

```rust
use bitacora::{TelescopeDB, Pixel, PixelCoord, LAB};

let telescope = TelescopeDB::new("data/telescope.db").await?;

let mut pixels = Vec::new();
for y in 0..1080 {
    for x in 0..1920 {
        pixels.push(Pixel {
            position: PixelCoord { x, y },
            color: LAB { l: 50.0, a: 0.0, b: 0.0 },
            timestamp: Utc::now().timestamp() as u64,
        });
    }
}

let frame_id = telescope.store_frame(&pixels).await?;
println!("Stored frame: {}", frame_id);
```

#### **Query Región Específica**

```rust
let region = BoundingBox {
    min: PixelCoord { x: 100, y: 100 },
    max: PixelCoord { x: 500, y: 500 },
};

let pixels = telescope.query_region(&region).await?;
println!("Found {} pixels in region", pixels.len());
```

---

### **Crear Templates Personalizados**

```rust
use bitacora::{MTTTemplate, ContextToken7D, Tensor7D};

let template = MTTTemplate {
    id: "my_custom_template".to_string(),
    pattern: r"how to (setup|configure|install) docker".to_string(),
    response_template: r#"
# Docker Setup Guide

## Prerequisites
- Linux, macOS, or Windows with WSL2
- 4GB RAM minimum

## Installation Steps

1. **Download Docker Desktop**
   Visit https://docker.com/get-started

2. **Install**
   {{#if os == "linux"}}
   ```bash
   sudo apt-get update
   sudo apt-get install docker-ce docker-ce-cli containerd.io
   ```
   {{else}}
   Run the installer from Docker Desktop
   {{/if}}

3. **Verify Installation**
   ```bash
   docker --version
   docker run hello-world
   ```

## Next Steps
- Read Docker docs: https://docs.docker.com
- Try running a container: `docker run -it ubuntu bash`
    "#.to_string(),
    ctx7d_signature: ContextToken7D {
        tensor: Tensor7D {
            semantic: 0.75,
            temporal: 0.60,
            spatial: 0.30,
            harmonic: 0.50,
            resonant: 0.55,
            emergent: 0.65,
            void_potential: 0.25,
        },
        metadata: Default::default(),
    },
    metadata: hashmap! {
        "author" => "user_123",
        "category" => "devops",
        "version" => "1.0",
    },
};

bitacora.register_template(template).await?;
println!("Template registered!");
```

---

### **Monitoreo y Debugging**

#### **Habilitar Logging**

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

let bitacora = Bitacora::new().await?;
// Ahora verás logs detallados
```

#### **Métricas de Performance**

```rust
let stats = bitacora.get_stats().await?;

println!("Total queries: {}", stats.total_queries);
println!("Local mode: {:.1}%", stats.local_mode_queries as f64 / stats.total_queries as f64 * 100.0);
println!("Avg latency (local): {}ms", stats.avg_execution_time_ms.local);
println!("p95 latency (local): {}ms", stats.p95_latency_ms.local);
```

---

## 🖥️ GUÍA PARA END-USERS

### **Interfaz Web (Coming Soon)**

La interfaz web de Bitácora estará disponible en `http://localhost:8080`.

#### **Realizar una Query**

1. **Abrir navegador** → `http://localhost:8080`
2. **Escribir pregunta** en el campo de texto
3. **Seleccionar modo:**
   - **Auto:** Sistema decide (recomendado)
   - **Local:** Máxima velocidad
   - **LLM:** Máxima calidad
4. **Presionar Enter** o click en "Ask"
5. **Ver respuesta** en tiempo real

#### **Ver Historial**

- Click en "History" en sidebar
- Ver todas tus preguntas previas
- Click en cualquiera para ver detalles
- Exportar historial como JSON

#### **Configurar Preferencias**

- Click en "Settings"
- Ajustar:
  - **Default mode:** Auto / Local / LLM
  - **Max results:** 1-10
  - **Theme:** Light / Dark
  - **Language:** English / Español

---

### **CLI (Command Line Interface)**

```bash
# Modo interactivo
bitacora-cli

> How do I fix a lifetime error?
[Bitácora] Mode: Local (145ms)
Let's analyze the lifetime requirements...

> exit
Goodbye!

# Modo one-shot
bitacora-cli "What is Rust ownership?"

# Con opciones
bitacora-cli --mode llm --model gpt-4 "Explain Raft consensus"

# Streaming
bitacora-cli --stream "Tell me a story about Rust"
```

---

## ⚙️ GUÍA PARA ADMINISTRADORES

### **Instalación en Servidor**

#### **Opción 1: Docker (Recomendado)**

```bash
# Crear directorio de datos
mkdir -p /opt/bitacora/data

# Ejecutar container
docker run -d \
  --name bitacora \
  -p 8080:8080 \
  -v /opt/bitacora/data:/data \
  -e OPENAI_API_KEY=sk-... \
  -e ANTHROPIC_API_KEY=sk-ant-... \
  bitacora/bitacora:v1.0

# Verificar
curl http://localhost:8080/api/v1/admin/health
```

#### **Opción 2: Systemd Service**

```bash
# Compilar
cargo build --release

# Copiar binario
sudo cp target/release/bitacora /usr/local/bin/

# Crear servicio
sudo tee /etc/systemd/system/bitacora.service > /dev/null <<EOF
[Unit]
Description=Bitacora v1.0
After=network.target

[Service]
Type=simple
User=bitacora
WorkingDirectory=/opt/bitacora
ExecStart=/usr/local/bin/bitacora serve
Restart=on-failure
Environment="OPENAI_API_KEY=sk-..."
Environment="ANTHROPIC_API_KEY=sk-ant-..."

[Install]
WantedBy=multi-user.target
EOF

# Iniciar
sudo systemctl daemon-reload
sudo systemctl enable bitacora
sudo systemctl start bitacora

# Verificar
sudo systemctl status bitacora
```

---

### **Configuración**

#### **Archivo de Configuración**

`/opt/bitacora/config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080
max_connections = 100

[database]
telescope_path = "/data/telescope.db"
voxel_path = "/data/voxel.db"

[llm]
default_provider = "openai"
timeout_seconds = 30
max_retries = 3

[llm.providers.openai]
model = "gpt-4"
api_key_env = "OPENAI_API_KEY"

[llm.providers.anthropic]
model = "claude-3-opus-20240229"
api_key_env = "ANTHROPIC_API_KEY"

[performance]
local_mode_threshold = 0.85  # Similarity threshold para modo local
max_concurrent_queries = 50

[logging]
level = "info"
file = "/var/log/bitacora/bitacora.log"
```

---

### **Monitoreo**

#### **Health Check Endpoint**

```bash
curl http://localhost:8080/api/v1/admin/health

# Response:
{
  "status": "healthy",
  "components": {
    "telescope_db": "ok",
    "voxel_db": "ok",
    "llm_providers": {
      "openai": "ok",
      "anthropic": "ok"
    }
  },
  "uptime_seconds": 86400
}
```

#### **Métricas Prometheus**

```bash
curl http://localhost:8080/metrics

# Output:
bitacora_queries_total{mode="local"} 1523
bitacora_queries_total{mode="llm"} 145
bitacora_latency_seconds{mode="local",quantile="0.95"} 0.145
bitacora_latency_seconds{mode="llm",quantile="0.95"} 3.2
```

#### **Dashboard Grafana**

Importar dashboard desde `dashboards/grafana/bitacora.json`:

- **Queries per second** (por modo)
- **Latencia p50/p95/p99**
- **Tasa de error**
- **Uso de memoria/CPU**
- **Distribución de modelos LLM**

---

### **Backup y Restore**

#### **Backup**

```bash
#!/bin/bash
# backup_bitacora.sh

BACKUP_DIR="/backup/bitacora/$(date +%Y%m%d_%H%M%S)"
mkdir -p $BACKUP_DIR

# Backup databases
cp /data/telescope.db $BACKUP_DIR/
cp /data/voxel.db $BACKUP_DIR/

# Backup templates
curl http://localhost:8080/api/v1/templates/export > $BACKUP_DIR/templates.json

# Backup config
cp /opt/bitacora/config.toml $BACKUP_DIR/

echo "Backup completed: $BACKUP_DIR"
```

#### **Restore**

```bash
#!/bin/bash
# restore_bitacora.sh

BACKUP_DIR=$1

if [ -z "$BACKUP_DIR" ]; then
  echo "Usage: $0 <backup_dir>"
  exit 1
fi

# Stop service
sudo systemctl stop bitacora

# Restore databases
cp $BACKUP_DIR/telescope.db /data/
cp $BACKUP_DIR/voxel.db /data/

# Restore config
cp $BACKUP_DIR/config.toml /opt/bitacora/

# Start service
sudo systemctl start bitacora

# Import templates
curl -X POST http://localhost:8080/api/v1/templates/import \
  -H "Content-Type: application/json" \
  -d @$BACKUP_DIR/templates.json

echo "Restore completed from: $BACKUP_DIR"
```

---

### **Troubleshooting**

#### **Problema: Alta Latencia**

```bash
# Verificar queries activos
curl http://localhost:8080/api/v1/admin/metrics | jq '.active_queries'

# Ver logs
tail -f /var/log/bitacora/bitacora.log | grep "slow_query"

# Ajustar cache
# Editar config.toml:
[cache]
template_cache_size = 10000  # Aumentar
ttl_seconds = 3600
```

#### **Problema: LLM Provider Caído**

```bash
# Verificar health
curl http://localhost:8080/api/v1/admin/health

# Forzar failover
curl -X POST http://localhost:8080/api/v1/admin/config \
  -H "Content-Type: application/json" \
  -d '{"llm.default_provider": "anthropic"}'

# Restart service
sudo systemctl restart bitacora
```

#### **Problema: Espacio en Disco**

```bash
# Verificar tamaño de DBs
du -sh /data/*.db

# Compactar TelescopeDB
sqlite3 /data/telescope.db "VACUUM;"

# Limpiar frames antiguos (> 30 días)
curl -X DELETE "http://localhost:8080/api/v1/telescope/frames?older_than=30d"
```

---

## 📚 REFERENCIAS

- **API_ENDPOINTS.md:** Referencia completa de la API
- **INSTALLATION.md:** Guía de instalación detallada
- **TROUBLESHOOTING.md:** Solución de problemas comunes
- **FAQ.md:** Preguntas frecuentes

---

**Estado:** 📋 Guía completa para 3 audiencias  
**Cobertura:** Developers, End-users, Admins  
**Próxima actualización:** Con UI web (Fase 3)

---

*Generado: 2025-10-26*  
*Sistema Bitácora v1.0 - User Documentation*  
*"Empowering every user, from code to conversation"* 📖
