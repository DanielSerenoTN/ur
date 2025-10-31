use utoipa::{openapi::security::{Http, HttpAuthScheme, SecurityScheme}, Modify, OpenApi};
use crate::auth::auth_handler::LoginRequest;
use crate::interactive_maps::interactive_maps_handler::SvgRequestSchema;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::auth_handler::login_with_zoho_handler,
        crate::auth::auth_handler::auth_refresh_token,
        crate::auth::auth_handler::logout_handler,
        crate::auth::auth_handler::get_me_handler,
        crate::interactive_maps::interactive_maps_handler::save_svg,
        crate::interactive_maps::interactive_maps_handler::save_svg_stream,
        crate::interactive_maps::interactive_maps_handler::delete_svg_by_id,
        crate::interactive_maps::interactive_maps_handler::get_paginated_svgs,
        crate::interactive_maps::interactive_maps_handler::get_svg_by_id,
        crate::status_colors::status_color_handler::get_all_status_colors
    ),
    modifiers(&SecurityAddon),
    components(
        schemas(
            LoginRequest,
            SvgRequestSchema,
            crate::status_colors::dto::status_color_dto::StatusColorResponse,
            crate::status_colors::dto::status_color_dto::StatusColorsListResponse
        )
    ),
    tags(
        (name = "auth", description = "Authentication related endpoints"),
        (name = "maps", description = "Maps related endpoints"),
        (name = "Status Colors", description = "Status colors management endpoints")
    ),
    servers(
        (url = "/api", description = "Local server")
    ),
    info(description = "This API facilitates the management of urvic processes")
)]

pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}
