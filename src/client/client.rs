use grpc_proto::pb::{CacheKvRequest, hello_service_server, HelloRequest, HelloResponse};
use grpc_proto::pb::hello_service_client::HelloServiceClient;
use tonic;
use tonic::{Request, Response, Status, transport::Server};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    // Connect to the gRPC server at the specified address
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    // Create a new client
    let mut client  = HelloServiceClient::new(channel);

    // Create a new HelloRequest
    let request = tonic::Request::new(CacheKvRequest {
        key: "the_key".to_string(),
        value: "the_value".to_string(),
        timeout: 1000,
    });

    // Call the SayHello RPC
    let response = client.cache_kv(request).await?;

    // Print out the response
    println!("RESPONSE={:?}", response);

    Ok(())
}