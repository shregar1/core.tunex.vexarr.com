//! Benchmark and profiling contracts for proving performance claims.

pub mod profiler;
pub mod report;
pub mod result;
pub mod runner;
pub mod scenario;

pub use profiler::{ProfilerBackend, ProfilerConfig};
pub use report::BenchmarkReport;
pub use result::{BenchmarkResult, PerformanceBaseline, RegressionThreshold};
pub use runner::BenchmarkRunner;
pub use scenario::{BenchmarkKind, BenchmarkScenario};
