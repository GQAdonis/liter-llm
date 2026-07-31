//! CLI-owned tracing subscriber installation.
//!
//! ~keep Libraries in the liter-llm workspace emit only; installing the global
//! ~keep subscriber is the application's job, and for the CLI that is here. By
//! ~keep default diagnostics go to a console fmt subscriber. With the `otel`
//! ~keep feature compiled and `OTEL_EXPORTER_OTLP_ENDPOINT` set, spans and metrics
//! ~keep also export via OTLP (the library's GenAI-semconv instruments then reach
//! ~keep the collector through the installed meter provider).

use tracing_subscriber::EnvFilter;

/// RAII guard for the installed telemetry pipeline.
///
/// ~keep Holds the OTLP tracer/meter providers (when active) so they flush and
/// ~keep shut down on drop; keep it alive for the whole command.
#[derive(Default)]
pub struct TelemetryGuard {
    #[cfg(feature = "otel")]
    _otel: Option<otel::OtelGuard>,
}

/// Install the process-wide `tracing` subscriber.
///
/// With the `otel` feature compiled and `OTEL_EXPORTER_OTLP_ENDPOINT` set to a
/// non-empty value, spans/metrics export to that OTLP collector; otherwise
/// diagnostics are written to the console at `default_filter` (unless `RUST_LOG`
/// overrides it). A failed OTLP init falls back to console logging.
pub fn init(default_filter: &str) -> TelemetryGuard {
    #[cfg(feature = "otel")]
    {
        if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
            && !endpoint.is_empty()
        {
            match otel::init_otlp(&endpoint, default_filter) {
                Ok(guard) => return TelemetryGuard { _otel: Some(guard) },
                Err(error) => {
                    init_console(default_filter);
                    tracing::warn!(%error, "OTLP telemetry init failed; falling back to console logging");
                    return TelemetryGuard::default();
                }
            }
        }
    }
    init_console(default_filter);
    TelemetryGuard::default()
}

fn init_console(default_filter: &str) {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter)))
        .try_init();
}

#[cfg(feature = "otel")]
mod otel {
    use opentelemetry::KeyValue;
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_otlp::{SpanExporter, WithExportConfig};
    use opentelemetry_sdk::Resource;
    use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    /// Shuts the OTLP tracer and meter providers down (flushing) on drop.
    pub struct OtelGuard {
        tracer_provider: SdkTracerProvider,
        meter_provider: SdkMeterProvider,
    }

    impl Drop for OtelGuard {
        fn drop(&mut self) {
            if let Err(e) = self.tracer_provider.shutdown() {
                tracing::warn!(error = %e, "error shutting down tracer provider");
            }
            if let Err(e) = self.meter_provider.shutdown() {
                tracing::warn!(error = %e, "error shutting down meter provider");
            }
        }
    }

    /// Build an OTLP TracerProvider + MeterProvider wired to `endpoint`, register the
    /// W3C TraceContext propagator, and install a subscriber that bridges `tracing`
    /// spans to OpenTelemetry alongside a JSON console layer.
    pub fn init_otlp(endpoint: &str, default_filter: &str) -> Result<OtelGuard, Box<dyn std::error::Error>> {
        let resource = Resource::builder()
            .with_service_name("liter-llm")
            .with_attribute(KeyValue::new("service.version", env!("CARGO_PKG_VERSION")))
            .build();

        let span_exporter = SpanExporter::builder().with_tonic().with_endpoint(endpoint).build()?;
        let tracer_provider = SdkTracerProvider::builder()
            .with_batch_exporter(span_exporter)
            .with_resource(resource.clone())
            .build();
        let tracer = tracer_provider.tracer("liter-llm");
        opentelemetry::global::set_tracer_provider(tracer_provider.clone());

        let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()?;
        let reader = PeriodicReader::builder(metric_exporter)
            .with_interval(std::time::Duration::from_secs(15))
            .build();
        let meter_provider = SdkMeterProvider::builder()
            .with_reader(reader)
            .with_resource(resource)
            .build();
        opentelemetry::global::set_meter_provider(meter_provider.clone());

        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));
        let fmt_layer = tracing_subscriber::fmt::layer().json();
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(otel_layer)
            .try_init()?;

        Ok(OtelGuard {
            tracer_provider,
            meter_provider,
        })
    }
}
