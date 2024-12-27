use super::query_filter::FilterItem as DFItems;
use super::query_filter::*;

#[test]
fn test_build_query_filter() {
    let mut filter = QueryFilter::new();
    filter.or(vec![
        DFItems::checkbox("Property 1", CheckboxFilterItem::equals()),
        DFItems::date("Property 2", DateFilterItem::equals("2024-12-20")),
        DFItems::and(vec![
            DFItems::select("Property 3", SelectFilterItem::equals("Option 1")),
            DFItems::rich_text("Property 4", RichTextFilterItem::contains("Sample Text")),
        ]),
    ]);

    let filter = filter.build();
    println!("{}", filter);
    assert_eq!(
        filter,
        r#"{"filter":{"or":[{"property":"Property 1","checkbox":{"equals":true}},{"property":"Property 2","date":{"equals":"2024-12-20"}},{"and":[{"property":"Property 3","select":{"equals":"Option 1"}},{"property":"Property 4","rich_text":{"contains":"Sample Text"}}]}]}}"#,
    );
}
