use shaku::{Component, Interface};

pub trait BookService: Interface {
  fn add_book(&self, name: &str);
}

#[derive(Component)]
#[shaku(interface = BookService)]
pub struct BookServiceImpl {}

impl BookService for BookServiceImpl {
  fn add_book(&self, name: &str) {
    println!("Added book {}", name);
  }
}
