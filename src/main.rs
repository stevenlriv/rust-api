// ENV
extern crate dotenv;
use dotenv::dotenv;
use std::env;

// Import the modules and types we need from the Actix Web framework:
// - `web` for routing helpers (like scopes, routes, etc.)
// - `App` to create an Actix application
// - `HttpServer` to create and run the HTTP server
use actix_web::{web, App, HttpServer};

// Declare references to local modules.
// Bring the `routes` module into scope.
mod models;
mod routes;

// This macro from Actix Web marks the `main` function
// as the starting point for the Actix Web application
// and sets up the asynchronous runtime.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Database
    dotenv().ok();
    let _db_master_host = env::var("DB_MASTER_HOST").expect("DB_MASTER_HOST must be set");

    // Create and run a new HTTP server.
    HttpServer::new(|| {
        // Build the Actix application for each thread.
        App::new()
            // Register a service. Here, we're creating a scoped resource under "/api".
            .service(
                web::scope("/api")
                    // Healthcheck
                    .route(
                        "/healthcheck",
                        web::get().to(routes::healthcheck::get_healthcheck),
                    )
                    // Define a POST route at "/api/items" that calls the `create_item` handler.
                    .route("/items", web::post().to(routes::items::create_item))
                    // Define a GET route at "/api/items" that calls the `get_items` handler.
                    .route("/items", web::get().to(routes::items::get_items))
                    // Define a GET route at "/api/items/{id}" that calls the `get_item` handler.
                    .route("/items/{id}", web::get().to(routes::items::get_item))
                    // Define a PUT route at "/api/items/{id}" that calls the `update_item` handler.
                    .route("/items/{id}", web::put().to(routes::items::update_item))
                    // Define a DELETE route at "/api/items/{id}" that calls the `delete_item` handler.
                    .route("/items/{id}", web::delete().to(routes::items::delete_item)),
            )
    })
    // Bind the server to the address "127.0.0.1:8080".
    // The `?` operator returns an error if binding fails.
    .bind("127.0.0.1:8080")?
    // Start the server and begin listening for incoming requests.
    .run()
    // Since `run()` is async, we `await` it to keep the server running.
    .await
}
