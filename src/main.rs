mod controllers;
mod utils;
mod models;

use std::env;
use anyhow::Context;
use askama::Template;
use axum::http::Method;
use axum::{Extension, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use tracing::{error, info};
use tokio::net::TcpListener;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use crate::controllers::*;
use crate::utils::database;

const IS_DEVELOPMENT: bool = cfg!(debug_assertions);
pub const APP_NAME: &str = "OOSZ";

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing();

    // Set variables from environment variables
    let database_url = env::var("DATABASE_URL")
        .context("DATABASE_URL not found in env file")
        .unwrap();
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    let db_pool = match database::create_pool(database_url).await {
        Ok(pool) => {
            info!("✅  Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("🔥  Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    info!("Initializing router...");

    let routes_all = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(index_controller::index))
        .nest("/auth", user_controller::get_router())
        .layer(Extension(db_pool))
        .fallback(index_controller::handler_404);

    let host_address = IS_DEVELOPMENT.then_some("localhost").unwrap_or("0.0.0.0");

    let listener_address = format!("{host_address}:{port}");

    let listener = TcpListener::bind(listener_address).await.context("failed to bind TcpListener").unwrap();

    info!("🚀 Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .context("error while starting server")
        .unwrap();

    Ok(())
}

