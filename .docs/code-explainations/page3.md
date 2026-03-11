[Prev](./page2.md) | [Next](./page4.md)

# Axum Extractors Explained

This document explains the imports from the `extract` module found in `src/api/handlers.rs`:

```rust
use axum::extract::{Query, Json, State, Multipart};
```

In Axum, **Extractors** are types that allow a handler function to automatically "extract" specific parts of an HTTP request (like the body, query parameters, or shared application state) simply by adding them as arguments to the function signature.

Here is a breakdown of each type and how it is used in the handlers.

## 1. Query
*   **What it does:** Parses the query string (the part of the URL after `?`) into a Rust struct.
*   **Usage:** It is used in `list_recordings` and `upload_handler` to extract the `DateFilter`.

```rust
// Example from src/api/handlers.rs
pub async fn list_recordings(
    Query(filter): Query<DateFilter> // Extracts ?date=2023-01-01
) ...
```

## 2. Json
*   **What it does:** Handles JSON data. When used as an argument, it deserializes the HTTP request body from JSON into a struct. When used as a return type (aliased as `AxumJson`), it serializes a struct back into a JSON response.
*   **Usage:** It is used in `delete_recording` to parse the `DeleteRequest` payload containing the file path to delete.

```rust
// Example from src/api/handlers.rs
pub async fn delete_recording(
    Json(payload): Json<DeleteRequest> // Extracts JSON body
) ...
```

## 3. State
*   **What it does:** Allows handlers to access shared application state that was set up when the router was created. This is commonly used for database connections or configuration.
*   **Usage:** It is used in almost every handler to inject the `PgPool` (PostgreSQL connection pool) so the handler can run database queries.

```rust
// Example from src/api/handlers.rs
pub async fn list_recordings(
    State(pool): State<PgPool>, // Access the database pool
) ...
```

## 4. Multipart
*   **What it does:** Allows the handler to process `multipart/form-data` requests, which are standard for file uploads. It provides a stream of fields and files.
*   **Usage:** It is used in `upload_handler` to receive the audio file being uploaded by the user.

```rust
// Example from src/api/handlers.rs
pub async fn upload_handler(
    mut multipart: Multipart // Stream of uploaded files/fields
) ...
```

[Prev](./page2.md) | [Next](./page4.md)