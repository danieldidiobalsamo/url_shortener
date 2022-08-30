use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use ammonia;
use url_shortener::Config;
use url_shortener_algo;
use url_shortener_redis_server::{self, RedisClient};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[get("/encode/{url}")]
async fn shorten_url_request(path: web::Path<String>) -> impl Responder {
    let conf = Config::new();

    let url = sanitize_input(&path.into_inner());
    let short = url_shortener_algo::encode_url(&url);

    let mut redis = RedisClient::new(&conf.redis_ip, &conf.redis_port);
    url_shortener_redis_server::add_url(&mut redis, &short, &url);

    HttpResponse::build(http::StatusCode::OK).body(short)
}

#[get("/decode/{url}")]
async fn retrieve_full_url(path: web::Path<String>) -> impl Responder {
    let conf = Config::new();

    let url = sanitize_input(&path.into_inner());

    let mut redis = RedisClient::new(&conf.redis_ip, &conf.redis_port);
    let full = url_shortener_redis_server::get_full_url(&mut redis, &url);

    HttpResponse::build(http::StatusCode::MOVED_PERMANENTLY)
        .insert_header(("Location", full))
        .body("redirecting...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = Config::new();

    let ip = conf.shortener_ip.clone();
    let port = conf.shortener_port.parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .allowed_header(http::header::LOCATION)
            .max_age(3600);

        App::new()
            .service(index)
            .service(shorten_url_request)
            .service(retrieve_full_url)
            .wrap(cors)
    })
    .bind((ip, port))?
    .run()
    .await
}

fn sanitize_input(url: &str) -> String {
    ammonia::clean(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_instead_of_url() {
        const NOT_URL: &str = "Hello<script> world </script>";

        assert_eq!(sanitize_input(&NOT_URL), "Hello");
    }

    #[test]
    fn good_input() {
        const URL: &str = "https://crates.io/";
        assert_eq!(sanitize_input(&URL), URL);
    }
}
