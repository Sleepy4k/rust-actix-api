use serde::Serialize;

#[doc = "Product struct"]
#[derive(Debug, Serialize)]
pub struct ProductStruct {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub amount: i32
}