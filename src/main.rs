use axum::{
    routing::{get, post},
    Router,
    extract::FromRef,
};
use tower_http::services::ServeDir;
use sqlx::postgres::PgPool;

mod api;
mod service;
mod models;

use api::handlers::{
    handler, upload_handler, list_recordings, style_handler, script_handler, delete_recording
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    match std::env::var("LOCAL_GEMINI_API_KEY") {
        Ok(key) => {
            let masked = if key.len() > 4 {
                format!("...{}", &key[key.len()-4..])
            } else {
                "****".to_string()
            };
            println!("LOCAL_GEMINI_API_KEY loaded successfully: {}", masked);
        },
        Err(_) => println!("LOCAL_GEMINI_API_KEY not found in environment or .env file"),
    }

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to Postgres");
    let state = AppState { db: pool };

    // Build our application with routes
    let app = create_app(state);

    // Define the address to listen on
    let mut port = 4000;
    let listener = loop {
        let addr = format!("127.0.0.1:{}", port);
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                println!("Listening on http://{}", addr);
                break listener;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AddrInUse {
                    println!("Port {} is in use, trying next port...", port);
                    port += 1;
                    if port > 4010 {
                        panic!("Could not find an open port between 4000 and 4010");
                    }
                } else {
                    panic!("Failed to bind to address: {}", e);
                }
            }
        }
    };

    // Run the server
    axum::serve(listener, app).await.unwrap();
}

fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/upload", post(upload_handler))
        .route("/recordings", get(list_recordings).delete(delete_recording))
        .nest_service("/files", ServeDir::new("recordings"))
        .route("/style.css", get(style_handler))
        .route("/script.js", get(script_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for collect
    use tower::ServiceExt; // for oneshot
    use walkdir::WalkDir;

    async fn get_test_state() -> AppState {
        dotenv::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&db_url).await.expect("Failed to connect to Postgres");
        AppState { db: pool }
    }

    #[tokio::test]
    async fn test_root() {
        let state = get_test_state().await;
        let app = create_app(state);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(&body).unwrap();

        assert!(body_str.contains("Record"));
    }

    #[tokio::test]
    async fn test_upload_success() {
        let state = get_test_state().await;
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/upload")
                    .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
                    .body(Body::from(
                        "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test_file.webm\"\r\n\r\nTest Data\r\n--X-BOUNDARY--\r\n"
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Check if a file was created in the recordings directory (recursively)
        let root_dir = "recordings";
        let mut file_found = false;

        for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("webm") {
                    // Check if content matches "Test Data" (size should be 9 bytes)
                    let metadata = std::fs::metadata(&path).unwrap();
                    if metadata.len() == 9 {
                        // Verify filename starts with test_
                        let filename = path.file_name().unwrap().to_string_lossy();
                        if filename.starts_with("test_") {
                            file_found = true;
                            // Clean up - we can't easily delete the folders here without more logic,
                            // but we can delete the file.
                            std::fs::remove_file(path).expect("Failed to delete test file");
                            
                            // Ideally we should clean up DB too, but we don't have the ID here easily.
                            // The user asked to "easily inspect db and delete necessary", so leaving it 
                            // with a distinct name helps them do that manually or via script.
                            break;
                        }
                    }
                }
            }
        }

        assert!(file_found, "Uploaded file not found in recordings directory");
    }

    #[tokio::test]
    async fn test_style_css() {
        let state = get_test_state().await;
        let app = create_app(state);

        let response = app
            .oneshot(Request::builder().uri("/style.css").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "text/css");
    }

    #[tokio::test]
    async fn test_script_js() {
        let state = get_test_state().await;
        let app = create_app(state);

        let response = app
            .oneshot(Request::builder().uri("/script.js").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "text/javascript");
    }

    #[tokio::test]
    async fn test_list_recordings() {
        let state = get_test_state().await;
        let app = create_app(state);

        let response = app
            .oneshot(Request::builder().uri("/recordings").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

        assert!(body_json.is_array(), "Response should be a JSON array");
    }
}
