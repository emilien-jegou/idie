use idie::modules;
use idienamo::DynamoConnection;
use serde_json::Value;
use std::{thread, time};

mod common;

use common::extract_json_body;

#[tokio::test]
async fn books_add_book() -> Result<(), Box<dyn std::error::Error>> {
  let connection = DynamoConnection::connect().unwrap();
  let book_router = modules::book::load(&connection).await.unwrap();

  let ten_millis = time::Duration::from_millis(100); // ratelimiter start empty
  thread::sleep(ten_millis);

  // -- ACTION
  let resp = warp::test::request()
    .method("POST")
    .path("/add_book")
    .reply(&book_router)
    .await;

  // -- CHECK
  assert_eq!(201, resp.status(), "http status");

  // response body is json
  let book: Value = extract_json_body(resp)?;

  Ok(())
}
