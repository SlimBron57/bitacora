# ⚙️ Configuration System Design - Arquitectura de Configuración

## 📋 **OVERVIEW**

El **Configuration System** maneja todos los aspectos de configuración del Sistema Híbrido de Navegación, desde preferencias de usuario hasta configuración específica por proyecto, topic y contexto. Diseñado para ser completamente configurable según especificación del usuario.

---

## 🏗️ **ARQUITECTURA DE CONFIGURACIÓN**

### **Hierarchy of Configuration**

```rust
pub struct ConfigurationManager {
    global_config: GlobalConfig,
    user_preferences: UserPreferences,
    project_configs: HashMap<ProjectId, ProjectConfig>,
    topic_configs: HashMap<TopicId, TopicConfig>,
    session_config: SessionConfig,
    runtime_overrides: RuntimeOverrides,
}

// Configuration hierarchy (más específico override menos específico)
// Runtime > Session > Topic > Project > User > Global
```

### **Global Configuration**

```rust
pub struct GlobalConfig {
    pub system: SystemConfig,
    pub threading: ThreadingConfig,
    pub ai: GlobalAIConfig,
    pub safety: SafetyConfig,
    pub logging: LoggingConfig,
}

pub struct SystemConfig {
    pub background_processing: bool,         // Todos los procesos en background
    pub foreground_api_only: bool,         // Foreground solo por API
    pub single_user_mode: bool,            // Sistema personal
    pub workspace_isolation: bool,         // Proyectos aislados
}

pub struct ThreadingConfig {
    pub mode: ThreadingMode,
    pub spark_pool_size: usize,
    pub project_pool_size: usize,
    pub resource_monitoring: bool,
    pub auto_throttling: bool,
}

pub enum ThreadingMode {
    Core,        // Uni-navegador
    Threads,     // Multi-navegador con niveles
    Hybrid,      // Automático según contexto
}
```

---

## 👤 **USER PREFERENCES**

### **User-Specific Configuration**

```rust
pub struct UserPreferences {
    pub ai_behavior: UserAIBehavior,
    pub interface_preferences: InterfacePreferences,
    pub workflow_preferences: WorkflowPreferences,
    pub learning_settings: LearningSettings,
}

pub struct UserAIBehavior {
    pub default_execution_mode: ExecutionMode,
    pub risk_tolerance: RiskLevel,
    pub confirmation_preferences: ConfirmationPreferences,
    pub context_preferences: ContextPreferences,
}

pub struct ConfirmationPreferences {
    // "Configurable por usuario" - todas las operaciones 
    pub auto_execute_commands: Vec<CommandCategory>,
    pub always_confirm_commands: Vec<CommandCategory>,
    pub risk_based_confirmation: RiskBasedSettings,
    pub batch_confirmation_allowed: bool,
}

pub struct RiskBasedSettings {
    pub low_risk_auto: bool,           // Auto-execute low risk commands
    pub medium_risk_confirm: bool,     // Confirm medium risk
    pub high_risk_manual: bool,        // Always require manual approval
    pub custom_risk_overrides: HashMap<CommandId, RiskLevel>,
}
```

### **Context-Specific Preferences**

```rust
pub struct ContextPreferences {
    pub development_context: DevelopmentContextConfig,
    pub planning_context: PlanningContextConfig,
    pub maintenance_context: MaintenanceContextConfig,
    pub learning_context: LearningContextConfig,
}

pub struct DevelopmentContextConfig {
    pub autonomy_level: u8,              // 1-10
    pub auto_action_logging: bool,
    pub auto_topic_switching: bool,
    pub code_analysis_depth: AnalysisDepth,
    pub preferred_focus: Vec<DevelopmentFocus>,
}

pub struct MaintenanceContextConfig {
    pub automation_level: u8,            // 1-10  
    pub cleanup_aggressiveness: u8,      // 1-10
    pub safety_checks: SafetyLevel,
    pub batch_operations: bool,
}
```

---

## 📁 **PROJECT-SPECIFIC CONFIGURATION**

### **Project Override System**

