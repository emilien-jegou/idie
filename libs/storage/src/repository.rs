use crate::{connection::Connection, entity::Entity};
use async_trait::async_trait;
use shaku::{Component, Interface, Module, ModuleBuildContext};
use std::error::Error;
use std::marker::PhantomData;

#[async_trait]
pub trait ShakuRepository<T: Entity + 'static>: Interface {
  async fn create_table(&self) -> Result<(), Box<dyn Error>>;
  async fn put(&self, entity: T) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Repository<T: Entity + 'static, C: Connection<T>> {
  phantom: PhantomData<T>,
  connection: C,
}

impl<T: Entity, C: Connection<T>> Repository<T, C> {
  pub fn new(connection: C) -> Repository<T, C> {
    Repository {
      connection,
      phantom: PhantomData,
    }
  }
}

#[async_trait]
impl<T: Entity + Send + Sync, C: Connection<T> + 'static> ShakuRepository<T> for Repository<T, C> {
  async fn create_table(&self) -> Result<(), Box<dyn Error>> {
    self.connection.create_table(T::get_table_name()).await
  }

  async fn put(&self, entity: T) -> Result<(), Box<dyn Error>> {
    self.connection.put(entity).await
  }
}

impl<T: Entity + Send + Sync + 'static, C: Connection<T> + 'static, M: Module> Component<M>
  for Repository<T, C>
{
  type Interface = dyn ShakuRepository<T>;
  type Parameters = C;

  fn build(_: &mut ModuleBuildContext<M>, params: Self::Parameters) -> Box<Self::Interface> {
    Box::new(Self::new(params))
  }
}
