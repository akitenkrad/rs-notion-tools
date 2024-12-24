use super::database_filter::*;

use super::database_filter::DatabaseFilterItems as DFItems;
#[test]
fn test_build_database_filter() {
    let mut filter = DatabaseFilter::new();
    filter.or(vec![
        DFItems::checkbox("Property 1", CheckboxFilterItem::default().equals()),
        DFItems::date("Property 2", DateFilterItem::default().equals("2024-12-20")),
        DFItems::and(vec![
            DFItems::select("Property 3", SelectFilterItem::default().equals("Option 1")),
            DFItems::rich_text(
                "Property 4",
                RichTextFilterItem::default().contains("Sample Text"),
            ),
        ]),
    ]);

    let filter = filter.build();
    println!("{}", filter);
    assert_eq!(
        filter,
        r#"{"filter":{"or":[{"property":"Property 1","checkbox":{"equals":true}},{"property":"Property 2","date":{"equals":"2024-12-20"}},{"and":[{"property":"Property 3","select":{"equals":"Option 1"}},{"property":"Property 4","rich_text":{"contains":"Sample Text"}}]}]}}"#,
    );
}
