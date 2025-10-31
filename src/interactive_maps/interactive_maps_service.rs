use uuid::Uuid;
use futures::TryStreamExt;
use actix_web::web;
use crate::common::types::PaginatedResponse;
use super::interactive_maps_repository::SvgRepository;
use crate::interactive_maps::entities::maps_entity::NewSvgItem;
use crate::interactive_maps::dto::svg_dto::{SvgRequest, SvgInfo};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type PooledConn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct SvgService {
    repository: SvgRepository,
}

impl SvgService {
    pub fn new(conn: PooledConn) -> Self {
        Self {
            repository: SvgRepository::new(conn),
        }
    }

    pub fn save_svg(&mut self, data: SvgRequest) -> Result<String, String> {
        let svg_name = data.name.clone();
        let prefix = svg_name.split('-')
            .next()
            .unwrap_or("")
            .to_string();

        let new_svg = NewSvgItem {
            id: Uuid::new_v4().to_string(),
            name: svg_name,
            prefix,
            content: data.content,
        };

        self.repository
            .create_svg(&new_svg)
            .map_err(|e| format!("Error al guardar SVG: {}", e))
    }

    pub fn get_svg_by_id(&mut self, svg_id: String) -> Result<(SvgInfo, String), String> {
        let svg_item = self.repository
            .get_svg_by_id(&svg_id)
            .map_err(|e| format!("Error al obtener SVG: {}", e))?;

        let svg_info = SvgInfo {
            prefix: svg_item.prefix,
            name: svg_item.name,
            id: svg_item.id,
        };

        Ok((svg_info, svg_item.content))
    }

    pub fn get_paginated_svgs(
        &mut self,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResponse<SvgInfo>, String> {
        let response = self.repository
            .get_paginated_svgs(page, per_page)
            .map_err(|e| format!("Error al obtener SVGs paginados: {}", e))?;

        let items: Vec<SvgInfo> = response.items
            .into_iter()
            .map(|item| SvgInfo {
                prefix: item.prefix,
                name: item.name,
                id: item.id,
            })
            .collect();

        Ok(PaginatedResponse {
            items,
            total_items: response.total_items,
            per_page: response.per_page,
            current_page: response.current_page,
            total_pages: response.total_pages,
        })
    }

    pub async fn save_svg_streaming(
        &mut self,
        mut payload: web::Payload,
        svg_name: String,
    ) -> Result<String, String> {
        let mut content = Vec::new();
        
        while let Some(chunk) = payload
            .try_next()
            .await
            .map_err(|e| format!("Error al leer payload: {}", e))? 
        {
            content.extend_from_slice(&chunk);
        }

        let prefix = svg_name.split('-')
            .next()
            .unwrap_or("")
            .to_string();

        let new_svg = NewSvgItem {
            id: Uuid::new_v4().to_string(),
            name: svg_name,
            prefix,
            content: String::from_utf8(content)
                .map_err(|e| format!("Error al convertir contenido a UTF-8: {}", e))?,
        };

        self.repository
            .create_svg(&new_svg)
            .map_err(|e| format!("Error al guardar SVG: {}", e))
    }

    pub fn delete_svg_by_id(&mut self, svg_id: String) -> Result<(), String> {
        match self.repository.delete_svg(&svg_id) {
            Ok(true) => Ok(()),
            Ok(false) => Err("SVG no encontrado".to_string()),
            Err(e) => Err(format!("Error al eliminar SVG: {}", e)),
        }
    }
}