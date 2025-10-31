use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection, Pool};
use diesel::mysql::MysqlConnection;
use diesel::result::Error as DieselError;
use crate::db::schema::zoho_code;
use super::entities::zoho_code_entity::{NewZohoCode, ZohoCode};
use chrono::Utc;

pub struct ZohoCodeRepository {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl ZohoCodeRepository {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        Self { pool }
    }

    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, DieselError> {
        self.pool.get().map_err(|_| {
            eprintln!("Failed to get DB connection");
            DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(String::from("Failed to get DB connection"))
            )
        })
    }

    pub fn create_zoho_code(&self, code: &str) -> Result<ZohoCode, DieselError> {
        let conn = &mut self.get_conn()?;
        let new_code = NewZohoCode {
            code: code.to_string(),
        };

        diesel::insert_into(zoho_code::table)
            .values(&new_code)
            .execute(conn)?;

        // Get the last inserted record
        zoho_code::table
            .filter(zoho_code::code.eq(code))
            .order(zoho_code::created_at.desc())
            .first(conn)
    }

    pub fn get_active_code(&self) -> Result<Option<ZohoCode>, DieselError> {
        let conn = &mut self.get_conn()?;

        zoho_code::table
            .filter(zoho_code::expired_at.is_null())
            .order(zoho_code::created_at.desc())
            .first(conn)
            .optional()
    }

    pub fn expire_all_active_codes(&self) -> Result<usize, DieselError> {
        let conn = &mut self.get_conn()?;
        let now = Utc::now().naive_utc();

        diesel::update(
            zoho_code::table.filter(zoho_code::expired_at.is_null())
        )
        .set(zoho_code::expired_at.eq(now))
        .execute(conn)
    }

    pub fn is_code_active(&self, code: &str) -> Result<bool, DieselError> {
        let conn = &mut self.get_conn()?;

        let count: i64 = zoho_code::table
            .filter(zoho_code::code.eq(code))
            .filter(zoho_code::expired_at.is_null())
            .count()
            .get_result(conn)?;

        Ok(count > 0)
    }
}