# Response

Unified, type-safe API response builder for Axum applications.

This crate provides a simple and consistent way to build JSON API responses in Axum web applications. It ensures a unified format for both success and error responses, making your APIs easier to maintain and consume.

## Features

- **Type-safe responses**: Use generics to ensure your data is serialized correctly.
- **Consistent structure**: All responses follow the same JSON format with `status`, optional `data`, and optional `message`.
- **Easy integration**: Designed specifically for Axum, with direct support for `IntoResponse`.
- **Lightweight**: Minimal dependencies, focused on simplicity.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
response = "1"
```

This crate depends on `axum`, `serde`, and `serde_json`, so ensure they are included in your project as well (or let Cargo handle them via transitive dependencies).

## Usage

Import the crate and use the `success` and `error` functions in your Axum handlers.

### Response Structure

All responses are serialized to JSON in the following format:

```json
{
  "status": "success" | "error",
  "data": { ... }  // Optional, for success responses
  "message": "..." // Optional, for error responses
}
```

### Success Response

Use `success` to return a 200 OK response with data:

```rust
use response::success;
use axum::response::IntoResponse;
use serde::Serialize;

async fn handler() -> impl IntoResponse {
    success(serde_json::json!({
        "key": "value"
    }))
}
```

### Error Response

Use `error` to return an error response with a custom status code and message:

```rust
use response::error;
use axum::http::StatusCode;
use axum::response::IntoResponse;

async fn handler() -> impl IntoResponse {
    error(StatusCode::BAD_REQUEST, "Invalid input".to_string())
}
```

## Examples

A complete demo is available in `examples/demo.rs`. Here's a snippet:

```rust
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
```

To run the example:

1. Clone the repository: `git clone https://github.com/canmi21/response.git`
2. Navigate to the project: `cd response`
3. Run the example: `cargo run --example demo`
4. Access `http://localhost:8080/success` or `http://localhost:8080/error` in your browser or via curl.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/canmi21/response).