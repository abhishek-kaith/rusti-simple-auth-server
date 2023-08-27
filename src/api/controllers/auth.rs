use std::ops::Add;

use axum::{
    extract::State,
    response::{AppendHeaders, IntoResponse},
    Extension, Json,
};
use tower_cookies::{cookie::time::OffsetDateTime, Cookie, Cookies};

use crate::{
    api::{
        middlewares::auth::AuthMiddleware,
        models::user::{
            LoginPayload, LoginResponse, RegisterUserPayload, UserResponse, VerifyUserPayload,
        },
        services::auth::AuthService,
    },
    error::CommResp,
    AxumState, Result,
};

pub struct Auth;

impl Auth {
    pub async fn register(
        State(state): AxumState,
        Json(payload): Json<RegisterUserPayload>,
    ) -> Result<Json<UserResponse>> {
        let res = AuthService::register(&state.pool, payload).await?;
        Ok(Json(res))
    }
}

impl Auth {
    pub async fn login(
        cookies: Cookies,
        State(state): AxumState,
        Json(payload): Json<LoginPayload>,
    ) -> Result<impl IntoResponse> {
        let user = AuthService::login(&state.pool, payload).await?;
        let session = AuthService::new_session(&state.pool, user.id).await?;
        let session_cookie = Cookie::build("session", session.token.to_string())
            .path("/")
            .secure(true)
            .http_only(true)
            .expires(OffsetDateTime::now_utc().add(std::time::Duration::new(60 * 86400, 0)))
            .finish();
        cookies.add(session_cookie);
        Ok((
            AppendHeaders([("x-auth-token", session.token.to_string())]),
            Json(LoginResponse {
                token: session.token.to_string(),
                user: UserResponse::from(user),
            }),
        ))
    }
}

impl Auth {
    pub async fn verify(
        State(state): AxumState,
        Json(payload): Json<VerifyUserPayload>,
    ) -> Result<Json<CommResp>> {
        AuthService::verify(&state.pool, payload).await?;
        Ok(Json(CommResp {
            status: 200,
            message: "User verified",
        }))
    }
}

impl Auth {
    pub async fn me(Extension(auth): Extension<AuthMiddleware>) -> Json<UserResponse> {
        Json(UserResponse::from(auth.user))
    }
}
