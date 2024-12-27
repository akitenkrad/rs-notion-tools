use crate::structs::common::*;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rollup {
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "Option::default")]
    pub array: Option<Vec<PageProperty>>,
    #[serde(default = "Option::default")]
    pub date: Option<Date>,
    #[serde(default = "Option::default")]
    pub incomplete: Option<bool>,
    #[serde(default = "Option::default")]
    pub number: Option<f64>,
    #[serde(default = "Option::default")]
    pub unsupported: Option<String>,
    #[serde(default = "Option::default")]
    pub function: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageProperty {
    // properties
    #[serde(default = "String::new", skip_serializing)]
    pub id: String,
    #[serde(default = "String::new", skip_serializing)]
    pub name: String,
    #[serde(rename = "type", default = "String::new", skip_serializing)]
    pub type_name: String,
    #[serde(default = "Option::default", skip_serializing)]
    pub created_by: Option<User>,
    #[serde(default = "Option::default", skip_serializing)]
    pub created_time: Option<String>,
    #[serde(default = "Option::default", skip_serializing)]
    pub last_edited_by: Option<User>,
    #[serde(default = "Option::default", skip_serializing)]
    pub last_edited_time: Option<String>,

    // fields
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub checkbox: Option<bool>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub formula: Option<Formula>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub multi_select: Option<Vec<SelectOption>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<User>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub relation: Option<Vec<Relation>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<Vec<RichText>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub rollup: Option<Rollup>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub select: Option<SelectOption>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub status: Option<SelectOption>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<RichText>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<UniqueId>,
}

impl PageProperty {
    pub fn checkbox(value: bool) -> Self {
        let mut prop = PageProperty::default();
        prop.checkbox = Some(value);
        return prop;
    }

    pub fn date(value: &str) -> Self {
        let mut prop = PageProperty::default();
        prop.date = Some(Date {
            start: value.to_string(),
        });
        return prop;
    }

    pub fn email(value: &str) -> Self {
        let mut prop = PageProperty::default();
        prop.email = Some(Email {
            email: value.to_string(),
        });
        return prop;
    }

    pub fn multi_select(value: Vec<&str>) -> Self {
        let mut prop = PageProperty::default();
        let mut options = Vec::new();
        value.iter().for_each(|v| {
            options.push(SelectOption {
                id: "".to_string(),
                name: v.to_string(),
                color: Color::Default,
            });
        });
        prop.multi_select = Some(options);
        return prop;
    }

    pub fn number(value: f64) -> Self {
        let mut prop = PageProperty::default();
        prop.number = Some(value);
        return prop;
    }

    pub fn phone_number(value: &str) -> Self {
        let mut prop = PageProperty::default();
        prop.phone_number = Some(PhoneNumber {
            phone_number: value.to_string(),
        });
        return prop;
    }

    pub fn relation(value: Vec<&str>) -> Self {
        let mut prop = PageProperty::default();
        let mut relations = Vec::new();
        value.iter().for_each(|v| {
            relations.push(Relation { id: v.to_string() });
        });
        prop.relation = Some(relations);
        return prop;
    }

    pub fn rich_text(value: Vec<RichText>) -> Self {
        let mut prop = PageProperty::default();
        prop.rich_text = Some(value);
        return prop;
    }

    pub fn select(value: &str) -> Self {
        let mut prop = PageProperty::default();
        prop.select = Some(SelectOption {
            id: "".to_string(),
            name: value.to_string(),
            color: Color::Default,
        });
        return prop;
    }

    pub fn status(value: &str) -> Self {
        let mut prop = PageProperty::default();
        prop.status = Some(SelectOption {
            id: "".to_string(),
            name: value.to_string(),
            color: Color::Default,
        });
        return prop;
    }
    pub fn title(value: RichText) -> Self {
        let mut prop = PageProperty::default();
        prop.title = Some(vec![value]);
        return prop;
    }
    pub fn url(value: &str) -> Self {
        let mut prop = PageProperty::default();
        prop.url = Some(value.to_string());
        return prop;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    #[serde(default = "Parent::default")]
    pub parent: Parent,
    #[serde(default = "FxHashMap::default")]
    pub properties: FxHashMap<String, PageProperty>,
    #[serde(default = "String::new", skip_serializing)]
    pub object: String,
    #[serde(default = "String::new", skip_serializing)]
    pub id: String,
    #[serde(default = "String::new", skip_serializing)]
    pub created_time: String,
    #[serde(default = "String::new", skip_serializing)]
    pub last_edited_time: String,
    #[serde(default = "User::default", skip_serializing)]
    pub created_by: User,
    #[serde(default = "User::default", skip_serializing)]
    pub last_edited_by: User,
    #[serde(default = "bool::default", skip_serializing_if = "std::ops::Not::not")]
    pub archived: bool,
    #[serde(default = "bool::default", skip_serializing)]
    pub in_trash: bool,
    #[serde(default = "Option::default", skip_serializing)]
    pub next_cursor: Option<String>,
    #[serde(default = "bool::default", skip_serializing)]
    pub has_more: bool,
    #[serde(rename = "type", default = "String::new", skip_serializing)]
    pub type_name: String,
    #[serde(default = "u32::default", skip_serializing)]
    pub status: u32,
    #[serde(default = "String::new", skip_serializing)]
    pub code: String,
    #[serde(default = "String::new", skip_serializing)]
    pub message: String,
}

impl Default for Page {
    fn default() -> Self {
        let page = Page {
            parent: Parent {
                type_name: ParentType::Database,
                database_id: None,
                page_id: None,
                workspace_id: None,
                block_id: None,
            },
            properties: FxHashMap::default(),
            object: "page".to_string(),
            id: "".to_string(),
            created_time: "".to_string(),
            last_edited_time: "".to_string(),
            created_by: User::default(),
            last_edited_by: User::default(),
            archived: false,
            in_trash: false,
            next_cursor: None,
            has_more: false,
            type_name: "page".to_string(),
            status: 200,
            code: "".to_string(),
            message: "".to_string(),
        };
        return page;
    }
}

impl Page {
    pub fn from_properties(properties: FxHashMap<String, PageProperty>) -> Self {
        let mut page = Page::default();
        page.properties = properties;
        return page;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse {
    #[serde(default = "String::new")]
    pub object: String,
    #[serde(default = "u32::default", skip_serializing)]
    pub status: u32,
    #[serde(default = "String::new", skip_serializing)]
    pub code: String,
    #[serde(default = "String::new", skip_serializing)]
    pub message: String,
    #[serde(default = "Vec::new")]
    pub results: Vec<Page>,
}