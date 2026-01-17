# Explanation of `upload_handler` in `src/api/handlers.rs`

This document provides a detailed, line-by-line explanation of the `upload_handler` function, which is responsible for handling audio file uploads, saving them to disk, and initiating the transcription process.

## Function Signature

```rust
pub async fn upload_handler(
    State(pool): State<PgPool>,
    Query(filter): Query<DateFilter>,
    mut multipart: Multipart
) -> impl IntoResponse {
```

-   `pub async fn upload_handler`: Defines a public asynchronous function named `upload_handler`.
-   `State(pool): State<PgPool>`: Extracts the PostgreSQL connection pool from the application state. This allows the handler to interact with the database.
-   `Query(filter): Query<DateFilter>`: Extracts query parameters into a `DateFilter` struct. This is used to optionally determine the date associated with the recording (e.g., for organization).
-   `mut multipart: Multipart`: Extracts the request body as a multipart stream, allowing the function to process file uploads.
-   `-> impl IntoResponse`: Returns a type that implements `IntoResponse`, which Axum converts into an HTTP response.

## Function Body

```rust
    let now: DateTime<Local> = Local::now();
```

-   Captures the current local date and time. This is used as a fallback if no specific date is provided in the query parameters.

### Determining the Directory

```rust
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
```

-   This block determines the year, month, and day to be used for the file path.
-   It checks if `filter.date` (from the query string) is present.
-   If present, it attempts to parse it as a date ("YYYY-MM-DD"). If successful, it uses that date.
-   If the date is missing or invalid, it defaults to the current date (`now`).

```rust
    let relative_dir = format!("{}/{}/{}", year, month, day);
    let upload_dir = format!("recordings/{}", relative_dir);

    if let Err(e) = fs::create_dir_all(&upload_dir) {
        eprintln!("Failed to create upload directory: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
```

-   `relative_dir`: Constructs a relative path string (e.g., "2023/10/27").
-   `upload_dir`: Prepends "recordings/" to the relative path to get the full storage directory.
-   `fs::create_dir_all(&upload_dir)`: Recursively creates the directory structure if it doesn't exist.
-   If directory creation fails, it logs the error and returns a `500 Internal Server Error`.

### Processing Multipart Fields

```rust
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("unknown").to_string();

        if name == "file" {
```

-   Iterates through the fields in the multipart form data asynchronously.
-   Checks if the field name is "file", which is expected to contain the audio data.

```rust
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
```

-   `field.bytes().await`: Reads the raw bytes of the uploaded file.
-   `timestamp`: Generates a UNIX timestamp to ensure unique filenames.
-   `prefix`: Preserves a "test_" prefix if present in the original filename (useful for testing/cleanup).
-   `filename`: Constructs the new filename using the prefix and timestamp (e.g., "recording_1698412345.webm").
-   `filepath_in_db`: The path string to be stored in the database.
-   `full_filepath`: The actual filesystem path where the file will be saved.

### Saving the File

```rust
                if let Ok(mut file) = File::create(&full_filepath) {
                    if file.write_all(&data).is_ok() {
                        println!("Saved file: {}", full_filepath.display());
```

-   `File::create`: Creates the file at the specified path.
-   `file.write_all`: writes the byte data to the file.
-   If successful, prints a confirmation message.

### Database Insertion

```rust
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
```

-   Executes an SQL `INSERT` query to add the recording metadata to the `recordings` table.
-   `created_at`: Uses the provided date filter (set to noon) if available, otherwise uses the current time.
-   `RETURNING id`: Returns the generated ID of the new record.

### Spawning Transcription

```rust
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
```

-   Matches on the result of the database insertion.
-   **On Success (`Ok(record)`)**:
    -   Clones the connection pool and file path for the background task.
    -   `tokio::spawn`: Starts a background asynchronous task to transcribe the audio. This prevents the HTTP request from blocking while transcription (which can be slow) occurs.
    -   Calls `transcribe_and_update` inside the spawned task.
-   **On Failure (`Err(e)`)**:
    -   Logs the database error.
    -   Returns a 500 Internal Server Error.

### Error Handling for File Operations

```rust
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
```

-   The `else` blocks handle failures for writing to the file or creating the file, returning 500 errors in those cases.
-   After processing the loop (or if no file was found in the current field), the function completes.
-   `StatusCode::OK.into_response()`: Returns a 200 OK status to the client, indicating the upload request was processed (though transcription happens in the background).
