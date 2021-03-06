// Copyright 2019 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Interfaces for Conjure HTTP clients and servers.
//!
//! Conjure services generate code that interacts with the types and traits in this crate, so that consumers are not
//! tightly bound to specific client and server implementations.
#![warn(missing_docs, clippy::all)]
#![doc(html_root_url = "https://docs.rs/conjure-http/0.6")]

#[doc(inline)]
pub use crate::path_params::PathParams;

#[doc(inline)]
pub use crate::query_params::QueryParams;

pub mod client;
pub mod path_params;
pub mod query_params;
pub mod server;

#[doc(hidden)]
pub mod private;
