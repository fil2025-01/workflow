[Prev](./page5.md) | [Next](./page7.md)

# Explaining `update_recording` Logic

This document explains the `update_recording` handler found in `src/api/handlers.rs`. This function is responsible for updating specific fields of a recording, such as the assigned group.

## Imports Explained

```rust
    extract::{FromRef, State},
    response::IntoResponse,
```

*   **`extract::{FromRef, State}`**:
    *   **What it does**: Imports both the `FromRef` trait and the `State` extractor from the `axum::extract` module using `{}` syntax to group them.
    *   **Usage**: `FromRef` is needed for the application state logic. `State` is specifically added here so it can be used in the function arguments (see `State(pool)` below) to access the database.
*   **`response::IntoResponse`**:
    *   **What it does**: Imports the `IntoResponse` trait from the `axum::response` module.
    *   **Usage**: This is added to support the return type `-> impl IntoResponse`. It tells Rust that whatever this function returns (like `StatusCode::OK` or `Json(...)`) can be converted into a standard HTTP response.

---

## Function Signature

```rust
pub async fn update_recording(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRecordingRequest>
) -> impl IntoResponse {
```

### 1. Function Declaration
*   **`pub async fn`**: Defines a public, asynchronous function. This allows the server to handle other requests while waiting for database operations within this function.
*   **`update_recording`**: The name of the handler function.

### 2. Arguments (Axum Extractors)
Axum uses "extractors" to pull data out of the incoming HTTP request and pass it as arguments to the function.

*   **`State(pool): State<PgPool>`**:
    *   **What it does**: Accesses the shared application state.
    *   **Usage**: It extracts the PostgreSQL connection pool (`PgPool`) so the function can run SQL queries (specifically the `UPDATE` statement).
*   **`Path(id): Path<Uuid>`**:
    *   **What it does**: Extracts a dynamic parameter from the URL path.
    *   **Usage**: If the route is defined as `/recordings/:id`, this grabs the `:id` part. It automatically validates that the string is a valid UUID and converts it to the `Uuid` type.
*   **`Json(payload): Json<UpdateRecordingRequest>`**:
    *   **What it does**: Reads the HTTP request body.
    *   **Usage**: It expects the body to be in JSON format. It attempts to deserialize (parse) that JSON into the `UpdateRecordingRequest` struct. If successful, the data is available in the `payload` variable.

### 3. Return Type
*   **`-> impl IntoResponse`**:
    *   **What it does**: Indicates that the function returns a type that Axum can convert into an HTTP response.
    *   **Usage**: This allows returning simple status codes (like `StatusCode::OK`) or errors without specifying a complex concrete type.

# Explaining `transcribe_and_update` Logic

This helper function handles the background processing of the audio file. It is called asynchronously via `tokio::spawn` inside `upload_handler`.

## Function Signature

```rust
async fn transcribe_and_update(
    pool: PgPool,
    id: Uuid,
    path: std::path::PathBuf
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
```

### 1. Arguments
*   **`pool: PgPool`**: A clone of the database connection pool. It is needed because this function runs in a separate task and needs its own handle to the database.
*   **`id: Uuid`**: The primary key of the recording row in the database. Used to identify which record to update.
*   **`path: PathBuf`**: The filesystem path to the saved audio file.

### 2. Transcription Process
```rust
    let transcription_text = transcribe_audio(path.clone()).await?;
    let json_value: serde_json::Value = serde_json::from_str(&transcription_text)?;
```
*   **`transcribe_audio`**: Calls the external service (Gemini API) to generate the transcript.
*   **`serde_json::from_str`**: The prompt sent to Gemini requests a raw JSON response. This line parses that string into a JSON object to ensure it is valid before saving it to the database.

### 3. Database Update
```rust
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
```
*   **`UPDATE recordings`**: Modifies the existing row created during the upload.
*   **`transcription_text = $1`**: Stores the structured JSON (title + transcript) in the `JSONB` column.
*   **`transcription_status = 'COMPLETED'`**: Updates the status flag so the frontend knows the transcription is ready.

[Prev](./page5.md) | [Next](./page7.md)