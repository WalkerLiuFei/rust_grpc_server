use std::default;
use std::time::Duration;

use sea_orm::{ActiveValue, ColumnTrait, DbErr, DeleteResult, EntityTrait, InsertResult, QueryFilter};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm::sea_query::{IntoCondition, SimpleExpr};
use tonic::async_trait;

use crate::{entities::uri_grpc_mapping, models::SetGRPCMappingRequest};
use crate::entities::uri_grpc_mapping::ActiveModel;
use crate::models::{DeleteGRPCMappingRequest, GetGRPCMappingRequest};
use crate::entities::uri_grpc_mapping::Column;
#[async_trait]
trait GatewayDBTrait<'s> {
    async fn insert_grpc_uri_mapping(&self, model: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr>;
    async fn delete_grpc_uri_mapping(&self, exprs: &[Column]) -> Result<DeleteResult, DbErr>;
    //  async fn get_grpc_uri_mapping(&self, request: &GetGRPCMappingRequest) -> Result<Option<ActiveModel>, DbErr>;
}


struct GatewayDBImpl {
    db_instance: DatabaseConnection,
}


impl GatewayDBImpl {
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
impl GatewayDBTrait<'_> for GatewayDBImpl {
    async fn insert_grpc_uri_mapping(&self, active_model: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
        uri_grpc_mapping::Entity::insert(active_model)
            .exec(&self.db_instance)
            .await
    }

    async fn delete_grpc_uri_mapping(&self, exprs: &Vec<SimpleExpr>) -> Result<DeleteResult, DbErr> {
        let mut delete_builder = uri_grpc_mapping::Entity::delete_many();
        for expr in exprs {
            delete_builder = delete_builder.filter(expr);
        }
        delete_builder.exec(&self.db_instance).await
    }

    // async fn get_grpc_uri_mapping(&self, req) -> Result<Option<ActiveModel>, DbErr> {
    //     uri_grpc_mapping::
    // }


}


#[tokio::test]
async fn insert_data() -> Result<(), DbErr> {
    let db_impl = GatewayDBImpl::new("mysql://root:3252860@127.0.0.1:3306/gateway".to_string()).await?;
    let insert_result = db_impl.insert_grpc_uri_mapping(ActiveModel {
        http_uri: ActiveValue::Set(String::from("http://test.url")),
        grpc_service: ActiveValue::Set(String::from("HelloWorld")),
        grpc_method: ActiveValue::Set(String::from("SayHello")),
        ..default::Default::default()
    }).await?;
    assert!(insert_result.last_insert_id > 0);
    println!("insert success: {:?}", insert_result);
    uri_grpc_mapping::Column::HttpUri.eq( String::from("http://test.url")).into_condition();

    let filter = uri_grpc_mapping::Column::HttpUri.eq(String::from("http://test.url"));
    let delete_result = db_impl.delete_grpc_uri_mapping(&vec![Column::HttpUri.eq(String::from("http://test.url"))]).await?;
    assert_eq!(delete_result.rows_affected, 1);
    Ok(())
}
