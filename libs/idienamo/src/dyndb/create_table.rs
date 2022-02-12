use dynomite::dynamodb::{
  AttributeDefinition, CreateTableError::ResourceInUse, CreateTableInput,
  KeySchemaElement, ProvisionedThroughput,
};
use rusoto_core::RusotoError::Service;
use std::error::Error;
use super::DynomiteClient;

/// create a table with a single string (S) primary key.
pub async fn create_table(
  client: &dyn DynomiteClient,
  table_name: &str,
) -> Result<(), Box<dyn Error>> {
  let res = client
    .create_table(CreateTableInput {
      table_name: table_name.into(),
      key_schema: vec![KeySchemaElement {
        attribute_name: "Id".into(),
        key_type: "HASH".into(),
      }],
      attribute_definitions: vec![AttributeDefinition {
        attribute_name: "Id".into(),
        attribute_type: "S".into(),
      }],
      provisioned_throughput: Some(ProvisionedThroughput {
        read_capacity_units: 1000,
        write_capacity_units: 1000,
      }),
      ..CreateTableInput::default()
    })
    .await;
  match res {
    Ok(_) => Ok(()),
    Err(Service(ResourceInUse(_))) => Ok(()),
    Err(x) => Err(x),
  }?;
  println!("Created table {}", table_name);
  Ok(())
}
