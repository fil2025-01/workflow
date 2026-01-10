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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for collect
    use tower::ServiceExt; // for oneshot

    #[tokio::test]
    async fn test_root() {
        let app = create_app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(&body).unwrap();

        assert!(body_str.contains("Record Audio"));
    }

    #[tokio::test]
    async fn test_upload_no_file() {
        let app = create_app();

        // Testing upload without a valid multipart body should probably result in a 400 or just processed as empty
        // Since we are checking for "file" field, it should just return OK but save nothing if body is empty or malformed
        // However, Multipart extractor expects a valid Content-Type header.

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/upload")
                    .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
                    .body(Body::from(
                        "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.webm\"\r\n\r\nTest Data\r\n--X-BOUNDARY--\r\n"
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Cleanup created file if any (filename is based on timestamp so hard to predict exactly here without mocking time or file system,
        // but we can check if it returns OK).
        // For a true unit test, we should abstract the filesystem, but for this simple integration test, checking the status code is a good start.
    }
}
