# 🔧📊 Monitor & Reporter Systems - Architectural Planning

**Fecha de Planificación**: 22 Agosto 2025  
**Estado**: Arquitectura Definida - Listo para Implementación  
**Integración**: Sistemas independientes que se conectan a Bitacora

## 🏗️ **ARQUITECTURA DE MONITOR SYSTEM**

### **📋 Visión General**
El **Monitor System** es un crate **independiente** que funciona como un servicio externo de monitoreo de hardware y sistema. Bitacora se conecta a él para obtener métricas en tiempo real.

### **🔧 Componentes Principales**

```
monitor/
├── src/
│   ├── lib.rs                    # API externa para Bitacora
│   ├── hardware/                 # Monitoreo de hardware
│   │   ├── cpu_monitor.rs        # CPU: usage, temp, freq
│   │   ├── memory_monitor.rs     # RAM, swap, virtual memory
│   │   ├── disk_monitor.rs       # Disk: usage, I/O, health
│   │   ├── gpu_monitor.rs        # GPU: usage, memory, temp
│   │   └── sensors.rs            # Sensores: temp, fans, power
│   ├── system/                   # Monitoreo del sistema
│   │   ├── os_info.rs            # OS, uptime, kernel
│   │   ├── load_monitor.rs       # System load, processes
│   │   └── service_monitor.rs    # Services, daemons
│   ├── network/                  # Monitoreo de red
│   │   ├── traffic_monitor.rs    # Bandwidth, packets
│   │   ├── interface_monitor.rs  # Network interfaces
│   │   └── connection_monitor.rs # Active connections
│   ├── process/                  # Monitoreo de procesos
│   │   ├── process_monitor.rs    # Process tree, resources
│   │   └── bitacora_monitor.rs   # Bitacora-specific monitoring
│   ├── metrics/                  # Recolección de métricas
│   │   ├── collector.rs          # Central collector
│   │   ├── aggregator.rs         # Aggregation & averaging
│   │   └── exporter.rs           # Export: JSON, Prometheus
│   └── collectors/               # Implementaciones de collectors
│       ├── realtime_collector.rs # Tiempo real
│       ├── periodic_collector.rs # Intervalos programados
│       └── event_collector.rs    # Basado en eventos
├── examples/
│   └── monitor_demo.rs           # Ejemplo de uso
└── README.md                     # Documentación completa
```

### **📊 Métricas que Recolecta**

#### **Hardware Metrics**
- **CPU**: Usage %, temperature, frequency, cores, threads
- **Memory**: RAM used/total, swap, virtual memory, page faults
- **Disk**: Usage %, I/O read/write, health status, temperature
- **GPU**: Usage %, memory used/total, temperature, power consumption
- **Sensors**: System temperature, fan speeds, power consumption

#### **System Metrics**
- **OS**: Version, uptime, kernel version, architecture
- **Load**: System load average (1m, 5m, 15m), running processes
- **Services**: Running services, failed services, service status

#### **Network Metrics**
- **Traffic**: Bandwidth in/out, packets sent/received, errors
- **Interfaces**: Interface status, IP addresses, connection quality
- **Connections**: Active TCP/UDP connections, listening ports

#### **Process Metrics**
- **General**: Process count, threads, memory usage per process
- **Bitacora-specific**: Resource usage of Bitacora processes

### **🔌 API Externa para Bitacora**

```rust
// API que Bitacora usa para conectarse
pub struct MonitorClient {
    endpoint: String,
}

impl MonitorClient {
    pub async fn get_system_metrics(&self) -> Result<SystemMetrics, MonitorError>;
    pub async fn get_hardware_metrics(&self) -> Result<HardwareMetrics, MonitorError>;
    pub async fn get_network_metrics(&self) -> Result<NetworkMetrics, MonitorError>;
    pub async fn get_process_metrics(&self) -> Result<ProcessMetrics, MonitorError>;
    pub async fn start_realtime_monitoring(&self) -> Result<MetricsStream, MonitorError>;
}
```

---

## 📝 **ARQUITECTURA DE REPORTER SYSTEM**

### **📋 Visión General**
El **Reporter System** es un sistema modular de 3 crates que pueden compilarse independientemente:
- **reporter-core**: Funcionalidad base (modelos, traits, config)
- **reporter-production**: Para producción (MongoDB, Elasticsearch, Kafka, Prometheus)
- **reporter-dev**: Para desarrollo (debug, testing, mocks)

### **🏗️ Estructura Multi-Crate**

