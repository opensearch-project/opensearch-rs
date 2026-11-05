/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The retry policy of the transport, configured through
//! [TransportBuilder](super::transport::TransportBuilder)

use crate::http::middleware::{RequestHandlerError, RequestHandlerErrorKind};
use std::{convert::TryFrom, time::Duration};

pub(crate) const DEFAULT_RETRY_ON_STATUS: &[u16] = &[502, 503, 504];

pub(crate) const DEFAULT_MAX_RETRIES: usize = 3;

pub(crate) const DEFAULT_RETRY_BACKOFF: Duration = Duration::from_millis(100);

pub(crate) const DEFAULT_RETRY_BUDGET: Duration = Duration::from_secs(5);

/// Bounds the doublings of the backoff ceiling, so that it cannot overflow
const MAX_BACKOFF_SHIFT: u32 = 16;

/// The longest delay before a retry, whatever the backoff and the number of
/// retries before it
pub(crate) const MAX_RETRY_DELAY: Duration = Duration::from_secs(60);

#[derive(Debug, Clone)]
pub(crate) struct RetryPolicy {
    pub(crate) max_retries: usize,
    pub(crate) retry_on_status: Vec<u16>,
    pub(crate) retry_on_timeout: bool,
    pub(crate) backoff: Duration,
    pub(crate) budget: Option<Duration>,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            retry_on_status: DEFAULT_RETRY_ON_STATUS.to_vec(),
            retry_on_timeout: false,
            backoff: DEFAULT_RETRY_BACKOFF,
            budget: Some(DEFAULT_RETRY_BUDGET),
        }
    }
}

impl RetryPolicy {
    pub(crate) fn retries_status(&self, status: u16) -> bool {
        self.retry_on_status.contains(&status)
    }

    /// Errors of request handlers (middleware) are not retried: they are not
    /// transient.
    pub(crate) fn retries_error(&self, err: &RequestHandlerError) -> bool {
        match &err.0 {
            RequestHandlerErrorKind::Http(err) => {
                if err.is_timeout() {
                    self.retry_on_timeout
                } else {
                    err.is_connect() || err.is_request()
                }
            }
            RequestHandlerErrorKind::Handler(_) => false,
        }
    }

    /// The delay before retry `attempt` (counted from 1): drawn uniformly
    /// from zero to the backoff doubled for each retry after the first
    /// (exponential backoff with full jitter), so that clients failing
    /// together do not retry together
    pub(crate) fn delay(&self, attempt: usize) -> Duration {
        let ceiling = self.ceiling(attempt);
        if ceiling.is_zero() {
            return Duration::ZERO;
        }
        let nanos = u64::try_from(ceiling.as_nanos()).unwrap_or(u64::MAX);
        Duration::from_nanos(fastrand::u64(0..=nanos))
    }

    fn ceiling(&self, attempt: usize) -> Duration {
        let shift = u32::try_from(attempt.saturating_sub(1))
            .unwrap_or(u32::MAX)
            .min(MAX_BACKOFF_SHIFT);
        self.backoff
            .saturating_mul(1u32 << shift)
            .min(MAX_RETRY_DELAY)
    }

    pub(crate) fn within_budget(&self, spent: Duration, delay: Duration) -> bool {
        match self.budget {
            Some(budget) => spent.saturating_add(delay) <= budget,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_follow_the_other_clients() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 3);
        assert!(policy.retries_status(502));
        assert!(policy.retries_status(503));
        assert!(policy.retries_status(504));
        assert!(!policy.retries_status(500));
        assert!(!policy.retries_status(429));
        assert!(!policy.retry_on_timeout);
        assert_eq!(policy.budget, Some(Duration::from_secs(5)));
    }

    #[test]
    fn the_backoff_ceiling_doubles_per_retry() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.ceiling(1), Duration::from_millis(100));
        assert_eq!(policy.ceiling(2), Duration::from_millis(200));
        assert_eq!(policy.ceiling(3), Duration::from_millis(400));
        assert_eq!(policy.ceiling(10), Duration::from_millis(51_200));
    }

    #[test]
    fn the_backoff_ceiling_stops_at_one_minute() {
        let policy = RetryPolicy::default();
        // 100 ms doubled 10 times is 102.4 s
        assert_eq!(policy.ceiling(11), MAX_RETRY_DELAY);
        // no overflow for absurd attempt counts
        assert_eq!(policy.ceiling(1000), MAX_RETRY_DELAY);
        assert_eq!(policy.ceiling(usize::MAX), MAX_RETRY_DELAY);

        // a backoff above the bound is bounded from the first retry
        let policy = RetryPolicy {
            backoff: Duration::MAX,
            budget: None,
            ..Default::default()
        };
        assert_eq!(policy.ceiling(1), MAX_RETRY_DELAY);
        for _ in 0..100 {
            assert!(policy.delay(1) <= MAX_RETRY_DELAY);
        }
    }

    #[test]
    fn delays_are_jittered_within_the_ceiling() {
        let policy = RetryPolicy::default();
        for attempt in 1..=5 {
            for _ in 0..100 {
                assert!(policy.delay(attempt) <= policy.ceiling(attempt));
            }
        }
        // full jitter: delays spread over the range rather than sitting at it
        let delays: Vec<Duration> = (0..100).map(|_| policy.delay(3)).collect();
        assert!(delays.iter().any(|d| *d < Duration::from_millis(200)));
    }

    #[test]
    fn a_zero_backoff_retries_immediately() {
        let policy = RetryPolicy {
            backoff: Duration::ZERO,
            ..Default::default()
        };
        assert_eq!(policy.delay(1), Duration::ZERO);
        assert_eq!(policy.delay(10), Duration::ZERO);
    }

    #[test]
    fn the_budget_bounds_the_delays_together() {
        let policy = RetryPolicy {
            budget: Some(Duration::from_secs(1)),
            ..Default::default()
        };
        assert!(policy.within_budget(Duration::from_millis(400), Duration::from_millis(600)));
        assert!(!policy.within_budget(Duration::from_millis(400), Duration::from_millis(601)));

        let unbounded = RetryPolicy {
            budget: None,
            ..Default::default()
        };
        assert!(unbounded.within_budget(Duration::MAX, Duration::MAX));
    }

    #[test]
    fn handler_errors_are_not_retried() {
        let policy = RetryPolicy::default();
        assert!(!policy.retries_error(&RequestHandlerError::new("rejected by middleware")));
    }
}
