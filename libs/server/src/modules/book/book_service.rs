use super::Book;
use async_trait::async_trait;
use idienamo::{self, Repository};
use shaku::{Component, Interface};
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait BookService: Interface {
  async fn add_book(&self, name: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Component)]
#[shaku(interface = BookService)]
pub struct BookServiceImpl {
  #[shaku(inject)]
  repository: Arc<dyn Repository<Book>>,
}

#[async_trait]
impl BookService for BookServiceImpl {
  async fn add_book(&self, name: &str) -> Result<(), Box<dyn Error>> {
    println!("here");
    self
      .repository
      .put(Book {
        id: Uuid::new_v4(),
        title: "AAAAAAAAAA".into(),
      })
      .await?;
    println!("Added book {}", name);
    Ok(())
  }
}
