use crate::routes::{health_check, subscribe};
use super::{controller};
use actix_web::{web, App, HttpServer, http};
use actix_web::dev::Server;
use actix_cors::Cors;
use tracing_actix_web::TracingLogger;
use sqlx::PgPool;
use std::net::TcpListener;



// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
pub fn run(
    listener: TcpListener,
	// New parameter!
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // transfer ownership of the AppState to the HttpServer via the `move`.
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://xenodochial-ardinghelli-436772.netlify.app")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .configure(controller::init_user_controller)
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}