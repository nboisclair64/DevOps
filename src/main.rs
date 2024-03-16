use axum::{Router, routing::get,};
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use std::io;
mod controllers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = Router::new();
    //Idea is to setup the /health route on the azure server
    //then have azure send the 200 http request back and catch it using the Tcp listener
    //Azure never provided us with permissions but the logic provided is how i would approach
    //the first part
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8477").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let mut signals = signal::new(&[SIGINT])?;
    
    for sig in signals.forever() {
            println!("Received signal: {:?}", sig);
            std::process::exit(0);
        }
    axum::serve(listener, app)
        .await.unwrap();
}

