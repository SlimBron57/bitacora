# 🤖 AI Decision Engine - Arquitectura y Comportamiento

## 📋 **OVERVIEW**

El **AI Decision Engine** es el cerebro del Sistema Híbrido de Navegación que automatiza la toma de decisiones sobre qué comandos ejecutar, cuándo ejecutarlos, y cómo manejar los resultados. Basado en las especificaciones del usuario para configuración total por el usuario.

---

## 🧠 **ARQUITECTURA DEL MOTOR AI**

### **Componentes Principales**

```rust
pub struct AIDecisionEngine {
    pub execution_mode: ExecutionMode,
    pub context_analyzer: ContextAnalyzer,
    pub command_registry: CommandRegistry,
    pub decision_logger: DecisionLogger,
    pub risk_assessor: RiskAssessor,
    pub user_preferences: UserPreferences,
}

// Modos de ejecución configurables por usuario
pub enum ExecutionMode {
    Auto {
        risk_tolerance: RiskLevel,
        confirmation_required: Vec<CommandCategory>,
    },
    SemiAuto {
        auto_approve: Vec<CommandCategory>,
        always_ask: Vec<CommandCategory>,
    },
    Manual {
        suggestions_enabled: bool,
        reasoning_visible: bool,
    },
}
```

### **Context Analysis System**

```rust
pub struct ContextAnalyzer {
    project_state: ProjectStateAnalyzer,
    topic_flow: TopicFlowAnalyzer, 
    action_history: ActionHistoryAnalyzer,
    spark_pattern: SparkPatternAnalyzer,
    ai_learning: AILearningEngine,
}

impl ContextAnalyzer {
    pub async fn analyze_current_context(
        &self,
        project_id: ProjectId,
        current_topic: Option<TopicId>,
        recent_actions: Vec<ActionId>
    ) -> ContextAnalysis {
        
        let project_state = self.project_state
            .analyze_project_status(project_id).await;
            
        let topic_momentum = self.topic_flow
            .analyze_topic_progression(current_topic).await;
            
        let action_patterns = self.action_history
            .find_relevant_patterns(recent_actions).await;
            
        let spark_insights = self.spark_pattern
            .extract_contextual_sparks(project_id).await;
        
        ContextAnalysis {
            project_state,
            topic_momentum, 
            action_patterns,
            spark_insights,
            confidence_score: self.calculate_confidence(&project_state, &topic_momentum),
            suggested_commands: self.generate_command_suggestions().await,
        }
    }
}
```

---

## 🎯 **COMMAND REGISTRY & AI DESCRIPTIONS**

### **Command Definition para AI**

```rust
pub struct CommandDefinition {
    pub id: CommandId,
    pub name: String,
    pub ai_description: AIDescription,
    pub execution_info: ExecutionInfo,
    pub safety_info: SafetyInfo,
    pub context_requirements: ContextRequirements,
}

pub struct AIDescription {
    pub purpose: String,              // "Creates a new topic for organizing related actions"
    pub when_to_use: String,          // "Use when starting work on a new feature or concept"
    pub expected_outcome: String,     // "New topic created with proper hierarchy"
    pub risk_level: RiskLevel,        // Low | Medium | High
    pub ai_confidence_needed: f32,    // 0.0 - 1.0, minimum confidence to auto-suggest
}

pub struct ExecutionInfo {
    pub parameters: Vec<ParameterDefinition>,
    pub execution_mode: CommandExecutionMode,
    pub rollback_possible: bool,
    pub affects_state: bool,
}

pub enum CommandExecutionMode {
    Immediate,        // Execute right away
    Batched,         // Can be batched with other commands
    UserConfirm,     // Always requires user confirmation
    AIDecision,      // Let AI decide based on context
}
```

### **Registry de Comandos con Descriptions**

