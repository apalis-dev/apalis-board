#[cfg(feature = "actix")]
pub mod actix;
#[cfg(feature = "axum")]
pub mod axum;

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
