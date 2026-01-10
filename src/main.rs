use axum::{
    extract::Multipart,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
    http::StatusCode,
};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;

#[tokio::main]
async fn main() {
    // Build our application with routes
    let app = create_app();

    // Define the address to listen on
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    // Run the server
    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/upload", post(upload_handler))
}

// Handler that returns HTML
async fn handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

// Handler for uploading audio
async fn upload_handler(mut multipart: Multipart) -> impl IntoResponse {
    let upload_dir = "recordings";
    if let Err(e) = fs::create_dir_all(upload_dir) {
        eprintln!("Failed to create upload directory: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("unknown").to_string();
        if name == "file" {
            if let Ok(data) = field.bytes().await {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let filename = format!("recording_{}.webm", timestamp);
                let filepath = Path::new(upload_dir).join(&filename);

                if let Ok(mut file) = File::create(&filepath) {
                    if file.write_all(&data).is_ok() {
                        println!("Saved file: {}", filepath.display());

                        if let Ok(metadata) = fs::metadata(&filepath) {
                            let size = metadata.len();
                            if size > 0 {
                                println!("Success! File is not empty. {}", size);
                            } else {
                                println!("Warning! File is empty. {}", size);
                            }
                        }
                    } else {
                        eprintln!("Failed to write to file: {}", filepath.display());
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }
                } else {
                    eprintln!("Failed to create file: {}", filepath.display());
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            }
        }
    }
    StatusCode::OK
}
