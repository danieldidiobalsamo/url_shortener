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
        match RedisClient::new(&conf.redis_socket) {
            Ok(mut redis) => {
                let short = url_shortener_algo::encode_url(&url);
                match redis.add_url::<String>(&short, &url) {
                    Ok(_) => HttpResponse::build(http::StatusCode::OK).body(short),
                    Err(err) => {
                        let msg = format!(
                            "Can't set key/value on redis: {:?} {:?}",
                            err.kind(),
                            err.detail()
                        );

                        println!("{}", msg);
                        HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).body("")
                    }
                }
            }

            Err(err) => {
                let msg = format!("Can't access redis server : {}", err);
                println!("{}", msg);
                HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).body("")
            }
        }
    }
}

/// Redirects client to decoded url
#[get("/decode/{key}")]
async fn retrieve_full_url(path: web::Path<String>) -> impl Responder {
    let conf = Config::new();

    let key = security::sanitize_input(&path.into_inner());

    match RedisClient::new(&conf.redis_socket) {
        Ok(mut redis) => match redis.get_full_url::<String>(&key) {
            Ok(url) => HttpResponse::build(http::StatusCode::MOVED_PERMANENTLY)
                .insert_header(("Location", url))
                .body("redirecting..."),
            Err(err) => {
                let msg = format!(
                    "Can't get full url corresponding to {:?}: {:?} {:?}",
                    &key,
                    err.kind(),
                    err.detail()
                );

                println!("{}", msg);
                HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).body("")
            }
        },

        Err(err) => {
            let msg = format!("Can't access redis server : {}", err);
            println!("{}", msg);
            HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).body("")
        }
    }
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
