use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Diagnostic {
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub level: Level,
    pub rendered: String,
    pub spans: Vec<Span>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Warning,
    Error,
}

#[derive(Debug, Deserialize)]
pub struct Span {
    pub file_name: String,
    pub line_start: usize,
    pub line_end: usize,
}
