pub mod logger {
    use futures::future::BoxFuture;
    use tide::{Context, Response, middleware};
    use std::time;

    /// Log each request using the standard `log` crate.
    #[derive(Debug)]
    pub(crate) struct Logger {
        _priv: (),
    }

    impl Logger {
        /// Create a new instance
        pub(crate) fn new() -> Self {
            Self { _priv: () }
        }
    }

    impl<State: Send + Sync + 'static> middleware::Middleware<State> for Logger {
        fn handle<'a>(
            &'a self,
            cx: Context<State>,
            next: middleware::Next<'a, State>,
        ) -> BoxFuture<'a, Response> {
            Box::pin(async move {
                let start_time = time::Instant::now();
                let uri = format!("{}", cx.request().uri());
                let method = format!("{}", cx.request().method());
                log::trace!("{} {}", method, uri);

                let res = next.run(cx).await;

                let status = res.status();
                let elapsed = start_time.elapsed();
                let args = format!("{} {} {} {:?}", method, status.as_u16(), uri, elapsed);
                if status.is_server_error() {
                    log::error!("{}", args);
                } else if status.is_client_error() {
                    log::warn!("{}", args);
                } else {
                    log::info!("{}", args);
                }

                res
            })
        }
    }
}
