use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Color {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "blue_background")]
    BlueBackground,
    #[serde(rename = "brown")]
    Brown,
    #[serde(rename = "brown_background")]
    BrownBackground,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "gray_background")]
    GrayBackground,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "green_background")]
    GreenBackground,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "orange_background")]
    OrangeBackground,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "yellow_background")]
    YellowBackground,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "pink_background")]
    PinkBackground,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "purple_background")]
    PurpleBackground,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "red_background")]
    RedBackground,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Icon {
    #[serde(default = "String::new")]
    pub emoji: String,
}

impl Icon {
    pub fn from_str(emoji: &str) -> Self {
        let icon = Icon {
            emoji: emoji.to_string(),
        };
        return icon;
    }
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
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub caption: Vec<RichText>,
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
pub struct SelectOption {
    #[serde(default = "String::new", skip_serializing)]
    pub id: String,
    #[serde(default = "String::new")]
    pub name: String,
    #[serde(default = "Color::default", skip_serializing)]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Relation {
    #[serde(default = "String::new")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UniqueId {
    #[serde(default = "u128::default")]
    pub number: u128,
    #[serde(default = "String::new")]
    pub prefix: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParentType {
    #[serde(rename = "database_id")]
    Database,
    #[serde(rename = "page_id")]
    Page,
    #[serde(rename = "workspace_id")]
    Workspace,
    #[serde(rename = "block_id")]
    Block,
}

impl Default for ParentType {
    fn default() -> Self {
        return ParentType::Database;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Parent {
    #[serde(rename = "type", default = "ParentType::default")]
    pub type_name: ParentType,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Parent {
    fn database(database_id: &str) -> Self {
        let parent = Parent {
            type_name: ParentType::Database,
            database_id: Some(database_id.to_string()),
            page_id: None,
            workspace_id: None,
            block_id: None,
        };
        return parent;
    }

    fn page(page_id: &str) -> Self {
        let parent = Parent {
            type_name: ParentType::Page,
            database_id: None,
            page_id: Some(page_id.to_string()),
            workspace_id: None,
            block_id: None,
        };
        return parent;
    }

    fn workspace(workspace_id: &str) -> Self {
        let parent = Parent {
            type_name: ParentType::Workspace,
            database_id: None,
            page_id: None,
            workspace_id: Some(workspace_id.to_string()),
            block_id: None,
        };
        return parent;
    }

    fn block(block_id: &str) -> Self {
        let parent = Parent {
            type_name: ParentType::Block,
            database_id: None,
            page_id: None,
            workspace_id: None,
            block_id: Some(block_id.to_string()),
        };
        return parent;
    }

    pub fn from(parent_type: ParentType, parent_id: &str) -> Self {
        match parent_type {
            ParentType::Database => return Parent::database(parent_id),
            ParentType::Page => return Parent::page(parent_id),
            ParentType::Workspace => return Parent::workspace(parent_id),
            ParentType::Block => return Parent::block(parent_id),
        }
    }
}

// ------ Rich Text ------
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
