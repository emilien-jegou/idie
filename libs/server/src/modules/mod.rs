use std::sync::Arc;

pub mod book;

pub type AppModule = Arc<book::BookModule>;

pub fn build() -> AppModule {
  Arc::new(book::BookModule::builder().build())
}
