use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::{PgPool, FromRow, Row, Error, postgres::PgRow};
use uuid::Uuid;
use chrono::Utc;
use crate::{models::user::User, constants};

#[derive(serde::Deserialize)]
pub struct UserFormData {
    name: String,
    email: String,
}
#[derive(serde::Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}
impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}



impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id = row.try_get("id")?;
        let name = row.try_get("name")?;
        let email = row.try_get("email")?;
        let created_at = row.try_get("created_at")?;
        
        Ok(User{ id, name, email, created_at })
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(get_user);
    cfg.service(post_user);
}

#[tracing::instrument(name = "Getting all users")]
#[get("/user")]
async fn get_all_users(pool: web::Data<PgPool>) -> impl Responder {
    let all_users = User::find_all(&pool).await;
    match all_users{
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(all_users) => HttpResponse::Ok().json(all_users),
    }
}

#[tracing::instrument(name = "Getting a single users",skip(pool),fields(user_id = %user_id,))]
#[get("/user/{id}")]
async fn get_user(user_id: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let user = User::get_user_by_id(&pool, &user_id).await;

    match user {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(user) => HttpResponse::Ok().json(user),
    }
}
// pub async fn get_user_by_id(pool: &PgPool, user_id: &str) -> Result<(), sqlx::Error> {
//     let that_boi = sqlx::query!(
//         r#"
//     SELECT id, name, email, created_at
//     FROM users
//     WHERE id = $1
//         "#,
//         user_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//         // Using the `?` operator to return early
//         // if the function failed, returning a sqlx::Error
//         // We will talk about error handling in depth later!
//     })?;
//     Ok(())
// }

#[tracing::instrument(name = "Adding a new user",skip(form),fields(user_name = %form.name,user_email = %form.email,))]
#[post("/user")]
async fn post_user(
    // web::Json<UserFormData> to test 
    form: web::Form<UserFormData>, 
    pool: web::Data<PgPool>
) -> HttpResponse {
    match add_user(&pool, &form).await {
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
        Uuid::new_v4().to_string(),
        form.name,
        form.email,
        Utc::now(),
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

////////////////////////////////////////////////////////////////////
// Templated response
////////////////////////////////////////////////////////////////////
// #[get("/user")]
// async fn get_all_users() -> impl Responder {
//     HttpResponse::Ok().json(vec![
//         User {
//             id: "612cd23d-654c-4a32-8e8e-3423dc0c869f".to_string(),
//             name : "Tom3".to_string(),
//             email: "thomas_mann33@hotmail.com".to_string(),
//             created_at: "2022-01-24 05:22:13.17567 UTC".parse().unwrap(),

//         },
//        User {
//              id: "d5fb85a6-5c2f-4db1-858a-3268b1f43e51".to_string(),
//              name : "le guin".to_string(),
//              email: "thomas_mann33@hotmail.com".to_string(),
//              created_at: "2022-01-24 13:42:38.045811 UTC".parse().unwrap(),

//         },
//     ])
// }
////////////////////////////////////////////////////////////////////
// Failed Get all attempts
////////////////////////////////////////////////////////////////////
//     // web::Json<UserFormData> to test 
//     pool: web::Data<PgPool>
// ) -> impl Responder  {
//     let records: Vec<User> = get_users(&pool).await?;

//     match records {
//         Ok(records) => {
//             HttpResponse::Ok().finish()
//         },
//         Err(_) => {
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// pub async fn get_users( pool: &PgPool) -> anyhow::Result<(), sqlx::Error> {
//     sqlx::query_as!(User,
//         r"
//     SELECT *
//     FROM users
//         ",
//     )
//     .fetch_all(pool)
//     .await?
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//     })?;
//     Ok(())
// }
////////////////////////////////////////////////////////////////////