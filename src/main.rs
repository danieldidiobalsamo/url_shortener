#![warn(missing_docs)]

//! Application REST API

use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use url_shortener::Config;
use url_shortener_algo;
use url_shortener_redis_server::{self, RedisClient};

mod security;

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

    let url = security::sanitize_input(&path.into_inner());

    if !security::is_url(&url) {
        HttpResponse::build(http::StatusCode::BAD_REQUEST).body("Provided url is not valid")
    } else {
        let short = url_shortener_algo::encode_url(&url);

        let mut redis = RedisClient::new(&conf.redis_socket);

        redis.add_url(&short, &url).unwrap_or_else(|err| {
            panic!(
                "Can't set key/value on redis: {:?} {:?}",
                err.kind(),
                err.detail()
            )
        });

        HttpResponse::build(http::StatusCode::OK).body(short)
    }
}

/// Redirects client to decoded url
#[get("/decode/{key}")]
async fn retrieve_full_url(path: web::Path<String>) -> impl Responder {
    let conf = Config::new();

    let key = security::sanitize_input(&path.into_inner());

    let mut redis = RedisClient::new(&conf.redis_socket);
    let full: String = match redis.get_full_url(&key) {
        Ok(url) => url,
        Err(err) => {
            println!("{:?} {:?}", err.kind(), err.detail());
            String::new()
        }
    };

    HttpResponse::build(http::StatusCode::MOVED_PERMANENTLY)
        .insert_header(("Location", full))
        .body("redirecting...")
}

/// Setup actix server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = Config::new();

    let socket = conf.split_app_socket();

    let ip = socket.0;
    let port = socket.1;

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
