use axum::{
    extract::{Multipart, Query, Json},
    response::{Html, IntoResponse, Json as AxumJson},
    routing::{get, post},
    Router,
    http::StatusCode,
};
use tower_http::services::ServeDir;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use walkdir::WalkDir;
use chrono::{DateTime, Local, Datelike, NaiveDate};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    match std::env::var("GEMINI_API_KEY") {
        Ok(key) => {
            let masked = if key.len() > 4 {
                format!("...{}", &key[key.len()-4..])
            } else {
                "****".to_string()
            };
            println!("GEMINI_API_KEY loaded successfully: {}", masked);
        },
        Err(_) => println!("GEMINI_API_KEY not found in environment or .env file"),
    }

    // Build our application with routes
    let app = create_app();

    // Define the address to listen on
    let mut port = 3000;
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
                    if port > 3010 {
                        panic!("Could not find an open port between 3000 and 3010");
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

fn create_app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/upload", post(upload_handler))
        .route("/recordings", get(list_recordings).delete(delete_recording))
        .nest_service("/files", ServeDir::new("recordings"))
        .route("/style.css", get(style_handler))
        .route("/script.js", get(script_handler))
}

// Handler that returns HTML
async fn handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

// Handler that returns CSS
async fn style_handler() -> impl IntoResponse {
    ([("content-type", "text/css")], include_str!("../style.css"))
}

// Handler that returns JS
async fn script_handler() -> impl IntoResponse {
    ([("content-type", "text/javascript")], include_str!("../script.js"))
}

#[derive(Serialize)]
struct RecordingFile {
    path: String,
    name: String,
    is_transcript: bool,
}

#[derive(Deserialize)]
struct DateFilter {
    date: Option<String>,
}

#[derive(Deserialize)]
struct DeleteRequest {
    path: String,
}

// Handler to delete a recording
async fn delete_recording(Json(payload): Json<DeleteRequest>) -> impl IntoResponse {
    // Basic security check: ensure path starts with /files/ and doesn't contain ..
    if !payload.path.starts_with("/files/") || payload.path.contains("..") {
        return StatusCode::BAD_REQUEST;
    }

    // Convert /files/2026/1/12/file.webm -> recordings/2026/1/12/file.webm
    let relative_path = &payload.path["/files/".len()..];
    let file_path = Path::new("recordings").join(relative_path);

    // Try to delete the file
    if fs::remove_file(&file_path).is_ok() {
        println!("Deleted file: {}", file_path.display());

        // Also try to delete corresponding transcript if it exists
        // Check extension
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy();
            if ext_str == "webm" {
                let txt_path = file_path.with_extension("txt");
                let json_path = file_path.with_extension("json");
                if txt_path.exists() {
                    let _ = fs::remove_file(txt_path);
                }
                if json_path.exists() {
                    let _ = fs::remove_file(json_path);
                }
            } else if ext_str == "txt" || ext_str == "json" {
                let webm_path = file_path.with_extension("webm");
                if webm_path.exists() {
                    let _ = fs::remove_file(webm_path);
                }
            }
        }

        StatusCode::OK
    } else {
        eprintln!("Failed to delete file: {}", file_path.display());
        StatusCode::NOT_FOUND
    }
}

// Handler to list recordings (optionally filtered by date)
async fn list_recordings(Query(filter): Query<DateFilter>) -> AxumJson<Vec<RecordingFile>> {
    let mut files = Vec::new();
    let root_dir = "recordings";

    let target_dir = if let Some(date_str) = filter.date {
        // user provided date: YYYY-MM-DD
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            format!("{}/{}/{}/{}", root_dir, date.year(), date.month(), date.day())
        } else {
            // Invalid date format, just return empty or default to root (let's return empty for safety)
            return Json(vec![]);
        }
    } else {
        // Default to today
        let now: DateTime<Local> = Local::now();
        format!("{}/{}/{}/{}", root_dir, now.year(), now.month(), now.day())
    };

    // Check if directory exists
    if Path::new(&target_dir).exists() {
        for entry in WalkDir::new(&target_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_string_lossy();
                    if ext_str == "webm" || ext_str == "txt" || ext_str == "json" {
                        // Create relative path for serving.
                        // Note: ServeDir is mounted at /files serving "recordings/"
                        // So if file is recordings/2026/1/12/file.webm, we want /files/2026/1/12/file.webm
                        let relative_path = path.strip_prefix(root_dir).unwrap_or(path);
                        let relative_path_str = relative_path.to_string_lossy().replace("\\", "/");

                        files.push(RecordingFile {
                            path: format!("/files/{}", relative_path_str),
                            name: entry.file_name().to_string_lossy().to_string(),
                            is_transcript: ext_str == "txt" || ext_str == "json",
                        });
                    }
                }
            }
        }
    }

    // Sort files by name (timestamp)
    files.sort_by(|a, b| b.name.cmp(&a.name));

    Json(files)
}