```rust
impl CommandRegistry {
    pub fn initialize_ai_commands() -> Self {
        let mut registry = CommandRegistry::new();
        
        // PROJECT Commands
        registry.register(CommandDefinition {
            id: "create_project",
            name: "Create New Project",
            ai_description: AIDescription {
                purpose: "Initializes a new development project with proper structure",
                when_to_use: "When user mentions starting a new project, app, or major feature",
                expected_outcome: "New project created with folder structure and initial topic",
                risk_level: RiskLevel::Low,
                ai_confidence_needed: 0.8,
            },
            execution_info: ExecutionInfo {
                parameters: vec![
                    ParameterDefinition::required("name", "Project name"),
                    ParameterDefinition::optional("description", "Project description"),
                ],
                execution_mode: CommandExecutionMode::AIDecision,
                rollback_possible: true,
                affects_state: true,
            },
            // ... rest of definition
        });
        
        // TOPIC Commands  
        registry.register(CommandDefinition {
            id: "create_topic", 
            name: "Create New Topic",
            ai_description: AIDescription {
                purpose: "Creates a focused topic within a project for grouping related work",
                when_to_use: "When switching to a new area of work or organizing actions by theme",
                expected_outcome: "New topic created and set as active working context",
                risk_level: RiskLevel::Low,
                ai_confidence_needed: 0.7,
            },
            execution_info: ExecutionInfo {
                parameters: vec![
                    ParameterDefinition::required("project_id", "Parent project"),
                    ParameterDefinition::required("title", "Topic title"),
                    ParameterDefinition::optional("description", "Detailed description"),
                ],
                execution_mode: CommandExecutionMode::Immediate,
                rollback_possible: false,
                affects_state: true,
            },
            // ... rest
        });
        
        // ACTION Commands
        registry.register(CommandDefinition {
            id: "log_action",
            name: "Log Development Action", 
            ai_description: AIDescription {
                purpose: "Records a specific development action with context and results",
                when_to_use: "After completing any significant development task or decision",
                expected_outcome: "Action logged with proper categorization and searchable context",
                risk_level: RiskLevel::Low,
                ai_confidence_needed: 0.9,
            },
            execution_info: ExecutionInfo {
                parameters: vec![
                    ParameterDefinition::required("topic_id", "Current topic"),
                    ParameterDefinition::required("action_type", "Type of action performed"),
                    ParameterDefinition::required("description", "What was done"),
                    ParameterDefinition::optional("code_changes", "Related code changes"),
                ],
                execution_mode: CommandExecutionMode::Immediate,
                rollback_possible: true,
                affects_state: true,
            },
            // ... rest
        });
        
        registry
    }
}
```

---

## 🎛️ **CONFIGURATION SYSTEM**

### **User Configuration Scopes**

```rust
pub struct UserPreferences {
    pub global_config: GlobalAIConfig,
    pub project_configs: HashMap<ProjectId, ProjectAIConfig>,
    pub topic_configs: HashMap<TopicId, TopicAIConfig>,
    pub learning_preferences: LearningConfig,
}

pub struct GlobalAIConfig {
    pub default_execution_mode: ExecutionMode,
    pub risk_tolerance: RiskLevel,
    pub auto_execution_categories: Vec<CommandCategory>,
    pub always_confirm_categories: Vec<CommandCategory>,
    pub learning_enabled: bool,
}

pub struct ProjectAIConfig {
    pub project_id: ProjectId,
    pub override_global: bool,
    pub execution_mode: Option<ExecutionMode>,
    pub custom_risk_levels: HashMap<CommandId, RiskLevel>,
    pub auto_topic_creation: bool,
    pub auto_action_logging: bool,
}

pub struct TopicAIConfig {
    pub topic_id: TopicId,
    pub focus_mode: FocusMode,
    pub auto_related_actions: bool,
    pub spark_sensitivity: f32,       // 0.0-1.0, how readily to create sparks
}

pub enum FocusMode {
    Development,    // Prioritize coding actions
    Planning,       // Prioritize structure and organization  
    Research,       // Prioritize documentation and learning
    Maintenance,    // Prioritize cleanup and optimization
}
```

### **Configuración CLI para Testing**

**Según especificación del usuario:** *"esto también nos servirá cuando implementemos el CLI para poder probar y configurar al mismo tiempo que vamos aprendiendo las configuraciones correctas para guiar a la AI dentro de bitacora"*

