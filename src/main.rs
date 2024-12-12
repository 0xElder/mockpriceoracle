use std::sync::LazyLock;

use axum::{response, routing, Router};
use serde_json::{json, Value};

const ENDPOINTS: [&str; 2] = [
    "/osmosis/poolmanager/pools/1/prices",
    "/osmosis/poolmanager/pools/2/prices",
];

static RESPONSES: LazyLock<[Value; 2]> = LazyLock::new(|| {
    [
        json!({ "spot_price": "2uelder" }),
        json!({ "spot_price": "4uelder"}),
    ]
});

#[tokio::main]
async fn main() {
    let mut app = Router::new();
    for (endpoint, response) in ENDPOINTS.into_iter().zip(RESPONSES.iter()) {
        app = app.route(
            endpoint,
            routing::get(|| async { response::Json(response.clone()) }),
        )
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8191").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