```rust
pub struct ProjectConfig {
    pub project_id: ProjectId,
    pub inherits_from: ConfigInheritance,
    pub ai_overrides: Option<ProjectAIConfig>,
    pub threading_overrides: Option<ProjectThreadingConfig>,
    pub workflow_overrides: Option<ProjectWorkflowConfig>,
    pub custom_commands: Vec<CustomCommandDefinition>,
}

pub enum ConfigInheritance {
    Global,                    // Use global defaults
    User,                     // Use user preferences  
    Template(TemplateId),     // Use project template
    Custom,                   // Fully custom configuration
}

pub struct ProjectAIConfig {
    pub override_execution_mode: Option<ExecutionMode>,
    pub project_specific_risk: HashMap<CommandId, RiskLevel>,
    pub auto_topic_creation_pattern: Option<TopicCreationPattern>,
    pub custom_ai_instructions: Option<String>,
}

pub struct TopicCreationPattern {
    pub trigger_keywords: Vec<String>,    
    pub auto_create_threshold: f32,      // 0.0-1.0 confidence needed
    pub default_topic_structure: TopicTemplate,
}
```

---

## 🎯 **TOPIC-LEVEL CONFIGURATION**

### **Fine-Grained Topic Control**

```rust
pub struct TopicConfig {
    pub topic_id: TopicId,
    pub focus_mode: FocusMode,
    pub ai_behavior_overrides: TopicAIBehavior,
    pub automation_rules: TopicAutomationRules,
    pub context_switching_rules: ContextSwitchingRules,
}

pub struct TopicAIBehavior {
    pub spark_creation_sensitivity: f32,     // 0.0-1.0
    pub auto_related_action_detection: bool,
    pub cross_topic_reference_auto: bool,
    pub documentation_auto_generation: bool,
}

pub struct TopicAutomationRules {
    pub auto_close_on_completion: bool,
    pub auto_archive_after_days: Option<u32>,
    pub auto_spark_related_topics: bool,
    pub batch_similar_actions: bool,
}

pub enum FocusMode {
    Development {
        code_focus: bool,
        testing_focus: bool,
        debugging_focus: bool,
    },
    Planning {
        architecture_focus: bool,
        requirement_focus: bool,
        design_focus: bool,
    },
    Research {
        exploration_mode: ExplorationMode,
        documentation_depth: DocumentationDepth,
    },
    Maintenance {
        cleanup_scope: CleanupScope,
        optimization_focus: bool,
    },
}
```

---

## 🔄 **CONFIGURATION SCOPES**

### **Scope Resolution System**

```rust
pub struct ConfigResolver {
    config_manager: ConfigurationManager,
}

impl ConfigResolver {
    pub fn resolve_execution_mode(
        &self,
        project_id: Option<ProjectId>,
        topic_id: Option<TopicId>,
        command_id: CommandId
    ) -> ExecutionMode {
        // Scope resolution order: Runtime > Session > Topic > Project > User > Global
        
        // 1. Check runtime overrides first
        if let Some(runtime_mode) = self.config_manager
            .runtime_overrides.execution_mode_override {
            return runtime_mode;
        }
        
        // 2. Check session config
        if let Some(session_mode) = self.config_manager
            .session_config.temporary_execution_mode {
            return session_mode;
        }
        
        // 3. Check topic-specific config
        if let Some(topic_id) = topic_id {
            if let Some(topic_config) = self.config_manager
                .topic_configs.get(&topic_id) {
                if let Some(topic_mode) = topic_config
                    .ai_behavior_overrides.execution_mode_override {
                    return topic_mode;
                }
            }
        }
        
        // 4. Check project-specific config
        if let Some(project_id) = project_id {
            if let Some(project_config) = self.config_manager
                .project_configs.get(&project_id) {
                if let Some(project_mode) = project_config
                    .ai_overrides.override_execution_mode {
                    return project_mode;
                }
            }
        }
        
        // 5. Fall back to user preferences
        if let Some(user_mode) = self.config_manager
            .user_preferences.ai_behavior.default_execution_mode {
            return user_mode;
        }
        
        // 6. Final fallback to global default
        self.config_manager.global_config.ai.default_execution_mode
    }
}
```

---

## 🎮 **CLI CONFIGURATION INTERFACE**

### **Interactive Configuration System** *(Para testing + learning)*

