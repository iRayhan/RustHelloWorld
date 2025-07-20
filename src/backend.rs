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
    let note_list = serde_json::to_string(&get_notes()).unwrap();
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
    println!("{:?}", header);
    next.run(req).await
}

pub async fn add_middleware2(mut req: Request, next: Next) -> Response {
    let header = req.headers_mut();
    header.insert(
        "x-custom-header-3",
        HeaderValue::from_static("add_middleware2"),
    );
    println!("{:?}", header);
    next.run(req).await
}
