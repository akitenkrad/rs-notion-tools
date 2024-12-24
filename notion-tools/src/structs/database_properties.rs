use core::str;

use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(default = "String::new")]
    pub object: String,
    #[serde(default = "String::new")]
    pub id: String,
    #[serde(default = "String::new")]
    pub name: String,
    #[serde(default = "Option::default")]
    pub avatar_url: Option<String>,
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "Email::default")]
    pub person: Email,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Date {
    #[serde(default = "String::new")]
    pub start: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Email {
    #[serde(default = "String::new")]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalUrl {
    #[serde(default = "String::new")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct File {
    #[serde(default = "String::new")]
    pub name: String,
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "ExternalUrl::default")]
    pub external: ExternalUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Formula {
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "bool::default")]
    pub boolean: bool,
    #[serde(default = "String::new")]
    pub date: String,
    #[serde(default = "f64::default")]
    pub number: f64,
    #[serde(default = "String::new")]
    pub string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhoneNumber {
    #[serde(default = "String::new")]
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Relation {
    #[serde(default = "String::new")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextObject {
    #[serde(default = "String::new")]
    pub content: String,
    #[serde(default = "Option::default")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationObject {
    #[serde(default = "bool::default")]
    pub bold: bool,
    #[serde(default = "bool::default")]
    pub italic: bool,
    #[serde(default = "bool::default")]
    pub strikethrough: bool,
    #[serde(default = "bool::default")]
    pub underline: bool,
    #[serde(default = "bool::default")]
    pub code: bool,
    #[serde(default = "String::new")]
    pub color: String,
}

impl Default for AnnotationObject {
    fn default() -> Self {
        let annotation = AnnotationObject {
            bold: false,
            italic: false,
            strikethrough: false,
            underline: false,
            code: false,
            color: "default".to_string(),
        };
        return annotation;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichText {
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "TextObject::default")]
    pub text: TextObject,
    #[serde(default = "AnnotationObject::default")]
    pub annotations: AnnotationObject,
    #[serde(default = "String::new")]
    pub plain_text: String,
    #[serde(default = "Option::default")]
    pub href: Option<String>,
}

impl Default for RichText {
    fn default() -> Self {
        let mut text = TextObject::default();
        text.content = "".to_string();
        let annotations = AnnotationObject::default();
        let rich_text = RichText {
            type_name: "text".to_string(),
            text,
            annotations,
            plain_text: "".to_string(),
            href: None,
        };
        return rich_text;
    }
}

impl RichText {
    pub fn from_str(value: &str) -> Self {
        let mut text = TextObject::default();
        text.content = value.to_string();
        let annotations = AnnotationObject::default();
        let mut rich_text = RichText::default();
        rich_text.type_name = "text".to_string();
        rich_text.text = text;
        rich_text.annotations = annotations;
        rich_text.plain_text = value.to_string();
        return rich_text;
    }

    pub fn from_str_with_annotations(
        value: &str,
        bold: bool,
        italic: bool,
        underline: bool,
        strikethrough: bool,
        code: bool,
        color: String,
    ) -> Self {
        let mut text = TextObject::default();
        text.content = value.to_string();
        let annotations = AnnotationObject {
            bold,
            italic,
            underline,
            strikethrough,
            code,
            color,
        };
        let mut rich_text = RichText::default();
        rich_text.text = text;
        rich_text.annotations = annotations;
        return rich_text;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rollup {
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "Option::default")]
    pub array: Option<Vec<DatabaseProperty>>,
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
pub struct UniqueId {
    #[serde(default = "u128::default")]
    pub number: u128,
    #[serde(default = "String::new")]
    pub prefix: String,
}

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
pub struct SelectOption {
    #[serde(default = "String::new", skip_serializing)]
    pub id: String,
    #[serde(default = "String::new")]
    pub name: String,
    #[serde(default = "String::new", skip_serializing)]
    pub color: String,
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
pub struct DatabasePropertySetting {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseProperty {
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

impl DatabaseProperty {
    pub fn checkbox(value: bool) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.checkbox = Some(value);
        return prop;
    }

    pub fn date(value: &str) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.date = Some(Date {
            start: value.to_string(),
        });
        return prop;
    }

    pub fn email(value: &str) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.email = Some(Email {
            email: value.to_string(),
        });
        return prop;
    }

    pub fn multi_select(value: Vec<&str>) -> Self {
        let mut prop = DatabaseProperty::default();
        let mut options = Vec::new();
        value.iter().for_each(|v| {
            options.push(SelectOption {
                id: "".to_string(),
                name: v.to_string(),
                color: "".to_string(),
            });
        });
        prop.multi_select = Some(options);
        return prop;
    }

    pub fn number(value: f64) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.number = Some(value);
        return prop;
    }

    pub fn phone_number(value: &str) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.phone_number = Some(PhoneNumber {
            phone_number: value.to_string(),
        });
        return prop;
    }

    pub fn relation(value: Vec<&str>) -> Self {
        let mut prop = DatabaseProperty::default();
        let mut relations = Vec::new();
        value.iter().for_each(|v| {
            relations.push(Relation { id: v.to_string() });
        });
        prop.relation = Some(relations);
        return prop;
    }

    pub fn rich_text(value: Vec<RichText>) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.rich_text = Some(value);
        return prop;
    }

    pub fn select(value: &str) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.select = Some(SelectOption {
            id: "".to_string(),
            name: value.to_string(),
            color: "".to_string(),
        });
        return prop;
    }

    pub fn status(value: &str) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.status = Some(SelectOption {
            id: "".to_string(),
            name: value.to_string(),
            color: "".to_string(),
        });
        return prop;
    }
    pub fn title(value: RichText) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.title = Some(vec![value]);
        return prop;
    }
    pub fn url(value: &str) -> Self {
        let mut prop = DatabaseProperty::default();
        prop.url = Some(value.to_string());
        return prop;
    }
}
