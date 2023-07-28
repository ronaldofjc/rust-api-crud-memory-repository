use std::env::var;
use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::http::header;
use tracing::{info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::entity::Book;

mod entity;
mod service;
mod repository;
mod integration_tests;

pub struct AppState {
    books: Mutex<Vec<Book>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    build_tracing();

    let port = var("PORT").unwrap_or("8090".to_string());
    let address = format!("127.0.0.1:{}", port);

    info!("Starting server on {}", address);

    let books = web::Data::new(AppState {
        books: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        let cors = build_cors();
        App::new()
            .app_data(books.clone())
            .route("/", web::get().to(service::hello))
            .route("/ping", web::get().to(service::ping))
            .route("/books", web::get().to(service::get_all))
            .route("/books", web::post().to(service::create))
            .route("/books/{id}", web::get().to(service::get_by_id))
            .route("/books/{id}", web::delete().to(service::remove_by_id))
            .route("/books/{id}", web::put().to(service::update_by_id))
            .wrap(cors)
    })
        .bind(&address)
        .unwrap_or_else(|err| {
            panic!("🔥🔥🔥 Couldn't start the server in port {}: {:?}", port, err)
        })
        .run()
        .await
}

fn build_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "actix-memory-repository=debug".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn build_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:8090")
        .allowed_origin("http://localhost:8090/")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::ACCEPT
        ])
        .supports_credentials()
}