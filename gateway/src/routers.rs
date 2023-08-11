use actix_web::{get, HttpResponse, post, Responder};
use crate::models;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}



#[post("/internal/call")]
pub(crate) async fn internal_call(req_body: String) -> impl Responder {
    let target = serde_json::from_str::<models::InternalCall>(&req_body);
    // FIXME : use service discovery to find the target service
    target.map(|target| {
        let body = serde_json::to_string(&target.body);
        let response = match body {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(_) => HttpResponse::InternalServerError().body("Internal error"),
        };
        response
    }).unwrap_or_else(|err| HttpResponse::BadRequest().body(format!("Invalid request body: {},{}", req_body,err)))
}

// add more ....


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}