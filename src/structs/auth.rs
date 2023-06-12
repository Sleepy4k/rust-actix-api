use serde::{Serialize, Deserialize};

#[doc = "Token struct"]
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStruct {
  pub id: i32,
  pub role: String,
  pub username: String,
  pub iat: u64,
  pub exp: u64,
}

#[doc = "Detail user struct"]
#[derive(Debug, Serialize)]
pub struct DetailUserStruct {
  pub id: i32,
  pub username: String,
  pub role: String,
}