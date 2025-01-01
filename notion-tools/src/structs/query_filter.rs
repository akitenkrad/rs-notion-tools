//! # Query Filter
//!
//! ## Build a query filter
//!
//! ### Simple filter
//! A simple filter is a filter that has only one condition, `Status=="ToDo"`.
//!
//! ```rust
//! # fn main() {
//! # use notion_tools::structs::query_filter::*;
//! let mut query_filter = QueryFilter::new();
//! query_filter.args(FilterItem::status(String::from("Status"), StatusFilterItem::equals(String::from("ToDo"))));
//! let filter = query_filter.build();
//! # }
//! ```
//!
//! ### Use `and` and `or` to combine multiple filters
//!
//! ```rust
//! # fn main() {
//! # use notion_tools::structs::query_filter::*;
//! let mut query_filter = QueryFilter::new();
//! query_filter.and(vec![
//!    FilterItem::status(String::from("Status"), StatusFilterItem::equals(String::from("Active"))),
//!    FilterItem::rich_text(String::from("Name"), RichTextFilterItem::contains(String::from("Zack"))),
//!    FilterItem::or(vec![
//!       FilterItem::number(String::from("Age"), NumberFilterItem::greater_than(18)),
//!       FilterItem::rich_text(String::from("Address"), RichTextFilterItem::contains(String::from("New York"))),
//!    ])
//! ]);
//! let filter = query_filter.build();
//! # }
//! ```
//!
//! ### Pagenation
//!
//! ```rust
//! # use notion_tools::Notion;
//! # use notion_tools::structs::query_filter::{QueryFilter, StatusFilterItem, FilterItem};
//! # #[tokio::main]
//! # async fn main() {
//! let notion = Notion::new();
//! let mut filter = QueryFilter::new();
//! filter.args(FilterItem::status(
//!     String::from("Status"),
//!     StatusFilterItem::is_not_empty(),
//! ));
//!
//! let mut has_more = true;
//!
//! while has_more {
//!     let response = notion.query_database(filter.clone()).await;
//!
//!     match response {
//!         Ok(response) => {
//!             has_more = response.has_more.unwrap_or(false);
//!             filter.start_cursor = response.next_cursor.unwrap_or(String::new());
//!             for result in response.results {
//!                // Do something with the result
//!             }
//!         }
//!         Err(e) => {
//!             println!("{:?}", e);
//!         }
//!     }
//!     # has_more = false;
//! }
//! # }
//! ```
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckboxFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<bool>,
}

impl CheckboxFilterItem {
    pub fn equals() -> Self {
        CheckboxFilterItem {
            equals: Some(true),
            ..Default::default()
        }
    }

