# 🎯 BitaFlow Template Guide

## Overview

BitaFlow templates (.bfl files) define autonomous navigation workflows using a combination of YAML front-matter and Markdown-based navigation flows. This guide covers everything you need to create powerful, reusable templates.

## Template Structure

Every BFL template consists of two main parts:

```yaml
---
# YAML Front-matter (metadata)
name: "Template Name"
alias: "BITA-NAV-DOMAIN-TOPIC-v1"
# ... more metadata
---

# Navigation Flow (Markdown)
## Steps
1. **First step** → `{{variable}}`
2. **Second step** → `{{another_variable}}`
```

## YAML Front-matter Reference

### Required Fields

```yaml
---
# Human-readable template name
name: "Debug Error Navigator"

# Unique identifier following BITA-NAV-{DOMAIN}-{TOPIC}-v{VERSION} pattern
alias: "BITA-NAV-DEBUG-ERROR-v1"

# Domain specialization
domain: "debug"

# Specific topic within domain
topic: "error"

# Template version (incremented for updates)
version: 1

# Autonomy level: Full | Interactive | Restricted | Manual
autonomy_level: "Interactive"

# Threading level: SparkIsolated | ProjectIsolated | TopicSerial | FullSerial
thread_level: "ProjectIsolated"
---
```

### Optional Fields

```yaml
---
# Template description
description: "Autonomous error debugging and resolution workflow"

# Template author information
author: "BitacoraTeam"
email: "team@bitacora.dev"

# Template tags for searchability
tags: ["debug", "error", "automation", "analysis"]

# Minimum navigator version required
min_navigator_version: "0.1.0"

# Template variables with defaults
variables:
  error_type: "{{error_type}}"
  severity: "HIGH"
  max_analysis_time: "300" # 5 minutes
  
# Template configuration
config:
  timeout_seconds: 600
  max_file_size: "10MB"
  allowed_extensions: [".rs", ".js", ".py", ".java"]
---
```

## Navigation Flow Syntax

### Basic Steps

Use numbered lists or bullet points to define navigation steps:

```markdown
# Navigation Flow

## Phase 1: Analysis
1. **Extract stack trace files** → `{{stack_trace_files}}`
2. **Identify error line** → `{{error_line}}`
3. **Map file dependencies** → `{{file_dependencies}}`

## Phase 2: Investigation  
- **Read error context** (±20 lines)
- **Search similar patterns** → `{{similar_patterns}}`
- **Analyze recent changes** → `{{recent_changes}}`
```

### Variable Substitution

Variables are defined using double braces `{{variable_name}}`:

```markdown
# Error Analysis
1. **Analyze error: {{error_message}}** in file `{{error_file}}`
2. **Check severity level: {{severity}}**
3. **Set timeout to {{max_analysis_time}} seconds**
```

### Conditional Steps

Use conditional syntax for optional steps:

```markdown
# Conditional Analysis
- **Required**: Analyze stack trace
- **Optional**: Check git history `{{?include_git_history}}`
- **If High Severity**: Notify team `{{?severity=HIGH}}`
- **If Production**: Create incident `{{?environment=production}}`
```

### Interactive Prompts

Define user interaction points:

```markdown
# User Interaction
1. **Analyze error automatically**
2. **Ask user**: Continue with fix suggestions? `{{ask_continue}}`
3. **If yes**: Generate fix recommendations
4. **If no**: Save analysis results only
```

### Output Variables

Specify what outputs the template generates:

```markdown
# Outputs Generated
- `{{stack_trace_files}}` - List of files in stack trace
- `{{error_functions}}` - Functions involved in error
- `{{suggested_fixes}}` - Recommended solutions
- `{{confidence_score}}` - Confidence in suggestions (0-1)
```

## Template Categories

### 1. Debug Templates
```yaml
domain: "debug"
topics: ["error", "performance", "memory", "crash", "regression"]
```

**Example**: Debug Error Navigator
```yaml
---
name: "Debug Error Navigator"
alias: "BITA-NAV-DEBUG-ERROR-v1"
domain: "debug"
topic: "error"
autonomy_level: "Interactive"
---

# Debug Flow
1. **Parse error message** → `{{error_details}}`
2. **Locate error source** → `{{source_location}}`
3. **Find root cause** → `{{root_cause}}`
4. **Suggest fixes** → `{{fix_suggestions}}`
```

### 2. Code Review Templates
```yaml
domain: "code-review"  
topics: ["pull-request", "security", "performance", "style", "architecture"]
```

**Example**: PR Review Navigator
```yaml
---
name: "Pull Request Review Navigator"
alias: "BITA-NAV-CODE-REVIEW-PR-v1"
domain: "code-review"
topic: "pull-request"
autonomy_level: "Restricted"
---

# Review Flow
1. **Fetch PR details** → `{{pr_info}}`
2. **Analyze code changes** → `{{code_analysis}}`
3. **Run security checks** → `{{security_results}}`
4. **Generate review comments** → `{{review_comments}}`
```

### 3. Research Templates
```yaml
domain: "research"
topics: ["architecture", "library", "pattern", "solution", "benchmark"]
```

**Example**: Architecture Research Navigator
```yaml
---
name: "Architecture Research Navigator"
alias: "BITA-NAV-RESEARCH-ARCHITECTURE-v1"  
domain: "research"
topic: "architecture"
autonomy_level: "Full"
---

# Research Flow
1. **Define research scope** → `{{scope_definition}}`
2. **Search existing patterns** → `{{pattern_analysis}}`
3. **Evaluate alternatives** → `{{alternatives}}`
4. **Recommend solution** → `{{recommendation}}`
```

### 4. Documentation Templates
```yaml
domain: "documentation"
topics: ["api", "guide", "tutorial", "reference", "changelog"]
```

