use std::time;
use std::{net::SocketAddr, thread};

use axum::{routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/works", get(works))
        .route("/fails", get(fails))
        .route("/also-fails", get(also_fails));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn works() -> &'static str {
    tokio::task::spawn_blocking(|| {
        info!("Starting works request");
        let ten_millis = time::Duration::from_secs(3);

        thread::sleep(ten_millis);
        info!("Ending works request");
        "Delayed Hello, World!"
    })
    .await
    .unwrap()
}

async fn fails() -> &'static str {
    info!("Starting fails request");
    let ten_millis = time::Duration::from_secs(3);

    thread::sleep(ten_millis);
    info!("Ending fails request");
    "Delayed Hello, World!"
}

async fn also_fails() -> &'static str {
    tokio::spawn(async_wait()).await.unwrap()
}

async fn async_wait() -> &'static str {
    info!("Starting fails request");
    let ten_millis = time::Duration::from_secs(3);

    thread::sleep(ten_millis);
    info!("Ending fails request");
    "Delayed Hello, World!"
}