// Handler for uploading audio
async fn upload_handler(mut multipart: Multipart) -> impl IntoResponse {
    let now: DateTime<Local> = Local::now();
    let upload_dir = format!("recordings/{}/{}/{}", now.year(), now.month(), now.day());

    if let Err(e) = fs::create_dir_all(&upload_dir) {
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
                let filepath = Path::new(&upload_dir).join(&filename);

                if let Ok(mut file) = File::create(&filepath) {
                    if file.write_all(&data).is_ok() {
                        println!("Saved file: {}", filepath.display());

                        if let Ok(metadata) = fs::metadata(&filepath) {
                            let size = metadata.len();
                            if size > 0 {
                                println!("Success! File is not empty. {}", size);

                                // Spawn transcription task
                                let filepath_clone = filepath.clone();
                                tokio::spawn(async move {
                                    if let Err(e) = transcribe_audio(filepath_clone).await {
                                        eprintln!("Transcription failed: {}", e);
                                    }
                                });

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

#[derive(Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
    InlineData { inline_data: InlineData },
}

#[derive(Serialize)]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Deserialize)]
struct GenerateContentResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Vec<CandidatePart>,
}

#[derive(Deserialize)]
struct CandidatePart {
    text: Option<String>,
}

async fn transcribe_audio(filepath: PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("LOCAL_GEMINI_API_KEY").map_err(|_| "LOCAL_GEMINI_API_KEY not set")?;

    // Read the file
    let mut file = File::open(&filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Encode to base64
    let base64_audio = general_purpose::STANDARD.encode(&buffer);

    // Construct request
    let request_body = GenerateContentRequest {
        contents: vec![Content {
            parts: vec![
                Part::Text { text: "Transcribe the following audio. Then, provide a short, clean, and descriptive title (max 6 words) summarizing the content. Return ONLY a raw JSON object (no markdown formatting) with the following structure:\n{\n  \"title\": \"Your Title\",\n  \"transcript\": \"Full Transcription\"\n}".to_string() },
                Part::InlineData {
                    inline_data: InlineData {
                        mime_type: "audio/webm".to_string(),
                        data: base64_audio,
                    },
                },
            ],
        }],
    };

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let res = client.post(&url)
        .json(&request_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await?;
        return Err(format!("API Error: {}", text).into());
    }

    let response_data: GenerateContentResponse = res.json().await?;

    if let Some(candidates) = response_data.candidates {
        if let Some(first_candidate) = candidates.first() {
            if let Some(first_part) = first_candidate.content.parts.first() {
                if let Some(text) = &first_part.text {
                    // Clean markdown code blocks if present
                    let clean_text = text.trim();
                    let clean_text = if clean_text.starts_with("```json") {
                        &clean_text[7..]
                    } else if clean_text.starts_with("```") {
                        &clean_text[3..]
                    } else {
                        clean_text
                    };

                    let clean_text = clean_text.trim_end_matches("```").trim();

                    // Save transcript
                    let mut txt_path = filepath.clone();
                    txt_path.set_extension("json");
                    let mut txt_file = File::create(&txt_path)?;
                    txt_file.write_all(clean_text.as_bytes())?;
                    println!("Transcription saved to: {}", txt_path.display());
                    return Ok(());
                }
            }
        }
    }

    Err("No transcription text found in response".into())
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
    async fn test_upload_success() {
        let app = create_app();

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
                        file_found = true;
                        // Clean up - we can't easily delete the folders here without more logic,
                        // but we can delete the file.
                        std::fs::remove_file(path).expect("Failed to delete test file");
                        break;
                    }
                }
            }
        }

        assert!(file_found, "Uploaded file not found in recordings directory");
    }

    #[tokio::test]
    async fn test_style_css() {
        let app = create_app();

        let response = app
            .oneshot(Request::builder().uri("/style.css").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "text/css");
    }

    #[tokio::test]
    async fn test_script_js() {
        let app = create_app();

        let response = app
            .oneshot(Request::builder().uri("/script.js").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "text/javascript");
    }

    #[tokio::test]
    async fn test_list_recordings() {
        let app = create_app();

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
