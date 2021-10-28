use opentelemetry::global;
use opentelemetry::sdk::{trace as sdktrace};
use opentelemetry::trace::TraceError;
use opentelemetry::{
    baggage::BaggageExt,
    metrics::{MetricsError, ObserverResult},
    trace::{TraceContextExt, Tracer},
    Context, Key, KeyValue,
};


pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
  global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
  let tracer = opentelemetry_jaeger::new_pipeline()
      .with_collector_endpoint("http://jaeger.gracey.home:80/api/traces")
      .with_service_name("buildpack")
      .install_batch(opentelemetry::runtime::Tokio)?;
      

  Ok(tracer)
}

pub fn flush_tracer() {
  global::shutdown_tracer_provider(); // sending remaining spans
}

