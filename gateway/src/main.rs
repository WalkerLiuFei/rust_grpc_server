use std::net::ToSocketAddrs;
use std::thread;

use actix_web::{App, HttpResponse, HttpServer, middleware, post, Responder, web};

use common::tracer as my_tracer;
use crate::clients::consul;

mod models;
mod tracer;
mod routers;

mod config;
mod entities;
mod clients;
mod service;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    my_tracer::init_tracer("gateway", &None);


    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routers::internal_call)
    }).bind("127.0.0.1:3000")?.run().await
}
