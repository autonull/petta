//! Reliability features for PeTTa
//!
//! Production-grade stability with circuit breakers, retries, and timeouts

use std::time::Duration;

/// Configuration for reliability features
#[derive(Debug, Clone)]
pub struct ReliabilityConfig {
    pub circuit_breaker_enabled: bool,
    pub failure_threshold: u32,
    pub retry_enabled: bool,
    pub max_retries: u32,
    pub timeout_enabled: bool,
    pub default_timeout: Duration,
}

impl Default for ReliabilityConfig {
    fn default() -> Self {
        Self {
            circuit_breaker_enabled: true,
            failure_threshold: 5,
            retry_enabled: true,
            max_retries: 3,
            timeout_enabled: true,
            default_timeout: Duration::from_secs(30),
        }
    }
}

impl ReliabilityConfig {
    pub fn permissive() -> Self {
        Self {
            circuit_breaker_enabled: false,
            retry_enabled: false,
            timeout_enabled: false,
            ..Default::default()
        }
    }

    pub fn strict() -> Self {
        Self {
            circuit_breaker_enabled: true,
            failure_threshold: 3,
            retry_enabled: true,
            max_retries: 5,
            timeout_enabled: true,
            default_timeout: Duration::from_secs(10),
        }
    }
}

/// Simple circuit breaker for cascade failure prevention
#[derive(Debug)]
pub struct CircuitBreaker {
    failures: std::sync::atomic::AtomicU32,
    state: std::sync::atomic::AtomicU8, // 0=Closed, 1=Open
}

impl CircuitBreaker {
    pub fn new() -> Self {
        Self {
            failures: std::sync::atomic::AtomicU32::new(0),
            state: std::sync::atomic::AtomicU8::new(0),
        }
    }

    pub fn state(&self) -> CircuitState {
        match self.state.load(std::sync::atomic::Ordering::Acquire) {
            0 => CircuitState::Closed,
            _ => CircuitState::Open,
        }
    }

    pub fn on_success(&self) {
        self.failures.store(0, std::sync::atomic::Ordering::Relaxed);
        self.state.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn on_failure(&self) {
        self.failures.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.state.store(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.failures.store(0, std::sync::atomic::Ordering::Relaxed);
        self.state.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker() {
        let cb = CircuitBreaker::new();
        assert_eq!(cb.state(), CircuitState::Closed);
        cb.on_failure();
        assert_eq!(cb.state(), CircuitState::Open);
        cb.reset();
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_reliability_config() {
        let permissive = ReliabilityConfig::permissive();
        assert!(!permissive.circuit_breaker_enabled);
        
        let strict = ReliabilityConfig::strict();
        assert!(strict.circuit_breaker_enabled);
        assert_eq!(strict.failure_threshold, 3);
    }
}
