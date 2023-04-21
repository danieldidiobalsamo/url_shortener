#![warn(missing_docs)]

//! Required configuration to make this application work, using environment values defined in :
//! deployment/rust-url-shortener/values.yaml

use std::{env, fs};

/// Contains all environment variables for the application
#[derive(Clone)]
pub struct Config {
    /// read only endpoint to redis cluster (leader and followers)
    pub redis_ro_endpoint: String,
    /// read write endpoint to cluster leader
    pub redis_rw_endpoint: String,
    /// user that is used by the app to connect to redis cluster
    pub redis_user: String,
    /// password of the user that is used by the app to connect to redis cluster
    pub redis_password: String,
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

        // reading the k8s secret containing redis credentials
        let redis_password =
            fs::read_to_string("/etc/redis-passwd/passwd").expect("can't find redis credentials");
        //////////////////////////////TMP
        let redis_user = String::from("default");

        Config {
            redis_ro_endpoint,
            redis_rw_endpoint,
            redis_user,
            redis_password,
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
