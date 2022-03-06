use async_trait::async_trait;
use dynomite::{Attributes, Item};
use std::error::Error;
use std::fmt;
use storage::{Connection, Entity};

pub mod dyndb;

#[derive(Default, Clone)]
pub struct DynamoConnection {
  pub client: Option<Box<dyn dyndb::DynomiteClient>>,
}

impl DynamoConnection {
  pub fn connect() -> Result<DynamoConnection, Box<dyn Error>> {
    Ok(DynamoConnection {
      client: Some(dyndb::create_client()?),
    })
  }
}


#[async_trait]
impl<T: Entity + Item + Into<Attributes> + Send + Sync + 'static> Connection<T> for DynamoConnection {
  async fn create_table(&self, name: &str) -> Result<(), Box<dyn Error>> {
    dyndb::create_table(&**self.client.as_ref().unwrap(), name).await
  }

  async fn put(&self, entity: T) -> Result<(), Box<dyn Error>> {
    dyndb::put_item(
      &**self.client.as_ref().unwrap(),
      T::get_table_name(),
      entity.into(),
    )
    .await
  }
}

impl fmt::Debug for DynamoConnection {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("DynamoConnection").finish()
  }
}