### 5. Testing Templates
```yaml
domain: "testing"
topics: ["unit", "integration", "e2e", "performance", "security"]
```

## Advanced Features

### Variable Processing

```yaml
variables:
  # Simple substitution
  project_name: "{{project}}"
  
  # Default values
  timeout: "{{timeout:300}}"
  
  # Computed values
  timestamp: "{{now()}}"
  random_id: "{{uuid()}}"
  
  # Conditional values
  log_level: "{{severity=HIGH ? 'DEBUG' : 'INFO'}}"
```

### Template Inheritance

```yaml
---
# Extend from base template
extends: "BITA-NAV-BASE-ANALYSIS-v1"

# Override specific fields
autonomy_level: "Interactive"
thread_level: "ProjectIsolated"

# Add new variables
variables:
  specialized_param: "{{custom_value}}"
---

# Additional steps (appended to parent)
## Specialized Analysis
1. **Custom analysis step** → `{{specialized_param}}`
```

### Multi-Phase Workflows

```yaml
---
name: "Multi-Phase CI/CD Navigator"
alias: "BITA-NAV-CICD-FULL-v1"
phases:
  - name: "build"
    autonomy_level: "Full"
    thread_level: "ProjectIsolated"
  - name: "test"  
    autonomy_level: "Interactive"
    thread_level: "TopicSerial"
  - name: "deploy"
    autonomy_level: "Manual"
    thread_level: "FullSerial"
---

# Phase 1: Build
1. **Compile code** → `{{build_result}}`
2. **Generate artifacts** → `{{artifacts}}`

# Phase 2: Test
1. **Run unit tests** → `{{test_results}}`
2. **Ask**: Continue to integration tests? `{{ask_integration}}`

# Phase 3: Deploy
1. **Manual approval required** → `{{deployment_approval}}`
2. **Deploy to staging** → `{{staging_result}}`
```

## Metrics and Learning

Templates automatically collect metrics:

```yaml
---
# Metrics configuration
metrics:
  track_execution_time: true
  track_user_satisfaction: true
  track_success_rate: true
  track_pattern_recognition: true
  
  # Custom metrics
  custom_metrics:
    - name: "files_analyzed"
      type: "counter"
    - name: "accuracy_score" 
      type: "gauge"
---
```

## Best Practices

### 1. Template Naming
- Use descriptive, action-oriented names
- Follow domain-topic hierarchy
- Include version numbers
- Use consistent naming conventions

```yaml
# Good
name: "Debug Memory Leak Navigator"
alias: "BITA-NAV-DEBUG-MEMORY-LEAK-v1"

# Bad
name: "Helper"  
alias: "HELPER-1"
```

### 2. Variable Design
- Use clear, descriptive variable names
- Provide sensible defaults
- Document variable purposes
- Group related variables

```yaml
variables:
  # Clear naming
  error_message: "{{error_msg}}"
  stack_trace_file: "{{trace_file}}"
  
  # Sensible defaults
  max_analysis_time: "{{timeout:300}}"
  confidence_threshold: "{{threshold:0.7}}"
```

### 3. Flow Structure
- Break complex workflows into phases
- Use clear step descriptions
- Indicate expected outputs
- Handle error cases

```markdown
# Good Structure
## Phase 1: Preparation
1. **Initialize workspace** → `{{workspace_ready}}`
2. **Validate inputs** → `{{inputs_valid}}`

## Phase 2: Execution  
1. **Process data** → `{{processed_data}}`
2. **Handle errors if any** → `{{error_handled}}`

## Phase 3: Cleanup
1. **Save results** → `{{results_saved}}`
2. **Clean temporary files** → `{{cleanup_complete}}`
```

### 4. Autonomy Levels
Choose appropriate autonomy levels:

- **Full**: Well-tested, safe operations
- **Interactive**: Operations requiring user judgment
- **Restricted**: Potentially risky operations  
- **Manual**: Critical or irreversible operations

### 5. Threading Levels
Select based on resource requirements:

- **SparkIsolated**: CPU-intensive operations
- **ProjectIsolated**: File system operations
- **TopicSerial**: Sequential dependencies
- **FullSerial**: Shared resource access

## Template Testing

### Unit Testing Templates
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_debug_template() {
        let mut engine = create_test_engine();
        let alias = engine.load_template_from_file("debug-error.bfl").await.unwrap();
        
        let mut context = HashMap::new();
        context.insert("error_type".into(), "NullPointer".into());
        
        let result = engine.execute_template(&alias, context).await.unwrap();
        
        assert!(result.success);
        assert!(result.execution_time < 10.0);
        assert!(!result.actions_taken.is_empty());
    }
}
```

## Template Repository Structure

```
templates/
├── debug/
│   ├── error-analysis.bfl
│   ├── performance-debug.bfl
│   └── memory-leak.bfl
├── code-review/
│   ├── pull-request.bfl
│   ├── security-review.bfl
│   └── architecture-review.bfl
├── research/
│   ├── library-evaluation.bfl
│   └── pattern-research.bfl
└── shared/
    ├── base-analysis.bfl
    └── common-utils.bfl
```

## Template Versioning

Follow semantic versioning:
- **v1 → v2**: Major workflow changes
- **v1.1 → v1.2**: Minor improvements  
- **v1.1.1 → v1.1.2**: Bug fixes

```yaml
# Version history example
versions:
  - version: 1
    description: "Initial template"
    date: "2025-08-28"
  - version: 2
    description: "Added ML-based pattern detection"
    date: "2025-09-15"
    breaking_changes: ["Removed legacy variable {{old_param}}"]
```

---

*For more examples, see the [templates/](../templates/) directory and [examples/](../examples/) folder.*
