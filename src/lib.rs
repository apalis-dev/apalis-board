#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(all(feature = "actix", not(target_arch = "wasm32")))]
pub mod actix;

#[cfg(all(feature = "axum", not(target_arch = "wasm32")))]
pub mod axum;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
pub mod web;
