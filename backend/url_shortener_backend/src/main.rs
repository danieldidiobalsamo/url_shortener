#![warn(missing_docs)]

//! Application REST API

use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// workspace packages
use url_shortener_backend::Config;
use url_shortener_redis::{self, RedisClient};

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
        return HttpResponse::build(http::StatusCode::BAD_REQUEST)
            .body("Provided url is not valid");
    }

    let protocol = url.split(':').collect::<Vec<&str>>()[0];
    if protocol != "https" && protocol != "http" {
        return HttpResponse::build(http::StatusCode::BAD_REQUEST)
            .body("Only https and http are allowed");
    }

    let short = url_shortener_algo::encode_url(&url);
    match redis.add_url::<String>(&short, &url) {
        Ok(_) => HttpResponse::build(http::StatusCode::OK).body(short),
        Err(err) => {
            let msg = format!(
                "Can't set key/value on redis: {:?} {:?}\nconnection dropped :{}",
                err.kind(),
                err.detail(),
                err.is_connection_dropped()
            );

            println!("{}", msg);
            HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).body("")
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
                "Can't get full url corresponding to {:?}: {:?} {:?}\n connection dropped :{}",
                &key,
                err.kind(),
                err.detail(),
                err.is_connection_dropped()
            );

            println!("{}", msg);
            HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).body("")
        }
    }
}

/// Reply kubernetes liveness probe
#[get("/health")]
async fn health(redis: web::Data<Mutex<RedisClient>>) -> impl Responder {
    let mut redis = redis.lock().unwrap();

    match redis.add_url::<String>("health probe", "health probe") {
        Ok(_) => HttpResponse::build(http::StatusCode::OK).body(""),
        Err(err) => {
            let msg = format!("Health probe failed : {:?}", err);
            let msg = format!(
                "{} \n connection dropped :{}",
                msg,
                err.is_connection_dropped()
            );

            println!("{}", msg);

            HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).body(msg)
        }
    }
}

/// Setup actix server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = Config::new();

    let redis = RedisClient::new(
        &conf.redis_ro_endpoint,
        &conf.redis_rw_endpoint,
        &conf.redis_user,
        &conf.redis_password,
    );
    let redis = web::Data::new(Mutex::new(redis));

    let socket = conf.split_app_socket();
    let ip = socket.0;
    let port = socket.1;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://short.home")
            .allowed_origin("http://short.home.backend")
            .allowed_methods(vec!["GET"]);

        App::new()
            .app_data(redis.clone())
            .service(shorten_url_request)
            .service(retrieve_full_url)
            .service(health)
            .wrap(cors)
    })
    .bind((ip, port))?
    .run()
    .await
}
