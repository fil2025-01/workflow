[Prev](./page5.md) | [Next](./page7.md)

# Structured Recording Storage

This document outlines the strategy to organize audio recordings into a date-based directory structure.

## Goal
To prevent the `recordings/` directory from becoming cluttered, files will be organized hierarchically by year, month, and day.

## Implementation Details

### Directory Pattern
Before saving a new recording, the backend will:
1.  Determine the current date using the `chrono` crate.
2.  Create a directory path following the pattern: `recordings/{Year}/{Month}/{Day}/` using `fs::create_dir_all`.
3.  Save the file within that specific directory.

### Technical Notes
- **Recursion**: The `WalkDir` crate is used in the `/recordings` endpoint to recursively scan the entire `recordings/` tree, ensuring that files from any date are discovered and listed in the UI.
- **Serving Files**: The `tower-http` `ServeDir` service is nested under `/files`, allowing the frontend to access deeply nested files using their relative paths (e.g., `/files/2026/1/12/recording_...webm`).
- **Date Handling**: The `chrono::Local::now()` function provides the local system time, ensuring directories match the user's current date.

### Example Structure
Based on the design, the file system will look like this:

```text
recordings
└── 2026
    └── 1
        ├── 12
        │   ├── recording_1768180897.webm
        │   └── ...
        └── 13
            ├── recording_1768269097.txt
            ├── recording_1768269097.webm
            └── ...
