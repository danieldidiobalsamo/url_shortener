#![warn(missing_docs)]

//! Provides an abstraction layer between redis server and backend

use redis::{Commands, Connection, FromRedisValue, RedisResult};

/// A struct that implements all necessary methods to interact with redis server
pub struct RedisClient {
    connection: Connection,
}

impl RedisClient {
    /// Makes a connection with redis://{ip}:{port}
    pub fn new(ip: &str, port: &str) -> RedisClient {
        let url = format!("redis://{}:{}", ip, port);

        let client =
            redis::Client::open(url.clone()).unwrap_or_else(|_| panic!("Bad url: {}", url));
        let connection = match client.get_connection() {
            Ok(connection) => connection,
            Err(err) => {
                println!("Can't create connection with redis server at '{url}'");
                panic!("{:?} : {:?} {:?}", err.category(), err.kind(), err.detail());
            }
        };

        Self {
            connection: connection,
        }
    }

    /// Performs "set <short_url> <full_url>"
    pub fn add_url<T: FromRedisValue>(
        &mut self,
        short_url: &str,
        full_url: &str,
    ) -> RedisResult<T> {
        self.connection.set(short_url, full_url)
    }

    /// Performs "get <short_url>" and returns full url
    pub fn get_full_url<T: FromRedisValue>(&mut self, short_url: &str) -> RedisResult<T> {
        self.connection.get(short_url)
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

        client.add_url(&short, &full).unwrap_or_else(|err| {
            panic!(
                "Can't set key/value on redis: {:?} {:?} | {:?} {:?}",
                short,
                full,
                err.kind(),
                err.detail()
            )
        });

        let url_from_server: String = match client.get_full_url(&short) {
            Ok(url) => url,
            Err(err) => {
                panic!("{:?} {:?}", err.kind(), err.detail());
            }
        };

        assert_eq!(full, url_from_server);
    }
}
