use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        // Generate a random unique identifier no longer needed with TracingLogger vs Logger 
        // request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
// orchestrates the work to be done by calling the required routines and translates their outcome into the proper response according to the rules and conventions of the HTTP protocol.
pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&pool, &form).await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
// takes care of the database logic and it has no awareness of the surrounding web framework - i.e. we are not passing web::Form or web::Data wrappers as input types;
pub async fn insert_subscriber(
    pool: &PgPool,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.email,
        form.name,
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


    // // Spans, like logs, have an associated level
    // // `info_span` creates a span at the info-level
    // let request_span = tracing::info_span!(
    //     "Adding a new subscriber.",
    //     %request_id,
    //     subscriber_email = %form.email,
    //     subscriber_name = %form.name
    // );
    // // Using `enter` in an async function is a recipe for disaster!
    // // Bear with me for now, but don't do this at home.
    // // See the following section on `Instrumenting Futures`
    // let _request_span_guard = request_span.enter();
        // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    // let query_span = tracing::info_span!(
    //     "Saving new subscriber details in the database"
    // );
    // // `Result` has two variants: `Ok` and `Err`.
	// // The first for successes, the second for failures.
	// // We use a `match` statement to choose what to do based
	// // on the outcome.
    // match sqlx::query!(
    //     r#"
    //     INSERT INTO subscriptions (id, email, name, subscribed_at)
    //     VALUES ($1, $2, $3, $4)
    //     "#,
    //     Uuid::new_v4(),
    //     form.email,
    //     form.name,
    //     Utc::now()
    // )
	// // We use `get_ref` to get an immutable reference to the `PgConnection`
	// // wrapped by `web::Data`.	
    // .execute(pool.get_ref())
    // // First we attach the instrumentation, then we `.await` it
    // .instrument(query_span)
    // .await