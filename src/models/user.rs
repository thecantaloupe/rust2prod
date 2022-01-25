use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query_as};
use chrono::{Utc, DateTime};



#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: String) -> CustomError {
        CustomError {
            error_status_code,
            error_message,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn find_all(db_pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let rows = query_as!(
            User,
            r#"
            SELECT id, name, email, created_at from users
            "#
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
            // Using the `?` operator to return early
            // if the function failed, returning a sqlx::Error
            // We will talk about error handling in depth later!
        })?;
        Ok(rows)
    }
}