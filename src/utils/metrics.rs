// src/utils/metrics.rs
use dharitri_sc::*;

pub struct PerformanceMetrics {
    pub response_time: u64,
    pub gas_used: u64,
    pub storage_used: u64,
}

pub fn collect_metrics() -> PerformanceMetrics {
    PerformanceMetrics {
        response_time: 0,
        gas_used: 0,
        storage_used: 0,
    }
}