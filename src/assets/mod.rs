use axum::{Router, extract::Path, response::IntoResponse, routing::get};

use crate::http_types::{ox_css, ox_js};

pub mod basel;
pub mod daisy;
pub mod tailwind;
pub mod themes;

pub fn router() -> Router {
    Router::new().route("/_assets/{file}", get(assets))
}

async fn assets(Path(file): Path<String>) -> impl IntoResponse {
    let extension = std::path::Path::new(&file)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();

    let content = match file.as_str() {
        "daisy.css" => daisy::CSS,
        "tw.js" => tailwind::JS,
        "themes.css" => basel::CSS,
        _ => "",
    };

    match extension {
        "css" => ox_css(content.to_owned()).into_response(),
        "js" => ox_js(content.to_owned()).into_response(),
        _ => ox_css(content.to_owned()).into_response(),
    }
}
