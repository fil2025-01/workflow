[Next](./page3.md)

# Steps Taken to Run the App

1.  **Project Analysis**:
    *   Read `Cargo.toml` and `src/main.rs` to identify dependencies (Axum, Tokio) and the entry point.
    *   Noted the application listens on `127.0.0.1:3000` and serves `index.html`.

2.  **Environment Check**:
    *   Attempted to run `cargo run`, but the command was not found in the system `PATH`.
    *   Located the `cargo` executable at `/Users/fil/.cargo/bin/cargo`.
    *   Verified the cargo version: `cargo 1.92.0 (344c4567c 2025-10-21)`.

3.  **Build Process**:
    *   Successfully compiled the project using the absolute path: `/Users/fil/.cargo/bin/cargo build`.
    *   Dependencies were downloaded and compiled without errors.

4.  **Execution Attempts**:
    *   Attempted to run the application in the background redirecting output to `server.log`.
    *   Checked for the process and listening port (3000), but the application did not appear to stay running in the background during the initial checks.

---

# Audio Recording Implementation

This document outlines the changes made to implement audio recording and uploading functionality.

## Overview
The implementation consists of a frontend button that records audio from the user's microphone and a backend endpoint that receives and saves the audio data.

## Components

### 1. Frontend (`index.html`)
- Added JavaScript logic to handle the `MediaRecorder` API.
- Implemented `navigator.mediaDevices.getUserMedia` to request microphone access.
- Added a click event listener to the "Record Audio" button to toggle recording state and update the UI.
- Once recording stops, the audio chunks are collected into a `Blob` and sent to the server via a `POST` request to `/upload` using the `Fetch API`.

### 2. Backend (`src/main.rs`)
- Added a new route `POST /upload` using `axum::routing::post`.
- Implemented `upload_handler` to process `multipart/form-data` using the `Multipart` extractor.
- Files are saved in the project root with a timestamp-based filename (e.g., `recording_1768035420.webm`).

### 3. Configuration (`Cargo.toml`)
- Updated the `axum` dependency to enable the `multipart` feature, which is required for handling file uploads.

## How to Test
1. Start the server:
   ```bash
   cargo run
   ```
2. Open `http://127.0.0.1:3000` in a web browser.
3. Click the **Record Audio** button and grant microphone permissions when prompted.
4. Click **Stop Recording** to complete the capture and upload the file.
5. Verify the recording by checking for a new `.webm` file in the project directory.

### Automated Tests
You can also run the backend integration tests to verify the server logic:
```bash
cargo test
```
This will test the root handler and the upload functionality (including file creation and cleanup).

## Development Notes: Refreshing Changes

If you modify `index.html` or any Rust files, follow these steps to see the changes:

### 1. Recompile and Restart
The current implementation uses `include_str!("../index.html")` in `src/main.rs`. This means the HTML content is **embedded into the binary at compile-time**. 
- **Stop the server:** Press `Ctrl+C` in the terminal where `cargo run` is executing.
- **Start again:** Run `cargo run` again. Cargo will detect the change in `index.html` or `.rs` files and recompile the binary.

### 2. Browser Cache
Browsers sometimes cache HTML or JavaScript. If you've restarted the server but don't see changes:
- Force a refresh using `Cmd + Shift + R` (Mac) or `Ctrl + F5` (Windows/Linux).

### 3. Automated Workflow (Optional)
To avoid manual restarts, you can use `cargo-watch`:
```bash
cargo install cargo-watch
cargo watch -x run
```
This will automatically recompile and restart your server whenever a file change is detected.

[Next](./page3.md)
