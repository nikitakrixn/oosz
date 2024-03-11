use std::collections::HashMap;
use askama::Template;
use axum::{extract::State, response::{IntoResponse, Redirect}, routing::{get, post}, Router, Extension, Json, ServiceExt, Form};
use serde::Deserialize;
use sqlx::PgPool;
use crate::models::page_data::PageData;

use crate::models::user::User;
use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "./pages/auth/register.html")]
struct RegisterTemplate {
    data: PageData,
}
#[derive(Debug, Deserialize)]
pub struct RegisterFormData {
    username: String,
    password: String,
}

async fn process_registration(
    Extension(pool): Extension<PgPool>,
    Form(formData): Form<RegisterFormData>,
) -> impl IntoResponse {
    let result = User::new(&pool, formData.username, formData.password).await;

    match result {
        Ok(_) => tracing::info!("Created new user"),
        Err(error) => {
            tracing::error!("Error inserting account into database: {}", error);
        }
    }
}

async fn register() -> impl IntoResponse {

    let data = PageData {
        title: "Страница регистрации".to_string()
    };

    let template = RegisterTemplate { data };

    HtmlTemplate(template)
}

pub fn get_router() -> Router {
    Router::new()
        .route("/register", get(register).post(process_registration))

}

