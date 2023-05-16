//! Provides functions to sanitize and check user inputs

use url::Url;

/// Sanitize content to prevent potential cross site scripting content
pub fn sanitize_input(url: &str) -> String {
    ammonia::clean(url)
}

/// Checks if the given input is a valid url
pub fn is_url(input: &str) -> bool {
    matches!(Url::parse(input), Ok(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    // sanitize function tests
    #[test]
    fn sanitize_html_instead_of_url() {
        const NOT_URL: &str = "Hello<script> world </script>";

        assert_eq!(sanitize_input(&NOT_URL), "Hello");
    }

    #[test]
    fn sanitize_good_input() {
        const URL: &str = "https://crates.io/";
        assert_eq!(sanitize_input(&URL), URL);
    }

    // is_url function tests
    #[test]
    fn refuse_not_url() {
        const URL: &str = "hello";
        assert_eq!(is_url(&URL), false);
    }

    #[test]
    fn validate_good_url() {
        const URL: &str = "https://crates.io/";
        assert_eq!(is_url(&URL), true);
    }

    #[test]
    fn refuse_incomplete_url() {
        const URL: &str = "crates.io";
        assert_eq!(is_url(&URL), false);
    }
}
