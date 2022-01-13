use std::sync::Arc;
use modules::book::book_service::BookService;
use shaku::HasComponent;
use warp::{ Filter, filters::BoxedFilter };

pub mod database;
pub mod modules;

fn healthcheck() -> BoxedFilter<(impl warp::Reply,)> {
  warp::get().and(warp::path("healthcheck")).map(warp::reply).boxed()
}

async fn add_book(module: Arc<modules::book::BookModule>) -> Result<impl warp::Reply, warp::Rejection> {
  let book_service: &dyn BookService = module.resolve_ref();

  book_service.add_book("Hey I am a book :D");
  Ok(warp::reply::with_status(
    "Added random to the book list",
    warp::http::StatusCode::CREATED,
  ))
}

#[tokio::main]
async fn main() {
  let modules = modules::book::build();

  let modules_filter = warp::any().map(move || modules.clone());

  let add_book = warp::post()
    .and(warp::path("add_book"))
    .and(warp::path::end())
    .and(modules_filter.clone())
    .and_then(add_book);

  let router = healthcheck().or(add_book);

  database::launch_dynamodb().await.unwrap();
  warp::serve(router).run(([0, 0, 0, 0], 3000)).await;
}

#[tokio::test]
async fn test_healthcheck() {
  let filter = healthcheck();

  // Check if healthcheck route exist.
  let res = warp::test::request()
    .path("/healthcheck")
    .reply(&filter)
    .await;

  assert_eq!(res.status(), 200);
}
