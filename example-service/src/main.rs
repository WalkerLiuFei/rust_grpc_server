use std::net::SocketAddr;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::initialize;
use opentelemetry::{Context, global};
use opentelemetry::trace::TraceContextExt;
use redis::{Commands, Connection, ConnectionAddr};
use tokio::sync::Mutex;
use tonic::{Code, Request, Response, Status, transport::Server};
use tonic_reflection::server;
use tracing::{info, Instrument, Span};
use tracing_attributes::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use common::{interceptor, tracer};
use proto::cachekv_pb::{cache_kv_service_server::CacheKvService, CacheKvRequest, CacheKvResponse, FILE_DESCRIPTOR_SET};
use proto::cachekv_pb::cache_kv_service_server::CacheKvServiceServer;

use crate::config::CONFIG;
mod config;

pub struct KVService {
    redis_con: Arc<Mutex<Connection>>,
}


#[tonic::async_trait]
impl CacheKvService for KVService {
    #[instrument(skip(self, request), fields(trace_id, span_id, parent_span_id))]
    async fn cache_kv(&self, request: Request<CacheKvRequest>) -> Result<Response<CacheKvResponse>, Status> {
        let mut redis_con = self.redis_con.try_lock().unwrap();
        let parent_cx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&interceptor::MyExtractor(&request))
        });
        Span::current().set_parent(parent_cx.clone());
        Span::current().record("trace_id", parent_cx.span().span_context().trace_id().to_string().as_str());


        let request_msg = request.into_inner();

        info!("go to set redis kv {:?}",request_msg);
        another_func();
        match redis_con.set_ex::<_, _, ()>(request_msg.key, request_msg.value, request_msg.timeout as usize) {
            Ok(_) => {
                Ok(Response::new(CacheKvResponse { message: "set cache success".parse().unwrap() }))
            }
            Err(err) => {
                Err(Status::new(Code::Unavailable, err.to_string()))
            }
        }
    }
}

#[instrument(fields(span_id))]
fn another_func() {
    info!("another func");
    println!("Current span ID: {:?}", Context::current().span().span_context().span_id());
    another_func2();
}

#[instrument(name = "another_func2", target = "another_func2")]
fn another_func2() {
    info!("another func2");

    for _ in 0..4 {
        tokio::spawn(async_func().instrument(Span::current()));
    }
    info!("async func called");
}


#[instrument(name = "aync_func")]
async fn async_func() {
    info!("async func calling");
    sleep(Duration::from_secs(1));
}

impl KVService {
    fn new(redis_con: Arc<Mutex<Connection>>) -> KVService {
        KVService {
            redis_con,
        }
    }
}

fn init_redis(config: config::RedisConfig) -> Arc<Mutex<Connection>> {
    info!("going to init redis{:?}",config);
    //"redis://default:redispw@localhost:55000"
    let redis_con = redis::Client::open(redis::ConnectionInfo {
        addr: ConnectionAddr::Tcp(config.host, config.port),
        redis: redis::RedisConnectionInfo {
            db: 0,
            username: Option::from(config.user),
            password: Option::from(config.password),
        },
    }).expect("panic, redis connect failed").get_connection().expect("failed to connect to redis");
    Arc::new(Mutex::new(redis_con))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize(&config::CONFIG);
    //init_logger();
    tracer::init_tracer(&config::CONFIG.name.as_str(), &config::CONFIG.jaeger_endpoint);

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port as u16));
    let redis_con = init_redis(config::CONFIG.redis_config.clone());

    let reflection = server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .layer(tonic::service::interceptor(interceptor::MyInterceptor::default()))
        .add_service(reflection)
        .add_service(CacheKvServiceServer::new(KVService::new(redis_con)))
        .serve(addr)

        .await?;
    Ok(())
}


