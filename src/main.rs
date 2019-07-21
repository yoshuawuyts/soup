//! Soupstagram

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]
#![feature(async_await, async_closure)]

mod views;
mod middleware;
mod stores;

use structopt::StructOpt;

type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Soup pics online
#[derive(Debug, structopt::StructOpt)]
pub struct Cli {
    /// Set the log level
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
    /// Network address
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    pub address: String,
    /// Insecure HTTP port
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "80")]
    pub port: u16,
}

#[runtime::main]
async fn main() -> Result<(), Exception> {
    let args = Cli::from_args();
    femme::start(log_level(args.verbosity))?;
    let app = create_app();
    let http_service = app.into_http_service();
    let mut listener = runtime::net::TcpListener::bind((&*args.address, args.port))
        .map_err(|e| {
            log::error!("cannot connect to {}:{:?}", &*args.address, args.port);
            e
        })?;
    log::info!("listening on {}", listener.local_addr()?);
    let server = http_service_hyper::Server::builder(listener.incoming())
        .with_spawner(runtime::task::Spawner::new());
    server.serve(http_service).await?;
    Ok(())
}


pub(crate) fn create_app() -> tide::App<()> {
    let mut app = tide::App::new();
    app.middleware(middleware::logger::Logger::new());
    app.at("/").get(|_| async move { "Hello, world!" });
    app
}

fn log_level(n: u8) -> log::LevelFilter {
    match n {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    }
}
