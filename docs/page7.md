[Prev](./page6.md)

# Date-Based Filtering for Recording History

This document outlines the plan to address the scalability issue of displaying all recordings at once by allowing users to filter by date.

## Problem
Currently, `index.html` fetches and renders every recording in the `recordings/` directory. As the number of files grows, this causes performance issues and infinite scrolling.

## Proposed Solution
Implement **Date Filtering** so users can view recordings for a specific day.

### 1. Backend (`src/main.rs`)
Modify the `/recordings` endpoint to support an optional date query parameter.

- **Query Parameters:**
  - `date`: Optional date string in `YYYY-MM-DD` format (e.g., `2026-01-13`).
- **Logic:**
  1.  If `date` is provided:
      - Construct the specific directory path: `recordings/YYYY/M/D`.
      - Check if it exists.
      - Read only that directory using `std::fs::read_dir` (no need for deep recursion if the structure is strict).
  2.  If `date` is **not** provided:
      - Default to the current date (today).
  3.  Return the list of files found in that specific date folder.

### 2. Frontend (`script.ts` & `index.html`)
Update the client-side UI to control the date selection.

- **UI Changes:**
  - Add a `<input type="date">` element above the recording list.
  - Default the value to today's date.
- **Logic:**
  - **On Page Load:**
    - Automatically set the date picker to the current date.
    - Fetch and display recordings for **Today**.
    - If empty, show a friendly "No recordings for today" message.
  - **On Date Change:**
    - When the user selects a past date, fetch recordings for that specific day.
    - Clear the list and render the results for the selected date.
  - **New Uploads:**
    - If a user records a new audio clip while on "Today's" view, the list refreshes to show the new item immediately.

## Implementation Steps
1.  **Backend**: Update `list_recordings` to accept `Query<FilterParams>` and target specific directories.
2.  **Frontend**: Add the date picker, handle the `change` event, and update the fetch URL.
3.  **Docs**: Update documentation to reflect the API changes.

[Prev](./page6.md)
