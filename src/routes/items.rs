// Bring in the `web` module (for extracting request data) and `HttpResponse` type
// from Actix Web.
use actix_web::{web, HttpResponse};

// Import the `Uuid` type from the `uuid` crate for generating and handling UUIDs.
use uuid::Uuid;

// Import the `json!` macro from `serde_json` to easily create JSON objects.
use serde_json::json;

// Import the `Item` struct from our `models` module.
use crate::models::item::Item;

// Asynchronous function to create a new item.
// `item` is extracted from the request body as JSON matching the `Item` struct.
pub async fn create_item(item: web::Json<Item>) -> HttpResponse {
    // Return an HTTP 200 (OK) response with the JSON of the created item.
    // `into_inner()` converts the `web::Json<Item>` wrapper into the raw `Item` struct.
    HttpResponse::Ok().json(item.into_inner())
}

// Asynchronous function to get a list of items.
pub async fn get_items() -> HttpResponse {
    // Create a vector of items to simulate data being fetched from a database.
    let items = vec![
        // First `Item` instance
        Item {
            // Generate a new UUID
            id: Uuid::new_v4(),
            // Set the name
            name: String::from("Item 1"),
            // Set the description
            description: String::from("Description 1"),
        },
        // Second `Item` instance
        Item {
            // Generate another new UUID
            id: Uuid::new_v4(),
            // Set the name
            name: String::from("Item 2"),
            // Set the description
            description: String::from("Description 2"),
        },
    ];
    // Return an HTTP 200 (OK) response with the JSON of the vector of items.
    HttpResponse::Ok().json(items)
}

// Asynchronous function to get a single item by its ID.
// `item_id` is extracted from the request path as a `Uuid`.
pub async fn get_item(item_id: web::Path<Uuid>) -> HttpResponse {
    // Construct an `Item` using the provided `item_id`, simulating a database lookup.
    let item = Item {
        // Dereference the `web::Path<Uuid>` to get a `Uuid`
        id: *item_id,
        // Set the name
        name: String::from("Item 1"),
        // Set the description
        description: String::from("Description 1"),
    };
    // Return an HTTP 200 (OK) response with the JSON of this single item.
    HttpResponse::Ok().json(item)
}

// Asynchronous function to update an item by its ID.
// `item_id` is extracted from the path, `item` is the JSON payload with the updated data.
pub async fn update_item(item_id: web::Path<Uuid>, item: web::Json<Item>) -> HttpResponse {
    // Extract the `Item` from the JSON wrapper.
    let mut updated_item = item.into_inner();
    // Overwrite the `id` of the extracted item with the path `item_id`.
    updated_item.id = *item_id;
    // Return an HTTP 200 (OK) response with the updated item.
    HttpResponse::Ok().json(updated_item)
}

// Asynchronous function to delete an item by its ID.
pub async fn delete_item(item_id: web::Path<Uuid>) -> HttpResponse {
    // Return an HTTP 200 (OK) response with a JSON object indicating that the item was deleted.
    // `json!` creates a JSON object that includes the stringified `item_id`.
    HttpResponse::Ok().json(json!({ "deleted": item_id.to_string() }))
}
