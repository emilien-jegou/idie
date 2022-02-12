use dynomite::dynamodb::DynamoDb;
use dyn_clone::{ DynClone };

pub mod create_table;
pub mod create_client;
pub mod put_item;

pub trait DynomiteClient: DynamoDb + DynClone + Sync + Send + 'static {}

dyn_clone::clone_trait_object!(DynomiteClient);

pub use create_table::create_table;
pub use create_client::create_client;
pub use put_item::put_item;
