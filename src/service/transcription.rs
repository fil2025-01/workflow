use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use base64::{Engine as _, engine::general_purpose};
use crate::models::dtos::{
    GenerateContentRequest, Content, Part, InlineData, GenerateContentResponse
};

pub async fn transcribe_audio(filepath: PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("LOCAL_GEMINI_API_KEY").map_err(|_| "LOCAL_GEMINI_API_KEY not set")?;

    // Read the file
    let mut file = File::open(&filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Encode to base64
    let base64_audio = general_purpose::STANDARD.encode(&buffer);

    // Construct request
    let request_body = GenerateContentRequest {
        contents: vec![Content {
            parts: vec![
                Part::Text { text: "Transcribe the following audio. Then, provide a short, clean, and descriptive title (max 6 words) summarizing the content. Return ONLY a raw JSON object (no markdown formatting) with the following structure:\n{\n  \"title\": \"Your Title\",\n  \"transcript\": \"Full Transcription\"\n}".to_string() },
                Part::InlineData {
                    inline_data: InlineData {
                        mime_type: "audio/webm".to_string(),
                        data: base64_audio,
                    },
                },
            ],
        }],
    };

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let res = client.post(&url)
        .json(&request_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await?;
        return Err(format!("API Error: {}", text).into());
    }

    let response_data: GenerateContentResponse = res.json().await?;

    if let Some(candidates) = response_data.candidates {
        if let Some(first_candidate) = candidates.first() {
            if let Some(first_part) = first_candidate.content.parts.first() {
                if let Some(text) = &first_part.text {
                    // Clean markdown code blocks if present
                    let clean_text = text.trim();
                    let clean_text = if clean_text.starts_with("```json") {
                        &clean_text[7..]
                    } else if clean_text.starts_with("```") {
                        &clean_text[3..]
                    } else {
                        clean_text
                    };
                    
                    let clean_text = clean_text.trim_end_matches("```").trim();

                    // Save transcript
                    let mut txt_path = filepath.clone();
                    txt_path.set_extension("json");
                    let mut txt_file = File::create(&txt_path)?;
                    txt_file.write_all(clean_text.as_bytes())?;
                    println!("Transcription saved to: {}", txt_path.display());
                    return Ok(());
                }
            }
        }
    }

    Err("No transcription text found in response".into())
}
