use std::sync::Mutex;
use actix_web::web::Data;
use serde::{Deserialize};
use serde_json::{json, Value};
use crate::AppState;

const TITLE: &str = "title_test";
const AUTHOR: &str = "author_test";

#[derive(Deserialize)]
struct HelloResponse {
    pub message: String,
}

#[derive(Deserialize)]
struct PongResponse {
    pub status: String,
}

#[cfg(test)]
mod tests {
    use actix_web::{test::{self, TestRequest}, App, web};
    use crate::entity::Book;
    use crate::integration_tests::{app_state, AUTHOR, build_req_body, HelloResponse, PongResponse, TITLE};
    use crate::{service};

    #[actix_rt::test]
    async fn test_hello_endpoint_with_success() {
        let app = test::init_service(App::new().route("/", web::get().to(service::hello))).await;
        let req = TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let hello_response: HelloResponse = test::read_body_json(resp).await;
        assert_eq!(hello_response.message, "API Rust with Actix Web is running!!!");
    }

    #[actix_rt::test]
    async fn test_ping_endpoint_with_success() {
        let app = test::init_service(App::new().route("/ping", web::get().to(service::ping))).await;
        let req = TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let pong_response: PongResponse = test::read_body_json(resp).await;
        assert_eq!(pong_response.status, "pong");
    }

    #[actix_rt::test]
    async fn test_create_book_endpoint_with_success() {
        let app = test::init_service(
            App::new()
                .app_data(app_state().clone())
                .route("/books", web::post().to(service::create))
        ).await;
        let req = TestRequest::post().uri("/books").set_json(build_req_body()).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let book_response: Book = test::read_body_json(resp).await;
        assert_eq!(book_response.title, TITLE);
        assert_eq!(book_response.author, AUTHOR);
        assert!(Some(book_response.id.to_owned()).is_some());
    }
}

fn app_state() -> Data<AppState> {
    Data::new(AppState {
        books: Mutex::new(Vec::new()),
    })
}

fn build_req_body() -> Value {
    json!({
        "title": TITLE,
        "author": AUTHOR,
    })
}


