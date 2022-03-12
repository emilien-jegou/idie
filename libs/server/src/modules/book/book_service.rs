use super::Book;
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::error::Error;
use std::sync::Arc;
use storage::{self, ShakuRepository};
use uuid::Uuid;
use log::info;

#[async_trait]
pub trait BookService: Interface {
  async fn add_book(&self, name: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Component)]
#[shaku(interface = BookService)]
pub struct BookServiceImpl {
  #[shaku(inject)]
  repository: Arc<dyn ShakuRepository<Book>>,
}

#[async_trait]
impl BookService for BookServiceImpl {
  async fn add_book(&self, name: &str) -> Result<(), Box<dyn Error>> {
    self
      .repository
      .put(Book {
        id: Uuid::new_v4(),
        title: "AAAAAAAAAA".into(),
      })
      .await?;
    info!("Successfully saved book {}", name);
    Ok(())
  }
}
