//! Mock serial port implementation for testing

use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// A mock serial port that can be used for testing
#[derive(Clone)]
pub struct MockSerialPort {
    /// Data to be read from the port
    read_buffer: Arc<Mutex<VecDeque<u8>>>,
    /// Data written to the port
    write_buffer: Arc<Mutex<Vec<u8>>>,
    /// Whether the port is open
    is_open: Arc<Mutex<bool>>,
    /// Port name
    name: String,
}

impl MockSerialPort {
    pub fn new(name: &str) -> Self {
        Self {
            read_buffer: Arc::new(Mutex::new(VecDeque::new())),
            write_buffer: Arc::new(Mutex::new(Vec::new())),
            is_open: Arc::new(Mutex::new(false)),
            name: name.to_string(),
        }
    }

    /// Add data to be read from the mock port
    pub fn add_read_data(&self, data: &[u8]) {
        let mut buffer = self.read_buffer.lock().unwrap();
        buffer.extend(data);
    }

    /// Get all data that was written to the mock port
    pub fn get_written_data(&self) -> Vec<u8> {
        self.write_buffer.lock().unwrap().clone()
    }

    /// Clear the write buffer
    pub fn clear_write_buffer(&self) {
        self.write_buffer.lock().unwrap().clear();
    }

    /// Open the mock port
    pub fn open(&self) -> io::Result<()> {
        let mut is_open = self.is_open.lock().unwrap();
        if *is_open {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Port is already open",
            ));
        }
        *is_open = true;
        Ok(())
    }

    /// Close the mock port
    pub fn close(&self) -> io::Result<()> {
        let mut is_open = self.is_open.lock().unwrap();
        if !*is_open {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Port is not open",
            ));
        }
        *is_open = false;
        Ok(())
    }

    /// Check if the port is open
    pub fn is_open(&self) -> bool {
        *self.is_open.lock().unwrap()
    }
}

impl Read for MockSerialPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.is_open() {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Port is not open",
            ));
        }

        let mut read_buffer = self.read_buffer.lock().unwrap();
        let available = read_buffer.len().min(buf.len());
        
        for i in 0..available {
            if let Some(byte) = read_buffer.pop_front() {
                buf[i] = byte;
            }
        }
        
        Ok(available)
    }
}

impl Write for MockSerialPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.is_open() {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Port is not open",
            ));
        }

        let mut write_buffer = self.write_buffer.lock().unwrap();
        write_buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_serial_port_open_close() {
        let port = MockSerialPort::new("MOCK1");
        
        assert!(!port.is_open());
        assert!(port.open().is_ok());
        assert!(port.is_open());
        
        // Cannot open again
        assert!(port.open().is_err());
        
        assert!(port.close().is_ok());
        assert!(!port.is_open());
        
        // Cannot close again
        assert!(port.close().is_err());
    }

    #[test]
    fn test_mock_serial_port_read_write() {
        let mut port = MockSerialPort::new("MOCK1");
        
        // Cannot read/write when closed
        let mut buf = [0u8; 10];
        assert!(port.read(&mut buf).is_err());
        assert!(port.write(b"test").is_err());
        
        // Open port
        assert!(port.open().is_ok());
        
        // Write data
        assert_eq!(port.write(b"Hello").unwrap(), 5);
        assert_eq!(port.get_written_data(), b"Hello");
        
        // Add read data
        port.add_read_data(b"World");
        
        // Read data
        let mut buf = [0u8; 10];
        let n = port.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"World");
        
        // No more data to read
        let n = port.read(&mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_mock_serial_port_partial_read() {
        let mut port = MockSerialPort::new("MOCK1");
        port.open().unwrap();
        
        // Add 10 bytes
        port.add_read_data(b"0123456789");
        
        // Read only 5 bytes
        let mut buf = [0u8; 5];
        let n = port.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"01234");
        
        // Read remaining 5 bytes
        let n = port.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"56789");
    }
}