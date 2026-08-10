use crate::{render_brief, render_full, Item};

/// The output shape requested on the command line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// One line per item plus the summary line.
    Brief,
    /// One block per item plus the summary line.
    Full,
    /// A machine-readable object with an `items` array and a `count` field.
    Json,
}

/// Render `items` in the requested `format`.
///
/// `Format::Json` exists for machine consumers: scripts, dashboards, and any
/// caller that should not parse the human-readable surfaces.
pub fn report(items: &[Item], format: Format) -> String {
    match format {
        Format::Brief => render_brief(items),
        Format::Full => render_full(items),
        Format::Json => render_json(items),
    }
}

fn render_json(items: &[Item]) -> String {
    let entries = items
        .iter()
        .map(|item| format!("{{\"title\":{:?},\"body\":{:?}}}", item.title, item.body))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"items\":[{}],\"count\":{}}}\n", entries, items.len())
}

/// Parse the `--format` argument. Defaults to `Format::Brief`.
pub fn parse_format(arg: Option<&str>) -> Result<Format, String> {
    match arg {
        None | Some("brief") => Ok(Format::Brief),
        Some("full") => Ok(Format::Full),
        Some("json") => Ok(Format::Json),
        Some(other) => Err(format!("unknown format: {other}")),
    }
}
