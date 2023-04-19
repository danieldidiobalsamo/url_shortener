#![warn(missing_docs)]

//! Provides an abstraction layer between redis server and backend

use redis::{Commands, Connection, FromRedisValue, RedisResult};

/// A struct that implements all necessary methods to interact with redis server
pub struct RedisClient {
    ro_connection: Connection, // read only connection to cluster (leader and followers)
    rw_connection: Connection, // read write connection to cluster leader
}

impl RedisClient {
    /// Makes a connection with redis://{ip}:{port}
    pub fn new(ro_endpoint: &str, rw_endpoint: &str) -> RedisClient {
        RedisClient {
            ro_connection: RedisClient::create_connection(ro_endpoint),
            rw_connection: RedisClient::create_connection(rw_endpoint),
        }
    }

    fn create_connection(endpoint: &str) -> Connection {
        let client = redis::Client::open(format!("redis://{}", endpoint.clone()))
            .unwrap_or_else(|_| panic!("Bad url: {}", endpoint));

        client.get_connection().unwrap_or_else(|err| {
            let msg = format!("Can't create connection with redis server at '{endpoint}'");
            let msg = format!(
                "{}\n{:?} : {:?} {:?}",
                msg,
                err.category(),
                err.kind(),
                err.detail()
            );
            panic!("{msg}");
        })
    }

    /// Performs "set <short_url> <full_url>"
    pub fn add_url<T: FromRedisValue>(
        &mut self,
        short_url: &str,
        full_url: &str,
    ) -> RedisResult<T> {
        self.rw_connection.set(short_url, full_url)
    }

    /// Performs "get <short_url>" and returns full url
    pub fn get_full_url<T: FromRedisValue>(&mut self, short_url: &str) -> RedisResult<T> {
        self.ro_connection.get(short_url)
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
