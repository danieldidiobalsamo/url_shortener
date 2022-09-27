#![warn(missing_docs)]

//! Required configuration to make this application work, using environment values defined in :
//! deployment/rust-url-shortener/values.yaml

use std::env;

/// Contains all environment variables for the application
#[derive(Clone)]
pub struct Config {
    /// socket on which redis server is listening
    pub redis_socket: String,
    /// socket on which url-shortener is listening
    pub app_socket: String,
}

impl Config {
    /// Returns a Config struct with all needed environment values gathered
    pub fn new() -> Config {
        let redis_socket =
            env::var("REDIS_SOCKET").expect("environment variable not defined : REDIS_SOCKET");
        let app_socket =
            env::var("APP_SOCKET").expect("environment variable not defined : APP_SOCKET");

        Config {
            redis_socket,
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
