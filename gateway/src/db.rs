use std::time::Duration;

use sea_orm::{ActiveValue, DbErr, EntityTrait, InsertResult};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tonic::async_trait;
use tracing::{info, log};
use tracing::level_filters::LevelFilter;

use crate::{entities::uri_grpc_mapping, models::SetGRPCMappingRequest};
use crate::entities::uri_grpc_mapping::ActiveModel;
#[async_trait]
trait GatewayDBTrait<'s> {
    async fn insert_grpc_uri_mapping(&self, request: &SetGRPCMappingRequest) -> Result<InsertResult<ActiveModel>, DbErr>;
    async fn delete_grpc_uri_mapping(&self, request: &SetGRPCMappingRequest);
    async fn update_grpc_uri_mapping(&self, request: &SetGRPCMappingRequest);
}


struct GatewayDBImpl {
    db_instance: DatabaseConnection,
}


impl GatewayDBImpl{
    pub async fn new(url: String) -> Result<GatewayDBImpl, DbErr> {
        //let db_cfg = config::CONFIG.db_config.clone();
       // info!("preparing to db_cfg: {:?}",db_cfg);

        let mut connection_options = ConnectOptions::new(url);
        connection_options.max_connections(20)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(5))
            .max_lifetime(Duration::from_secs(60));
            //.sqlx_logging_level(LevelFilter::Info);
        let db_connection = Database::connect(connection_options)
            .await?;
        Ok(Self {
            db_instance: db_connection,
        })
    }
}

#[async_trait]
impl GatewayDBTrait<'_> for GatewayDBImpl{
    async fn insert_grpc_uri_mapping(&self, request: &SetGRPCMappingRequest) -> Result<InsertResult<ActiveModel>, DbErr> {
        let mapping = ActiveModel {
            http_uri: ActiveValue::Set(String::from(&request.http_uri)),
            grpc_service: ActiveValue::Set(String::from(&request.grpc_service)),
            grpc_method: ActiveValue::Set(String::from(&request.grpc_method)),
            ..Default::default() // no need to set default fields
        };
        uri_grpc_mapping::Entity::insert(mapping)
            .exec(&self.db_instance)
            .await
    }

    async fn delete_grpc_uri_mapping(&self, request: &SetGRPCMappingRequest) {
        todo!()
    }

    async fn update_grpc_uri_mapping(&self, request: &SetGRPCMappingRequest) {
        todo!()
    }
}


#[tokio::test]
async fn insert_data() ->Result<(),DbErr >{
    let db_impl = GatewayDBImpl::new("mysql://root:3252860@127.0.0.1:3306/gateway".to_string()).await?;
    let insert_result = db_impl.insert_grpc_uri_mapping(&SetGRPCMappingRequest {
        http_uri: String::from("http://test.url"),
        grpc_service: String::from("HelloWorld"),
        grpc_method: String::from("SayHello"),
    }).await?;
    assert!(insert_result.last_insert_id > 0);
    Ok(())
}
