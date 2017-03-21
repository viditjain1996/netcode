use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};
use std::io;

pub const NETCODE_CONNECT_TOKEN_BYTES: usize = 4096;
pub const NETCODE_KEY_BYTES: usize = 32;
pub const NETCODE_USER_DATA_BYTES: usize = 256;
pub const NETCODE_CONNECT_TOKEN_PRIVATE_BYTES: usize = 1024;
pub const NETCODE_TIMEOUT_SECONDS: u32 = 5;

pub const NETCODE_MAX_SERVERS_PER_CONNECT: usize = 16;

pub const NETCODE_MAX_CLIENTS: usize = 256;
pub const NETCODE_MAX_PACKET_SIZE: usize = 1200;

pub const NETCODE_VERSION_LEN: usize = 13;
pub const NETCODE_VERSION_STRING: &'static [u8; NETCODE_VERSION_LEN] = b"NETCODE 1.00\0";
pub const NETCODE_CHALLENGE_TOKEN_BYTES: usize = 360;
