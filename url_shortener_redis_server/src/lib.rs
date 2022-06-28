use redis::{Commands, Connection};

pub struct RedisClient {
    connection: Connection,
}

impl RedisClient {
    pub fn new(ip: &str, port: &str) -> RedisClient {
        let ip = format!("redis://{}:{}", ip, port);
        let client = redis::Client::open(ip).unwrap();
        let connection = client.get_connection().unwrap();

        Self {
            connection: connection,
        }
    }

    pub fn get_connection(&mut self) -> &mut Connection {
        &mut self.connection
    }
}

pub fn add_url(client: &mut RedisClient, short_url: &str, full_url: &str) {
    let connnection = client.get_connection();

    let _: () = connnection.set(short_url, full_url).unwrap();
}

pub fn get_full_url(client: &mut RedisClient, short_url: &str) -> String {
    let connnection = client.get_connection();
    let full: String = connnection.get(short_url).unwrap();

    full
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

        add_url(&mut client, short, full);

        let url_from_server = get_full_url(&mut client, short);

        assert_eq!(full, url_from_server);
    }
}
