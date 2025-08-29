# 🧭 BitacoraNavigator - HybridNavigator with BitaFlow Integration

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-Active%20Development-green.svg)]()

> **The Revolutionary Navigation System for Autonomous Development Workflows**

BitacoraNavigator combines intelligent AI-powered navigation with the BitaFlow DSL to create autonomous specialized navigators that learn, evolve, and execute complex development workflows automatically.

## 🚀 **What is HybridNavigator?**

HybridNavigator is a sophisticated navigation system that provides:

- **🧵 4-Level Threading System** - From SparkIsolated to FullSerial execution
- **🤖 AI Decision Engine** - Smart navigation decisions with learning capabilities
- **🔒 Safety Controller** - Deadlock detection and resource management
- **📊 Performance Metrics** - Real-time execution analytics
- **🎯 BitaFlow Integration** - Template-based autonomous workflows

## 🔥 **BitaFlow Integration - The Game Changer**

The revolutionary combination of HybridNavigator + BitaFlow DSL creates:

### **Autonomous Specialized Navigators**
```rust
// Load a specialized debugging template
let debug_navigator = engine.load_template_from_file("debug-error-trace.bfl").await?;

// Execute autonomous error resolution
let result = engine.execute_template("BITA-NAV-DEBUG-ERROR-v1", context).await?;
```

### **Template-Driven Workflows**
```yaml
---
name: "Debug Error Navigator" 
alias: "BITA-NAV-DEBUG-ERROR-v1"
domain: "debug"
topic: "error"
autonomy_level: "Interactive"
thread_level: "ProjectIsolated"
---

# Navigation Flow
1. **Extract stack trace files** → `{{stack_trace_files}}`
2. **Identify error line** → `{{error_line}}`  
3. **Map file dependencies** → `{{file_dependencies}}`
4. **Search recent changes** → `{{recent_changes}}`
```

## 🏗️ **Architecture Overview**

```
┌─────────────────────────────────────────────────────────┐
│                HybridNavigator Core                     │
├─────────────────┬─────────────────┬─────────────────────┤
│ Threading Mgr   │ AI Engine       │ Safety Controller   │
│ • 4 Levels      │ • Decision Tree │ • Deadlock Detect  │
│ • Concurrency   │ • Learning      │ • Resource Locks   │ 
│ • Isolation     │ • Feedback      │ • Conflict Avoid   │
└─────────────────┴─────────────────┴─────────────────────┘
                            │
┌─────────────────────────────────────────────────────────┐
│              BitaFlow Integration Engine               │
├─────────────────┬─────────────────┬─────────────────────┤
│ Template Mgr    │ Alias Validator │ Execution Engine    │
│ • .bfl Parsing  │ • BITA-NAV-*    │ • Step Runner       │
│ • YAML Meta     │ • Versioning    │ • Autonomy Levels   │
│ • Variables     │ • Validation    │ • Flow Control      │
└─────────────────┴─────────────────┴─────────────────────┘
```

## ⚡ **Quick Start**

### **1. Add Dependency**
```toml
[dependencies]
bitacora-navigator = "0.1.0"
```

### **2. Create Navigator**
```rust
use bitacora_navigator::{HybridNavigator, BitaflowNavigatorEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize navigator
    let navigator = HybridNavigator::new_core()?;
    let mut engine = BitaflowNavigatorEngine::new(navigator);
    
    // Load template
    let alias = engine.load_template_from_file("my-template.bfl").await?;
    
    // Execute workflow
    let result = engine.execute_template(&alias, context).await?;
    
    println!("Workflow completed: {}", result.output);
    Ok(())
}
```

### **3. Create Your First Template**
```yaml
---
name: "Code Review Navigator"
alias: "BITA-NAV-CODE-REVIEW-PR-v1" 
domain: "code-review"
topic: "pr"
autonomy_level: "Interactive"
thread_level: "ProjectIsolated"
variables:
  pr_number: "{{pr_id}}"
  reviewer: "{{user_name}}"
---

# Code Review Navigation Flow
1. **Fetch PR details** → `{{pr_details}}`
2. **Analyze changed files** → `{{changed_files}}`
3. **Run static analysis** → `{{analysis_results}}`
4. **Generate review comments** → `{{review_comments}}`
5. **Post review** → `{{review_posted}}`
```

## 🎯 **Core Features**

### **Threading Levels**
- **Level 0: SparkIsolated** - Maximum isolation for critical operations
- **Level 1: ProjectIsolated** - Isolated within project boundaries  
- **Level 2: TopicSerial** - Serial execution within topic scope
- **Level 3: FullSerial** - Completely serial execution

### **Autonomy Levels**
- **Full** - Complete autonomous execution
- **Interactive** - Confirmation prompts for critical actions
- **Restricted** - Only pre-approved actions allowed
- **Manual** - Full user control with step logging

### **Template System**
- **YAML Front-matter** - Rich metadata and configuration
- **BFL Navigation Flow** - Step-by-step workflow definition
- **Variable Substitution** - Dynamic template personalization
- **Version Management** - Template evolution and backwards compatibility

## 📊 **Performance & Metrics**

The system automatically tracks:
- **Execution Time** - Performance optimization
- **Success Rate** - Template effectiveness  
- **User Satisfaction** - Feedback integration
- **Pattern Recognition** - Learning improvement
- **Resource Utilization** - Threading efficiency

## 🔧 **Development**

### **Build**
```bash
cargo build
```

### **Test**
```bash
cargo test
```

### **Run Examples**
```bash
cargo run --example bitaflow_demo
```

## 📚 **Documentation**

- **[API Documentation](docs/API.md)** - Complete API reference
- **[Template Guide](docs/TEMPLATE_GUIDE.md)** - Creating BFL templates
- **[Architecture](docs/ARCHITECTURE.md)** - System design deep-dive
- **[Examples](examples/)** - Working code examples

## 🤝 **Contributing**

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔮 **Roadmap**

- [x] HybridNavigator Core Architecture
- [x] BitaFlow Integration Engine  
- [x] Template Execution System
- [ ] YAML Parser Integration
- [ ] Variable Substitution System
- [ ] AI Template Generator
- [ ] Template Learning Engine
- [ ] VelaKeys Integration

## 💎 **The Vision**

BitacoraNavigator represents the future of development workflows - autonomous, intelligent, and continuously evolving. It's not just a tool, it's **The Treasure of Bitacora** 🔥

---

**Made with ❤️ by the Bitacora Team** | **[Website](https://bitacora.dev)** | **[Documentation](https://docs.bitacora.dev)**
