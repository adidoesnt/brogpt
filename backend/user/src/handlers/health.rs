use actix_web::{
    body::BoxBody, get, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: u16,
    message: String,
}

impl Responder for HealthResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/health")]
async fn handler() -> impl Responder {
    let message: &str = "Server is healthy!";
    HealthResponse {
        status: 200,
        message: message.to_string(),
    }
}
