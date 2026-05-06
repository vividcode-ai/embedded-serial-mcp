//! Configuration management for the serial MCP server
//! 
//! This module provides comprehensive configuration handling including command line
//! arguments, configuration files, validation, and logging setup.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use clap::Parser;
use crate::error::{SerialError, ConfigError, Result};

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "serial-mcp-rs")]
#[command(about = "A Model Context Protocol server for serial port communication")]
#[command(version)]
pub struct Args {
    /// Path to configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Log file path
    #[arg(long)]
    pub log_file: Option<PathBuf>,

    /// Maximum number of concurrent connections
    #[arg(long, default_value = "10")]
    pub max_connections: usize,

    /// Connection timeout in seconds
    #[arg(long, default_value = "30")]
    pub connection_timeout: u64,

    /// Default baud rate for serial connections
    #[arg(long, default_value = "115200")]
    pub default_baud_rate: u32,

    /// Default timeout for operations in milliseconds
    #[arg(long, default_value = "1000")]
    pub default_timeout_ms: u64,

    /// Maximum buffer size in bytes
    #[arg(long, default_value = "8192")]
    pub max_buffer_size: usize,

    /// Connection retry count
    #[arg(long, default_value = "3")]
    pub retry_count: u32,

    /// Enable auto-discovery of serial ports
    #[arg(long)]
    pub auto_discovery: bool,

    /// Allow multiple connections to the same port
    #[arg(long)]
    pub allow_port_sharing: bool,

    /// Restrict port access to specific patterns
    #[arg(long)]
    pub restrict_ports: bool,

    /// Generate default configuration file
    #[arg(long)]
    pub generate_config: bool,

    /// Validate configuration and exit
    #[arg(long)]
    pub validate_config: bool,

    /// Show current configuration and exit
    #[arg(long)]
    pub show_config: bool,
}

/// Main configuration structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub serial: SerialConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            serial: SerialConfig::default(),
            security: SecurityConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load(config_path: Option<&PathBuf>) -> Result<Self> {
        if let Some(path) = config_path {
            let content = std::fs::read_to_string(path)
                .map_err(|e| SerialError::InvalidConfig(format!("Failed to read config file: {}", e)))?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| SerialError::InvalidConfig(format!("Invalid TOML syntax: {}", e)))?;
            config.validate()?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Merge command line arguments into configuration
    pub fn merge_args(&mut self, args: &Args) {
        self.server.max_connections = args.max_connections;
        self.server.connection_timeout_seconds = args.connection_timeout;
        self.serial.default_baud_rate = args.default_baud_rate;
        self.serial.default_timeout_ms = args.default_timeout_ms;
        self.serial.max_buffer_size = args.max_buffer_size;
        self.serial.retry_count = args.retry_count;
        self.serial.auto_discovery = args.auto_discovery;
        self.serial.allow_port_sharing = args.allow_port_sharing;
        self.security.restrict_ports = args.restrict_ports;
        self.logging.level = args.log_level.clone();
        self.logging.file = args.log_file.clone();
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Server validation
        if self.server.max_connections == 0 {
            return Err(ConfigError::InvalidValue {
                field: "server.max_connections".to_string(),
                value: "0".to_string(),
            }.into());
        }

        if self.server.max_connections > 1000 {
            return Err(ConfigError::ValueOutOfRange {
                field: "server.max_connections".to_string(),
                value: self.server.max_connections.to_string(),
                min: "1".to_string(),
                max: "1000".to_string(),
            }.into());
        }

        // Serial validation
        if self.serial.default_baud_rate == 0 {
            return Err(ConfigError::InvalidValue {
                field: "serial.default_baud_rate".to_string(),
                value: "0".to_string(),
            }.into());
        }

        let valid_baud_rates = [300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 28800, 38400, 57600, 115200, 230400, 460800, 921600];
        if !valid_baud_rates.contains(&self.serial.default_baud_rate) {
            return Err(ConfigError::InvalidValue {
                field: "serial.default_baud_rate".to_string(),
                value: self.serial.default_baud_rate.to_string(),
            }.into());
        }

        if self.serial.max_buffer_size == 0 {
            return Err(ConfigError::InvalidValue {
                field: "serial.max_buffer_size".to_string(),
                value: "0".to_string(),
            }.into());
        }

        if self.serial.max_buffer_size > 1024 * 1024 {  // 1MB max
            return Err(ConfigError::ValueOutOfRange {
                field: "serial.max_buffer_size".to_string(),
                value: self.serial.max_buffer_size.to_string(),
                min: "1".to_string(),
                max: "1048576".to_string(),
            }.into());
        }

        // Logging validation
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            return Err(ConfigError::InvalidValue {
                field: "logging.level".to_string(),
                value: self.logging.level.clone(),
            }.into());
        }

        Ok(())
    }

    /// Generate TOML configuration string
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self)
            .map_err(|e| SerialError::InvalidConfig(format!("Failed to serialize config: {}", e)))
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub max_connections: usize,
    pub connection_timeout_seconds: u64,
    pub worker_threads: Option<usize>,
    pub enable_metrics: bool,
    pub metrics_interval_seconds: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            connection_timeout_seconds: 30,
            worker_threads: None,
            enable_metrics: false,
            metrics_interval_seconds: 60,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialConfig {
    pub default_baud_rate: u32,
    pub default_data_bits: u8,
    pub default_stop_bits: String,
    pub default_parity: String,
    pub default_flow_control: String,
    pub default_timeout_ms: u64,
    pub max_buffer_size: usize,
    pub retry_count: u32,
    pub retry_delay_ms: u64,
    pub auto_discovery: bool,
    pub discovery_interval_seconds: u64,
    pub allow_port_sharing: bool,
    pub default_line_ending: String,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            default_baud_rate: 115200,
            default_data_bits: 8,
            default_stop_bits: "One".to_string(),
            default_parity: "None".to_string(),
            default_flow_control: "None".to_string(),
            default_timeout_ms: 1000,
            max_buffer_size: 8192,
            retry_count: 3,
            retry_delay_ms: 1000,
            auto_discovery: false,
            discovery_interval_seconds: 5,
            allow_port_sharing: false,
            default_line_ending: "\n".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    pub restrict_ports: bool,
    pub allowed_ports: Vec<String>,
    pub blocked_ports: Vec<String>,
    pub max_data_size: usize,
    pub rate_limit_enabled: bool,
    pub rate_limit_requests_per_second: u32,
    pub enable_authentication: bool,
    pub allowed_clients: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            restrict_ports: false,
            allowed_ports: vec![],
            blocked_ports: vec![],
            max_data_size: 65536,  // 64KB
            rate_limit_enabled: false,
            rate_limit_requests_per_second: 100,
            enable_authentication: false,
            allowed_clients: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<PathBuf>,
    pub format: String,
    pub timestamp_format: String,
    pub include_location: bool,
    pub include_thread_names: bool,
    pub rotate_logs: bool,
    pub max_log_files: usize,
    pub max_log_size_mb: usize,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file: None,
            format: "text".to_string(),
            timestamp_format: "rfc3339".to_string(),
            include_location: false,
            include_thread_names: false,
            rotate_logs: false,
            max_log_files: 10,
            max_log_size_mb: 10,
        }
    }
}

