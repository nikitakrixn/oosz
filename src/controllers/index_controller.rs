
use askama::Template;
use axum::response::IntoResponse;

use crate::models::page_data::PageData;
use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "./pages/index.html")]
struct IndexTemplate {
    data: PageData
}

pub async fn index() -> impl IntoResponse {

    let data = PageData {
        title: "Главная страница".to_string()
    };

    let template = IndexTemplate { data };

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "./pages/404.html")]
struct NotFoundTemplate {
    data: PageData
}
pub async fn handler_404() -> impl IntoResponse {
    let data = PageData {
        title: "Страница не найдена".to_string()
    };
    let template = NotFoundTemplate { data };

    HtmlTemplate(template)
}