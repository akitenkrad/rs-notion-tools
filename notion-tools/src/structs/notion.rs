use crate::structs::block::*;
use crate::structs::common::*;
use crate::structs::database_properties::*;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionDatabase {
    #[serde(default = "String::new")]
    pub object: String,
    #[serde(default = "u32::default", skip_serializing)]
    pub status: u32,
    #[serde(default = "String::new", skip_serializing)]
    pub code: String,
    #[serde(default = "String::new", skip_serializing)]
    pub message: String,
    #[serde(default = "String::new", skip_serializing)]
    pub request_id: String,
    #[serde(default = "String::new")]
    pub id: String,
    #[serde(default = "String::new")]
    pub created_time: String,
    #[serde(default = "String::new")]
    pub last_edited_time: String,
    #[serde(default = "User::default")]
    pub created_by: User,
    #[serde(default = "User::default")]
    pub last_edited_by: User,
    #[serde(default = "String::new")]
    pub url: String,
    #[serde(default = "bool::default")]
    pub archived: bool,
    #[serde(default = "bool::default")]
    pub is_inline: bool,
    #[serde(default = "Option::default")]
    pub public_url: Option<String>,
    #[serde(default = "Vec::default")]
    pub title: Vec<RichText>,
    #[serde(default = "Vec::default")]
    pub description: Vec<RichText>,
    #[serde(default = "FxHashMap::default")]
    pub properties: FxHashMap<String, DatabasePropertySetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    #[serde(default = "Parent::default")]
    pub parent: Parent,
    #[serde(default = "FxHashMap::default")]
    pub properties: FxHashMap<String, DatabaseProperty>,
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
    pub fn from_properties(properties: FxHashMap<String, DatabaseProperty>) -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponse {
    #[serde(default = "String::new")]
    pub object: String,
    #[serde(default = "u32::default", skip_serializing)]
    pub status: u32,
    #[serde(default = "String::new", skip_serializing)]
    pub code: String,
    #[serde(default = "String::new", skip_serializing)]
    pub message: String,
    #[serde(default = "Vec::new")]
    pub results: Vec<Block>,
}
