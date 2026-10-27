// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Praxis Contributors

//! Adapter-neutral local HTTP transport obligations.

use async_trait::async_trait;
use bytes::Bytes;
use praxis_filter::{FilterError, FilterPipeline, HttpFilterContext, Response, local_http::LocalOutcome};

/// Live adapters currently cannot install local-handler pipelines.
///
/// # Errors
/// Returns an error until the adapter implements all local transport duties.
pub fn require_supported_adapter(pipeline: &FilterPipeline) -> Result<(), FilterError> {
    if pipeline.has_local_http_routes() {
        return Err("local HTTP serving is unsupported by this adapter (Issue 2)".into());
    }
    Ok(())
}

/// Managed completion handed to the adapter once, including failed delivery.
#[derive(Clone, Copy, Debug)]
pub struct LocalCompletion {
    /// Final handler outcome.
    pub outcome: LocalOutcome,
    /// Last selected final response status.
    pub status: u16,
    /// Payload accepted from the request stream.
    pub request_bytes: u64,
    /// Filtered payload successfully written to the response stream.
    pub response_bytes: u64,
    /// Whether header commitment was attempted; errors cannot replace it.
    pub committed: bool,
    /// False requires HTTP/1 close or HTTP/2 stream termination, not connection closure.
    pub request_eof: bool,
}


/// Adapter obligations: bounded reads, awaited writes, no upstream capability.
#[async_trait]
pub trait LocalTransport: Send {
    /// Send eligible 100-continue on the first approved body demand only.
    async fn accept_body(&mut self) -> Result<(), FilterError>;
    /// Read one bounded payload chunk; explicit EOF must be sticky.
    async fn read_body(&mut self) -> Result<Option<Bytes>, FilterError>;
    /// Commit final headers. Any error may mean partial commitment.
    async fn write_head(&mut self, head: &Response) -> Result<(), FilterError>;
    /// Await downstream backpressure before another source pull.
    async fn write_body(&mut self, chunk: Bytes) -> Result<(), FilterError>;
    /// Complete delivery; an unread request requires HTTP/1 close or H2 stream termination.
    async fn finish(&mut self, request_eof: bool) -> Result<(), FilterError>;
    /// Abort delivery/affected stream, preserving unrelated HTTP/2 streams.
    async fn abort(&mut self);
    /// Record request metrics and adapter completion once. No fallback logging.
    fn complete(&mut self, completion: LocalCompletion, ctx: &HttpFilterContext<'_>);
}
