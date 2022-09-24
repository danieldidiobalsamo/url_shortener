#![warn(missing_docs)]

//! Required configuration to make this application work, using environment values defined in :
//! deployment/rust-url-shortener-templates/app/app-deployment.yaml

use std::env;

/// Contains all environment variables for the application
#[derive(Clone)]
pub struct Config {
    /// IP of redis server
    pub redis_ip: String,
    /// port of redis server
    pub redis_port: String,
    /// IP on which the application is listening
    pub shortener_ip: String,
    /// port on which the application is listening
    pub shortener_port: String,
}

impl Config {
    /// Returns a Config struct with all needed environment values gathered
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