```rust
pub struct CLIConfigInterface {
    config_manager: ConfigurationManager,
    learning_session: Option<LearningSession>,
    test_scenarios: Vec<TestScenario>,
}

impl CLIConfigInterface {
    pub async fn start_interactive_config(&mut self) -> Result<(), ConfigError> {
        println!("🔧 Bitacora Configuration & Learning Session");
        println!("This will help configure AI behavior while testing the CLI\n");
        
        // Start learning session
        self.learning_session = Some(LearningSession::new());
        
        // Main configuration loop
        loop {
            let choice = self.present_config_menu().await?;
            
            match choice {
                ConfigChoice::TestScenarios => self.run_test_scenarios().await?,
                ConfigChoice::AIBehavior => self.configure_ai_behavior().await?,
                ConfigChoice::ThreadingMode => self.configure_threading().await?,
                ConfigChoice::ProjectDefaults => self.configure_project_defaults().await?,
                ConfigChoice::ReviewLearned => self.review_learned_preferences().await?,
                ConfigChoice::ApplyAndExit => break,
            }
        }
        
        // Apply learned configuration
        self.apply_learned_configuration().await?;
        Ok(())
    }
    
    async fn run_test_scenarios(&mut self) -> Result<(), ConfigError> {
        println!("🧪 Running CLI Test Scenarios with Configuration Learning\n");
        
        for scenario in &self.test_scenarios {
            println!("📋 Scenario: {}", scenario.name);
            println!("   {}", scenario.description);
            
            // Execute scenario and capture user preferences
            let user_response = self.execute_scenario_with_learning(scenario).await?;
            
            // Learn from user's decisions
            self.learning_session.as_mut().unwrap()
                .record_scenario_response(scenario, user_response).await?;
                
            println!("   ✓ Response recorded for learning\n");
        }
        
        Ok(())
    }
    
    async fn execute_scenario_with_learning(
        &mut self, 
        scenario: &TestScenario
    ) -> Result<ScenarioResponse, ConfigError> {
        
        match scenario.scenario_type {
            ScenarioType::ProjectCreation => {
                // Test project creation flow
                println!("   AI suggests: Create project 'feature-auth'");
                let response = self.get_user_choice(vec![
                    "1. Auto-create (AI decides always)",
                    "2. Confirm first (AI asks before creating)", 
                    "3. Manual only (I'll create projects myself)",
                ]).await?;
                
                ScenarioResponse::ProjectCreation {
                    auto_mode: match response {
                        1 => AutoMode::Always,
                        2 => AutoMode::Confirm,
                        3 => AutoMode::Manual,
                        _ => AutoMode::Confirm,
                    }
                }
            },
            
            ScenarioType::ActionLogging => {
                println!("   You completed: 'Fixed authentication bug'");
                println!("   AI suggests: Log this as action in topic 'Authentication'");
                let response = self.get_user_choice(vec![
                    "1. Auto-log all similar actions",
                    "2. Ask before logging", 
                    "3. I'll log actions manually",
                ]).await?;
                
                ScenarioResponse::ActionLogging {
                    logging_mode: match response {
                        1 => LoggingMode::Auto,
                        2 => LoggingMode::Confirm,
                        3 => LoggingMode::Manual,
                        _ => LoggingMode::Confirm,
                    }
                }
            },
            
            ScenarioType::TopicSwitching => {
                println!("   You mention: 'Now working on database schema'");
                println!("   AI suggests: Switch to topic 'Database Design'");
                let response = self.get_user_choice(vec![
                    "1. Auto-switch when context changes",
                    "2. Suggest topic switches",
                    "3. Manual topic management only",
                ]).await?;
                
                ScenarioResponse::TopicSwitching {
                    switching_mode: match response {
                        1 => SwitchingMode::Auto,
                        2 => SwitchingMode::Suggest,  
                        3 => SwitchingMode::Manual,
                        _ => SwitchingMode::Suggest,
                    }
                }
            }
        }
    }
}
```

---

## 📊 **LEARNING & ADAPTATION**

### **Configuration Learning Engine**

