//! Handler modules for different command types

// Legacy handlers
pub mod session;
pub mod git;
pub mod template;
pub mod storage;
pub mod status;
pub mod config;
pub mod help;

// NEW: PROJECT → TOPIC → ACTION sequential architecture handlers (simplified)
pub mod simple_project;
pub mod simple_topic;
pub mod simple_action;

// NEW: SPARK transversal service handler (simplified)
pub mod simple_spark;

// NEW: WORKFLOW integration handler (simplified)
pub mod simple_workflow;

// Legacy handler exports
pub use session::SessionHandler;
pub use git::GitHandler;
pub use template::TemplateHandler;
pub use storage::StorageHandler;
pub use status::StatusHandler;
pub use config::ConfigHandler;
pub use help::HelpHandler;

// NEW: Sequential architecture handler exports (simplified for demonstration)
pub use simple_project::ProjectHandler;
pub use simple_topic::TopicHandler;
pub use simple_action::ActionHandler;

// NEW: Transversal service handler exports (simplified)
pub use simple_spark::SparkHandler;

// NEW: Workflow integration handler exports (simplified)
pub use simple_workflow::WorkflowHandler;
