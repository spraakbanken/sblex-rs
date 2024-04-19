use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;


pub fn init_telemetry() -> Result<(), init_tracing_opentelemetry::Error> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()
    
}
