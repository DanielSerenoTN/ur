use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::mysql::MysqlConnection;
use crate::interactive_maps::entities::maps_entity::{SvgItem, NewSvgItem};
use crate::db::schema::maps_svg;
use crate::common::types::PaginatedResponse;

pub type PooledConn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct SvgRepository {
    conn: PooledConn,
}

impl SvgRepository {
    pub fn new(conn: PooledConn) -> Self {
        Self { conn }
    }

    pub fn create_svg(&mut self, new_svg: &NewSvgItem) -> Result<String, diesel::result::Error> {
        diesel::insert_into(maps_svg::table)
            .values(new_svg)
            .execute(&mut self.conn)?;

        Ok(new_svg.id.clone())
    }

    pub fn get_svg_by_id(&mut self, svg_id: &str) -> Result<SvgItem, diesel::result::Error> {
        maps_svg::table
            .filter(maps_svg::id.eq(svg_id))
            .first(&mut self.conn)
    }

    pub fn get_paginated_svgs(&mut self, page_num: i64, items_per_page: i64) -> Result<PaginatedResponse<SvgItem>, diesel::result::Error> {
        let total_items = maps_svg::table
            .count()
            .get_result::<i64>(&mut self.conn)?;
            
        let total_pages = (total_items + items_per_page - 1) / items_per_page;
        let offset = (page_num - 1) * items_per_page;
        
        let items = maps_svg::table
            .order(maps_svg::name.asc())
            .offset(offset)
            .limit(items_per_page)
            .load::<SvgItem>(&mut self.conn)?;

        Ok(PaginatedResponse {
            items,
            total_items,
            per_page: items_per_page,
            current_page: page_num,
            total_pages,
        })
    }

    pub fn delete_svg(&mut self, svg_id: &str) -> Result<bool, diesel::result::Error> {
        let deleted_count = diesel::delete(
            maps_svg::table.filter(maps_svg::id.eq(svg_id))
        ).execute(&mut self.conn)?;
        
        Ok(deleted_count > 0)
    }
}