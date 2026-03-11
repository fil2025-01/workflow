[Prev](./page9.md) | [Next](./page11.md)

# JSON-Based Transcription Storage

This document details the migration from raw text files to structured JSON for storing recording metadata and transcriptions.

## Overview
To improve title accuracy and data reliability, the backend now requests structured JSON from the Gemini API. This allows the application to distinguish between a "summarized title" and the "full transcript" without relying on fragile text parsing.

## Changes

### 1. Backend Enhancements (`src/main.rs`)
- **Prompt Engineering**: The Gemini prompt was updated to explicitly request a raw JSON object:
  ```json
  {
    "title": "Short Summary",
    "transcript": "Full text..."
  }
  ```
- **Storage**: Responses are now saved as `.json` files (e.g., `recording_12345.json`) instead of `.txt`.
- **File Management**: The `list_recordings` and `delete_recording` handlers were updated to recognize and manage `.json` files alongside existing `.webm` and `.txt` files.

### 2. Frontend logic (`script.ts`)
- **Hybrid Parsing**: The `loadRecordings` function now detects the file extension:
  - **`.json`**: Uses native `res.json()` parsing for high reliability.
  - **`.txt`**: Maintains legacy regex parsing to support recordings made before this update.
- **UI Mapping**:
  - `data.title` is displayed in the main table column.
  - `data.transcript` is used for the hover tooltip.

## Benefits
- **Clean UI**: Titles no longer contain "Title: " prefixes or messy regex artifacts.
- **Extensibility**: The JSON format allows us to store more metadata in the future (e.g., sentiment, duration, or language) without breaking the file structure.
- **Reliability**: Structured data significantly reduces the chance of rendering errors caused by unexpected LLM response formatting.

[Prev](./page9.md) | [Next](./page11.md)
