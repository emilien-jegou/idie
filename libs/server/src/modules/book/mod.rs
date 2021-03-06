use idienamo::{self, DynamoConnection};
use shaku::{ HasComponent, module };
use std::error::Error;
use std::sync::Arc;
use storage::Repository;
use storage::ShakuRepository;
use warp::{Filter, Rejection, Reply};

pub mod book_controller;
pub mod book_entity;
pub mod book_service;
mod routes;

pub use book_controller::BookController;
pub use book_entity::Book;
pub use book_service::BookService;

module! {
    pub BookModule {
        components = [Repository<Book, DynamoConnection>, book_service::BookServiceImpl, book_controller::BookControllerImpl],
        providers = []
    }
}

pub async fn load(
  connection: &DynamoConnection,
) -> Result<impl Filter<Extract = impl Reply, Error = Rejection> + Clone, Box<dyn Error>> {
  let module = Arc::new(
    BookModule::builder()
      .with_component_parameters::<Repository<Book, DynamoConnection>>(connection.clone())
      .build(),
  );

  let book_repository: &dyn ShakuRepository<Book> = (*module).resolve_ref();

  book_repository.create_table().await?;

  let routes = routes::entry(module);

  Ok(routes)
}
