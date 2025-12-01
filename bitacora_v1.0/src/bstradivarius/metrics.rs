// bitacora_v1.0/src/watcher/metrics.rs
//! 📊 Performance Metrics Tracker
//!
//! Real-time performance monitoring.
//! Eduardo: "Medir performance y updates bajo solicitud en la terminal"

use super::*;
use std::time::{Instant, Duration};
use std::collections::VecDeque;

/// Metrics tracker
pub struct MetricsTracker {
    start_time: Instant,
    processing_times: VecDeque<Duration>,
    max_samples: usize,
}

impl MetricsTracker {
    /// Create new tracker
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            processing_times: VecDeque::with_capacity(100),
            max_samples: 100,
        }
    }
    
    /// Record processing time
    pub fn record_processing(&mut self, duration: Duration) {
        self.processing_times.push_back(duration);
        
        // Keep only recent samples
        if self.processing_times.len() > self.max_samples {
            self.processing_times.pop_front();
        }
    }
    
    /// Get average processing time (ms)
    pub fn avg_processing_ms(&self) -> f64 {
        if self.processing_times.is_empty() {
            return 0.0;
        }
        
        let total: Duration = self.processing_times.iter().sum();
        let avg = total / self.processing_times.len() as u32;
        
        avg.as_secs_f64() * 1000.0
    }
    
    /// Get uptime in seconds
    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
    
    /// Get percentile (p50, p95, p99)
    pub fn percentile(&self, p: f64) -> f64 {
        if self.processing_times.is_empty() {
            return 0.0;
        }
        
        let mut sorted: Vec<_> = self.processing_times.iter().collect();
        sorted.sort();
        
        let index = ((sorted.len() as f64) * p) as usize;
        let index = index.min(sorted.len() - 1);
        
        sorted[index].as_secs_f64() * 1000.0
    }
}

impl Default for MetricsTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_tracker() {
        let mut tracker = MetricsTracker::new();
        
        tracker.record_processing(Duration::from_millis(10));
        tracker.record_processing(Duration::from_millis(20));
        tracker.record_processing(Duration::from_millis(30));
        
        let avg = tracker.avg_processing_ms();
        assert!((avg - 20.0).abs() < 0.1);
    }
    
    #[test]
    fn test_percentile() {
        let mut tracker = MetricsTracker::new();
        
        for i in 1..=100 {
            tracker.record_processing(Duration::from_millis(i));
        }
        
        let p50 = tracker.percentile(0.5);
        assert!((p50 - 50.0).abs() < 5.0);
        
        let p95 = tracker.percentile(0.95);
        assert!((p95 - 95.0).abs() < 5.0);
    }
}