```rust
pub struct CLIConfigManager {
    config_path: PathBuf,
    current_session: SessionConfig,
    learning_mode: bool,
}

impl CLIConfigManager {
    pub async fn interactive_config_session(&mut self) -> Result<(), ConfigError> {
        println!("🤖 AI Configuration Learning Session");
        println!("Let's configure how AI should behave in different situations...\n");
        
        // Test diferentes escenarios en vivo
        self.test_project_creation_scenario().await?;
        self.test_topic_switching_scenario().await?;
        self.test_action_logging_scenario().await?;
        
        // Aprender de las decisiones del usuario
        self.update_preferences_from_session().await?;
        
        Ok(())
    }
    
    async fn test_project_creation_scenario(&mut self) -> Result<(), ConfigError> {
        println!("📁 Scenario: You mention starting a new feature...");
        println!("AI suggests: 'Create new project: feature-authentication'");
        
        let response = self.get_user_response(
            "Should AI auto-create projects like this? (always/ask/never)"
        ).await?;
        
        match response.as_str() {
            "always" => {
                self.current_session.project_creation = AutoMode::Always;
                println!("✓ AI will auto-create projects when context is clear");
            },
            "ask" => {
                self.current_session.project_creation = AutoMode::Confirm;
                println!("✓ AI will suggest but ask for confirmation");
            },
            "never" => {
                self.current_session.project_creation = AutoMode::Manual;
                println!("✓ AI will not auto-create projects");
            },
            _ => return Err(ConfigError::InvalidResponse),
        }
        
        Ok(())
    }
}
```

---

## 🧐 **AI DECISION LOGIC**

### **Decision Tree per Command**

```rust
pub struct DecisionEngine {
    context: ContextAnalysis,
    user_prefs: UserPreferences,
    command_registry: CommandRegistry,
}

impl DecisionEngine {
    pub async fn should_execute_command(
        &self,
        command_id: CommandId,
        context: &ContextAnalysis
    ) -> AIDecision {
        
        let command_def = self.command_registry.get(&command_id)?;
        
        // 1. Check user configuration first
        let user_preference = self.get_user_preference_for_command(&command_id);
        
        match user_preference {
            UserPreference::AlwaysExecute => {
                return AIDecision::Execute {
                    confidence: 1.0,
                    reasoning: "User configured to always execute this command type".to_string(),
                };
            },
            UserPreference::NeverExecute => {
                return AIDecision::Skip {
                    reasoning: "User configured to never auto-execute this command".to_string(),
                };
            },
            UserPreference::DecideBasedOnContext => {
                // Continue with AI analysis
            }
        }
        
        // 2. Analyze context relevance
        let context_score = self.analyze_context_relevance(command_def, context);
        
        // 3. Assess risk level
        let risk_assessment = self.assess_command_risk(command_def, context);
        
        // 4. Check AI confidence threshold
        let ai_confidence = self.calculate_ai_confidence(context_score, risk_assessment);
        
        if ai_confidence < command_def.ai_description.ai_confidence_needed {
            return AIDecision::Suggest {
                confidence: ai_confidence,
                reasoning: format!(
                    "Confidence {} below threshold {}", 
                    ai_confidence, 
                    command_def.ai_description.ai_confidence_needed
                ),
            };
        }
        
        // 5. Final decision based on execution mode
        match self.user_prefs.global_config.default_execution_mode {
            ExecutionMode::Auto { risk_tolerance, .. } => {
                if risk_assessment.level <= risk_tolerance {
                    AIDecision::Execute {
                        confidence: ai_confidence,
                        reasoning: format!("Auto-execution approved: risk {} <= tolerance {}", 
                                         risk_assessment.level, risk_tolerance),
                    }
                } else {
                    AIDecision::Confirm {
                        confidence: ai_confidence,
                        reasoning: "Risk exceeds auto-execution tolerance".to_string(),
                    }
                }
            },
            ExecutionMode::SemiAuto { auto_approve, .. } => {
                if auto_approve.contains(&command_def.category) {
                    AIDecision::Execute { confidence: ai_confidence, reasoning: "Category auto-approved".to_string() }
                } else {
                    AIDecision::Confirm { confidence: ai_confidence, reasoning: "Category requires confirmation".to_string() }
                }
            },
            ExecutionMode::Manual { .. } => {
                AIDecision::Suggest {
                    confidence: ai_confidence,
                    reasoning: "Manual mode - suggesting only".to_string(),
                }
            }
        }
    }
}

pub enum AIDecision {
    Execute { 
        confidence: f32, 
        reasoning: String 
    },
    Confirm { 
        confidence: f32, 
        reasoning: String 
    },
    Suggest { 
        confidence: f32, 
        reasoning: String 
    },
    Skip { 
        reasoning: String 
    },
}
```

