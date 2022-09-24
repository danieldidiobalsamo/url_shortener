#![warn(missing_docs)]

//! Provides an abstraction layer between redis server and backend

use redis::{Commands, Connection};

/// A struct that implements all necessary methods to interact with redis server
pub struct RedisClient {
    connection: Connection,
}

impl RedisClient {
    /// Makes a connection with redis://{ip}:{port}
    pub fn new(ip: &str, port: &str) -> RedisClient {
        let ip = format!("redis://{}:{}", ip, port);
        let client = redis::Client::open(ip).unwrap();
        let connection = client.get_connection().unwrap();

        Self {
            connection: connection,
        }
    }

    /// Returns a mutable reference to redis server connection
    pub fn get_connection(&mut self) -> &mut Connection {
        &mut self.connection
    }

    /// Performs "set <short_url> <full_url>"
    pub fn add_url(&mut self, short_url: &str, full_url: &str) {
        let connnection = self.get_connection();

        let _: () = connnection.set(short_url, full_url).unwrap();
    }

    /// Performs "get <short_url>" and returns full url
    pub fn get_full_url(&mut self, short_url: &str) -> String {
        let connnection = self.get_connection();
        let full: String = connnection.get(short_url).unwrap();

        full
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_client() -> RedisClient {
        RedisClient::new("127.0.0.1", "6379")
    }

    #[test]
    fn add_key_and_get() {
        let full = "https://crates.io/";
        let short = "d2af598";

        let mut client = setup_client();

        client.add_url(short, full);

        let url_from_server = client.get_full_url(short);

        assert_eq!(full, url_from_server);
    }
}