    pub fn does_not_equal() -> Self {
        CheckboxFilterItem {
            does_not_equal: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DateFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub on_or_after: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub on_or_before: Option<String>,
}

impl DateFilterItem {
    pub fn equals(value: String) -> Self {
        DateFilterItem {
            equals: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn after(value: String) -> Self {
        DateFilterItem {
            after: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn before(value: String) -> Self {
        DateFilterItem {
            before: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn is_empty() -> Self {
        DateFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        DateFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn on_or_after(value: String) -> Self {
        DateFilterItem {
            on_or_after: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn on_or_before(value: String) -> Self {
        DateFilterItem {
            on_or_before: Some(value.to_string()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilesFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

impl FilesFilterItem {
    pub fn is_empty() -> Self {
        FilesFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        FilesFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormulaFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub checkbox: Option<CheckboxFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub date: Option<DateFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub string: Option<RichTextFilterItem>,
}

impl FormulaFilterItem {
    pub fn checkbox(item: CheckboxFilterItem) -> Self {
        FormulaFilterItem {
            checkbox: Some(item),
            ..Default::default()
        }
    }

    pub fn date(item: DateFilterItem) -> Self {
        FormulaFilterItem {
            date: Some(item),
            ..Default::default()
        }
    }

    pub fn number(item: NumberFilterItem) -> Self {
        FormulaFilterItem {
            number: Some(item),
            ..Default::default()
        }
    }

    pub fn string(item: RichTextFilterItem) -> Self {
        FormulaFilterItem {
            string: Some(item),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiSelectFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<bool>,
}

impl MultiSelectFilterItem {
    pub fn contains(value: String) -> Self {
        MultiSelectFilterItem {
            contains: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn does_not_contain(value: String) -> Self {
        MultiSelectFilterItem {
            does_not_contain: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn equals() -> Self {
        MultiSelectFilterItem {
            equals: Some(true),
            ..Default::default()
        }
    }

    pub fn does_not_equal() -> Self {
        MultiSelectFilterItem {
            does_not_equal: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumberFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub less_than: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal_to: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal_to: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

impl NumberFilterItem {
    pub fn equals(value: u128) -> Self {
        NumberFilterItem {
            equals: Some(value),
            ..Default::default()
        }
    }

    pub fn does_not_equal(value: u128) -> Self {
        NumberFilterItem {
            does_not_equal: Some(value),
            ..Default::default()
        }
    }

    pub fn greater_than(value: u128) -> Self {
        NumberFilterItem {
            greater_than: Some(value),
            ..Default::default()
        }
    }

    pub fn less_than(value: u128) -> Self {
        NumberFilterItem {
            less_than: Some(value),
            ..Default::default()
        }
    }

    pub fn greater_than_or_equal_to(value: u128) -> Self {
        NumberFilterItem {
            greater_than_or_equal_to: Some(value),
            ..Default::default()
        }
    }

    pub fn less_than_or_equal_to(value: u128) -> Self {
        NumberFilterItem {
            less_than_or_equal_to: Some(value),
            ..Default::default()
        }
    }

    pub fn is_empty() -> Self {
        NumberFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        NumberFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeopleFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

impl PeopleFilterItem {
    pub fn contains(value: String) -> Self {
        PeopleFilterItem {
            contains: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn does_not_contain(value: String) -> Self {
        PeopleFilterItem {
            does_not_contain: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn is_empty() -> Self {
        PeopleFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        PeopleFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

impl RelationFilterItem {
    pub fn contains(value: String) -> Self {
        RelationFilterItem {
            contains: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn does_not_contain(value: String) -> Self {
        RelationFilterItem {
            does_not_contain: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn is_empty() -> Self {
        RelationFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        RelationFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RichTextFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

impl RichTextFilterItem {
    pub fn contains(value: String) -> Self {
        RichTextFilterItem {
            contains: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn does_not_contain(value: String) -> Self {
        RichTextFilterItem {
            does_not_contain: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn equals(value: String) -> Self {
        RichTextFilterItem {
            equals: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn does_not_equal(value: String) -> Self {
        RichTextFilterItem {
            does_not_equal: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn starts_with(value: String) -> Self {
        RichTextFilterItem {
            starts_with: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn ends_with(value: String) -> Self {
        RichTextFilterItem {
            ends_with: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn is_empty() -> Self {
        RichTextFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        RichTextFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelectFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equals: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

impl SelectFilterItem {
    pub fn equals(value: String) -> Self {
        SelectFilterItem {
            equals: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn does_not_equals(value: String) -> Self {
        SelectFilterItem {
            does_not_equals: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn is_empty() -> Self {
        SelectFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        SelectFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equals: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

impl StatusFilterItem {
    pub fn equals(value: String) -> Self {
        StatusFilterItem {
            equals: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn does_not_equals(value: String) -> Self {
        StatusFilterItem {
            does_not_equals: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn is_empty() -> Self {
        StatusFilterItem {
            is_empty: Some(true),
            ..Default::default()
        }
    }

    pub fn is_not_empty() -> Self {
        StatusFilterItem {
            is_not_empty: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimestampFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub last_edited_time: Option<DateFilterItem>,
}

impl TimestampFilterItem {
    pub fn timestamp(value: String) -> Self {
        TimestampFilterItem {
            timestamp: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn created_time(item: DateFilterItem) -> Self {
        TimestampFilterItem {
            created_time: Some(item),
            ..Default::default()
        }
    }

    pub fn last_edited_time(item: DateFilterItem) -> Self {
        TimestampFilterItem {
            last_edited_time: Some(item),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub less_than: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal_to: Option<u128>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal_to: Option<u128>,
}

impl IdFilterItem {
    pub fn equals(value: u128) -> Self {
        IdFilterItem {
            equals: Some(value),
            ..Default::default()
        }
    }

    pub fn does_not_equal(value: u128) -> Self {
        IdFilterItem {
            does_not_equal: Some(value),
            ..Default::default()
        }
    }

    pub fn greater_than(value: u128) -> Self {
        IdFilterItem {
            greater_than: Some(value),
            ..Default::default()
        }
    }

    pub fn less_than(value: u128) -> Self {
        IdFilterItem {
            less_than: Some(value),
            ..Default::default()
        }
    }

    pub fn greater_than_or_equal_to(value: u128) -> Self {
        IdFilterItem {
            greater_than_or_equal_to: Some(value),
            ..Default::default()
        }
    }

    pub fn less_than_or_equal_to(value: u128) -> Self {
        IdFilterItem {
            less_than_or_equal_to: Some(value),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterItem {
    #[serde(default = "String::new", skip_serializing_if = "String::is_empty")]
    pub property: String,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<FilterItem>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<FilterItem>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub checkbox: Option<CheckboxFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub date: Option<DateFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub files: Option<FilesFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub formula: Option<FormulaFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub multi_select: Option<MultiSelectFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub people: Option<PeopleFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub relation: Option<RelationFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<RichTextFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub select: Option<SelectFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimestampFilterItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub id: Option<IdFilterItem>,
}

impl FilterItem {
    pub fn and(items: Vec<FilterItem>) -> Self {
        FilterItem {
            and: Some(items),
            ..Default::default()
        }
    }

    pub fn or(items: Vec<FilterItem>) -> Self {
        FilterItem {
            or: Some(items),
            ..Default::default()
        }
    }

    pub fn checkbox(property: String, item: CheckboxFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            checkbox: Some(item),
            ..Default::default()
        }
    }

    pub fn date(property: String, item: DateFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            date: Some(item),
            ..Default::default()
        }
    }

    pub fn files(property: String, item: FilesFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            files: Some(item),
            ..Default::default()
        }
    }

    pub fn formula(property: String, item: FormulaFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            formula: Some(item),
            ..Default::default()
        }
    }

    pub fn multi_select(property: String, item: MultiSelectFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            multi_select: Some(item),
            ..Default::default()
        }
    }

    pub fn number(property: String, item: NumberFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            number: Some(item),
            ..Default::default()
        }
    }

    pub fn people(property: String, item: PeopleFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            people: Some(item),
            ..Default::default()
        }
    }

    pub fn relation(property: String, item: RelationFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            relation: Some(item),
            ..Default::default()
        }
    }

    pub fn rich_text(property: String, item: RichTextFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            rich_text: Some(item),
            ..Default::default()
        }
    }

    pub fn select(property: String, item: SelectFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            select: Some(item),
            ..Default::default()
        }
    }

    pub fn status(property: String, item: StatusFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            status: Some(item),
            ..Default::default()
        }
    }

    pub fn timestamp(property: String, item: TimestampFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            timestamp: Some(item),
            ..Default::default()
        }
    }

    pub fn id(property: String, item: IdFilterItem) -> Self {
        FilterItem {
            property: property.to_string(),
            id: Some(item),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub filter: FilterItem,
    #[serde(default = "String::new", skip_serializing_if = "String::is_empty")]
    pub start_cursor: String,
}

impl QueryFilter {
    pub fn new() -> Self {
        QueryFilter {
            filter: FilterItem::default(),
            start_cursor: String::new(),
        }
    }

    pub fn args(&mut self, item: FilterItem) {
        self.filter = item;
    }

    pub fn and(&mut self, items: Vec<FilterItem>) {
        self.filter.and = Some(items);
    }

    pub fn or(&mut self, items: Vec<FilterItem>) {
        self.filter.or = Some(items);
    }

    pub fn build(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
