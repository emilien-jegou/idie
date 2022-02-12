use dynomite::{Attributes, Item};

pub trait Entity: Item + Into<Attributes> {
  fn get_table_name() -> &'static str;
}
