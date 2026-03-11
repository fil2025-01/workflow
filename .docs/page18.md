[Prev](./page17.md) | [Next](./page19.md)

# Implementation Detail: Leptos Migration

The migration from vanilla HTML/TS to Leptos has been completed. This document details the technical choices and architecture of the new frontend.

## 1. Hybrid Backend (Axum + Leptos)
We maintained the existing Axum Rest API while mounting Leptos at the root.
*   **Legacy Compatibility**: The old UI is still accessible at `/legacy`.
*   **Shared State**: Both Axum handlers and Leptos Server Functions share the same `PgPool` from `AppState`.
*   **Static Asset Handling**: `cargo-leptos` manages the compilation of WASM and CSS into `target/site/pkg`.

## 2. Server Functions (RPC)
We replaced client-side `fetch` calls with type-safe `#[server]` functions:
*   `get_recordings(date)`: Queries the DB via `list_recordings_inner`.
*   `get_groups()`: Fetches task groups.
*   `update_recording_group(id, group_id)`: Updates metadata.
*   `delete_recording(id)`: Removes DB record and file from disk.

## 3. Reactive UI Architecture
*   **Signals**: Used for UI state like `is_recording` and `view_history`.
*   **Resources**: `create_resource` handles async data fetching for recordings and groups, automatically refetching when the `selected_date` signal changes.
*   **Server Actions**: `create_server_action` manages mutations (Update/Delete), providing a robust way to handle "dispatch" and "versioning" for UI synchronization.
*   **Polling**: A `create_effect` monitors the recordings list and triggers a refetch every 3 seconds if any recording is in `PENDING` status.

## 4. Audio Recording in Rust (WASM)
The core recording logic was ported to Rust using `web-sys`:
*   **Microphone Access**: `navigator.media_devices().get_user_media(...)`.
*   **MediaRecorder**: Managed via `Rc<RefCell<Option<MediaRecorder>>>` to allow sharing between async tasks and UI events.
*   **Blob Handling**: Audio chunks are collected in WASM memory and converted to a `web_sys::Blob` upon stopping.
*   **Upload**: Uses `web_sys::window().fetch_with_str_and_init` to POST the `FormData` to the existing `/upload` REST endpoint.

## 5. Result
The application now benefits from a single language (Rust) across the entire stack, providing compile-time safety for data models and API interactions.

[Prev](./page17.md) | [Next](./page19.md)
