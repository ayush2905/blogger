use std::env;
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result
}
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Debug, Clone)]
struct AppState {
    templates: tera::Tera,
    conn: DatabaseConnection,
}

pub struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG","debug");
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");
    let port = env::var("PORT").expect("PORT is not set in the .env file");
    let host = env::var("HOST").expect("HOST is not set in the .env file");

    let server_url = format!("{}:{}", host, port)

    let conn = sea_orm::Database::connect(&db_url).await.unwrap();


}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(new);
    cfg.service(create);
    cfg.service(edit);
    cfg.service(update);
    cfg.service(delete);
}