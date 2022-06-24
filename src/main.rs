use actix_web::{get, http, App, HttpRequest, HttpResponse, HttpServer, Responder};
use url_shortener_algo;
use url_shortener_redis_server::{self, RedisClient};

#[get("/encode")]
async fn shorten_url_request(req: HttpRequest) -> impl Responder {
    let url = req.query_string();
    let short = url_shortener_algo::encode_url(&url);

    let mut redis = RedisClient::new("127.0.0.1", "6379");
    url_shortener_redis_server::add_url(&mut redis, &short, &url);

    HttpResponse::build(http::StatusCode::OK).body(short) // tmp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(shorten_url_request))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
