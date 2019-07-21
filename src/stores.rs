use futures::future::BoxFuture;
use tide::Context;

/// An extension trait for `Context`, providing form extraction.
pub(crate) trait ContextExt {
    /// Get a thing.
    fn get(&mut self) -> BoxFuture<'_, ()>;
}

impl<State: Send + Sync + 'static> ContextExt for Context<State> {
    fn get(&mut self) -> BoxFuture<'_, ()> {
        let _body = self.take_body();
        Box::pin(async move {
            unimplemented!();
        })
    }
}
