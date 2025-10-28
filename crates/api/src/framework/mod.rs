use include_dir::{Dir, File, include_dir};

#[cfg(feature = "actix")]
pub mod actix;

#[cfg(feature = "axum")]
pub mod axum;

/// Embed the built frontend directory into the lib.
static APP_DIST: Dir = include_dir!("$CARGO_MANIFEST_DIR/../board/dist");

pub trait RegisterRoute<B, T> {
    fn register(self, backend: B) -> Self;
}

pub struct ApiBuilder<R> {
    router: R,
    #[allow(dead_code)]
    root: bool,
}

impl<R> ApiBuilder<R> {
    /// Create a new ApiBuilder with default settings
    pub fn new(router: R) -> Self {
        Self { router, root: true }
    }
    /// Create a new ApiBuilder with a custom scope
    /// If `register_root` is true, the root routes (/, /tasks, /workers, /overview)
    /// will be registered on the provided scope.
    pub fn new_with_router(router: R, register_root: bool) -> Self {
        Self {
            router,
            root: register_root,
        }
    }

    /// Finalize the builder and return the router
    pub fn build(self) -> R {
        self.router
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServeApp;

impl ServeApp {
    pub fn new() -> Self {
        ServeApp
    }

    /// Get an embedded file by URI path.
    pub fn get_file(path: &str) -> Option<&File<'static>> {
        let normalized = path.trim_start_matches('/');

        APP_DIST.get_file(normalized).or_else(|| {
            // Fallback for SPA-style routing (e.g., React/Vue/Svelte)
            if normalized.is_empty() || !normalized.contains('.') {
                APP_DIST.get_file("index.html")
            } else {
                None
            }
        })
    }

    /// Return a MIME type based on file extension.
    pub fn content_type(path: &str) -> &'static str {
        if path.ends_with(".html") {
            "text/html; charset=utf-8"
        } else if path.ends_with(".js") {
            "application/javascript"
        } else if path.ends_with(".css") {
            "text/css"
        } else if path.ends_with(".wasm") {
            "application/wasm"
        } else {
            "application/octet-stream"
        }
    }

    /// Return an appropriate Cache-Control header value.
    pub fn cache_control(path: &str) -> Option<&'static str> {
        if path.ends_with(".html") {
            // HTML files change often; no long-term caching.
            Some("no-cache")
        } else if path.ends_with(".js") || path.ends_with(".css") || path.ends_with(".wasm") {
            // Cache static assets aggressively (1 year).
            Some("public, max-age=31536000, immutable")
        } else {
            None
        }
    }
}
