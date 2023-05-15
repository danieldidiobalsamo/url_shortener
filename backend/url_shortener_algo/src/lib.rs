#![warn(missing_docs)]

//! This crate is responsible to convert a url into a key, which is going to be stored in redis
//!
//! A url is shortened using this process :
//! - hash the url using MD5
//! - extract the 7 first characters

/// Takes a url and returns 7 characters long key
///
/// Example :
/// ```
/// use url_shortener_algo as short;
/// let raw = "https://crates.io/";
/// assert_eq!("d2af598", short::encode_url(raw));
/// ```
pub fn encode_url(url: &str) -> String {
    let digest = format!("{:x}", md5::compute(url));
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