---

## 🎓 **AI LEARNING ENGINE**

### **Learning from User Decisions**

```rust
pub struct AILearningEngine {
    decision_history: DecisionHistory,
    pattern_detector: PatternDetector,
    preference_updater: PreferenceUpdater,
}

impl AILearningEngine {
    pub async fn learn_from_user_decision(
        &mut self,
        context: ContextAnalysis,
        ai_suggestion: AIDecision,
        user_response: UserResponse
    ) -> Result<(), LearningError> {
        
        // Record the decision
        let decision_record = DecisionRecord {
            timestamp: Utc::now(),
            context: context.clone(),
            ai_suggestion,
            user_response: user_response.clone(),
            session_id: self.get_current_session_id(),
        };
        
        self.decision_history.record(decision_record).await?;
        
        // Analyze patterns
        if let Some(pattern) = self.pattern_detector.detect_new_pattern(&decision_record).await? {
            // Update user preferences based on pattern
            self.preference_updater.update_from_pattern(pattern).await?;
            
            // Log learning for user visibility
            info!("🧠 AI Learning: Detected preference pattern for {} commands in {} context", 
                  pattern.command_category, pattern.context_type);
        }
        
        Ok(())
    }
    
    pub async fn suggest_configuration_improvements(&self) -> Vec<ConfigSuggestion> {
        let mut suggestions = Vec::new();
        
        // Analyze recent decision patterns
        let recent_decisions = self.decision_history.get_recent(100).await;
        
        // Find inconsistencies
        if let Some(inconsistency) = self.find_configuration_inconsistencies(&recent_decisions) {
            suggestions.push(ConfigSuggestion {
                category: inconsistency.category,
                suggestion: format!(
                    "You've been {} approving {} commands. Consider changing auto-execution to {}",
                    inconsistency.frequency,
                    inconsistency.command_type,
                    inconsistency.suggested_mode
                ),
                confidence: inconsistency.confidence,
            });
        }
        
        suggestions
    }
}
```

---

## 🔍 **DIFFERENT AI CONTEXTS** *(Según discusión)*

### **Context-Specific Behaviors**

```rust
pub enum AIContext {
    Development { 
        autonomy_level: u8,       // 1-10, how autonomous AI should be
        focus: DevelopmentFocus,  // Coding, Debugging, Testing, etc.
    },
    Planning { 
        validation_level: u8,     // 1-10, how much validation needed
        scope: PlanningScope,     // Architecture, Feature, Task, etc.
    },
    Maintenance { 
        automation_level: u8,     // 1-10, how much to automate
        safety: SafetyLevel,      // Conservative, Standard, Aggressive
    },
    Learning { 
        documentation_level: u8,  // 1-10, how much to document
        exploration: ExplorationMode, // Guided, Free, Structured
    },
}

impl AIDecisionEngine {
    pub fn adjust_behavior_for_context(&mut self, context: AIContext) {
        match context {
            AIContext::Development { autonomy_level, focus } => {
                // Higher autonomy = more auto-execution
                if autonomy_level >= 8 {
                    self.execution_mode = ExecutionMode::Auto { 
                        risk_tolerance: RiskLevel::Medium,
                        confirmation_required: vec![CommandCategory::FileSystem],
                    };
                }
                
                // Focus affects command priorities
                self.context_analyzer.set_development_focus(focus);
            },
            
            AIContext::Planning { validation_level, scope } => {
                // Higher validation = more confirmations
                if validation_level >= 7 {
                    self.execution_mode = ExecutionMode::SemiAuto {
                        auto_approve: vec![CommandCategory::Documentation],
                        always_ask: vec![CommandCategory::Structure, CommandCategory::Architecture],
                    };
                }
            },
            
            AIContext::Maintenance { automation_level, safety } => {
                // High automation + conservative safety
                if automation_level >= 8 && safety == SafetyLevel::Conservative {
                    self.execution_mode = ExecutionMode::Auto {
                        risk_tolerance: RiskLevel::Low,
                        confirmation_required: vec![CommandCategory::Deletion, CommandCategory::FileSystem],
                    };
                }
            },
            
            AIContext::Learning { documentation_level, exploration } => {
                // Always document in learning mode
                self.force_documentation = documentation_level >= 6;
                self.spark_creation_threshold = 0.3; // Lower threshold for creating sparks
            }
        }
    }
}
```

