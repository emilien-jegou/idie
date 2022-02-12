use super::{BookController, BookModule};
use shaku::HasComponent;
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

/// POST /add_book
pub fn add_book(module: Arc<BookModule>) -> BoxedFilter<(impl warp::Reply,)> {
  warp::path!("add_book")
    .and(warp::post())
    .and(warp::any().map(move || module.resolve()))
    .and_then(|controller: Arc<dyn BookController>| async move { controller.add_book().await })
    .boxed()
}

pub fn entry(
  module: Arc<BookModule>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  add_book(module)
}