```
reporter/
├── core/                         # Crate base (siempre requerido)
│   ├── src/
│   │   ├── models/
│   │   │   ├── log_entry.rs      # Estructura de log entries
│   │   │   ├── log_level.rs      # Log levels (Error, Warn, Info, Debug, Trace)
│   │   │   └── log_context.rs    # Contexto de logging
│   │   ├── traits/
│   │   │   ├── collector_trait.rs # Trait para collectors
│   │   │   ├── processor_trait.rs # Trait para processors  
│   │   │   └── exporter_trait.rs  # Trait para exporters
│   │   └── config/
│   │       └── reporter_config.rs # Configuración base
│   └── Cargo.toml
├── production/                   # Crate para producción
│   ├── src/
│   │   ├── aggregation/
│   │   │   ├── time_window.rs    # Agregación por ventanas de tiempo
│   │   │   └── metrics_calc.rs   # Cálculo de métricas
│   │   ├── storage/
│   │   │   ├── mongodb_storage.rs # Almacenamiento en MongoDB
│   │   │   └── elasticsearch_storage.rs # Integración Elasticsearch
│   │   └── export/
│   │       ├── prometheus_exporter.rs # Métricas a Prometheus
│   │       └── kafka_exporter.rs      # Streaming a Kafka
│   └── Cargo.toml                # Depende de reporter-core
└── dev/                         # Crate para desarrollo
    ├── src/
    │   ├── debug/
    │   │   ├── log_formatter.rs  # Pretty printing de logs
    │   │   └── log_viewer.rs     # Viewer interactivo
    │   ├── testing/
    │   │   ├── mock_collector.rs # Mock collectors para tests
    │   │   └── test_data_gen.rs  # Generación de datos de prueba
    │   └── mock/
    │       └── mock_storage.rs   # Mock storage implementations
    └── Cargo.toml               # Depende de reporter-core
```

### **🎯 Compilación Independiente**

```bash
# Compilar solo core (mínimo funcional)
cargo build --package reporter-core

# Compilar production + core (para producción)
cargo build --package reporter-production

# Compilar dev + core (para desarrollo)
cargo build --package reporter-dev

# Compilar todo el sistema reporter
cargo build --workspace --include="reporter-*"
```

### **📊 Funcionalidades por Crate**

#### **reporter-core**: Base Funcional
- ✅ Modelos de log entries
- ✅ Traits para extensibilidad
- ✅ Configuración básica
- ✅ Error handling

#### **reporter-production**: Para Producción
- ✅ Agregación de logs por ventanas de tiempo
- ✅ Storage en MongoDB y Elasticsearch
- ✅ Export a Prometheus metrics
- ✅ Streaming a Kafka
- ✅ Compresión y optimización

#### **reporter-dev**: Para Desarrollo
- ✅ Pretty printing con colores
- ✅ Viewer interactivo de logs
- ✅ Mock implementations para testing
- ✅ Generación de datos de prueba
- ✅ Debug utilities

---

## 🔗 **INTEGRACIÓN CON BITACORA**

### **Monitor Integration**
```rust
// En bitacora-core o bitacora-commands
use monitor::MonitorClient;

pub struct BitacoraMonitorIntegration {
    monitor_client: MonitorClient,
}

impl BitacoraMonitorIntegration {
    pub async fn collect_development_metrics(&self, session_id: &Uuid) -> Result<DevMetrics, Error> {
        let system_metrics = self.monitor_client.get_system_metrics().await?;
        let hardware_metrics = self.monitor_client.get_hardware_metrics().await?;
        
        // Combinar con métricas de sesión de Bitacora
        Ok(DevMetrics::combine(session_metrics, system_metrics, hardware_metrics))
    }
}
```

### **Reporter Integration**
```rust
// En bitacora-core o bitacora-session
use reporter_core::traits::*;
use reporter_production::storage::MongoDbStorage;

pub struct BitacoraReportingSystem {
    storage: Box<dyn LogStorage>,
}

impl BitacoraReportingSystem {
    pub async fn log_session_event(&self, session_id: &Uuid, event: SessionEvent) -> Result<(), Error> {
        let log_entry = LogEntry::from_session_event(session_id, event);
        self.storage.store_log(log_entry).await?;
        Ok(())
    }
}
```

---

## 🎯 **PRÓXIMOS PASOS**

### **Implementación Prioritaria**
1. **✅ Completar bitacora-commands** (próximo inmediato)
2. **🔧 Implementar monitor system** (independiente, alta prioridad)
3. **📝 Implementar reporter-core** (base para logging)
4. **🚀 Integrar monitor + reporter con Bitacora**

### **Beneficios Estratégicos**

#### **Monitor System**
- 📊 Correlación entre performance de desarrollo y recursos del sistema
- 🔍 Identificación de bottlenecks durante sesiones de desarrollo
- 📈 Métricas históricas para optimización de workflow
- ⚠️ Alertas proactivas sobre recursos del sistema

#### **Reporter System**
- 📝 Logging comprehensivo de todas las operaciones Bitacora
- 🔍 Debug y troubleshooting mejorados
- 📊 Analytics avanzados sobre patrones de desarrollo
- 🎯 Observabilidad completa del sistema

---

## 🚀 **LISTOS PARA IMPLEMENTACIÓN**

Ambos sistemas están **arquitecturalmente definidos** y listos para implementación después de completar **bitacora-commands**. Proporcionarán capacidades de **observabilidad de clase enterprise** al ecosistema Bitacora.

**Estado**: ✅ **PLANIFICACIÓN COMPLETA** - Listos para desarrollo
