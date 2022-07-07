use std::env;

#[derive(Clone)]
pub struct Config {
    pub redis_ip: String,
    pub redis_port: String,
    pub shortener_ip: String,
    pub shortener_port: String,
}

impl Config {
    pub fn new() -> Config {
        let redis_ip = env::var("REDIS_IP").expect("environment variable not defined : REDIS_IP");
        let redis_port =
            env::var("REDIS_PORT").expect("environment variable not defined : REDIS_PORT");
        let shortener_ip =
            env::var("SHORTENER_IP").expect("environment variable not defined : SHORTENER_IP");
        let shortener_port =
            env::var("SHORTENER_PORT").expect("environment variable not defined : SHORTENER_PORT");

        Config {
            redis_ip,
            redis_port,
            shortener_ip,
            shortener_port,
        }
    }
}
