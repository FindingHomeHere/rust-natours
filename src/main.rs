use dotenvy;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use std::env;
use std::time::Instant;
use tracing::info;

mod controllers;
mod models;
mod router;
mod schema;

// MIDDLEWARE
#[handler]
async fn app_logger(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let start_time = Instant::now();
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    ctrl.call_next(req, depot, res).await;

    let duration = start_time.elapsed();
    let status = res.status_code.unwrap_or(StatusCode::OK);

    info!(
        method = %method,
        path = %path,
        status = %status.as_u16(),
        duration_ms = duration.as_millis(),
        "HTTP Request processed"
    )
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    // Initialize Logging
    tracing_subscriber::fmt().init();

    // Create router
    let base_router = Router::new()
        .push(
            Router::with_path("api/v1")
                .push(router::tour_routes())
                .push(router::user_routes()),
        )
        .unshift(
            Router::with_path("/app/{*path}")
                .get(StaticDir::new(vec!["static"]).defaults("index.html")),
        );

    // Logging middleware
    let logger = Service::new(base_router).hoop(app_logger);

    println!("{logger:?}");

    // Bind server to port 3000
    let acceptor = TcpListener::new(env::var("HOST").unwrap()).bind().await;

    Server::new(acceptor).serve(logger).await;
}
