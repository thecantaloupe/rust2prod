use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct UserFormData {
    name: String,
    email: String,
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(get_user);
    cfg.service(post_user);
}

#[tracing::instrument(name = "Getting all users", skip(pool))]
#[get("/user")]
async fn get_all_users(pool: web::Data<PgPool>) -> impl Responder {
    let users = get_users(&pool).await;

    match users {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(users) => HttpResponse::Ok().json(users),
    }
}
pub async fn get_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    SELECT *
    FROM users
        "#,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}

#[tracing::instrument(name = "Getting a single users",skip(pool),fields(user_id = %user_id,))]
#[get("/user/{id}")]
async fn get_user(user_id: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let user = get_user_by_id(&pool, &user_id).await;

    match user {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(user) => HttpResponse::Ok().json(user),
    }
}
pub async fn get_user_by_id(pool: &PgPool, user_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    SELECT id, name, email, created_at
    FROM users
    WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Adding a new user",
    skip(form),
    fields(
        user_name = %form.name,
        user_email = %form.email,
    )
)]
#[post("/user")]
async fn post_user(
    // web::Json<UserFormData> to test 
    form: web::Form<UserFormData>, 
    pool: web::Data<PgPool>
) -> HttpResponse {
    match add_user(&pool, &form).await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
pub async fn add_user(
    pool: &PgPool, 
    form: &UserFormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO users (id, name, email, created_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}
