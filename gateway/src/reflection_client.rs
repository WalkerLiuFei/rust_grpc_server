use tokio_stream::StreamExt;
use tonic::Request;
use tonic::transport::Channel;
use tonic_reflection::pb::{server_reflection_client, ServerReflectionRequest};
use tonic_reflection::pb::server_reflection_request::MessageRequest;
use tonic_reflection::pb::server_reflection_response::MessageResponse;
use tracing::{error, info};

struct ReflectionClient {
    client: server_reflection_client::ServerReflectionClient<Channel>,
}

impl ReflectionClient {
    //tonic::include_proto!("grpc.reflection.v1alpha");
    // Create a reflection client
    async fn create_reflection_client(uri: &str) -> Result<ReflectionClient, Box<dyn std::error::Error>> {
        //let channel = Channel::builder(uri.parse::<Uri>()?).connect().await.unwrap();
        // Construct client and send request, extract response
        let conn = tonic::transport::Endpoint::new(uri.to_string())
            .unwrap()
            .connect()
            .await
            .unwrap();
        // let channel = Channel::from_static(uri).connect_timeout(Duration::new( 10,0));
        let client = server_reflection_client::ServerReflectionClient::new(conn);
        Ok(ReflectionClient { client })
    }

    async fn list_services(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let request = ServerReflectionRequest {
            host: "".to_string(),
            message_request: Some(MessageRequest::ListServices(String::new())),
        };

        let request = Request::new(tokio_stream::iter(vec![request]));

        let mut inbound = self.client.server_reflection_info(request).await.expect("Failed to get response").into_inner();

        let response = inbound.next().await
            .expect("steamed response")
            .expect("successful response")
            .message_response
            .expect("some MessageResponse");
        let mut result: Vec<String> = vec![];
        if let MessageResponse::ListServicesResponse(services) = response {
            services.service.into_iter().for_each(|service| {
                info!("service={:?}", service);
                result.push(service.name);
            });
        } else {
            error!("error getting services, not the expected response type");
        }
        Ok(result)
    }
}
