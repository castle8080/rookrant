use std::{net::SocketAddr, str::FromStr};

use axum::Router;
use axum::middleware::from_fn_with_state;
use axum_server::tls_rustls::RustlsConfig;

use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use tower::ServiceBuilder;

use clap::{Parser, arg, ArgAction};

mod controllers;
mod views;
mod services;
mod errors;
mod middleware;
mod url_constants;
mod crypto;

use errors::AppResult;
use crypto::certs::generate_self_signed_cert_files_if_not_exists;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, default_value_t=true, action=ArgAction::Set)]
    tls: bool,

    #[arg(long, default_value_t = String::from("127.0.0.1:3000"))]
    listen: String,

    #[arg(long, default_value_t = String::from("local"))]
    environment: String,
}

async fn create_app_handler(environment: &String) -> AppResult<Router> {
    let serve_dir = ServeDir::new("www")
        .append_index_html_on_directories(true);

    let state = services::state::configure(environment).await?;

    Ok(Router::new()
        .merge(controllers::rant::get_routes())
        .merge(controllers::welcome::get_routes())
        .merge(controllers::login::get_routes())
        .merge(controllers::utils::get_routes())
        .with_state(state.clone())
        .fallback_service(serve_dir)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(from_fn_with_state(
                    state.clone(),
                    middleware::authentication::authentication_layer))
        ))
}

#[tokio::main]
async fn main() -> AppResult<()> {
    env_logger::init();

    let cli = Cli::parse();
    log::info!("Application arguments: {:?}", cli);

    let app = create_app_handler(&cli.environment).await?;

    let addr = SocketAddr::from_str(&cli.listen).unwrap();
    log::info!("Listenging to: {}", addr);

    let cert_file = "etc/certs/certificate.pem";
    let key_file = "etc/certs/key.pem";

    if cli.tls {
        let _ = generate_self_signed_cert_files_if_not_exists(cert_file, key_file)?;
        let config = RustlsConfig::from_pem_file(cert_file, key_file).await?;

        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await?;
    }
    else {
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?
    }

    Ok(())
}
