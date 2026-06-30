//! Human- and machine-friendly rendering for CLI command results.
//!
//! Every list/get command renders through this module so output is consistent:
//! a clean, aligned table for humans by default, or `--output json` for scripts
//! and pipelines. The table renderer is dependency-free and unicode-width aware
//! (so CJK/Thai/emoji columns still line up).

use serde_json::Value;
use std::fmt::Write as _;
use unicode_width::UnicodeWidthStr;

/// How a command should present its result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum OutputFormat {
    /// Aligned, human-readable table (default).
    #[default]
    Table,
    /// Pretty-printed JSON, for scripts and pipelines.
    Json,
}

/// Render a list of records.
///
/// `columns` is the ordered list of `(header, extractor)` pairs used for table
/// output; `Json` ignores them and prints the raw values. An empty result set
/// prints a short notice to stderr in table mode so a successful-but-empty
/// query never looks like a silent failure.
pub fn print_records(format: OutputFormat, rows: &[Value], columns: &[Column]) {
    match format {
        OutputFormat::Json => print_json(&Value::Array(rows.to_vec())),
        OutputFormat::Table => {
            if rows.is_empty() {
                eprintln!("No results.");
                return;
            }
            let headers: Vec<&str> = columns.iter().map(|c| c.header).collect();
            let table: Vec<Vec<String>> = rows
                .iter()
                .map(|row| columns.iter().map(|c| (c.extract)(row)).collect())
                .collect();
            print!("{}", render_table(&headers, &table));
        }
    }
}

/// Render a single object: pretty JSON for `Json`, and a vertical
/// `key: value` view for `Table` (objects don't fit a row layout well).
pub fn print_object(format: OutputFormat, value: &Value, fields: &[Column]) {
    match format {
        OutputFormat::Json => print_json(value),
        OutputFormat::Table => {
            let label_width = fields.iter().map(|f| f.header.width()).max().unwrap_or(0);
            for f in fields {
                println!(
                    "{:<width$}  {}",
                    f.header,
                    (f.extract)(value),
                    width = label_width
                );
            }
        }
    }
}

/// Pretty-print a JSON value to stdout (falls back to its `Display` form if it
/// somehow cannot be serialized, which should not happen for in-memory values).
pub fn print_json(value: &Value) {
    match serde_json::to_string_pretty(value) {
        Ok(s) => println!("{s}"),
        Err(_) => println!("{value}"),
    }
}

/// A named table/object column and how to pull its cell text from a record.
pub struct Column {
    pub header: &'static str,
    pub extract: fn(&Value) -> String,
}

impl Column {
    pub const fn new(header: &'static str, extract: fn(&Value) -> String) -> Self {
        Self { header, extract }
    }
}

/// Render an aligned text table with a header row and a separator line.
///
/// Column widths use display width (not byte or `char` length) so wide glyphs
/// stay aligned. Returns the full table as a string ending in a newline.
fn render_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let cols = headers.len();
    let mut widths: Vec<usize> = headers.iter().map(|h| h.width()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate().take(cols) {
            widths[i] = widths[i].max(cell.width());
        }
    }

    let mut out = String::new();
    write_row(&mut out, headers.iter().map(|h| h.to_string()), &widths);
    // Separator line.
    let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
    write_row(&mut out, sep.into_iter(), &widths);
    for row in rows {
        write_row(&mut out, row.iter().cloned(), &widths);
    }
    out
}

/// Write a single padded row (` | `-separated) to `out`, padding each cell to
/// its column's display width.
fn write_row(out: &mut String, cells: impl Iterator<Item = String>, widths: &[usize]) {
    let mut first = true;
    for (i, cell) in cells.enumerate() {
        if !first {
            out.push_str(" | ");
        }
        first = false;
        let pad = widths
            .get(i)
            .copied()
            .unwrap_or(0)
            .saturating_sub(cell.width());
        let _ = write!(out, "{cell}");
        out.push_str(&" ".repeat(pad));
    }
    // Trim trailing padding on the last cell for cleaner output.
    while out.ends_with(' ') {
        out.pop();
    }
    out.push('\n');
}

// ─── Field extractors for common Notion objects ──────────────────────────

/// `id` field, or `-` when absent.
pub fn field_id(v: &Value) -> String {
    str_or_dash(v.get("id"))
}

