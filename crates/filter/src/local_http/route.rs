// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Praxis Contributors

//! Declarative method/path ownership and route-scoped transfer bounds.

use http::Method;


/// Methods owned by a route; HEAD is never implicitly added to GET.
#[derive(Clone, Debug)]
pub enum LocalMethods {
    /// Every admitted method.
    All,
    /// An explicit, nonempty set of methods.
    Explicit(Vec<Method>),
}


/// Original admitted path scope, ignoring queries and subsequent rewrites.
#[derive(Clone, Debug)]
pub enum LocalPath {
    /// Exact URI path.
    Exact(String),
    /// Existing segment-boundary prefix semantics, including trailing slash.
    SegmentPrefix(String),
}


/// Request-body ownership must explicitly be exclusive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalBodyOwnership {
    /// Invalid for local routes; retained to diagnose missing declarations.
    Unspecified,
    /// Only the handler may pull request payload bytes.
    Exclusive,
}

/// Stable route declaration. Limits count transferred bytes, never buffers.
#[derive(Clone, Debug)]
pub struct LocalRoute {
    /// Stable, pipeline-unique identifier passed to the handler factory.
    pub id: String,
    /// Explicit methods or all methods.
    pub methods: LocalMethods,
    /// Exact or segment-prefix original path.
    pub path: LocalPath,
    /// Mandatory exclusive request-body ownership.
    pub body: LocalBodyOwnership,
    /// Finite, positive request payload limit; independent of global ceilings.
    pub request_limit: u64,
    /// Finite, positive response limit counted after response filtering.
    pub response_limit: u64,
}
