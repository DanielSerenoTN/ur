use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::db::schema::products;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = products)]
pub struct Product {
    pub id: String,
    pub product_name: Option<String>,
    pub estatus_venta: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub id: String,
    pub product_name: Option<String>,
    pub estatus_venta: Option<String>,
}