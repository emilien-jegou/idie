use crate::entity::Entity;
use dynomite::dynamodb::PutItemInput;
use dynomite::{Attributes, Item};
use super::DynomiteClient;
use std::error::Error;

/// create a table with a single string (S) primary key.
pub async fn put_item<T: Entity + Item>(
  client: &dyn DynomiteClient,
  attributes: Attributes,
) -> Result<(), Box<dyn Error>> {
  client
    .put_item(PutItemInput {
      table_name: T::get_table_name().into(),
      item: attributes,
      ..PutItemInput::default()
    })
    .await?;
  Ok(())
}
