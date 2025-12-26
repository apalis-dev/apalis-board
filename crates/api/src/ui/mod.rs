use include_dir::{Dir, File, include_dir};

/// Embed the built frontend directory into the lib.
static APP_DIST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/dist");

/// A utility to serve the embedded frontend files.
#[derive(Clone, Debug, Default)]
pub struct ServeUI;

impl ServeUI {
    /// Create a new `ServeUI` instance.
    #[must_use] 
    pub fn new() -> Self {
        Self
    }

    /// Get an embedded file by URI path.
    #[must_use] 
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
    #[must_use] 
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

    /// Return cache control headers based on file type.
    #[must_use] 
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
