use axum::{
    extract::{Multipart, Query, Json},
    response::{Html, IntoResponse, Json as AxumJson},
    http::StatusCode,
};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;
use chrono::{DateTime, Local, Datelike, NaiveDate};
use crate::models::dtos::{DateFilter, RecordingFile, DeleteRequest};
use crate::service::transcription::transcribe_audio;

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
    ([("content-type", "text/javascript")], include_str!("../../static/script.js"))
}

// Handler to list recordings (optionally filtered by date)
pub async fn list_recordings(Query(filter): Query<DateFilter>) -> AxumJson<Vec<RecordingFile>> {
    let mut files = Vec::new();
    let root_dir = "recordings";

    let target_dir = if let Some(date_str) = filter.date {
        // user provided date: YYYY-MM-DD
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            format!("{}/{}/{}/{}", root_dir, date.year(), date.month(), date.day())
        } else {
            return AxumJson(vec![]);
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

    AxumJson(files)
}

// Handler for uploading audio
pub async fn upload_handler(Query(filter): Query<DateFilter>, mut multipart: Multipart) -> impl IntoResponse {
    let now: DateTime<Local> = Local::now();
    
    // Determine the upload directory based on the optional date query param
    let upload_dir = if let Some(date_str) = filter.date {
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            format!("recordings/{}/{}/{}", date.year(), date.month(), date.day())
        } else {
            // Fallback to today if parsing fails
            format!("recordings/{}/{}/{}", now.year(), now.month(), now.day())
        }
    } else {
        // Default to today
        format!("recordings/{}/{}/{}", now.year(), now.month(), now.day())
    };

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

// Handler to delete a recording
pub async fn delete_recording(Json(payload): Json<DeleteRequest>) -> impl IntoResponse {
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