/// `object` type (`page`, `database`, `user`, `block`, …).
pub fn field_object(v: &Value) -> String {
    str_or_dash(v.get("object"))
}

/// Best-effort human title: the `title`-typed page property, the database
/// `title` array, or a user `name`; `Untitled` when nothing matches.
pub fn field_title(v: &Value) -> String {
    // Page: properties.<col>.title[].plain_text where type == "title".
    if let Some(props) = v.get("properties").and_then(Value::as_object) {
        if let Some(t) = props
            .values()
            .find(|p| p.get("type").and_then(Value::as_str) == Some("title"))
            .and_then(|p| p.get("title"))
            .and_then(Value::as_array)
            .map(|a| join_plain_text(a))
        {
            if !t.is_empty() {
                return t;
            }
        }
    }
    // Database: top-level `title` rich-text array.
    if let Some(t) = v
        .get("title")
        .and_then(Value::as_array)
        .map(|a| join_plain_text(a))
    {
        if !t.is_empty() {
            return t;
        }
    }
    // User: `name`.
    if let Some(name) = v.get("name").and_then(Value::as_str) {
        return name.to_string();
    }
    "Untitled".to_string()
}

/// `url` field, or `-` when absent.
pub fn field_url(v: &Value) -> String {
    str_or_dash(v.get("url"))
}

/// Block `type` discriminator, or `-`.
pub fn field_block_type(v: &Value) -> String {
    str_or_dash(v.get("type"))
}

/// `last_edited_time`, or `-`.
pub fn field_last_edited(v: &Value) -> String {
    str_or_dash(v.get("last_edited_time"))
}

fn join_plain_text(arr: &[Value]) -> String {
    arr.iter()
        .filter_map(|rt| rt.get("plain_text").and_then(Value::as_str))
        .collect()
}

fn str_or_dash(v: Option<&Value>) -> String {
    v.and_then(Value::as_str).unwrap_or("-").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn table_aligns_columns() {
        let out = render_table(
            &["ID", "Name"],
            &[
                vec!["1".to_string(), "Alice".to_string()],
                vec!["22".to_string(), "Bob".to_string()],
            ],
        );
        let lines: Vec<&str> = out.lines().collect();
        // "Alice" is the widest cell in column 2 (5 columns), so the column
        // and its separator are 5 wide.
        assert_eq!(lines[0], "ID | Name");
        assert_eq!(lines[1], "-- | -----");
        assert_eq!(lines[2], "1  | Alice");
        assert_eq!(lines[3], "22 | Bob");
    }

    #[test]
    fn title_from_named_property_column() {
        let page = json!({
            "object": "page",
            "id": "p1",
            "properties": { "Name": { "type": "title", "title": [{ "plain_text": "Hello" }] } }
        });
        assert_eq!(field_title(&page), "Hello");
        assert_eq!(field_id(&page), "p1");
        assert_eq!(field_object(&page), "page");
    }

    #[test]
    fn title_falls_back_across_shapes() {
        let db = json!({ "object": "database", "title": [{ "plain_text": "My DB" }] });
        assert_eq!(field_title(&db), "My DB");
        let user = json!({ "object": "user", "name": "Alice" });
        assert_eq!(field_title(&user), "Alice");
        let bare = json!({ "object": "page" });
        assert_eq!(field_title(&bare), "Untitled");
    }

    #[test]
    fn json_array_round_trips_records() {
        // print_records with Json should serialize the array; just ensure the
        // extractor wiring compiles and column extraction is correct.
        let rows = [json!({ "id": "a", "object": "page" })];
        let cols = [
            Column::new("ID", field_id),
            Column::new("Type", field_object),
        ];
        let table = render_table(
            &["ID", "Type"],
            &rows
                .iter()
                .map(|r| cols.iter().map(|c| (c.extract)(r)).collect())
                .collect::<Vec<_>>(),
        );
        assert!(table.contains("a"));
        assert!(table.contains("page"));
    }

    #[test]
    fn wide_glyphs_stay_aligned() {
        // A CJK title is 2 display columns per char; alignment must use width.
        let out = render_table(&["T"], &[vec!["中文".to_string()], vec!["x".to_string()]]);
        let lines: Vec<&str> = out.lines().collect();
        // Separator must be as wide as the widest cell (中文 == 4 columns).
        assert_eq!(lines[1], "----");
    }
}
