use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    api::{models::user::User, services::auth::AuthService},
    AppError, AxumState, Result,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthMiddleware {
    pub user: User,
    pub token: Uuid,
}

pub async fn auth<B>(
    cookies: Cookies,
    State(state): AxumState,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse> {
    let cookie = cookies
        .get("session")
        .map(|c| c.value().to_string())
        .ok_or_else(|| AppError::Unauthorized)?;
    let token = Uuid::parse_str(&cookie).map_err(|_| AppError::Unauthorized)?;
    let user = AuthService::get_user_by_session_token(&state.pool, token).await?;
    req.extensions_mut().insert(AuthMiddleware { user, token });
    Ok(next.run(req).await)
}
