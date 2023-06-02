// pub mod helloworld {
//     tonic::include_proto!("helloworld");
// }


use crate::pb::hello_service_server::HelloServiceServer;

pub mod pb {
    include!("./helloworld/helloworld.rs");
}


// fn test(){
// HelloServiceServer::new
// }
