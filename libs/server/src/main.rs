use idienamo::DynamoConnection;
use warp::{filters::BoxedFilter, Filter};

mod modules;
mod middlewares;

fn healthcheck() -> BoxedFilter<(impl warp::Reply,)> {
  warp::get()
    .and(warp::path("healthcheck"))
    .map(warp::reply)
    .boxed()
}

#[tokio::main]
async fn main() {
  env_logger::init();
  let connection = DynamoConnection::connect().unwrap();
  let book_router = modules::book::load(&connection).await.unwrap();

  let router = healthcheck().or(book_router);

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
