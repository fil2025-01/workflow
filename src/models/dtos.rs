use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct TaskGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ordering: i32,
}

#[derive(Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct RecordingFile {
    pub id: Uuid,
    pub path: String,
    pub name: String,
    pub status: String,
    pub transcription: Option<serde_json::Value>,
    pub group_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub path: String,
}

#[derive(Deserialize)]
pub struct DateFilter {
    pub date: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRecordingRequest {
    pub group_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
}

#[derive(Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Part {
    Text { text: String },
    InlineData { inline_data: InlineData },
}

#[derive(Serialize)]
pub struct InlineData {
    pub mime_type: String,
    pub data: String,
}

#[derive(Deserialize)]
pub struct GenerateContentResponse {
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: CandidateContent,
}

#[derive(Deserialize)]
pub struct CandidateContent {
    pub parts: Vec<CandidatePart>,
}

#[derive(Deserialize)]
pub struct CandidatePart {
    pub text: Option<String>,
}