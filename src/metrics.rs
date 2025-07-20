use axum_prometheus::{metrics_exporter_prometheus::PrometheusHandle, PrometheusMetricLayer};

pub fn setup_metrics() -> (PrometheusMetricLayer<'static>, PrometheusHandle) {
    let (prometheus_layer, prometheus_handle) = PrometheusMetricLayer::pair();
    (prometheus_layer, prometheus_handle)
}
