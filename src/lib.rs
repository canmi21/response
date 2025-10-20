/* src/lib.rs */

use axum::{
	http::StatusCode,
	response::{IntoResponse, Json},
};
use serde::Serialize;

/// Generic API response structure.
///
/// This structure provides a unified format for both success and error
/// responses, ensuring consistent JSON output across APIs.
#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
	status: &'static str,
	#[serde(skip_serializing_if = "Option::is_none")]
	data: Option<T>,
	#[serde(skip_serializing_if = "Option::is_none")]
	message: Option<String>,
}

/// Creates a success (200 OK) API response.
///
/// # Arguments
/// * `data` - Serializable payload to include in the response.
///
/// # Returns
/// A JSON response with `status = "success"` and the provided data.
pub fn success<T: Serialize>(data: T) -> impl IntoResponse {
	let response = ApiResponse {
		status: "success",
		data: Some(data),
		message: None,
	};
	(StatusCode::OK, Json(response))
}

/// Creates an error API response.
///
/// # Arguments
/// * `status_code` - The HTTP status code to return.
/// * `message` - Description of the error.
///
/// # Returns
/// A JSON response with `status = "error"` and the given message.
pub fn error(status_code: StatusCode, message: String) -> impl IntoResponse {
	let response = ApiResponse::<()> {
		status: "error",
		data: None,
		message: Some(message),
	};
	(status_code, Json(response))
}
