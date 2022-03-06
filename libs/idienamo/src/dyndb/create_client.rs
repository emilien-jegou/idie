use dynomite::{
  dynamodb::DynamoDbClient,
  retry::{ Policy, RetryingDynamoDb},
  Retries,
};
use std::error::Error;
use super::DynomiteClient;

use rusoto_core::Region;
use std::env;

impl DynomiteClient for RetryingDynamoDb<DynamoDbClient> {}

pub fn create_client() -> Result<Box<dyn DynomiteClient>, Box<dyn Error>> {
  let client = DynamoDbClient::new(Region::Custom {
    name: env::var("AWS_REGION")?,
    endpoint: env::var("AWS_DYNDB_ENDPOINT")?,
  })
  .with_retries(Policy::default());

  Ok(Box::new(client))
}
