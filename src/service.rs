use actix_web::{web, HttpResponse};
use actix_web::web::{Json, Path};
use serde_json::json;
use uuid::Uuid;
use crate::{AppState, repository};
use crate::entity::{BookRequest};

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(Json(json!({ "message": "API Rust with Actix Web is running!!!"})))
}

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json(Json(json!({ "status": "pong"})))
}

pub async fn get_all(data: web::Data<AppState>) -> HttpResponse {
    let books = repository::get_all(data).await;
    HttpResponse::Ok().json(books)
}

pub async fn get_by_id(id: Path<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let result = repository::get_by_id(id, data).await;
    match result {
        Ok(b) => return HttpResponse::Ok().json(b),
        Err(err) => HttpResponse::NotFound().json(Json(json!({ "message": err.message})))
    }
}

pub async fn remove_by_id(id: Path<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let result = repository::remove_by_id(id, data).await;
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::NotFound().json(Json(json!({ "message": err.message})))
    }
}

pub async fn update_by_id(id: Path<Uuid>, body: Json<BookRequest>, data: web::Data<AppState>) -> HttpResponse {
    let result = repository::update_by_id(id, body, data).await;
    match result {
        Ok(b) => return HttpResponse::Ok().json(b),
        Err(err) => HttpResponse::NotFound().json(Json(json!({ "message": err.message})))
    }
}

pub async fn create(payload: Json<BookRequest>, data: web::Data<AppState>) -> HttpResponse {
    if has_invalid_params(payload.title.clone(), payload.author.clone()) {
        return HttpResponse::BadRequest().json(Json(json!({ "message": "fields title and author are mandatory"})));
    }
    let result = repository::create(payload, data).await;
    match result {
        Ok(b) => return HttpResponse::Ok().json(b),
        Err(err) => HttpResponse::BadRequest().json(Json(json!({ "message": err.message})))
    }
}

fn has_invalid_params(title: Option<String>, author: Option<String>) -> bool {
    if title.is_none() || author.is_none() { return true }  return false
}