// Import the traits needed for serialization and deserialization
// from the `serde` library.
use serde::{Deserialize, Serialize};

// Import the `Uuid` type from the `uuid` crate.
use uuid::Uuid;

// Automatically implement the `Serialize` and `Deserialize` traits
// for the `Item` struct using `serde`.
#[derive(Serialize, Deserialize)]
pub struct Item {
    // The `id` field is a UUID to uniquely identify an `Item`.
    pub id: Uuid,
    // The `name` field is a `String` that holds the item's name.
    pub name: String,
    // The `description` field is a `String` that provides more details about the item.
    pub description: String,
}
