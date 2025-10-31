use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time};
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, HttpResponse, Result as ActixResult};
use actix_files::{Files, NamedFile};
use auth::{auth_handler::{auth_refresh_token, get_me_handler, login_with_zoho_handler, logout_handler}, auth_service::AuthService, auth_service_trait::AuthServiceTrait};
use status_colors::status_color_handler::get_all_status_colors;
use common::auth_middleware::AuthGuard;
use crate::{common::config::Config, interactive_maps::interactive_maps_handler::{get_paginated_svgs, get_svg_by_id, save_svg, save_svg_stream}};
use env_logger::Env;
use interactive_maps::interactive_maps_handler::delete_svg_by_id;
use zoho::{zoho_handler::{get_products_by_ids_handler, get_url_base_zoho}, zoho_service::ZohoService, zoho_trait::ZohoServiceTrait};
use crate::db::init_pool;
use common::swagger_config;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod common;
mod http;
mod interactive_maps;
mod zoho;
mod auth;
mod db;
mod products;
mod zoho_code;
mod status_colors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let pool = init_pool();
    let config = Config::new();

    let product_repository = products::products_repository::ProductRepository::new(
        pool.clone(),
    );
    
    let product_service = Arc::new(Mutex::new(products::products_service::ProductService::new(product_repository)));
    let zoho_service: Arc<dyn ZohoServiceTrait> = Arc::new(ZohoService::new(product_service.clone()));
    let zoho_service_data = web::Data::new(zoho_service.clone());

    // Crear instancias para ZohoCode
    let zoho_code_repository = zoho_code::zoho_code_repository::ZohoCodeRepository::new(pool.clone());
    let zoho_code_service = Arc::new(zoho_code::zoho_code_service::ZohoCodeService::new(zoho_code_repository));
    
    let status_color_repository = status_colors::status_color_repository::StatusColorRepository::new(pool.clone());
    let status_color_service = Arc::new(status_colors::status_color_service::StatusColorService::new(status_color_repository));
    let status_color_service_data = web::Data::new(status_color_service.clone());
    
    let auth_service: Arc<dyn AuthServiceTrait> = Arc::new(AuthService::new(
        config.jwt_secret.clone(),
        config.jwt_refresh_secret.clone(),
        config.token_expiration,
        config.token_refresh_expiration,
        zoho_code_service.clone(),
    ));
    let auth_service_data = web::Data::new(auth_service.clone());
    let auth_guard = AuthGuard::new(auth_service.clone());

    let sync_interval_hours: u64 = std::env::var("ZOHO_SYNC_INTERVAL_MINUTES")
        .unwrap_or_else(|_| "5".to_string()) 
        .parse()
        .unwrap_or(5); 

    let zoho_code_sync_interval: u64 = std::env::var("ZOHO_CODE_SYNC_INTERVAL_MINUTES")
        .unwrap_or_else(|_| "30".to_string()) 
        .parse()
        .unwrap_or(30);

    // Job para sincronización de productos
    let zoho_service_clone = zoho_service.clone();
    tokio::spawn(async move {
        loop {
            println!("Running Zoho product sync...");
            let service = zoho_service_clone.clone();
            let result = service.fetch_all_products_from_zoho().await;
            
            match result {
                Ok(_) => println!("Zoho product sync completed successfully."),
                Err(err) => eprintln!("Zoho product sync failed: {:?}", err),
            }
            
            println!("Next sync in {} minutes...", sync_interval_hours);
            time::sleep(Duration::from_secs(sync_interval_hours * 60)).await;
        }
    });

    // Job para sincronización de códigos de Zoho
    let zoho_code_sync_service = zoho_code::zoho_code_sync_service::ZohoCodeSyncService::new(zoho_code_service.clone());
    
    tokio::spawn(async move {
        time::sleep(Duration::from_secs(5)).await;
        
        println!("Initializing Zoho code service...");
        if let Err(e) = zoho_code_sync_service.initialize_with_zoho().await {
            eprintln!("Failed to initialize Zoho code service: {:?}", e);
        }

        zoho_code_sync_service.start_automatic_sync(zoho_code_sync_interval).await;
    });

    HttpServer::new(move || {
        let unique_origins: HashSet<String> = config.cors_allowed_origins.iter().cloned().collect();

        let mut cors = Cors::default();
        for origin in &unique_origins {
            cors = cors.allowed_origin(origin);
        }
    
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE","OPTIONS"])
            .allow_any_header()
            .allow_any_method()
            .expose_any_header()
            .max_age(3600)
            .supports_credentials();
        
        App::new()
            .app_data(web::JsonConfig::default().limit(10 * 1024 * 1024))
            .app_data(web::Data::new(pool.clone()))
            .app_data(zoho_service_data.clone())
            .app_data(auth_service_data.clone())
            .app_data(status_color_service_data.clone())
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", swagger_config::ApiDoc::openapi())
            )
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/auth")
                    .service(login_with_zoho_handler)
                    .service(auth_refresh_token)
                    .service(logout_handler)
                    .service(get_me_handler)
            )
            .service(
                web::scope("/api/maps")
                    .wrap(auth_guard.clone())
                    .service(save_svg_stream)
                    .service(save_svg)
                    .service(get_svg_by_id)
                    .service(get_paginated_svgs)
                    .service(delete_svg_by_id),
            )
            .service(
                web::scope("/api/zoho")
                    .service(get_products_by_ids_handler)
                    .service(get_url_base_zoho)
                    .service(get_svg_by_id)
            )
            .service(
                web::scope("/api")
                    .route("/status-colors", web::get().to(get_all_status_colors))
            )
            .service(
                Files::new("/assets", "./static/assets")
                    .use_etag(true)
                    .use_last_modified(true)
            )
            .service(
                Files::new("/", "./static")
                    .index_file("index.html")
                    .use_etag(true)
                    .use_last_modified(true)
                    .default_handler(web::route().to(serve_spa))
            )
    })
    .bind(config.serv_addrs)?
    .run()
    .await
}

async fn serve_spa() -> ActixResult<NamedFile> {
    let path = "./static/index.html";
    Ok(NamedFile::open(path)?)
}
