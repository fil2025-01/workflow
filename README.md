# Workflow Audio Recorder & Manager

A personal productivity tool for recording, transcribing, and organizing daily audio notes. Designed to help segment your day into distinct phases: Delegation, Implementation, and Recurring Tasks.

## 🚀 Features

*   **Audio Recording**: Record voice notes directly from your browser.
*   **Automatic Transcription**: Seamlessly transcribes audio in the background (using Gemini API).
*   **Task Grouping**: Organize recordings into "Day Parts":
    *   **Part 1**: Delegation ("Can I delegate this?")
    *   **Part 2**: Implementation ("Doing the work")
    *   **Part 3**: Recurring Tasks ("Emails, admin")
*   **Time & Date Filtering**: Filter recordings by date and view them chronologically.
*   **Persistent Storage**: Metadata and transcripts are stored in PostgreSQL; audio files are saved locally.
*   **Modern UI**: Clean interface with real-time status updates and playback controls.
*   **Isomorphic Rust**: Frontend and backend unified using [Leptos](https://leptos.dev).

## 🛠️ Tech Stack

### Backend
*   **Language**: Rust (Edition 2021)
*   **Framework**: [Axum](https://github.com/tokio-rs/axum)
*   **Database**: PostgreSQL (via `sqlx`)
*   **Async Runtime**: Tokio

### Frontend
*   **Framework**: [Leptos](https://leptos.dev) (Rust to WASM)
*   **Styling**: Custom CSS

## 📋 Prerequisites

*   **Rust**: `cargo` (latest stable)
*   **PostgreSQL**: A running Postgres instance.
*   **Gemini API Key**: For transcription services.
*   **WASM Target**: `rustup target add wasm32-unknown-unknown`
*   **Leptos CLI**: `cargo install cargo-leptos`

## ⚙️ Setup

1.  **Clone the repository**:
    ```bash
    git clone <repository-url>
    cd workflow
    ```

2.  **Environment Configuration**:
    Create a `.env` file based on `.env.example`:
    ```bash
    cp .env.example .env
    ```
    Update the values:
    ```env
    DATABASE_URL=postgres://user:password@localhost:5432/workflow_db
    LOCAL_GEMINI_API_KEY=your_gemini_api_key
    ```

3.  **Database Setup**:
    Ensure your Postgres database exists, then run migrations:
    ```bash
    # If you have sqlx-cli installed:
    sqlx migrate run

    # Or use the provided helper (if available/configured) or run the SQL files in /migrations manually.
    ```

## 🚀 Usage

### Development (Recommended)
This compiles both the frontend (WASM) and backend, and watches for changes.

1.  **Start the Server**:
    ```bash
    cargo leptos watch
    ```
    The server will start on `http://127.0.0.1:4000`.

2.  **Open the Application**:
    Navigate to `http://localhost:4000` in your web browser.
    *   **Leptos UI**: `http://localhost:4000`
    *   **Legacy UI**: `http://localhost:4000/legacy` (if preserved)

### Running Only Backend (Legacy Mode)
If you only want to run the backend and legacy static site without compiling WASM:

```bash
cargo run --features ssr
```

## 📂 Project Structure

*   `src/`: Source code.
    *   `app.rs`: Leptos frontend application.
    *   `main.rs`: Axum server entry point.
    *   `lib.rs`: Shared code and hydration entry point.
    *   `api/`: HTTP handlers.
    *   `models/`: Database structs and DTOs.
    *   `service/`: Business logic (transcription).
*   `static/`: Frontend assets (CSS, etc).
*   `migrations/`: SQL migration files.
*   `recordings/`: Local storage for audio files.
*   `docs/`: Project documentation and plans.

## 🤝 Contributing

1.  Fork the repository.
2.  Create a feature branch (`git checkout -b feature/amazing-feature`).
3.  Commit your changes (`git commit -m 'Add amazing feature'`).
4.  Push to the branch (`git push origin feature/amazing-feature`).
5.  Open a Pull Request.

## 📄 License

This project is licensed under the ISC License.