use dynomite::Item;
use storage::{self, derives::Entity};

use uuid::Uuid;

#[derive(Item, Clone, Entity)]
pub struct Book {
  #[dynomite(partition_key, rename = "Id")]
  pub id: Uuid,
  #[dynomite(rename = "bookTitle", default)]
  pub title: String,
}
