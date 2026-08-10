use reportline::cli::{parse_format, report, Format};
use reportline::Item;

#[test]
fn parse_format_defaults_to_brief() {
    assert_eq!(parse_format(None), Ok(Format::Brief));
}

#[test]
fn parse_format_rejects_an_unknown_format() {
    assert!(parse_format(Some("yaml")).is_err());
}

#[test]
fn json_report_carries_every_item_and_a_count() {
    let items = vec![Item::new("first", "one")];
    let rendered = report(&items, Format::Json);
    assert!(rendered.contains("\"title\":\"first\""));
    assert!(rendered.contains("\"count\":1"));
}
