#![warn(missing_docs)]

//! Required configuration to make this application work, using environment values defined in :
//! deployment/rust-url-shortener/values.yaml

use std::env;

/// Contains all environment variables for the application
#[derive(Clone)]
pub struct Config {
    /// read only endpoint to redis cluster (leader and followers)
    pub redis_ro_endpoint: String,
    /// read write endpoint to cluster leader
    pub redis_rw_endpoint: String,
    /// socket on which url-shortener is listening
    pub app_socket: String,
}

impl Config {
    /// Returns a Config struct with all needed environment values gathered
    pub fn new() -> Config {
        let redis_ro_endpoint = env::var("REDIS_RO_ENDPOINT")
            .expect("environment variable not defined : REDIS_RO_ENDPOINT");
        let redis_rw_endpoint = env::var("REDIS_RW_ENDPOINT")
            .expect("environment variable not defined : REDIS_RW_ENDPOINT");
        let app_socket =
            env::var("APP_SOCKET").expect("environment variable not defined : APP_SOCKET");

        Config {
            redis_ro_endpoint,
            redis_rw_endpoint,
            app_socket,
        }
    }

    /// Split socket string into separate ip/port variables
    pub fn split_app_socket(&self) -> (String, u16) {
        let mut infos = self.app_socket.split(":");

        let ip = String::from(infos.next().unwrap());
        let port = infos.next().unwrap().parse::<u16>().unwrap();

        (ip, port)
    }
}
