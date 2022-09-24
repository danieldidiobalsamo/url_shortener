#![warn(missing_docs)]

//! Application REST API

use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use ammonia;
use url::Url;
use url_shortener::Config;
use url_shortener_algo;
use url_shortener_redis_server::{self, RedisClient};

/// Returns home page
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

/// Returns url corresponding key
#[get("/encode/{url}")]
async fn shorten_url_request(path: web::Path<String>) -> impl Responder {
    let conf = Config::new();

    let url = sanitize_input(&path.into_inner());

    if !is_url(&url) {
        HttpResponse::build(http::StatusCode::BAD_REQUEST).body("Provided url is not valid")
    } else {
        let short = url_shortener_algo::encode_url(&url);

        let mut redis = RedisClient::new(&conf.redis_ip, &conf.redis_port);
        redis.add_url(&short, &url);

        HttpResponse::build(http::StatusCode::OK).body(short)
    }
}

/// Redirects client to decoded url
#[get("/decode/{key}")]
async fn retrieve_full_url(path: web::Path<String>) -> impl Responder {
    let conf = Config::new();

    let key = sanitize_input(&path.into_inner());

    let mut redis = RedisClient::new(&conf.redis_ip, &conf.redis_port);
    let full = redis.get_full_url(&key);

    HttpResponse::build(http::StatusCode::MOVED_PERMANENTLY)
        .insert_header(("Location", full))
        .body("redirecting...")
}

/// Setup actix server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = Config::new();

    let ip = conf.shortener_ip.clone();
    let port = conf.shortener_port.parse::<u16>().unwrap();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(shorten_url_request)
            .service(retrieve_full_url)
    })
    .bind((ip, port))?
    .run()
    .await
}

/// Sanitize content to prevent potential cross site scripting content
fn sanitize_input(url: &str) -> String {
    ammonia::clean(url)
}

/// Checks if the given input is a valid url
fn is_url(input: &str) -> bool {
    if let Ok(_) = Url::parse(input) {
        return true;
    } else {
        return false;
    }
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
