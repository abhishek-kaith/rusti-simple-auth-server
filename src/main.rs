use std::{net::SocketAddr, sync::Arc};
use api::routes::Route;
use axum::{extract::State, Router};
pub use error::{AppError, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tower_cookies::CookieManagerLayer;

mod api;
mod config;
mod error;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .pretty()
        .init();

    let config = config::Config::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let state = AppState::new(config)
        .await
        .expect("Failed to initialize application state");

    let app = Router::new()
        .nest("/api", Route::api(state))
        .layer(CookieManagerLayer::new())
        .fallback_service(Route::static_route())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub type AxumState = State<Arc<AppState>>;

pub struct AppState {
    pub pool: Pool<Postgres>,
    pub config: config::Config,
}

impl AppState {
    pub async fn new(config: config::Config) -> std::result::Result<Arc<AppState>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.db_url)
            .await?;
        Ok(Arc::new(AppState { pool, config }))
    }
}
