use actix_web::{
    body::BoxBody, http::header::ContentType, post, HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;

struct _SignupRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct SignupResponse {
    status: u16,
    message: String,
}

impl Responder for SignupResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/signup")]
async fn handler() -> impl Responder {
    let message = "User signed up successfully!";
    SignupResponse {
        status: 200,
        message: message.to_string(),
    }
}
