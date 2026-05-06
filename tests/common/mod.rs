//! Common test utilities and helpers

pub mod mock_serial;

use serial_mcp_rs::serial::{ConnectionConfig, DataBits, StopBits, Parity, FlowControl};

/// Create a test connection configuration
pub fn test_connection_config(port: &str) -> ConnectionConfig {
    ConnectionConfig {
        port: port.to_string(),
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        stop_bits: StopBits::One,
        parity: Parity::None,
        flow_control: FlowControl::None,
    }
}