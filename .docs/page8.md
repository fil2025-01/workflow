[Prev](./page7.md) | [Next](./page9.md)

# UI Enhancements and File Management

This document details the improvements made to the user interface flow and the addition of file management capabilities.

## 1. Alternating Views
To provide a cleaner user experience, the application now splits the "Recording" and "History" states into separate views.

- **Recording View (Default):**
  - Displays the "Record Audio" button.
  - Contains a "View Recordings" button to switch to the history view.
- **History View:**
  - Displays the date filter and the list of recordings.
  - Contains a "Back" button to return to the recording view.
  - Automatically scrolls to the top when switching back to ensure the main actions are visible.

## 2. Recording Deletion
Users can now delete individual recordings directly from the interface.

### Frontend
- A red **"Delete"** button is added to each recording card.
- Clicking it triggers a confirmation dialog (`confirm()`).
- Upon confirmation, a `DELETE` request is sent to the `/recordings` endpoint with the file path.
- The list automatically refreshes upon successful deletion.

### Backend
- **Endpoint:** `DELETE /recordings`
- **Payload:** JSON object `{"path": "/files/..."}`
- **Logic:**
  - Validates the path to ensure it is relative to the `recordings/` directory (security check).
  - Deletes the specified file (e.g., `.webm`).
  - Automatically checks for and deletes the corresponding pair file (e.g., if deleting audio, it also deletes the `.txt` transcript, and vice-versa) to keep the storage clean.

[Prev](./page7.md) | [Next](./page9.md)