```rust
pub struct ConfigLearningEngine {
    decision_history: Vec<ConfigDecision>,
    pattern_analyzer: PatternAnalyzer,
    preference_generator: PreferenceGenerator,
}

pub struct ConfigDecision {
    pub timestamp: DateTime<Utc>,
    pub scenario: ScenarioType,
    pub ai_suggestion: AIDecision,
    pub user_response: UserResponse,
    pub context: ConfigContext,
}

impl ConfigLearningEngine {
    pub async fn analyze_learned_patterns(&self) -> ConfigurationSuggestions {
        let mut suggestions = ConfigurationSuggestions::new();
        
        // Analyze project creation patterns
        let project_decisions = self.decision_history.iter()
            .filter(|d| matches!(d.scenario, ScenarioType::ProjectCreation))
            .collect::<Vec<_>>();
            
        if project_decisions.len() >= 3 {
            let approval_rate = self.calculate_approval_rate(&project_decisions);
            
            if approval_rate > 0.8 {
                suggestions.add(ConfigSuggestion {
                    category: ConfigCategory::AIBehavior,
                    suggestion: "Consider setting project creation to 'Auto' mode".to_string(),
                    confidence: approval_rate,
                    impact: ConfigImpact::HighProductivity,
                });
            }
        }
        
        // Analyze risk tolerance patterns
        let risk_decisions = self.analyze_risk_tolerance_patterns().await;
        if let Some(risk_suggestion) = risk_decisions {
            suggestions.add(risk_suggestion);
        }
        
        suggestions
    }
    
    pub async fn generate_config_from_learning(&self) -> UserPreferences {
        let patterns = self.pattern_analyzer.analyze_all_patterns(&self.decision_history).await;
        
        self.preference_generator.generate_preferences(patterns).await
    }
}
```

---

## 🛠️ **CONFIGURATION TEMPLATES**

### **Pre-Built Configuration Templates**

```rust
pub struct ConfigTemplate {
    pub id: TemplateId,
    pub name: String,
    pub description: String,
    pub config: UserPreferences,
    pub target_user_type: UserType,
}

impl ConfigTemplate {
    pub fn create_templates() -> Vec<ConfigTemplate> {
        vec![
            // Conservative Developer
            ConfigTemplate {
                id: "conservative_dev",
                name: "Conservative Developer",
                description: "High safety, manual confirmations, learning enabled",
                config: UserPreferences {
                    ai_behavior: UserAIBehavior {
                        default_execution_mode: ExecutionMode::SemiAuto {
                            auto_approve: vec![CommandCategory::Documentation],
                            always_ask: vec![
                                CommandCategory::FileSystem,
                                CommandCategory::ProjectStructure,
                                CommandCategory::Deletion,
                            ],
                        },
                        risk_tolerance: RiskLevel::Low,
                        // ... rest of config
                    },
                    // ... rest
                },
                target_user_type: UserType::ConservativeDeveloper,
            },
            
            // Power User  
            ConfigTemplate {
                id: "power_user",
                name: "Power User", 
                description: "High automation, AI autonomy, minimal confirmations",
                config: UserPreferences {
                    ai_behavior: UserAIBehavior {
                        default_execution_mode: ExecutionMode::Auto {
                            risk_tolerance: RiskLevel::High,
                            confirmation_required: vec![CommandCategory::Deletion],
                        },
                        // ... rest
                    },
                    // ... rest
                },
                target_user_type: UserType::PowerUser,
            },
            
            // Learning Mode
            ConfigTemplate {
                id: "learning_mode",
                name: "Learning & Exploration",
                description: "Balanced automation with high documentation",
                config: UserPreferences {
                    ai_behavior: UserAIBehavior {
                        default_execution_mode: ExecutionMode::SemiAuto {
                            auto_approve: vec![
                                CommandCategory::Documentation,
                                CommandCategory::Logging,
                                CommandCategory::SparkCreation,
                            ],
                            always_ask: vec![CommandCategory::Deletion],
                        },
                        // ... rest with high documentation settings
                    },
                    // ... rest
                },
                target_user_type: UserType::LearnerExplorer,
            },
        ]
    }
}
```

---

## 💾 **CONFIGURATION PERSISTENCE**

### **Configuration Storage**

```rust
pub struct ConfigStorage {
    config_dir: PathBuf,
    backup_manager: ConfigBackupManager,
}

impl ConfigStorage {
    pub async fn save_configuration(
        &self,
        config: &ConfigurationManager
    ) -> Result<(), StorageError> {
        
        // Create backup first
        self.backup_manager.create_backup().await?;
        
        // Save different configuration levels
        self.save_global_config(&config.global_config).await?;
        self.save_user_preferences(&config.user_preferences).await?;
        self.save_project_configs(&config.project_configs).await?;
        self.save_topic_configs(&config.topic_configs).await?;
        
        // Save learning data
        self.save_learning_data(&config.learning_engine).await?;
        
        Ok(())
    }
    
    pub async fn load_configuration(&self) -> Result<ConfigurationManager, StorageError> {
        let mut config = ConfigurationManager::default();
        
        // Load in dependency order
        config.global_config = self.load_global_config().await?;
        config.user_preferences = self.load_user_preferences().await?;
        config.project_configs = self.load_project_configs().await?;
        config.topic_configs = self.load_topic_configs().await?;
        
        // Load learning data
        config.learning_engine = self.load_learning_data().await?;
        
        Ok(config)
    }
}

// Configuration file structure
// ~/.bitacora/config/
// ├── global.toml           # Global system configuration
// ├── user.toml            # User preferences  
// ├── learning.json        # AI learning data
// ├── projects/            # Project-specific configs
// │   ├── project_a.toml
// │   └── project_b.toml
// ├── topics/              # Topic-specific configs  
// │   ├── topic_1.toml
// │   └── topic_2.toml
// └── backups/             # Configuration backups
//     ├── 20250827_120000.tar.gz
//     └── 20250826_180000.tar.gz
```

