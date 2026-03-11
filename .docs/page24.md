# CLI Usage and Installation

The `workflow` app now includes a command-line interface (CLI) for recording and uploading audio directly from your terminal.

## Installation

To install the `workflow` command globally on your system:

```bash
cargo install --path .
```

This will compile the CLI binary and place it in your cargo bin directory (usually `~/.cargo/bin`), making the `workflow` command available anywhere in your terminal.

## Usage

### Recording Audio

To record audio and automatically upload it to the server:

```bash
workflow --record
```

- **Start Recording:** The command will start recording immediately using your default input device.
- **Stop and Upload:** Press `Ctrl+C` to stop recording. The audio will be saved as a temporary WAV file, uploaded to the server for transcription, and then deleted locally.

### Server Requirement

The CLI interacts with the workflow server. Ensure the server is running before using the CLI command:

```bash
cargo run --bin workflow-server --features ssr
```

## How it Works

1.  **Audio Capture:** Uses the `cpal` crate to capture raw audio samples from the microphone and `hound` to encode them into a WAV file.
2.  **Upload:** Sends a `POST` request to the `/upload` endpoint of the local server with the audio data.
3.  **Transcription:** The server detects the `.wav` extension, handles the MIME type correctly, and sends the audio to Gemini for processing.
