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
    pub fn new(
        ro_endpoint: &str,
        rw_endpoint: &str,
        username: &str,
        password: &str,
    ) -> RedisClient {
        RedisClient {
            ro_connection: RedisClient::create_connection(ro_endpoint, username, password),
            rw_connection: RedisClient::create_connection(rw_endpoint, username, password),
        }
    }

    fn create_connection(endpoint: &str, username: &str, password: &str) -> Connection {
        let url = format!("redis://{}:{}@{}", username, password, endpoint);

        let client = redis::Client::open(url.clone()).unwrap_or_else(|err| {
            let msg = format!("Redis connection issue: {url}");
            let msg = format!("{}\n{:?}:{:?}", msg, err.category(), err);
            panic!("{msg}");
        });

        client.get_connection().unwrap_or_else(|err| {
            let msg = format!("Can't create connection with redis server at '{url}'");
            let msg = format!("{}\n{:?}:{:?}", msg, err.category(), err);
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
