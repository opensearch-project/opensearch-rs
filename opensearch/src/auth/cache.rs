/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Opt-in client-side caching for AWS credentials used by SigV4 signing.
//!
//! See [`CachedCredentialsProvider`] for usage and the crate-level
//! documentation for the rationale.

use std::fmt;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};

use aws_credential_types::{
    provider::{future, ProvideCredentials, Result as ProviderResult, SharedCredentialsProvider},
    Credentials,
};

/// Default safety margin subtracted from a credential's expiry.
///
/// Refresh begins this far ahead of the inner credentials' expiry so that
/// in-flight signed requests do not race with expiry on the server side.
pub const DEFAULT_BUFFER_TIME: Duration = Duration::from_secs(5 * 60);

/// A [`ProvideCredentials`] adapter that caches the wrapped provider's
/// output until shortly before it expires.
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(feature = "aws-auth")] {
/// use aws_credential_types::provider::SharedCredentialsProvider;
/// use opensearch::auth::{cache::CachedCredentialsProvider, Credentials};
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let aws_config = aws_config::load_from_env().await;
/// let region = aws_config.region().expect("region").clone();
/// let inner = aws_config.credentials_provider().expect("creds");
///
/// let cached = CachedCredentialsProvider::from_shared(inner);
/// let creds = Credentials::AwsSigV4(SharedCredentialsProvider::new(cached), region);
/// # let _ = creds;
/// # Ok(())
/// # }
/// # }
/// ```
///
/// # Concurrency
///
/// Cache hits take a [`std::sync::RwLock`] read guard, with no `.await`.
/// Refreshes are serialised by a [`tokio::sync::Mutex`] and use a
/// double-checked lookup so that concurrent callers crossing the expiry
/// boundary trigger at most one inner provider call.
pub struct CachedCredentialsProvider {
    inner: SharedCredentialsProvider,
    cache: RwLock<Option<CachedEntry>>,
    refresh_lock: tokio::sync::Mutex<()>,
    buffer_time: Duration,
}

#[derive(Clone)]
struct CachedEntry {
    credentials: Credentials,
    /// `expiry - buffer_time`, or `None` if the inner credentials have no
    /// expiry, in which case the entry never goes stale.
    refresh_after: Option<SystemTime>,
}

impl CachedEntry {
    fn is_fresh(&self, now: SystemTime) -> bool {
        match self.refresh_after {
            None => true,
            Some(deadline) => now < deadline,
        }
    }
}

impl fmt::Debug for CachedCredentialsProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CachedCredentialsProvider")
            .field("buffer_time", &self.buffer_time)
            .finish_non_exhaustive()
    }
}

impl CachedCredentialsProvider {
    /// Wrap a [`ProvideCredentials`] implementation with [`DEFAULT_BUFFER_TIME`].
    pub fn new(inner: impl ProvideCredentials + 'static) -> Self {
        Self::from_shared(SharedCredentialsProvider::new(inner))
    }

    /// Wrap an existing [`SharedCredentialsProvider`].
    pub fn from_shared(inner: SharedCredentialsProvider) -> Self {
        Self {
            inner,
            cache: RwLock::new(None),
            refresh_lock: tokio::sync::Mutex::new(()),
            buffer_time: DEFAULT_BUFFER_TIME,
        }
    }

    /// Override the expiry buffer (default: [`DEFAULT_BUFFER_TIME`]).
    pub fn with_buffer_time(mut self, buffer_time: Duration) -> Self {
        self.buffer_time = buffer_time;
        self
    }

    fn read_fresh(&self) -> Option<Credentials> {
        let now = SystemTime::now();
        let guard = self.cache.read().unwrap_or_else(|p| p.into_inner());
        guard
            .as_ref()
            .filter(|e| e.is_fresh(now))
            .map(|e| e.credentials.clone())
    }

    fn last_known(&self) -> Option<Credentials> {
        let guard = self.cache.read().unwrap_or_else(|p| p.into_inner());
        guard.as_ref().map(|e| e.credentials.clone())
    }

    fn store(&self, credentials: Credentials) {
        let refresh_after = credentials
            .expiry()
            .map(|exp| exp.checked_sub(self.buffer_time).unwrap_or(exp));
        let entry = CachedEntry {
            credentials,
            refresh_after,
        };
        let mut guard = self.cache.write().unwrap_or_else(|p| p.into_inner());
        *guard = Some(entry);
    }

    async fn load_credentials(&self) -> ProviderResult {
        // Fast path: cache hit, no `.await`, shared lock only.
        if let Some(creds) = self.read_fresh() {
            return Ok(creds);
        }

        // Serialise refreshes so concurrent callers fan in to a single
        // inner invocation.
        let _guard = self.refresh_lock.lock().await;

        // Double-check: another task may have refreshed while we waited.
        if let Some(creds) = self.read_fresh() {
            return Ok(creds);
        }

        let fresh = self.inner.provide_credentials().await?;
        self.store(fresh.clone());
        Ok(fresh)
    }
}

