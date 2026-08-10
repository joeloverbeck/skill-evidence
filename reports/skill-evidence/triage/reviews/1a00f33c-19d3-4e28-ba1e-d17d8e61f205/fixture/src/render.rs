use crate::Item;

/// Render the brief report for `items`.
pub fn render_brief(items: &[Item]) -> String {
    let mut out = String::new();
    for item in items {
        out.push_str(&format!("- {}\n", item.title));
    }
    out.push_str(&format!("Summary: {} items\n", items.len()));
    out
}

/// Render the full report for `items`, one block per item.
pub fn render_full(items: &[Item]) -> String {
    let mut out = String::new();
    for item in items {
        out.push_str(&format!("## {}\n\n{}\n\n", item.title, item.body));
    }
    out.push_str(&format!("Summary: {} items\n", items.len()));
    out
}
