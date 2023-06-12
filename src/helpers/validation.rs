use serde_json::Value;
use std::any::type_name;

pub trait IsEmpty {
  fn is_empty(&self) -> bool;
}

impl IsEmpty for &str {
  fn is_empty(&self) -> bool {
    self.trim().is_empty()
  }
}

impl IsEmpty for String {
  fn is_empty(&self) -> bool {
    self.trim().is_empty()
  }
}

impl IsEmpty for Vec<Value> {
  fn is_empty(&self) -> bool {
    self.is_empty()
  }
}

impl IsEmpty for i32 {
  fn is_empty(&self) -> bool {
    false
  }
}

#[doc = "Check if value is empty"]
pub fn check_if_empty<T: IsEmpty>(data: T) -> bool {
  data.is_empty()
}

#[doc = "Check type of value"]
pub fn check_type<T>(_value: T) -> String {
  let type_name = type_name::<T>().to_string();

  println!("type of value: {}", type_name);

  type_name
}
