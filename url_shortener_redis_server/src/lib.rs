use redis::Commands;

fn add_url(short_url: &str, full_url: &str) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut connnection = client.get_connection().unwrap();

    let _: () = connnection.set(short_url, full_url).unwrap();
}

fn get_full_url(short_url: &str) -> String {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut connnection = client.get_connection().unwrap();

    let full: String = connnection.get(short_url).unwrap();

    full
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_key_and_get() {
        let full = "https://crates.io/";
        let short = "d2af598";

        add_url(short, full);

        let url_from_server = get_full_url(short);

        assert_eq!(full, url_from_server);
    }
}
