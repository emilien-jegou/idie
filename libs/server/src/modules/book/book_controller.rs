use super::BookService;
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use warp::{Rejection, Reply};

#[async_trait]
pub trait BookController: Interface {
  async fn add_book(&self) -> Result<Box<dyn Reply>, Rejection>;
}

#[derive(Component)]
#[shaku(interface = BookController)]
pub struct BookControllerImpl {
  #[shaku(inject)]
  book_service: Arc<dyn BookService>,
}

#[async_trait]
impl BookController for BookControllerImpl {
  async fn add_book(&self) -> Result<Box<dyn Reply>, Rejection> {
    self.book_service.add_book("hey!").await.unwrap();
    Ok(Box::new(warp::reply::with_status(
      "Added book 'hey' to the list",
      warp::http::StatusCode::CREATED,
    )))
  }
}
