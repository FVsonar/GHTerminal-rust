use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../frontend/dist"]
struct FrontendAssets;

pub async fn index() -> impl IntoResponse {
    serve_embedded("index.html").await
}

pub async fn serve_static(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    serve_embedded(path).await
}

async fn serve_embedded(path: &str) -> impl IntoResponse {
    match FrontendAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            let resp = axum::response::Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap();
            resp.into_response()
        }
        None => {
            // SPA fallback: 返回 index.html
            match FrontendAssets::get("index.html") {
                Some(content) => axum::response::Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(Body::from(content.data))
                    .unwrap()
                    .into_response(),
                None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            }
        }
    }
}