---

## 📊 **MONITORING & OBSERVABILITY**

### **AI Decision Metrics**

```rust
pub struct AIMetrics {
    pub decisions_made: u64,
    pub auto_executions: u64,
    pub user_confirmations: u64,
    pub user_overrides: u64,
    pub accuracy_score: f32,        // How often user agrees with AI suggestions
    pub learning_progress: f32,     // How much AI has improved
}

pub struct DecisionLogger {
    metrics: Arc<Mutex<AIMetrics>>,
    decision_log: Vec<DecisionLogEntry>,
}

impl DecisionLogger {
    pub async fn log_ai_decision(
        &mut self,
        decision: AIDecision,
        context: ContextAnalysis,
        outcome: DecisionOutcome
    ) {
        let entry = DecisionLogEntry {
            timestamp: Utc::now(),
            decision,
            context,
            outcome,
            user_satisfaction: None, // Filled later if user provides feedback
        };
        
        self.decision_log.push(entry);
        self.update_metrics(&entry).await;
        
        // Log for observability
        info!("🤖 AI Decision: {:?} with confidence {:.2} - Outcome: {:?}",
              entry.decision, entry.get_confidence(), entry.outcome);
    }
}
```

---

## 🚀 **IMPLEMENTACIÓN INCREMENTAL**

### **Phase 1: Basic Decision Engine**
```rust
// Core decision logic
- ExecutionMode::Manual solamente
- Command registry básico
- Context analysis simple
```

### **Phase 2: Configuration System**
```rust
// User preferences
- GlobalAIConfig implementation
- CLI configuration interface
- Learning básico from user decisions
```

### **Phase 3: Advanced AI**
```rust
// Full AI capabilities
- ExecutionMode::Auto and SemiAuto
- Context-specific behaviors  
- Advanced learning engine
```

### **Phase 4: Optimization**
```rust
// Performance & accuracy
- Decision optimization
- Learning algorithm tuning
- Metrics & observability dashboard
```

---

## 📝 **CONFIGURATION EXAMPLES**

### **Conservative User**
```toml
[ai.global]
default_execution_mode = "SemiAuto"
risk_tolerance = "Low" 
learning_enabled = true

[ai.auto_approve]
categories = ["Documentation", "Logging"]

[ai.always_confirm] 
categories = ["FileSystem", "ProjectStructure", "Deletion"]
```

### **Power User**
```toml
[ai.global]
default_execution_mode = "Auto"
risk_tolerance = "High"
learning_enabled = true

[ai.auto_approve]
categories = ["All"]

[ai.always_confirm]
categories = ["Deletion"]

[ai.contexts]
development_autonomy = 9
maintenance_automation = 10
```

---

## 🎯 **INTEGRATION CON CLI TESTING**

### **CLI Configuration Flow**

```bash
# Start configuration learning session
bitacora config ai --interactive

# Test specific scenarios  
bitacora config ai --test-scenario project_creation
bitacora config ai --test-scenario action_logging
bitacora config ai --test-scenario topic_switching

# Review learned preferences
bitacora config ai --review-learning

# Apply learned configuration
bitacora config ai --apply-learned
```

Este sistema permite que el CLI testing y la configuración AI sucedan simultáneamente, exactamente como especificó el usuario: *"para poder probar y configurar al mismo tiempo que vamos aprendiendo las configuraciones correctas"*.

---

*AI Decision Engine Architecture - Bitacora V1.0 Hybrid Navigator*  
*Documentado el 27 de Agosto, 2025*
