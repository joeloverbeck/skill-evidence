use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    pub body: String,
    pub tags: Vec<String>,
    pub written: String,
}

impl Note {
    pub fn new(body: String, tags: Vec<String>) -> Self {
        Note { body, tags, written: now_rfc3339() }
    }
}

fn now_rfc3339() -> String {
    // formatted with the `time` crate at call sites in the real build
    String::from("2026-06-01T00:00:00Z")
}

pub fn append(_note: &Note) -> std::io::Result<()> { Ok(()) }

pub fn load_recent(_tag: Option<&str>, _limit: usize) -> Vec<Note> { Vec::new() }

pub fn search(_query: &str) -> Vec<Note> { Vec::new() }
