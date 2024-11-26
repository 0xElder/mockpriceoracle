use std::sync::LazyLock;

use axum::{response, routing, Router};
use serde_json::{json, Value};

const ENDPOINT: &str = "/osmosis/poolmanager/pools/1/prices";

static RESPONSE: LazyLock<Value> = LazyLock::new(|| json!({ "spot_price": "2elder" }));

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        ENDPOINT,
        routing::get(|| async { response::Json(&*RESPONSE) }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8191").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
