use crate::dyndb::create_client;
use std::error::Error;
use std::fmt;
use super::dyndb::DynomiteClient;

#[derive(Default, Clone)]
pub struct DynamoConnection {
  pub client: Option<Box<dyn DynomiteClient>>,
}

impl DynamoConnection {
  pub fn connect() -> Result<DynamoConnection, Box<dyn Error>> {
    Ok(DynamoConnection {
      client: Some(create_client()?),
    })
  }
}

impl fmt::Debug for DynamoConnection {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("DynamoConnection").finish()
  }
}
