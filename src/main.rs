use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
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
    let url = path.into_inner();
    let short = url_shortener_algo::encode_url(&url);

    let mut redis = RedisClient::new("127.0.0.1", "6379");
    url_shortener_redis_server::add_url(&mut redis, &short, &url);

    HttpResponse::build(http::StatusCode::OK).body(short)
}

#[get("/decode/{url}")]
async fn retrieve_full_url(path: web::Path<String>) -> impl Responder {
    let url = path.into_inner();

    let mut redis = RedisClient::new("127.0.0.1", "6379");
    let full = url_shortener_redis_server::get_full_url(&mut redis, &url);

    HttpResponse::build(http::StatusCode::MOVED_PERMANENTLY)
        .insert_header(("Location", full))
        .body("redirecting...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(shorten_url_request)
            .service(retrieve_full_url)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
