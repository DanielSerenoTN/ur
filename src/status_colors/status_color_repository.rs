use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection, Pool};
use diesel::mysql::MysqlConnection;
use diesel::result::Error as DieselError;
use crate::db::schema::status_colors;
use super::entities::status_color_entity::StatusColor;

pub struct StatusColorRepository {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl StatusColorRepository {
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

    pub fn get_all_colors(&self) -> Result<Vec<StatusColor>, DieselError> {
        let conn = &mut self.get_conn()?;

        status_colors::table
            .order(status_colors::name.asc())
            .load::<StatusColor>(conn)
    }


}