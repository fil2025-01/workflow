use axum::{
    response::{Html, IntoResponse},
};

// Handler that returns HTML
pub async fn handler() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

// Handler that returns CSS
pub async fn style_handler() -> impl IntoResponse {
    ([("content-type", "text/css")], include_str!("../../static/style.css"))
}

// Handler that returns JS
pub async fn script_handler() -> impl IntoResponse {
    ([("content-type", "text/javascript")], include_str!("../../static/js/script.js"))
}
