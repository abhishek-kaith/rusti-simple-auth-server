use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct Session {
    pub token: Uuid,
    pub user_id: Uuid,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
