use std::net::SocketAddr;

use std::io::Error;
use tokio::net::TcpListener;
use utoipa::{
    OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use axum_dice::routers::status::{self, STATUS_TAG, STATUS_DESCRIPTION};
use axum_dice::routers::dice::{self};
use axum_dice::config::Config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::default_config();
    println!("Starting {}...", config.title);
    
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = STATUS_TAG, description = STATUS_DESCRIPTION),
            (name = "Dice", description = "Endpoints for rolling dice")
        )
    )]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(status::router())
        .merge(dice::router())
        .split_for_parts();

    let router = router
        .merge(
            SwaggerUi::new("/docs").url("/api-docs/openapi.json", api.clone()),
        )
        .merge(Redoc::with_url("/redoc", api.clone()));

    let address = SocketAddr::from((config.ip, config.port));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await
}
