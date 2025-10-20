/* examples/demo.rs */

use axum::http::StatusCode;
use axum::{Router, routing::get};
use response::{error, success};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn success_handler() -> impl axum::response::IntoResponse {
	success(serde_json::json!({
		"message": "Everything is fine!"
	}))
}

async fn error_handler() -> impl axum::response::IntoResponse {
	error(StatusCode::BAD_REQUEST, "Something went wrong.".to_string())
}

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/success", get(success_handler))
		.route("/error", get(error_handler));

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("Listening on http://{}", addr);

	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
