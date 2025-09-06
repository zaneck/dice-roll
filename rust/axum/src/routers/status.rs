use hyper::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(status))
}

#[derive(Serialize, ToSchema)]
struct StatusResponse {
    status: &'static str,
}
pub const STATUS_TAG: &str = "Status";
pub const STATUS_DESCRIPTION: &str = "API status endpoint";

#[utoipa::path(
        get,
        path = "/status",
        tag = STATUS_TAG,
        responses(
            (status = 200, description = "API is running", body = StatusResponse),
            (status = 500, description = "Internal server error", body = String)
        ))]
async fn status() -> Result<Json<StatusResponse>, StatusCode> {
    Ok(Json(StatusResponse { status: "OK" }))
}
