use axum::body::Body;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::{middleware, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SampleNote {
    id: i32,
    note: String,
}

fn get_notes() -> Vec<SampleNote> {
    let mut note_list = Vec::new();
    note_list.push(SampleNote {
        id: 0,
        note: "hi".into(),
    });
    note_list
}

pub async fn get_note_handler() -> String {
    serde_json::to_string(&get_notes()).unwrap()
}

pub fn get_notes_route() -> Router {
    get_router()
        .route("/", get(get_note_handler))
        .route_layer(middleware::from_fn(add_middleware1))
        .route_layer(middleware::from_fn(add_middleware2))
}

fn get_router() -> Router {
    Router::new()
}

pub async fn get_listener() -> TcpListener {
    TcpListener::bind("0.0.0.0:3000").await.unwrap()
}

pub async fn add_middleware1(mut req: Request, next: Next) -> Response {
    let header = req.headers_mut();
    header.insert(
        "x-custom-header-1",
        HeaderValue::from_static("add_middleware1"),
    );
    header.insert(
        "x-custom-header-1",
        HeaderValue::from_static("add_middleware1"),
    );
    println!("add_middleware1 {:?}", header);
    next.run(req).await
}

// header validation
pub async fn add_middleware2(mut req: Request, next: Next) -> Response {
    let header = req.headers_mut();
    let header_get = header.get("x-custom-header-2");
    match header_get {
        Some(value) => {
            let value = value.to_str().unwrap();
            if value == "test header value" {
                next.run(req).await
            } else {
                Response::builder()
                    .status(401)
                    .body(Body::from("Unauthorized Body"))
                    .unwrap()
            }
        }
        None => Response::builder()
            .status(502)
            .body(Body::from("Empty Body"))
            .unwrap(),
    }
}
