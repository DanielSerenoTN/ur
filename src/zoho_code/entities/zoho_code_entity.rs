use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::db::schema::zoho_code;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = zoho_code)]
pub struct ZohoCode {
    pub id: i32,
    pub code: String,
    pub created_at: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = zoho_code)]
pub struct NewZohoCode {
    pub code: String,
}

impl ZohoCode {
    pub fn is_active(&self) -> bool {
        self.expired_at.is_none()
    }

    pub fn time_until_expiration(&self) -> Option<chrono::Duration> {
        self.expired_at.map(|expired_at| {
            expired_at.signed_duration_since(chrono::Utc::now().naive_utc())
        })
    }
}