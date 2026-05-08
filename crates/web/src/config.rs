//! Compile-time configuration for the web UI.
//!
//! # API base
//!
//! `APALIS_BOARD_API_PATH` — HTTP API prefix (no trailing slash), default `"/api/v1"`.
//!
//! # Public URL (static assets + router)
//!
//! `TRUNK_BUILD_PUBLIC_URL` — the same environment variable Trunk documents for
//! `[build].public_url` / `--public-url`. This crate reads it at **Rust compile time** via
//! `option_env!`, so it must be set in the environment whenever `cargo` compiles this package
//! (for example: `TRUNK_BUILD_PUBLIC_URL=/my-app/ trunk build`, or the same export in CI).
//!
//! Trunk’s HTML pipeline uses the resolved `public_url` from config; that resolution does not
//! by itself define environment variables for `rustc`. If you only set `public_url` in
//! `Trunk.toml` or pass `--public-url` without exporting `TRUNK_BUILD_PUBLIC_URL`, the WASM
//! router base may still default to `"/"` unless you also export this variable to the same value.

use std::borrow::Cow;

pub const API_PATH: &str = match option_env!("APALIS_BOARD_API_PATH") {
    Some(path) => path,
    None => "/api/v1",
};

pub const PUBLIC_URL: &str = match option_env!("TRUNK_BUILD_PUBLIC_URL") {
    Some(url) => url,
    None => "/",
};

/// [`leptos_router::components::Router`] `base` value (empty at `/`).
#[must_use]
pub fn router_base() -> Cow<'static, str> {
    match PUBLIC_URL.trim_end_matches('/') {
        "" | "/" => Cow::Borrowed(""),
        path => Cow::Borrowed(path),
    }
}
