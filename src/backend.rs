use axum::Router;
use axum::routing::{get};
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

pub fn get_notes_route() -> Router {
    let note_list = serde_json::to_string(&get_notes()).unwrap();
    get_router().route("/", get(|| async { note_list }))
}

fn get_router() -> Router {
    Router::new()
}

pub async fn get_listener() -> TcpListener {
    TcpListener::bind("0.0.0.0:3000").await.unwrap()
}
