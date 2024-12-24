use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckboxFilterItem {
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equals: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<bool>,
}

impl CheckboxFilterItem {
    pub fn equals(&mut self) -> CheckboxFilterItem {
        self.equals = Some(true);
        self.clone()
    }

    pub fn does_not_equal(&mut self) -> CheckboxFilterItem {
        self.does_not_equal = Some(true);
        self.clone()
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
    pub fn equals(&mut self, value: &str) -> DateFilterItem {
        self.equals = Some(value.to_string());
        self.clone()
    }

    pub fn after(&mut self, value: &str) -> DateFilterItem {
        self.after = Some(value.to_string());
        self.clone()
    }

    pub fn before(&mut self, value: &str) -> DateFilterItem {
        self.before = Some(value.to_string());
        self.clone()
    }

    pub fn is_empty(&mut self) -> DateFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> DateFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
    }

    pub fn on_or_after(&mut self, value: &str) -> DateFilterItem {
        self.on_or_after = Some(value.to_string());
        self.clone()
    }

    pub fn on_or_before(&mut self, value: &str) -> DateFilterItem {
        self.on_or_before = Some(value.to_string());
        self.clone()
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
    pub fn is_empty(&mut self) -> FilesFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> FilesFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
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
    pub fn checkbox(&mut self, item: CheckboxFilterItem) -> FormulaFilterItem {
        self.checkbox = Some(item);
        self.clone()
    }

    pub fn date(&mut self, item: DateFilterItem) -> FormulaFilterItem {
        self.date = Some(item);
        self.clone()
    }

    pub fn number(&mut self, item: NumberFilterItem) -> FormulaFilterItem {
        self.number = Some(item);
        self.clone()
    }

    pub fn string(&mut self, item: RichTextFilterItem) -> FormulaFilterItem {
        self.string = Some(item);
        self.clone()
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
    pub fn contains(&mut self, value: &str) -> MultiSelectFilterItem {
        self.contains = Some(value.to_string());
        self.clone()
    }

    pub fn does_not_contain(&mut self, value: &str) -> MultiSelectFilterItem {
        self.does_not_contain = Some(value.to_string());
        self.clone()
    }

    pub fn equals(&mut self) -> MultiSelectFilterItem {
        self.equals = Some(true);
        self.clone()
    }

    pub fn does_not_equal(&mut self) -> MultiSelectFilterItem {
        self.does_not_equal = Some(true);
        self.clone()
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
    pub fn equals(&mut self, value: u128) -> NumberFilterItem {
        self.equals = Some(value);
        self.clone()
    }

    pub fn does_not_equal(&mut self, value: u128) -> NumberFilterItem {
        self.does_not_equal = Some(value);
        self.clone()
    }

    pub fn greater_than(&mut self, value: u128) -> NumberFilterItem {
        self.greater_than = Some(value);
        self.clone()
    }

    pub fn less_than(&mut self, value: u128) -> NumberFilterItem {
        self.less_than = Some(value);
        self.clone()
    }

    pub fn greater_than_or_equal_to(&mut self, value: u128) -> NumberFilterItem {
        self.greater_than_or_equal_to = Some(value);
        self.clone()
    }

    pub fn less_than_or_equal_to(&mut self, value: u128) -> NumberFilterItem {
        self.less_than_or_equal_to = Some(value);
        self.clone()
    }

    pub fn is_empty(&mut self) -> NumberFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> NumberFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
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
    pub fn contains(&mut self, value: &str) -> PeopleFilterItem {
        self.contains = Some(value.to_string());
        self.clone()
    }

    pub fn does_not_contain(&mut self, value: &str) -> PeopleFilterItem {
        self.does_not_contain = Some(value.to_string());
        self.clone()
    }

    pub fn is_empty(&mut self) -> PeopleFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> PeopleFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
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
    pub fn contains(&mut self, value: &str) -> RelationFilterItem {
        self.contains = Some(value.to_string());
        self.clone()
    }

    pub fn does_not_contain(&mut self, value: &str) -> RelationFilterItem {
        self.does_not_contain = Some(value.to_string());
        self.clone()
    }

    pub fn is_empty(&mut self) -> RelationFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> RelationFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
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
    pub fn contains(&mut self, value: &str) -> RichTextFilterItem {
        self.contains = Some(value.to_string());
        self.clone()
    }

    pub fn does_not_contain(&mut self, value: &str) -> RichTextFilterItem {
        self.does_not_contain = Some(value.to_string());
        self.clone()
    }

    pub fn equals(&mut self, value: &str) -> RichTextFilterItem {
        self.equals = Some(value.to_string());
        self.clone()
    }

    pub fn does_not_equal(&mut self, value: &str) -> RichTextFilterItem {
        self.does_not_equal = Some(value.to_string());
        self.clone()
    }

    pub fn starts_with(&mut self, value: &str) -> RichTextFilterItem {
        self.starts_with = Some(value.to_string());
        self.clone()
    }

    pub fn ends_with(&mut self, value: &str) -> RichTextFilterItem {
        self.ends_with = Some(value.to_string());
        self.clone()
    }

    pub fn is_empty(&mut self) -> RichTextFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> RichTextFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
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
    pub fn equals(&mut self, value: &str) -> SelectFilterItem {
        self.equals = Some(value.to_string());
        self.clone()
    }

    pub fn does_not_equals(&mut self, value: &str) -> SelectFilterItem {
        self.does_not_equals = Some(value.to_string());
        self.clone()
    }

    pub fn is_empty(&mut self) -> SelectFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> SelectFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
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
    pub fn equals(&mut self, value: &str) -> StatusFilterItem {
        self.equals = Some(value.to_string());
        self.clone()
    }

    pub fn does_not_equals(&mut self, value: &str) -> StatusFilterItem {
        self.does_not_equals = Some(value.to_string());
        self.clone()
    }

    pub fn is_empty(&mut self) -> StatusFilterItem {
        self.is_empty = Some(true);
        self.clone()
    }

    pub fn is_not_empty(&mut self) -> StatusFilterItem {
        self.is_not_empty = Some(true);
        self.clone()
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
    pub fn timestamp(&mut self, value: &str) -> TimestampFilterItem {
        self.timestamp = Some(value.to_string());
        self.clone()
    }

    pub fn created_time(&mut self, item: DateFilterItem) -> TimestampFilterItem {
        self.created_time = Some(item);
        self.clone()
    }

    pub fn last_edited_time(&mut self, item: DateFilterItem) -> TimestampFilterItem {
        self.last_edited_time = Some(item);
        self.clone()
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
    pub fn equals(&mut self, value: u128) -> IdFilterItem {
        self.equals = Some(value);
        self.clone()
    }

    pub fn does_not_equal(&mut self, value: u128) -> IdFilterItem {
        self.does_not_equal = Some(value);
        self.clone()
    }

    pub fn greater_than(&mut self, value: u128) -> IdFilterItem {
        self.greater_than = Some(value);
        self.clone()
    }

    pub fn less_than(&mut self, value: u128) -> IdFilterItem {
        self.less_than = Some(value);
        self.clone()
    }

    pub fn greater_than_or_equal_to(&mut self, value: u128) -> IdFilterItem {
        self.greater_than_or_equal_to = Some(value);
        self.clone()
    }

    pub fn less_than_or_equal_to(&mut self, value: u128) -> IdFilterItem {
        self.less_than_or_equal_to = Some(value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseFilterItems {
    #[serde(default = "String::new", skip_serializing_if = "String::is_empty")]
    pub property: String,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<DatabaseFilterItems>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<DatabaseFilterItems>>,
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

impl DatabaseFilterItems {
    pub fn and(items: Vec<DatabaseFilterItems>) -> Self {
        DatabaseFilterItems {
            and: Some(items),
            ..Default::default()
        }
    }

    pub fn or(items: Vec<DatabaseFilterItems>) -> Self {
        DatabaseFilterItems {
            or: Some(items),
            ..Default::default()
        }
    }

    pub fn checkbox(property: &str, item: CheckboxFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            checkbox: Some(item),
            ..Default::default()
        }
    }

    pub fn date(property: &str, item: DateFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            date: Some(item),
            ..Default::default()
        }
    }

    pub fn files(property: &str, item: FilesFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            files: Some(item),
            ..Default::default()
        }
    }

    pub fn formula(property: &str, item: FormulaFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            formula: Some(item),
            ..Default::default()
        }
    }

    pub fn multi_select(property: &str, item: MultiSelectFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            multi_select: Some(item),
            ..Default::default()
        }
    }

    pub fn number(property: &str, item: NumberFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            number: Some(item),
            ..Default::default()
        }
    }

    pub fn people(property: &str, item: PeopleFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            people: Some(item),
            ..Default::default()
        }
    }

    pub fn relation(property: &str, item: RelationFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            relation: Some(item),
            ..Default::default()
        }
    }

    pub fn rich_text(property: &str, item: RichTextFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            rich_text: Some(item),
            ..Default::default()
        }
    }

    pub fn select(property: &str, item: SelectFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            select: Some(item),
            ..Default::default()
        }
    }

    pub fn status(property: &str, item: StatusFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            status: Some(item),
            ..Default::default()
        }
    }

    pub fn timestamp(property: &str, item: TimestampFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            timestamp: Some(item),
            ..Default::default()
        }
    }

    pub fn id(property: &str, item: IdFilterItem) -> Self {
        DatabaseFilterItems {
            property: property.to_string(),
            id: Some(item),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseFilter {
    pub filter: DatabaseFilterItems,
}

impl DatabaseFilter {
    pub fn new() -> Self {
        DatabaseFilter {
            filter: DatabaseFilterItems::default(),
        }
    }

    pub fn args(&mut self, item: DatabaseFilterItems) {
        self.filter = item;
    }

    pub fn and(&mut self, items: Vec<DatabaseFilterItems>) {
        self.filter.and = Some(items);
    }

    pub fn or(&mut self, items: Vec<DatabaseFilterItems>) {
        self.filter.or = Some(items);
    }

    pub fn build(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
