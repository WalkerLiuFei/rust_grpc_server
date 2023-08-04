use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use tracing::info;
use tracing_attributes::instrument;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;


pub fn init_tracer(service_name: &str, _jeager_endpoint: &str) {
    // Initialize the global tracer
    //global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(_jeager_endpoint)
        .with_service_name(service_name)
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("failed to install_batch");

    // Initialize tracing_subscriber next
    let subscriber = Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_target(false)
        ).with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_subscriber::EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global default subscriber");

    // Now you can use tracing and env_logger
    info!("tracing initialized");
}


#[tokio::test]
pub async fn test_mytracer()  {
    init_tracer("test", "127.0.0.1:6831");
    test_tracing();
}

#[instrument]
fn test_tracing(){
    info!("this is a func to test tracing");
}