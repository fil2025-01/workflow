[Prev](./page4.md) | [Next](./page6.md)

# UI Enhancements and Server Improvements

This document covers the latest updates to the user interface and server infrastructure.

## 1. Recording History UI
The application now includes a chronological list of all recordings stored in the `recordings/` directory.

### Features
- **Automatic Discovery**: The server recursively scans the `recordings/` folder (supporting nested structures like `recordings/2026/01/13/`).
- **Media Playback**: Each recording is displayed with an integrated HTML5 `<audio>` player.
- **Transcription Display**: If a `.txt` transcription file exists for a recording, its content is automatically fetched and displayed below the player.
- **Chronological Sorting**: Recordings are displayed from oldest to newest (top to bottom).
- **Auto-Refresh**: The list refreshes automatically after a new recording is successfully uploaded and processed.

## 2. Dynamic Port Selection
To prevent "Address already in use" errors, the server now intelligently selects an available port at startup.

### Behavior
1. Starts by attempting to bind to port `3000`.
2. If port `3000` is occupied, it increments the port number and tries again (e.g., `3001`, `3002`).
3. It will attempt up to 10 consecutive ports (up to `3010`).
4. Once an open port is found, the server prints the final URL to the console (e.g., `Listening on http://127.0.0.1:3005`).

## 3. Development Workflow (Reminder)
Since the frontend uses TypeScript and the assets are embedded in the Rust binary:
1. Edit `script.ts`.
2. Run `npm run build` to update `script.js`.
3. Run `cargo run` to recompile the binary with the new assets.

[Prev](./page4.md) | [Next](./page6.md)