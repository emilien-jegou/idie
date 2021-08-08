use dynomite::{
  dynamodb::{
    AttributeDefinition, CreateTableInput, DynamoDb, DynamoDbClient, KeySchemaElement,
    ProvisionedThroughput, PutItemInput,
  },
  retry::Policy,
  Item, Retries,
};
use rusoto_core::Region;
use std::error::Error;
use uuid::Uuid;
use std::env;

#[derive(Item, Clone)]
pub struct Book {
  #[dynomite(partition_key, rename = "Id")]
  id: Uuid,
  #[dynomite(rename = "bookTitle", default)]
  title: String,
}

/// create a book table with a single string (S) primary key.
/// if this table does not already exists
/// this may take a second or two to provision.
/// it will fail if this table already exists but that's okay,
/// this is just an example :)
async fn bootstrap<D>(client: &D, table_name: String)
where
  D: DynamoDb,
{
  let _ = client
    .create_table(CreateTableInput {
      table_name,
      key_schema: vec![KeySchemaElement {
        attribute_name: "Id".into(),
        key_type: "HASH".into(),
      }],
      attribute_definitions: vec![AttributeDefinition {
        attribute_name: "Id".into(),
        attribute_type: "S".into(),
      }],
      provisioned_throughput: Some(ProvisionedThroughput {
        read_capacity_units: 1,
        write_capacity_units: 1,
      }),
      ..CreateTableInput::default()
    })
    .await;
}

// this will create a rust book shelf in your aws account!
pub async fn launch_dynamodb() -> Result<(), Box<dyn Error>> {
  // create rusoto client
  let client = DynamoDbClient::new(Region::Custom {
    name: env::var("AWS_REGION")?,
    endpoint: env::var("AWS_DYNDB_ENDPOINT")?,
  })
  .with_retries(Policy::default());

  let table_name = "books".to_string();

  bootstrap(&client, table_name.clone()).await;

  let book = Book {
    id: Uuid::new_v4(),
    title: "rust".into(),
  };

  // add a book to the shelf
  client
    .put_item(PutItemInput {
      table_name: table_name.clone(),
      item: book.clone().into(), // <= convert book into it's attribute map representation
      ..PutItemInput::default()
    })
    .await?;

  Ok(())
}
