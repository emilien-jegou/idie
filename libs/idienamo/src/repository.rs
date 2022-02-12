use crate::dyndb;
use crate::{connection::DynamoConnection, entity::Entity};
use async_trait::async_trait;
use shaku::{Component, Interface, Module, ModuleBuildContext};
use std::error::Error;
use std::marker::PhantomData;

#[async_trait]
pub trait Repository<T: Entity>: Interface {
  async fn create_table(&self) -> Result<(), Box<dyn Error>>;
  async fn put(&self, entity: T) -> Result<(), Box<dyn Error>>;
  /* Unimplemented dynamodb actions */
  //pub async fn get(&self) -> Result<Box<T>, Box<dyn Error>>;
  //pub async fn update(&self) -> Result<(), Box<dyn Error>>;
  //pub async fn delete(&self) -> Result<Box<T>, Box<dyn Error>>;
  //pub async fn batch_get(&self) -> Result<Box<T>, Box<dyn Error>>;
  //pub async fn batch_write(&self) -> Result<Box<T>, Box<dyn Error>>;
  //pub async fn query(&self) -> Result<Box<T>, Box<dyn Error>>;
  //pub async fn scan(&self) -> Result<Box<T>, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct DynamoRepository<T: Entity + 'static> {
  phantom: PhantomData<T>,
  connection: DynamoConnection,
}

impl<T: Entity> DynamoRepository<T> {
  pub fn new(connection: DynamoConnection) -> DynamoRepository<T> {
    DynamoRepository {
      connection,
      phantom: PhantomData,
    }
  }
}

#[async_trait]
impl<T: Entity + Send + Sync + 'static> Repository<T> for DynamoRepository<T> {
  async fn create_table(&self) -> Result<(), Box<dyn Error>> {
    dyndb::create_table(
      &**self.connection.client.as_ref().unwrap(),
      T::get_table_name(),
    )
    .await
  }

  async fn put(&self, entity: T) -> Result<(), Box<dyn Error>> {
    dyndb::put_item::<T>(&**self.connection.client.as_ref().unwrap(), entity.into()).await
  }
}

impl<T: Entity + Send + Sync + 'static, M: Module> Component<M> for DynamoRepository<T> {
  type Interface = dyn Repository<T>;
  type Parameters = DynamoConnection;

  fn build(_: &mut ModuleBuildContext<M>, params: Self::Parameters) -> Box<Self::Interface> {
    Box::new(Self::new(params))
  }
}