---

## 🔧 **CONFIGURATION EXAMPLES**

### **Global Configuration (global.toml)**

```toml
[system]
background_processing = true
foreground_api_only = true  
single_user_mode = true
workspace_isolation = true

[threading]
mode = "Hybrid"
spark_pool_size = 8
project_pool_size = 4
resource_monitoring = true
auto_throttling = true

[ai]  
default_execution_mode = "SemiAuto"
risk_tolerance = "Medium"
learning_enabled = true

[safety]
conflict_detection = true
rollback_enabled = true  
risk_assessment = true
```

### **User Preferences (user.toml)**

```toml
[ai_behavior]
default_execution_mode = { type = "SemiAuto", auto_approve = ["Documentation", "Logging"], always_ask = ["FileSystem", "Deletion"] }
risk_tolerance = "Medium"

[confirmation_preferences]
batch_confirmation_allowed = true
low_risk_auto = true
medium_risk_confirm = true  
high_risk_manual = true

[context_preferences.development]
autonomy_level = 7
auto_action_logging = true
auto_topic_switching = false
code_analysis_depth = "Medium"

[context_preferences.maintenance]
automation_level = 8
cleanup_aggressiveness = 6
safety_checks = "Standard"
batch_operations = true
```

### **Project Configuration (projects/bitacora_v1.toml)**

```toml
[project]
id = "bitacora_v1"
inherits_from = "User"

[ai_overrides]
override_execution_mode = { type = "Auto", risk_tolerance = "High", confirmation_required = ["Deletion"] }

[ai_overrides.project_specific_risk]
"create_project" = "Low"
"delete_topic" = "High"  
"restructure_project" = "Medium"

[workflow_overrides]
auto_topic_creation = true
topic_creation_threshold = 0.8

[custom_commands]
[[custom_commands]]
id = "deploy_to_staging"
name = "Deploy to Staging"
risk_level = "Medium"
auto_execution = false
```

---

## 🚀 **IMPLEMENTATION ROADMAP**

### **Phase 1: Core Configuration System**
- [x] Configuration hierarchy design
- [ ] Basic ConfigurationManager implementation
- [ ] Global and User configuration loading/saving
- [ ] Simple CLI configuration interface

### **Phase 2: Scope Resolution**
- [ ] ConfigResolver implementation
- [ ] Project and Topic configuration override system
- [ ] Runtime configuration override system
- [ ] Configuration validation and conflict resolution

### **Phase 3: Learning Integration** 
- [ ] ConfigLearningEngine implementation
- [ ] Interactive CLI configuration with learning
- [ ] Pattern analysis and preference generation
- [ ] Configuration suggestion system

### **Phase 4: Advanced Features**
- [ ] Configuration templates system
- [ ] Backup and restore functionality
- [ ] Configuration migration tools
- [ ] Advanced CLI configuration interface

---

## 🎯 **INTEGRATION POINTS**

### **With AI Decision Engine**
- Configuration feeds execution mode decisions
- Risk tolerance affects AI behavior  
- Learning data updates preferences automatically

### **With Threading System**
- Threading mode configuration affects executor selection
- Resource limits from configuration control thread pools
- Safety settings influence lock strategies

### **With CLI Testing** *(Según especificación)*
- Interactive configuration during CLI testing
- Learning from user decisions in test scenarios  
- Configuration validation through real usage
- Preference refinement through testing cycles

---

*Configuration System Design - Bitacora V1.0 Hybrid Navigator*  
*Documentado el 27 de Agosto, 2025*
