use super::{BookController, BookModule};
use ratelimit;
use shaku::HasComponent;
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};
use crate::middlewares::rate_limiter::rate_limiter;

/// POST /add_book
pub fn add_book(module: Arc<BookModule>) -> BoxedFilter<(impl warp::Reply,)> {
  let limiter = ratelimit::Builder::new()
    .capacity(50)
    .quantum(40)
    .interval(std::time::Duration::new(1, 0))
    .build();

  warp::path!("add_book")
    .and(warp::post())
    .and(rate_limiter(limiter).untuple_one())
    .and(warp::any().map(move || module.resolve()))
    .and_then(|controller: Arc<dyn BookController>| async move { controller.add_book().await })
    .boxed()
}

pub fn entry(
  module: Arc<BookModule>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  add_book(module)
}
