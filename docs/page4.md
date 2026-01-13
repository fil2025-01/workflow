[Prev](./page3.md) | [Next](./page5.md)

# Gemini Audio Transcription Plan

This document outlines the strategy to integrate Google's Gemini model to transcribe audio recordings stored in the `recordings/` directory.

## Goal
Automatically convert user-uploaded `.webm` audio files into text using the Gemini API (specifically a multimodal model like `gemini-1.5-flash`) and save the transcript locally.

## Prerequisites

1.  **Google AI Studio API Key**:
    *   Get an API key from Google AI Studio.
    *   Set it as an environment variable: `GEMINI_API_KEY`.

2.  **Rust Dependencies**:
    Update `Cargo.toml` to include libraries for HTTP requests and JSON handling:
    *   `reqwest`: For making HTTP requests to the Gemini API.
    *   `serde` & `serde_json`: For parsing API responses.
    *   `base64`: To encode audio data for the API payload.
    *   `dotenv` (optional): To manage environment variables.

## Implementation Steps

### 1. Backend Logic (`src/main.rs`)

We will modify the server to handle transcription.

#### A. Triggering Transcription
The simplest approach is to trigger the transcription process immediately after the file is successfully saved in the `upload_handler`. To prevent blocking the HTTP response, we can spawn a tokio task.

#### B. Constructing the API Request
Gemini supports multimodal input (text + audio). We will use the `generateContent` endpoint.

**Endpoint:**
`POST https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=YOUR_API_KEY`

**Payload:**
```json
{
  "contents": [
    {
      "parts": [
        { "text": "Please transcribe the following audio file." },
        {
          "inline_data": {
            "mime_type": "audio/webm",
            "data": "BASE64_ENCODED_STRING"
          }
        }
      ]
    }
  ]
}
```

#### C. Processing the Response
1.  Parse the JSON response to extract the candidate text.
2.  Create a new text file with the same timestamp as the recording (e.g., `recording_1768035420.txt`).
3.  Write the transcribed text to this file in the `recordings/` directory.

## Future Enhancements
- **Frontend Display**: Update `index.html` to poll for the `.txt` file and display the transcription to the user.

[Prev](./page3.md) | [Next](./page5.md)