use axum::{
    extract::{Query, Json, State, Multipart},
    response::{Html, IntoResponse, Json as AxumJson},
    http::StatusCode,
};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local, Datelike, NaiveDate};
use sqlx::PgPool;
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
    ([("content-type", "text/javascript")], include_str!("../../static/js/script.js"))
}

// Handler to list recordings (optionally filtered by date)
pub async fn list_recordings(
    State(pool): State<PgPool>,
    Query(filter): Query<DateFilter>
) -> impl IntoResponse {
    let date_str = filter.date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    // Parse date_str to NaiveDate for SQL query
    let query_date = match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let recordings = sqlx::query_as!(
        RecordingFile,
        r#"
        SELECT
            '/files/' || file_path as "path!",
            filename as "name!",
            transcription_status as "status!",
            transcription_text as "transcription"
        FROM recordings
        WHERE date(created_at) = $1
        ORDER BY created_at DESC
        "#,
        query_date
    )
    .fetch_all(&pool)
    .await;

    match recordings {
        Ok(files) => AxumJson(files).into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// Handler for uploading audio
pub async fn upload_handler(
    State(pool): State<PgPool>,
    Query(filter): Query<DateFilter>,
    mut multipart: Multipart
) -> impl IntoResponse {
    let now: DateTime<Local> = Local::now();

    // Determine the upload directory based on the optional date query param
    let (year, month, day) = if let Some(date_str) = filter.date.clone() {
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            (date.year(), date.month(), date.day())
        } else {
            (now.year(), now.month(), now.day())
        }
    } else {
        (now.year(), now.month(), now.day())
    };

    let relative_dir = format!("{}/{}/{}", year, month, day);
    let upload_dir = format!("recordings/{}", relative_dir);

    if let Err(e) = fs::create_dir_all(&upload_dir) {
        eprintln!("Failed to create upload directory: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("unknown").to_string();

        if name == "file" {
            let file_name = field.file_name().unwrap_or("").to_string();
            if let Ok(data) = field.bytes().await {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // If uploaded filename starts with "test_", preserve that prefix for easier cleanup
                let prefix = if file_name.starts_with("test_") { "test_" } else { "" };
                let filename = format!("{}recording_{}.webm", prefix, timestamp);

                let filepath_in_db = format!("{}/{}", relative_dir, filename);
                let full_filepath = Path::new(&upload_dir).join(&filename);

                if let Ok(mut file) = File::create(&full_filepath) {
                    if file.write_all(&data).is_ok() {
                        println!("Saved file: {}", full_filepath.display());

                        // Insert into database
                        let res = sqlx::query!(
                            r#"
                            INSERT INTO recordings (filename, file_path, created_at)
                            VALUES ($1, $2, $3)
                            RETURNING id
                            "#,
                            filename,
                            filepath_in_db,
                            if let Some(ref ds) = filter.date {
                                NaiveDate::parse_from_str(ds, "%Y-%m-%d").ok().and_then(|d| d.and_hms_opt(12, 0, 0)).map(|dt| DateTime::<Local>::from_naive_utc_and_offset(dt.and_utc().naive_utc(), *Local::now().offset()))
                            } else {
                                Some(Local::now())
                            }
                        )
                        .fetch_one(&pool)
                        .await;

                        match res {
                            Ok(record) => {
                                // Spawn transcription task
                                let pool_clone = pool.clone();
                                let full_filepath_clone = full_filepath.clone();
                                let record_id = record.id;

                                tokio::spawn(async move {
                                    if let Err(e) = transcribe_and_update(pool_clone, record_id, full_filepath_clone).await {
                                        eprintln!("Transcription failed: {}", e);
                                    }
                                });
                            },
                            Err(e) => {
                                eprintln!("Failed to insert into DB: {}", e);
                                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                            }
                        }
                    } else {
                        eprintln!("Failed to write to file: {}", full_filepath.display());
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                } else {
                    eprintln!("Failed to create file: {}", full_filepath.display());
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            }
        }
    }
    StatusCode::OK.into_response()
}

// Helper function to transcribe and update the database
async fn transcribe_and_update(pool: PgPool, id: sqlx::types::uuid::Uuid, path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let transcription_text = transcribe_audio(path.clone()).await?;
    let json_value: serde_json::Value = serde_json::from_str(&transcription_text)?;

    sqlx::query!(
        r#"
        UPDATE recordings
        SET transcription_text = $1, transcription_status = 'COMPLETED'
        WHERE id = $2
        "#,
        json_value,
        id
    )
    .execute(&pool)
    .await?;

    Ok(())
}

// Handler to delete a recording
pub async fn delete_recording(
    State(pool): State<PgPool>,
    Json(payload): Json<DeleteRequest>
) -> impl IntoResponse {
    if !payload.path.starts_with("/files/") || payload.path.contains("..") {
        return StatusCode::BAD_REQUEST;
    }

    let relative_path = &payload.path["/files/".len()..];

    // Delete from DB first
    let res = sqlx::query!(
        "DELETE FROM recordings WHERE file_path = $1 RETURNING id",
        relative_path
    )
    .fetch_optional(&pool)
    .await;

    match res {
        Ok(Some(_)) => {
            // Delete from disk
            let file_path = Path::new("recordings").join(relative_path);
            let _ = fs::remove_file(&file_path);
            StatusCode::OK
        },
        Ok(None) => StatusCode::NOT_FOUND,
        Err(e) => {
            eprintln!("DB error on delete: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}