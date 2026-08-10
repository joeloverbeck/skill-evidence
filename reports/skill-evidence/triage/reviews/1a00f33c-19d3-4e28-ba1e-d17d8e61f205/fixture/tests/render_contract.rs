use reportline::{render_brief, render_full, Item};

#[test]
fn brief_report_always_ends_with_a_summary_line() {
    assert_eq!(render_brief(&[]), "Summary: 0 items\n");
}

#[test]
fn brief_report_lists_each_item_before_the_summary_line() {
    let items = vec![Item::new("first", "one"), Item::new("second", "two")];
    assert_eq!(
        render_brief(&items),
        "- first\n- second\nSummary: 2 items\n"
    );
}

#[test]
fn full_report_always_ends_with_a_summary_line() {
    assert_eq!(render_full(&[]), "Summary: 0 items\n");
}
