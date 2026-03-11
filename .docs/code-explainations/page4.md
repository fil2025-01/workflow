[Prev](./page3.md) | [Next](./page5.md)

# Explaining `list_recordings` Logic

This document explains the logic inside the `list_recordings` function found in `src/api/handlers.rs`. This handler retrieves recording metadata from the database, optionally filtered by date.

```rust
pub async fn list_recordings(
    State(pool): State<PgPool>,
    Query(filter): Query<DateFilter>
) -> impl IntoResponse {
```

## 1. Function Signature & Extractors
*   **`pub async fn list_recordings`**: Defines a public, asynchronous function named `list_recordings`.
*   **`State(pool)`**: An Axum extractor that retrieves the shared PostgreSQL connection pool (`PgPool`) from the application state. This allows the function to run database queries.
*   **`Query(filter)`**: An extractor that parses query parameters from the URL (e.g., `?date=2023-10-27`) into the `DateFilter` struct.
*   **`-> impl IntoResponse`**: The function returns a type that can be converted into an HTTP response (either JSON data or an error status code).

## 2. Date Handling
```rust
let date_str = filter.date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

let query_date = match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
    Ok(d) => d,
    Err(_) => return StatusCode::BAD_REQUEST.into_response(),
};
```
*   **`filter.date.unwrap_or_else(...)`**: Checks if the user provided a date.
    *   If `Some(date)`, it uses it.
    *   If `None`, it executes the closure `|| ...` to get the current date (`Local::now()`) formatted as "YYYY-MM-DD".
*   **`NaiveDate::parse_from_str(...)`**: Attempts to parse the string `date_str` into a `NaiveDate` object using the format "%Y-%m-%d".
*   **`match ...`**: Handles the parsing result.
    *   **`Ok(d) => d`**: If valid, extracts the date object.
    *   **`Err(_)`**: If invalid, returns a `400 Bad Request` status code immediately, stopping the function.

## 3. Database Query
```rust
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
```
*   **`sqlx::query_as!(RecordingFile, ...)`**: A macro that performs compile-time verification of the SQL query and maps the results to the `RecordingFile` struct.
*   **SQL Logic**:
    *   **`'/files/' || file_path`**: Concatenates the string `/files/` with the stored file path.
        *   **Why?** The database stores the relative disk path (e.g., `2023/10/27/rec.webm`). The frontend needs a public URL to access the file. By prepending `/files/`, we create a valid URL path (e.g., `/files/2023/10/27/rec.webm`) that matches the static file serving route configured in the application (which likely maps `/files/` URLs to the `recordings/` directory on disk).
    *   **`as "path!"`**: Renames the column to `path` and asserts it is not null (`!`), matching the struct field.
    *   **`WHERE date(created_at) = $1`**: Filters rows where the date part of `created_at` matches the `query_date` (passed as `$1`).
    *   **`ORDER BY created_at DESC`**: Sorts results so the newest recordings appear first.
*   **`.fetch_all(&pool)`**: Executes the query against the database pool.
*   **`.await`**: Awaits the asynchronous database operation.

## 4. Response Handling
```rust
match recordings {
    Ok(files) => AxumJson(files).into_response(),
    Err(e) => {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
```
*   **`match recordings`**: Checks if the database query succeeded or failed.
*   **`Ok(files)`**: If successful, wraps the vector of `RecordingFile` objects in `AxumJson` (to serialize them) and converts it into an HTTP response.
*   **`Err(e)`**: If an error occurred (e.g., DB connection lost), it logs the error to the console and returns a `500 Internal Server Error` status code.

[Prev](./page3.md) | [Next](./page5.md)