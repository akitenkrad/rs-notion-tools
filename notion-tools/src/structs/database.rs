//! Database struct and its dependencies.
//!
//! This module defines the `Database` struct and its associated sub-structs used for representing
//! and manipulating database objects in the application. The primary struct, `Database`, contains
//! various fields that describe the properties and metadata of a database, such as its ID, creation
//! time, last edited time, URL, and more. Additionally, it includes nested structs to represent
//! specific property types within the database, such as `FormulaExpression`, `MultiSelectObject`,
//! `NumberFormat`, `RelationObject`, `RollupObject`, `SelectObject`, and `StatusObject`.
//!
//! Each sub-struct is equipped with serialization and deserialization capabilities using Serde,
//! and provides methods to check if the struct is empty. The `DatabaseProperty` struct is used to
//! define the properties of a database, with various fields representing different property types
//! and their corresponding data.
//!
use crate::structs::common::*;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormulaExpression {
    #[serde(default = "String::new")]
    pub expression: String,
}

impl FormulaExpression {
    pub fn is_empty(&self) -> bool {
        self.expression.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiSelectObject {
    #[serde(default = "Vec::default")]
    pub options: Vec<SelectOption>,
}

impl MultiSelectObject {
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumberFormat {
    #[serde(default = "String::new")]
    pub format: String,
}

impl NumberFormat {
    pub fn is_empty(&self) -> bool {
        self.format.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationObject {
    #[serde(default = "String::new")]
    pub database_id: String,
    #[serde(default = "String::new")]
    pub synced_property_id: String,
    #[serde(default = "String::new")]
    pub synced_property_name: String,
}

impl RelationObject {
    pub fn is_empty(&self) -> bool {
        self.database_id.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RollupObject {
    #[serde(default = "String::new")]
    pub rollup_property_id: String,
    #[serde(default = "String::new")]
    pub rollup_property_name: String,
    #[serde(default = "String::new")]
    pub relation_property_id: String,
    #[serde(default = "String::new")]
    pub relation_property_name: String,
    #[serde(default = "String::new")]
    pub function: String,
}

impl RollupObject {
    pub fn is_empty(&self) -> bool {
        self.rollup_property_id.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelectObject {
    #[serde(default = "Vec::default")]
    pub options: Vec<SelectOption>,
}

impl SelectObject {
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusObject {
    #[serde(default = "Vec::default")]
    pub options: Vec<SelectOption>,
}

impl StatusObject {
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseProperty {
    #[serde(default = "String::new")]
    pub id: String,
    #[serde(default = "String::new")]
    pub name: String,
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub checkbox: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub created_by: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub created_time: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub date: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub email: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub files: FxHashMap<String, String>,
    #[serde(
        default = "FormulaExpression::default",
        skip_serializing_if = "FormulaExpression::is_empty"
    )]
    pub formula: FormulaExpression,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub last_edited_by: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub last_edited_time: FxHashMap<String, String>,
    #[serde(
        default = "MultiSelectObject::default",
        skip_serializing_if = "MultiSelectObject::is_empty"
    )]
    pub multi_select: MultiSelectObject,
    #[serde(
        default = "NumberFormat::default",
        skip_serializing_if = "NumberFormat::is_empty"
    )]
    pub number: NumberFormat,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub people: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub phone_number: FxHashMap<String, String>,
    #[serde(
        default = "RelationObject::default",
        skip_serializing_if = "RelationObject::is_empty"
    )]
    pub relation: RelationObject,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub rich_text: FxHashMap<String, String>,
    #[serde(
        default = "RollupObject::default",
        skip_serializing_if = "RollupObject::is_empty"
    )]
    pub rollup: RollupObject,
    #[serde(
        default = "SelectObject::default",
        skip_serializing_if = "SelectObject::is_empty"
    )]
    pub select: SelectObject,
    #[serde(
        default = "StatusObject::default",
        skip_serializing_if = "StatusObject::is_empty"
    )]
    pub status: StatusObject,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub title: FxHashMap<String, String>,
    #[serde(
        default = "FxHashMap::default",
        skip_serializing_if = "FxHashMap::is_empty"
    )]
    pub url: FxHashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
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
    pub properties: FxHashMap<String, DatabaseProperty>,
}
