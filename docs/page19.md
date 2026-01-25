[Prev](./page18.md) | [Next](./page20.md)

# Project Evolution Summary

This document summarizes the development journey of the "Workflow" application, tracking its evolution from a basic backend prototype to a modern, full-stack Rust web application.

## Phase 1: Foundation (Backend & Static Files)
**Goal:** Establish a reliable server for audio uploads and basic file management.
- **Tech Stack:** Axum, Tokio, HTML/CSS/JS (Vanilla).
- **Key Features:**
    - `/upload` endpoint receiving `multipart/form-data`.
    - Local file storage in `recordings/YYYY/MM/DD/`.
    - Simple API to list recordings.
    - Basic UI to record audio using browser `MediaRecorder` API.

## Phase 2: Intelligence (Gemini Integration)
**Goal:** Automate transcription of audio notes.
- **Feature:** Implemented `src/service/transcription.rs`.
- **Logic:**
    - On upload, the server calls Google Gemini 2.0 Flash API.
    - The prompt instructs Gemini to return a structured JSON response containing a Title, Transcript, and later, an Improved Transcript.
    - **Optimization:** Moved the prompt to `src/service/prompt.md` for readability and maintainability.

## Phase 3: Persistence (PostgreSQL Migration)
**Goal:** Move from file-system metadata to a structured relational database.
- **Tech Stack:** SQLx, PostgreSQL.
- **Actions:**
    - Designed schema: `recordings` table with `JSONB` for transcription data.
    - Migrated existing file-based data to DB using a custom script.
    - Updated Axum handlers (`src/api/recordings.rs`) to read/write from Postgres.

## Phase 4: Organization (Task Groups)
**Goal:** Categorize recordings into actionable "Day Parts".
- **Schema:** Added `task_groups` table (e.g., Delegation, Implementation, Recurring).
- **UI:** Added a dropdown to assign each recording to a specific group.
- **Logic:** Implemented endpoints to fetch groups and update recording metadata.

## Phase 5: Modernization (Leptos Migration)
**Goal:** Unify the stack and improve UI reactivity using a Rust frontend.
- **Tech Stack:** Leptos (Full-stack), Wasm-bindgen, Web-sys.
- **Architecture:**
    - **Hybrid Server:** Axum serves both the REST API (for compatibility) and the Leptos SSR app.
    - **Server Functions:** Replaced manual `fetch` calls with type-safe `#[server]` functions (`get_recordings`, `update_recording`).
    - **Reactivity:** Used Signals and Resources to manage UI state.
    - **Components:** Refactored monolithic HTML into `RecordButton`, `RecordingList`, `DateFilter`.
    - **Recording Logic:** Ported JavaScript `MediaRecorder` logic to Rust (WASM) using `web-sys`.

## Phase 6: Polish & Robustness
**Goal:** Ensure a smooth user experience.
- **Polling:** Implemented a robust `set_interval` loop to automatically check for transcription completion (PENDING -> COMPLETED).
- **Refetching:** Ensured "Continue Recording" respects the selected date filter and immediately updates the list upon completion.
- **Formatting:** Added `chrono` to display readable timestamps (e.g., "10:00 PM").
- **Asset Handling:** Configured `ServeDir` to correctly serve compiled WASM/CSS assets, fixing 404 errors.

[Prev](./page18.md) | [Next](./page20.md)