use shaku::module;

pub mod book_service;

module! {
    pub BookModule {
        components = [book_service::BookServiceImpl],
        providers = []
    }
}
