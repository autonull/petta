//! Engine configuration types
//!
//! This module provides configuration types for the PeTTa execution engine,
//! including backend selection and engine settings.

use std::path::{Path, PathBuf};

// Re-export core config types
pub use crate::engine::{EngineConfig, EngineConfigBuilder};

// Re-export backend enum
pub use crate::engine::Backend;