impl ProvideCredentials for CachedCredentialsProvider {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        future::ProvideCredentials::new(self.load_credentials())
    }

    fn fallback_on_interrupt(&self) -> Option<Credentials> {
        // Match the convention from `awslabs/smithy-rs#2720`: surface the
        // last cached value when a refresh is interrupted.
        self.last_known()
            .or_else(|| self.inner.fallback_on_interrupt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_credential_types::provider::error::CredentialsError;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[derive(Debug)]
    struct CountingProvider {
        calls: Arc<AtomicUsize>,
        expiry: Option<SystemTime>,
    }

    impl CountingProvider {
        fn new(expiry: Option<SystemTime>) -> (Self, Arc<AtomicUsize>) {
            let calls = Arc::new(AtomicUsize::new(0));
            (
                Self {
                    calls: Arc::clone(&calls),
                    expiry,
                },
                calls,
            )
        }
    }

    impl ProvideCredentials for CountingProvider {
        fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
        where
            Self: 'a,
        {
            future::ProvideCredentials::new(async move {
                self.calls.fetch_add(1, Ordering::SeqCst);
                Ok(Credentials::new(
                    "AKIAIOSFODNN7EXAMPLE",
                    "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                    None,
                    self.expiry,
                    "test",
                ))
            })
        }
    }

    #[derive(Debug)]
    struct FailingProvider;

    impl ProvideCredentials for FailingProvider {
        fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
        where
            Self: 'a,
        {
            future::ProvideCredentials::new(async {
                Err(CredentialsError::provider_error("synthetic failure"))
            })
        }
    }

    #[tokio::test]
    async fn first_call_invokes_inner_provider() {
        let (provider, calls) =
            CountingProvider::new(Some(SystemTime::now() + Duration::from_secs(3600)));
        let cached = CachedCredentialsProvider::new(provider);

        cached.provide_credentials().await.unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn cache_hit_does_not_invoke_inner_provider() {
        let expiry = SystemTime::now() + Duration::from_secs(3600);
        let (provider, calls) = CountingProvider::new(Some(expiry));
        let cached = CachedCredentialsProvider::new(provider);

        for _ in 0..5 {
            cached.provide_credentials().await.unwrap();
        }

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn credentials_without_expiry_are_cached_indefinitely() {
        let (provider, calls) = CountingProvider::new(None);
        let cached = CachedCredentialsProvider::new(provider);

        for _ in 0..3 {
            cached.provide_credentials().await.unwrap();
        }

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn buffer_time_forces_immediate_refresh() {
        // Expiry 30s away, buffer 60s -> always stale.
        let expiry = SystemTime::now() + Duration::from_secs(30);
        let (provider, calls) = CountingProvider::new(Some(expiry));
        let cached =
            CachedCredentialsProvider::new(provider).with_buffer_time(Duration::from_secs(60));

        cached.provide_credentials().await.unwrap();
        cached.provide_credentials().await.unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn expired_entry_is_refreshed() {
        let expiry = SystemTime::now() - Duration::from_secs(60);
        let (provider, calls) = CountingProvider::new(Some(expiry));
        let cached =
            CachedCredentialsProvider::new(provider).with_buffer_time(Duration::from_secs(0));

        cached.provide_credentials().await.unwrap();
        cached.provide_credentials().await.unwrap();

        assert!(calls.load(Ordering::SeqCst) >= 2);
    }

    #[tokio::test]
    async fn concurrent_callers_share_a_single_refresh() {
        let expiry = SystemTime::now() + Duration::from_secs(3600);
        let (provider, calls) = CountingProvider::new(Some(expiry));
        let cached = Arc::new(CachedCredentialsProvider::new(provider));

        let mut handles = Vec::new();
        for _ in 0..32 {
            let cached = Arc::clone(&cached);
            handles.push(tokio::spawn(async move {
                cached.provide_credentials().await.unwrap();
            }));
        }
        for h in handles {
            h.await.unwrap();
        }

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn provider_errors_are_propagated_and_do_not_poison_cache() {
        let cached = CachedCredentialsProvider::new(FailingProvider);

        assert!(cached.provide_credentials().await.is_err());
        assert!(cached.provide_credentials().await.is_err());
    }

    #[tokio::test]
    async fn fallback_on_interrupt_returns_last_known_credentials() {
        let expiry = SystemTime::now() + Duration::from_secs(3600);
        let (provider, _calls) = CountingProvider::new(Some(expiry));
        let cached = CachedCredentialsProvider::new(provider);

        let primed = cached.provide_credentials().await.unwrap();

        let fallback = cached.fallback_on_interrupt().expect("fallback present");
        assert_eq!(fallback.access_key_id(), primed.access_key_id());
    }
}
