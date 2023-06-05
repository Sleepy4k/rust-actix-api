use serde::Serialize;

#[doc = "Spare part struct"]
#[derive(Debug, Serialize)]
pub struct SparePartStruct {
    pub id: i32,
    pub name: String,
    pub price: i32
}