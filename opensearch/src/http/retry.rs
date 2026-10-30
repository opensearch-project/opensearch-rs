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

/// The response statuses retried by default: a node that is overloaded,
/// restarting, or behind a proxy that cannot reach it
pub(crate) const DEFAULT_RETRY_ON_STATUS: &[u16] = &[502, 503, 504];

/// The number of retries after the first attempt, by default
pub(crate) const DEFAULT_MAX_RETRIES: usize = 3;

/// The ceiling of the delay before the first retry, by default; it doubles
/// for each retry after it
pub(crate) const DEFAULT_RETRY_BACKOFF: Duration = Duration::from_millis(100);

/// The bound of the delays of one request's retries together, by default
pub(crate) const DEFAULT_RETRY_BUDGET: Duration = Duration::from_secs(5);

/// The largest number of doublings of the backoff ceiling, so that it cannot
/// overflow with a large number of retries
const MAX_BACKOFF_SHIFT: u32 = 16;

/// When and how the transport sends a request again after a failed attempt
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
    /// Whether a response with this status is retried
    pub(crate) fn retries_status(&self, status: u16) -> bool {
        self.retry_on_status.contains(&status)
    }

    /// Whether an attempt that failed with this error is retried: a failure
    /// to send the request or to receive the response (connection refused or
    /// reset), and a timeout only when enabled. Errors of request handlers
    /// (middleware) are not retried: they are not transient.
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

    /// The largest delay before retry `attempt` (counted from 1)
    fn ceiling(&self, attempt: usize) -> Duration {
        let shift = u32::try_from(attempt.saturating_sub(1))
            .unwrap_or(u32::MAX)
            .min(MAX_BACKOFF_SHIFT);
        self.backoff.saturating_mul(1u32 << shift)
    }

    /// Whether a retry after `spent` of delays, waiting `delay` more, stays
    /// within the budget
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
        // bounded, and no overflow for absurd attempt counts
        assert_eq!(policy.ceiling(1000), policy.ceiling(17));
        assert_eq!(policy.ceiling(usize::MAX), policy.ceiling(17));
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
