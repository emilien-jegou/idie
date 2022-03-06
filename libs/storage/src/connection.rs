use super::Entity;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Connection<T: Entity>: Default + Clone + Send + Sync {
  async fn create_table(&self, table_name: &str) -> Result<(), Box<dyn Error>>;
  async fn put(&self, entity: T) -> Result<(), Box<dyn Error>>;
}
