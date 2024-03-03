
use askama::Template;
use axum::response::IntoResponse;

use crate::models::page_data::PageData;
use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "./pages/index.html")]
struct IndexTemplate {
    pub data: PageData,
}

pub async fn index() -> impl IntoResponse {

    let data = PageData {
        title: "Главная страница".to_string(),
        message: "Привет, мир!".to_string(),
    };

    let template = IndexTemplate { data };

    HtmlTemplate(template)
}