use serde::{Deserialize, Serialize};
// use diesel::prelude::{Insertable};
// use diesel::query_builder::{AsChangeset};

#[derive(Deserialize, Serialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32
}
