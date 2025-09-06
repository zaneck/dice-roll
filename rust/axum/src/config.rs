use std::net::Ipv4Addr;

pub struct Config {
    pub title: String,
    pub description: String,
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl Config {
    pub fn default_config() -> Self {
        Config {
            title: "Dice Roll API".to_string(),
            description: "API for rolling dice".to_string(),
            ip: Ipv4Addr::UNSPECIFIED,
            port: 8080,
        }
    }
}
