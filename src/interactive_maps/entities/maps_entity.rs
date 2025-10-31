use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::db::schema;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::maps_svg)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SvgItem {
    #[diesel(column_name = id)]
    pub id: String,
    #[diesel(column_name = name)]
    pub name: String,
    #[diesel(column_name = prefix)]
    pub prefix: String,
    #[diesel(column_name = content)]
    pub content: String,
    #[diesel(column_name = created_at)]
    pub created_at: NaiveDateTime,
    #[diesel(column_name = updated_at)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::maps_svg)]
pub struct NewSvgItem {
    #[diesel(column_name = id)]
    pub id: String,
    #[diesel(column_name = name)]
    pub name: String,
    #[diesel(column_name = prefix)]
    pub prefix: String,
    #[diesel(column_name = content)]
    pub content: String,
}


impl NewSvgItem {
    pub fn new(name: String, prefix: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            prefix,
            content,
        }
    }
}