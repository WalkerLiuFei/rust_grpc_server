
use hyper::{Body, Request, Response, Server};
use tracing::log::info;


pub(crate) fn print_request_info(request : Request<Body>) {
    //info!("request method: {:?}", request.headers(),request);
}