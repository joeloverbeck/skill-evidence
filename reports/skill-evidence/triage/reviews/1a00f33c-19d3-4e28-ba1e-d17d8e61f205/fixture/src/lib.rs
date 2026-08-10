pub mod cli;
pub mod render;

pub use render::{render_brief, render_full};

/// One entry in a report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub title: String,
    pub body: String,
}

impl Item {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }
}
