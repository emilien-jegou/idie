use super::DynomiteClient;
use dynomite::dynamodb::PutItemInput;
use dynomite::{Attributes};
use std::error::Error;

/// create a table with a single string (S) primary key.
pub async fn put_item(
  client: &dyn DynomiteClient,
  table_name: &str,
  attributes: Attributes,
) -> Result<(), Box<dyn Error>> {
  client
    .put_item(PutItemInput {
      table_name: table_name.into(),
      item: attributes,
      ..PutItemInput::default()
    })
    .await?;
  Ok(())
}
