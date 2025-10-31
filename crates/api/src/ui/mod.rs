use include_dir::{Dir, File, include_dir};

/// Embed the built frontend directory into the lib.
static APP_DIST: Dir = include_dir!("$CARGO_MANIFEST_DIR/../board/dist");

#[derive(Clone, Debug, Default)]
pub struct ServeUI;

impl ServeUI {
    pub fn new() -> Self {
        ServeUI
    }

    /// Get an embedded file by URI path.
    pub fn get_file(path: &str) -> Option<&File<'static>> {
        let normalized = path.trim_start_matches('/');

        APP_DIST.get_file(normalized).or_else(|| {
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

    pub fn cache_control(path: &str) -> Option<&'static str> {
        if path.ends_with(".html") {
            // Don't cache HTML files.
            Some("no-cache")
        } else if path.ends_with(".js") || path.ends_with(".css") || path.ends_with(".wasm") {
            // Cache static assets aggressively (1 year).
            Some("public, max-age=31536000, immutable")
        } else {
            None
        }
    }
}
