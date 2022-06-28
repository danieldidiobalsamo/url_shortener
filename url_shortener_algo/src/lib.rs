use md5;

/// url are shortened using this process :
/// - hash the url using MD5
/// - extracting the 7 first characters

pub fn encode_url(url: &str) -> String {
    let digest = format!("{:x}", md5::compute(&url));
    digest[0..7].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_sample_url() {
        let raw = "https://crates.io/";
        let encoded = encode_url(raw);

        assert_eq!("d2af598", encoded);
    }
}
