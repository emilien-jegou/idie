use std::sync::Arc;
use shaku::module;

pub mod book_service;

module! {
    pub BookModule {
        components = [book_service::BookServiceImpl],
        providers = []
    }
}

pub fn build() -> Arc<BookModule> {
  Arc::new(BookModule::builder().build())
}
