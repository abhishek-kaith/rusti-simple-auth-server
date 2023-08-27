use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::AppState;

use super::{controllers::auth::Auth, middlewares::auth::auth};

pub struct Route;

impl Route {
    pub fn api(state: Arc<AppState>) -> Router {
        let auth = Router::new()
            .route("/register", post(Auth::register))
            .route("/verify", post(Auth::verify))
            .route("/login", post(Auth::login))
            .route(
                "/me",
                get(Auth::me).route_layer(middleware::from_fn_with_state(state.clone(), auth)),
            );

        Router::new()
            .nest("/auth", auth)
            .route("/ping", get(ping))
            .with_state(state)
    }
}

impl Route {
    pub fn static_route() -> Router {
        let static_dir = ServeDir::new("./public/dist");
        Router::new().nest_service("/", static_dir)
    }
}

async fn ping() -> crate::Result<axum::Json<serde_json::Value>> {
    Ok(axum::Json(serde_json::json!({ "message": "pong" })))
}
