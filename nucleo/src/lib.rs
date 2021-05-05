
pub mod config {
    #[derive(Debug, Default, Clone)]
    pub struct ServerConfig{
        pub host: String,
        pub port: u32,
        pub ttl: u32,
        pub buffer_size: u32,
        pub timeout_read: u32,
        pub timeout_write: u32,
        pub encoding: String,
        pub data_dir: String
    }
}

pub mod socket {
}