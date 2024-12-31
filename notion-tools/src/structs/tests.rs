use super::common::*;
use super::page::*;
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

#[test]
fn test_page_property_get_value() {
    let porp = PageProperty::checkbox(true);
    assert_eq!(porp.get_value(), "true");

    let porp = PageProperty::date("2024-12-20");
    assert_eq!(porp.get_value(), "2024-12-20");

    let porp = PageProperty::number(123.456);
    assert_eq!(porp.get_value(), "123.456");

    let porp = PageProperty::select("Option 1");
    assert_eq!(porp.get_value(), "Option 1");

    let porp = PageProperty::title(RichText::from_str("Sample Title"));
    assert_eq!(porp.get_value(), "Sample Title");

    let porp = PageProperty::rich_text(vec![RichText::from_str("Sample Text")]);
    assert_eq!(porp.get_value(), "Sample Text");

    let porp = PageProperty::url("https://example.com");
    assert_eq!(porp.get_value(), "https://example.com");

    let porp = PageProperty::email("sample@sample.com");
    assert_eq!(porp.get_value(), "sample@sample.com");

    let porp = PageProperty::phone_number("1234567890");
    assert_eq!(porp.get_value(), "1234567890");

    let prop = PageProperty::multi_select(vec!["Option 1", "Option 2"]);
    assert_eq!(prop.get_value(), "Option 1, Option 2");

    let prop = PageProperty::relation(vec!["Page ID 1", "Page ID 2"]);
    assert_eq!(prop.get_value(), "Page ID 1, Page ID 2");
}
