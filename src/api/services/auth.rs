use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    api::models::{user::{RegisterUserPayload, UserResponse,User, LoginPayload, VerifyUserPayload}, session::Session},
    AppError, Result, utils 
};

pub struct AuthService;

impl AuthService {
    pub async fn new_session(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Session> {
        let create_session = sqlx::query_as!(
            Session,
            r#"INSERT INTO "session" (user_id, expiration) VALUES ($1,$2) RETURNING *"#,
            user_id,
            chrono::Utc::now() + chrono::Duration::days(60),
        ).fetch_one(pool).await.map_err(|e| {
            tracing::error!("Error creating session: {:?}", e);
            AppError::InternalServerError            
        });
        create_session
    }
}

impl AuthService {
    pub async fn register(
        pool: &Pool<Postgres>,
        payload: RegisterUserPayload,
    ) -> Result<UserResponse> {
        let hashed_password = utils::argon::hash_password(&payload.password)?;
        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO "users" (name,email, password, verification_token) VALUES ($1, $2, $3, $4) RETURNING *"#,
            payload.name,
            payload.email,
            hashed_password,
            uuid::Uuid::new_v4().to_string(),
        )
        .fetch_one(pool)
        .await.map_err(|e| {
                let err = e.into_database_error();
                if let Some(error) = err {
                  return match error.kind() {
                    sqlx::error::ErrorKind::UniqueViolation => AppError::UserAlreadyExists,
                    _ => AppError::InternalServerError,
                    }
                } 
                AppError::InternalServerError
            })?;
        tracing::info!("User created: {:?}", user);
        Ok(UserResponse::from(user))
    }
}

impl AuthService {
    pub async fn login(
        pool: &Pool<Postgres>,
        payload: LoginPayload,
    ) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM "users" WHERE email = $1"#,
            payload.email,
        )
        .fetch_one(pool)
        .await.map_err(|_e| {
            AppError::InvalidCredentials
        })?;
        if !user.is_verified {
            return Err(AppError::UserNotVerified);
        }
        utils::argon::verify_password(&payload.password, &user.password)?;
        tracing::info!("User logged in: {:?}", user);
        Ok(user)
    }
}

impl AuthService {
    pub async fn verify(
        pool: &Pool<Postgres>,
        payload: VerifyUserPayload,
    ) -> Result<()> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM "users" WHERE id = $1"#,
            payload.id,
        )
        .fetch_one(pool)
        .await.map_err(|_e| {
            AppError::InvalidVerificationToken
        })?;
       match user.verification_token {
           Some(token) => {
               if token == payload.verification_token {
                   sqlx::query!("UPDATE \"users\" SET is_verified = true, verification_token = NULL WHERE id = $1", payload.id).execute(pool).await.map_err(|_e| {
                       tracing::error!("Error verifying user");
                       AppError::InternalServerError                       
                   })?;
                   Ok(())
               }
               else {
                   Err(AppError::InvalidVerificationToken)
               }
           }
           None => Err(AppError::InvalidVerificationToken),
       } 
    }
}

impl AuthService {
    pub async fn get_user_by_session_token(pool: &Pool<Postgres>, token: Uuid) -> Result<User> {
       let user = sqlx::query_as!(
           User,
           r#"SELECT "users".id, "users".name, "users".email, "users".password, 
           "users".verification_token, "users".is_verified, "users".reset_token, "users".created_at, "users".updated_at
           FROM "users" 
           LEFT JOIN "session" 
           ON "users".id = "session".user_id WHERE "session".token = $1"#,
           token
       ).fetch_one(pool).await.map_err(|_e| {
           AppError::Unauthorized
       });
       user
    }
}
