//! Embedded Serial MCP Library
//! 
//! A comprehensive Model Context Protocol server for serial port communication.
//! Provides AI assistants with serial communication capabilities including
//! port discovery, connection management, data transmission, and protocol handling.

pub mod config;
pub mod error;
pub mod utils;
pub mod serial;
pub mod session;
pub mod tools;

// Re-export main types for convenience
pub use config::{Config, Args};
pub use error::{SerialError, Result};
pub use serial::{ConnectionManager, SerialConnection, PortInfo};
pub use session::{SessionManager, SerialSession, SessionState};
pub use tools::SerialHandler;
pub use utils::{DataFormat, DataConverter, PortType};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");