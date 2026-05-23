use async_trait::async_trait;
use tunex_core::{
    benchmark::{
        BenchmarkKind, BenchmarkReport, BenchmarkResult, BenchmarkRunner, BenchmarkScenario,
        PerformanceBaseline, ProfilerBackend, ProfilerConfig, RegressionThreshold,
    },
    TunexResult,
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

struct TestBenchmarkRunner;

#[async_trait]
impl BenchmarkRunner for TestBenchmarkRunner {
    async fn run_scenario(
        &self,
        scenario: BenchmarkScenario,
        _profiler: Option<ProfilerConfig>,
    ) -> TunexResult<BenchmarkResult> {
        Ok(BenchmarkResult::new(
            scenario.name,
            scenario.kind,
            u64::from(scenario.measurement_iterations),
        )
        .with_tokens_per_sec(10_000.0)
        .with_latency_micros(10, 20, 30))
    }

    async fn run_report(
        &self,
        title: &str,
        scenarios: &[BenchmarkScenario],
        profiler: Option<ProfilerConfig>,
    ) -> TunexResult<BenchmarkReport> {
        let mut report = BenchmarkReport::new(title);
        for scenario in scenarios {
            report = report.with_result(
                self.run_scenario(scenario.clone(), profiler.clone())
                    .await?,
            );
        }
        Ok(report)
    }
}

#[test]
fn benchmark_contract_types_are_send_sync_and_object_safe() {
    assert_send_sync::<BenchmarkScenario>();
    assert_send_sync::<BenchmarkKind>();
    assert_send_sync::<BenchmarkResult>();
    assert_send_sync::<BenchmarkReport>();
    assert_send_sync::<ProfilerBackend>();
    assert_send_sync::<ProfilerConfig>();
    assert_send_sync::<PerformanceBaseline>();
    assert_send_sync::<RegressionThreshold>();
    assert_object_safe::<dyn BenchmarkRunner>();

    let _runner = TestBenchmarkRunner;
}

#[test]
fn benchmark_scenarios_cover_required_core_performance_categories() {
    let scenarios = [
        BenchmarkScenario::new("tokens", BenchmarkKind::TokensPerSecond)
            .with_batch_size(4)
            .with_sequence_length(4096),
        BenchmarkScenario::new("batches", BenchmarkKind::BatchThroughput).with_batch_size(16),
        BenchmarkScenario::new("queue", BenchmarkKind::QueueLatency).with_payload_bytes(1024),
        BenchmarkScenario::new("scheduler", BenchmarkKind::SchedulerOverhead),
        BenchmarkScenario::new("serialization", BenchmarkKind::Serialization)
            .with_payload_bytes(4096),
        BenchmarkScenario::new("allocations", BenchmarkKind::MemoryAllocation),
        BenchmarkScenario::new("checkpoint", BenchmarkKind::CheckpointLatency)
            .with_tag("checkpoint"),
    ];

    assert_eq!(scenarios.len(), 7);
    assert_eq!(scenarios[0].batch_size, Some(4));
    assert_eq!(scenarios[0].sequence_length, Some(4096));
    assert_eq!(scenarios[2].payload_bytes, Some(1024));
    assert_eq!(scenarios[6].tags.as_slice(), &["checkpoint".to_string()]);
}

#[test]
fn profiler_config_models_supported_backends() {
    let config = ProfilerConfig::new(ProfilerBackend::Flamegraph)
        .with_sample_rate_hz(997)
        .with_output_path("target/profiles/flamegraph.svg")
        .with_arg("--reverse");

    assert!(config.enabled);
    assert_eq!(config.backend, ProfilerBackend::Flamegraph);
    assert_eq!(config.sample_rate_hz, Some(997));
    assert_eq!(
        config.output_path.as_deref(),
        Some("target/profiles/flamegraph.svg")
    );
    assert_eq!(config.args.as_slice(), &["--reverse".to_string()]);
    assert!(
        !ProfilerConfig::new(ProfilerBackend::Tracing)
            .disabled()
            .enabled
    );
}

#[test]
fn benchmark_results_detect_regression_against_baseline() {
    let mut baseline = PerformanceBaseline::new("serialization", BenchmarkKind::Serialization);
    baseline.tokens_per_sec = Some(10_000.0);
    baseline.latency_p95_micros = Some(100);
    baseline.allocations = Some(1_000);

    let threshold = RegressionThreshold {
        max_throughput_drop_basis_points: 500,
        max_latency_increase_basis_points: 1000,
        max_allocation_increase_basis_points: 1000,
    };

    let good = BenchmarkResult::new("serialization", BenchmarkKind::Serialization, 100)
        .with_tokens_per_sec(9_600.0)
        .with_latency_micros(50, 105, 110)
        .with_allocations(1_050, 10_000);
    let bad = BenchmarkResult::new("serialization", BenchmarkKind::Serialization, 100)
        .with_tokens_per_sec(9_000.0)
        .with_latency_micros(50, 125, 130)
        .with_allocations(1_200, 10_000);

    assert!(!good.regressed_against(&baseline, threshold));
    assert!(bad.regressed_against(&baseline, threshold));
}

#[test]
fn benchmark_report_aggregates_results_and_baselines() {
    let mut baseline = PerformanceBaseline::new("queue", BenchmarkKind::QueueLatency);
    baseline.latency_p95_micros = Some(100);
    let result = BenchmarkResult::new("queue", BenchmarkKind::QueueLatency, 100)
        .with_latency_micros(50, 120, 140);
    let report = BenchmarkReport::new("core contracts")
        .with_threshold(RegressionThreshold {
            max_latency_increase_basis_points: 500,
            ..RegressionThreshold::default()
        })
        .with_baseline(baseline)
        .with_result(result);

    assert_eq!(report.title, "core contracts");
    assert_eq!(report.results.len(), 1);
    assert_eq!(report.baselines.len(), 1);
    assert!(report.has_regression());
}
