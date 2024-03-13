// use opentelemetry::global;
// use opentelemetry::trace::TraceError;
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::trace as sdktrace;
use opentelemetry_sdk::trace::Tracer;
// use opentelemetry::{trace::TracerProvider as _, Context, KeyValue};
// use opentelemetry_sdk::propagation::TraceContextPropagator;
// use opentelemetry_sdk::trace::{Span, Tracer};
// use opentelemetry_sdk::{
// runtime,
// trace::{BatchConfig, BatchSpanProcessor, RandomIdGenerator, Sampler, TracerProvider},
// };
// use opentelemetry_sdk::{trace, Resource};
// use opentelemetry_semantic_conventions::{
//     resource::{DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION},
//     SCHEMA_URL,
// };
// use std::env;
use tracing::{info, Subscriber};
use tracing_opentelemetry::OpenTelemetryLayer;
// use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, registry::LookupSpan, Layer};

// Create a Resource that captures information about the entity for which telemetry is recorded.
// fn resource() -> Resource {
//     Resource::from_schema_url(
//         [
//             KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
//             KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
//             KeyValue::new(DEPLOYMENT_ENVIRONMENT, "develop"),
//         ],
//         SCHEMA_URL,
//     )
// }

// pub fn init_telemetry() {
//     // Define Tracer
//     let provider = init_trace();
//     let subscriber = Registry::default();
//     // Layer to filter traces based on level - trace, debug, info, warn, error.
//     let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));
//     // Layer to add our configured tracer.
//     let tracing_layer = tracing_opentelemetry::layer().with_tracer(stdout_tracer);
//     // Setting a trace context propagation data.
//     global::set_text_map_propagator(TraceContextPropagator::new());
//     global::set_tracer_provider(provider);
//     subscriber.with(env_filter).with(tracing_layer).init();
// }

pub fn init_telemetry() -> Result<(), init_tracing_opentelemetry::Error> {
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()
    //setup a temporary subscriber to log output during setup
    // let subscriber = tracing_subscriber::registry()
    //     .with(build_loglevel_filter_layer())
    //     .with(build_logger_text());
    // let _guard = tracing::subscriber::set_default(subscriber);
    // // let meter_provider = init_meter_provider();

    // // let provider = init_trace();

    // // let tracer = provider.tracer("fm-server");
    // info!("init logging & tracing");

    // // let subscriber = tracing_subscriber::registry()
    // //     .with(build_otel_layer()?)
    // //     .with(build_loglevel_filter_layer())
    // //     .with(build_logger_text());
    // // let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    // let subscriber = tracing_subscriber::registry()
    //     //     .with(EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("debug"))?)
    //     // .with(tracing_subscriber::fmt::layer().json())
    //     .with(build_loglevel_filter_layer())
    //     .with(build_logger_text());
    // //     // .with(telemetry_layer)
    // //     // .with(MetricsLayer::new(meter_provider.clone()))
    // //     // .with(OpenTelemetryLayer::new(init_trace()))
    // //     .init();
    // tracing::subscriber::set_global_default(subscriber)?;
    // Ok(())
    // OtelGuard { meter_provider }
}
// fn init_trace() -> TracerProvider {
//     let exporter = opentelemetry_stdout::SpanExporter::default();
//     let processor = BatchSpanProcessor::builder(exporter, runtime::Tokio).build();
//     TracerProvider::builder()
//         .with_span_processor(processor)
//         .build()
// }

// Construct Tracer for OpenTelemetryLayer
// fn init_tracer() -> opentelemetry_sdk::trace::Tracer {
// opentelemetry_sdk::trace::Tracer
// opentelemetry_stdout::SpanExporter::default()
//     opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_trace_config(
//             opentelemetry_sdk::trace::Config::default()
//                 // Customize sampling strategy
//                 .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
//                     1.0,
//                 ))))
//                 // If export trace to AWS X-Ray, you can use XrayIdGenerator
//                 .with_id_generator(RandomIdGenerator::default())
//                 .with_resource(resource()),
//         )
//         .with_batch_config(BatchConfig::default())
//         .with_exporter(opentelemetry_stdout::SpanExporter::default())
//         .install_batch(runtime::Tokio)
//         .unwrap()
// }

#[must_use]
pub fn build_logger_text<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    use tracing_subscriber::fmt::format::FmtSpan;
    if cfg!(debug_assertions) {
        Box::new(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_line_number(true)
                .with_thread_names(true)
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_timer(tracing_subscriber::fmt::time::uptime()),
        )
    } else {
        Box::new(
            tracing_subscriber::fmt::layer()
                .json()
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_timer(tracing_subscriber::fmt::time::uptime()),
        )
    }
}

#[must_use]
pub fn build_loglevel_filter_layer() -> tracing_subscriber::filter::EnvFilter {
    // filter what is output on log (fmt)
    // std::env::set_var("RUST_LOG", "warn,otel::tracing=info,otel=debug");
    std::env::set_var(
        "RUST_LOG",
        format!(
            // `otel::tracing` should be a level info to emit opentelemetry trace & span
            // `otel::setup` set to debug to log detected resources, configuration read and infered
            "{},otel::tracing=trace,otel=debug",
            std::env::var("RUST_LOG")
                .or_else(|_| std::env::var("OTEL_LOG_LEVEL"))
                .unwrap_or_else(|_| "debug".to_string())
        ),
    );
    EnvFilter::from_default_env()
}

pub fn build_otel_layer<S>() -> Result<OpenTelemetryLayer<S, Tracer>, TraceError>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    use init_tracing_opentelemetry::{
        init_propagator, //stdio,
        resource::DetectResource,
        stdio,
    };
    let otel_rsrc = DetectResource::default()
        //.with_fallback_service_name(env!("CARGO_PKG_NAME"))
        //.with_fallback_service_version(env!("CARGO_PKG_VERSION"))
        .build();
    // let otel_tracer = otlp::init_tracer(otel_rsrc, otlp::identity)?;
    // to not send trace somewhere, but continue to create and propagate,...
    // then send them to `axum_tracing_opentelemetry::stdio::WriteNoWhere::default()`
    // or to `std::io::stdout()` to print
    //
    let otel_tracer = stdio::init_tracer(
        otel_rsrc.clone(),
        // stdio::identity::<std::io::Stdout>,
        |builder| {
            builder.with_config(sdktrace::config().with_resource(otel_rsrc).with_sampler(
                sdktrace::Sampler::ParentBased(Box::new(sdktrace::Sampler::TraceIdRatioBased(
                    1f64,
                ))),
            ))
        },
        std::io::stdout(),
    )?;
    init_propagator()?;
    Ok(tracing_opentelemetry::layer()
        .with_error_records_to_exceptions(true)
        .with_tracer(otel_tracer))
}
