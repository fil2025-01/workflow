[Prev](./page16.md) | [Next](./page18.md)

# Plan: Migration to Leptos (Rust Frontend)

This document outlines the strategy for replacing the current vanilla HTML/TypeScript frontend with **Leptos**, a full-stack, isomorphic Rust web framework. This will unify the tech stack (Rust on both backend and frontend) and provide better type safety and performance.

## 1. Project Restructuring

### Option A: Integrated (Recommended)
We will integrate Leptos directly into the existing Axum backend. Leptos is designed to work seamlessly with Axum.
*   **Benefits**: Shared types (DTOs), server-side rendering (SSR), simple deployment (single binary).
*   **Structure**:
    *   `src/app.rs`: Main Leptos application logic.
    *   `src/components/`: Reusable UI components.
    *   `src/pages/`: Page-level components.

## 2. Dependencies & Configuration

### Cargo.toml Updates
*   Add `leptos`, `leptos_axum`, `leptos_meta`, `leptos_router`.
*   Add `wasm-bindgen` and `console_error_panic_hook` for the client side.
*   Configure the `[package.metadata.leptos]` section for `cargo-leptos` (the build tool).

### Build Tooling
*   Install `cargo-leptos`: `cargo install cargo-leptos`.
*   Add `style` configuration (e.g., Tailwind CSS) to the Leptos config.

## 3. Migration Steps

### Phase 1: Setup & Hello World
1.  Initialize Leptos configuration in `Cargo.toml`.
2.  Modify `src/main.rs` to wrap the Axum router with `LeptosRoutes`.
3.  Create a basic `App` component in `src/app.rs` rendering a simple "Hello World".
4.  Verify that `cargo leptos watch` runs and serves the app.

### Phase 2: Component Architecture
Refactor the existing HTML/JS into Rust components:
*   **`RecordButton`**: Handles audio recording logic (using `web-sys` for MediaRecorder API).
*   **`RecordingList`**: Displays the table of recordings.
    *   **`RecordingRow`**: Individual row component.
*   **`DateFilter`**: Component for selecting the date.
*   **`TaskGroupSelector`**: Component for the dropdown.

### Phase 3: State Management
*   Use Leptos Signals (`create_signal`, `create_resource`) to manage state.
*   **Recordings Resource**: A `Resource` that fetches recordings from the backend when the `date` signal changes.
*   **Polling**: Use `create_effect` or `set_interval` to refresh the resource for pending transcriptions.

### Phase 4: Server Functions (RPC)
Replace REST API calls with Leptos **Server Functions**.
*   Instead of `fetch('/recordings')`, define a `#[server]` function `get_recordings(date: Option<String>)`.
*   Instead of `fetch('/upload')`, define a `#[server]` function `upload_audio(...)` (handling multipart might require specific Leptos patterns or keeping the Axum handler).
*   **Action**: `delete_recording` and `update_group` become server functions.

## 4. Technical Challenges & Solutions

*   **Audio Recording (WASM)**: Accessing `navigator.mediaDevices` requires `web-sys` with specific features enabled. We will need to write Rust wrappers for the JS MediaRecorder API or use a crate like `web-sys`.
*   **Styling**: We can continue using the existing CSS/Tailwind. Leptos supports importing stylesheets.
*   **Hydration**: Ensure the app hydrates correctly on the client side for interactivity (buttons, inputs).

## 5. Execution Plan

1.  **Dependency Install**: Update `Cargo.toml`.
2.  **Scaffold**: Create `src/app.rs` and update `main.rs`.
3.  **Port Features**:
    *   List Recordings (Server Function + Resource).
    *   Delete/Update (Server Actions).
    *   Record Audio (Client-side `web-sys` logic).
4.  **Cleanup**: Remove `static/` directory and old API handlers (if replaced by Server Functions).

[Prev](./page16.md) | [Next](./page18.md)
