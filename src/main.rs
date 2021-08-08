use warp::Filter;

pub mod database;

fn healthcheck() -> impl Filter<Extract = impl warp::Reply> + Clone {
  warp::path("healthcheck").map(warp::reply)
}

#[tokio::main]
async fn main() {
  let router = healthcheck();

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
