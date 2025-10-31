use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use std::env;
use std::time::Duration;
use dotenv::dotenv;

pub mod schema;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(15) 
        .min_idle(Some(5)) 
        .connection_timeout(Duration::from_secs(30))  
        .idle_timeout(Some(Duration::from_secs(120)))  
        .max_lifetime(Some(Duration::from_secs(900))) 
        .build(manager)
        .expect("Failed to create pool.")
}


pub fn get_connection(pool: &DbPool) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, diesel::result::Error> {
    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            eprintln!("Error obteniendo conexión de MySQL: {:?}", e);
            Err(diesel::result::Error::NotFound)
        }
    }
}
