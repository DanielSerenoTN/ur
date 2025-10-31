use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection, Pool};
use diesel::mysql::MysqlConnection;
use diesel::result::Error as DieselError;
use crate::db::schema::products;
use super::entities::products_entity::{NewProduct, Product};
use chrono::{Utc, NaiveDateTime};
use diesel::debug_query;
use diesel::mysql::Mysql;

pub struct ProductRepository {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl ProductRepository {
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

    pub fn upsert_product(&self, product: &NewProduct) -> Result<String, DieselError> {
        let conn = &mut self.get_conn()?;
        let now: NaiveDateTime = Utc::now().naive_utc();
        
        conn.transaction(|conn| {
            let updated = diesel::update(products::table.filter(products::id.eq(&product.id)))
                .set((
                    products::product_name.eq(&product.product_name),
                    products::estatus_venta.eq(&product.estatus_venta),
                    products::updated_at.eq(now),
                ))
                .execute(conn)?;

            if updated == 0 {
                diesel::insert_into(products::table)
                    .values(product)
                    .execute(conn)?;
            }

            Ok(product.id.clone())
        })
    }

    pub fn get_many_by_ids(&self, ids: Vec<&str>) -> Result<Vec<Product>, DieselError> {
        println!("get_many_by_ids: Searching for products with IDs: {:?}", ids);

        let conn = &mut self.get_conn()?;
        let query = products::table
            .filter(products::product_name.eq_any(ids));

        let sql_query = debug_query::<Mysql, _>(&query).to_string();
        println!("Generated SQL Query: {}", sql_query);

        match query.load::<Product>(conn) {
            Ok(products) => {
                println!("Found {} products", products.len());
                Ok(products)
            }
            Err(e) => {
                println!("Failed to fetch products: {:?}", e);
                Err(e)
            }
        }
    }
}
