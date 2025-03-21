//! # Constants
//!
//! Constants and types used throughout the application

/// Local host IPv4 address and Redis port
pub const LOCAL_SOCKET_ADDR_STR: &str = "127.0.0.1:6379";

/// Local host IPv4 address and arbitrary port chosen by OS
pub const LOCAL_SOCKET_ADDR_STR_TEST: &str = "127.0.0.1:0";

/// Length of buffer for handling connections, 512 bytes
pub const BUFFER_LEN: usize = 512;

/// Application exit codes
#[derive(Debug)]
pub enum ExitCode {
    Ok = 0,
    Shutdown = -1,
}
