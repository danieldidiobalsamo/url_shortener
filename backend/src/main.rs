#![warn(missing_docs)]

//! Application REST API

use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use url_shortener_algo;
use url_shortener_backend::Config;
use url_shortener_redis_server::{self, RedisClient};

mod security;

/// Returns url corresponding key
#[get("/encode/{url}")]
async fn shorten_url_request(
    redis: web::Data<Mutex<RedisClient>>,
    path: web::Path<String>,
) -> impl Responder {
    let url = security::sanitize_input(&path.into_inner());
    let mut redis = redis.lock().unwrap();

    if !security::is_url(&url) {
        HttpResponse::build(http::StatusCode::BAD_REQUEST).body("Provided url is not valid")
    } else {
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
}

/// Redirects client to decoded url
#[get("/decode/{key}")]
async fn retrieve_full_url(
    redis: web::Data<Mutex<RedisClient>>,
    path: web::Path<String>,
) -> impl Responder {
    let key = security::sanitize_input(&path.into_inner());
    let mut redis = redis.lock().unwrap();

    match redis.get_full_url::<String>(&key) {
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
    }
}

/// Setup actix server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = Config::new();

    let redis = RedisClient::new(&conf.redis_socket).unwrap();
    let redis = web::Data::new(Mutex::new(redis));

    let socket = conf.split_app_socket();
    let ip = socket.0;
    let port = socket.1;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"]);

        App::new()
            .app_data(redis.clone())
            .service(shorten_url_request)
            .service(retrieve_full_url)
            .wrap(cors)
    })
    .bind((ip, port))?
    .run()
    .await
}
