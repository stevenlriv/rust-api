use actix_web::{HttpResponse, Responder};

// For getting the current UTC time.
use chrono::Utc;

// For building JSON response.
use serde_json::json;

pub async fn get_healthcheck() -> impl Responder {
    // The `Utc::now().to_rfc3339()` gives an ISO 8601 timestamp.
    let json_data = json!({
        "status": "ok",
        "timestamp": Utc::now().to_rfc3339()
    });

    // Return an HTTP 200 response with JSON body.
    HttpResponse::Ok().json(json_data)
}
