use crate::schema::rustaceans;
use diesel::{DieselNumericOps, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
