use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/object", post(detected));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("Starting web server");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    debug!("root");
    "Hello, World!"
}

async fn detected(Json(payload): Json<RequestObject>) -> impl IntoResponse {
    debug!("detected");
    (
        StatusCode::OK,
        Json(ResponseObject {
            id: 1,
            object: payload.object,
        }),
    )
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct RequestObject {
    object: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct ResponseObject {
    id: u64,
    object: String,
}
