use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::db::schema::status_colors;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Clone)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = status_colors)]
pub struct StatusColor {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub hexadecimal: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = status_colors)]
pub struct NewStatusColor {
    pub name: String,
    pub status: String,
    pub hexadecimal: String,
}
