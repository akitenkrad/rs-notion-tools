use crate::structs::common::*;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bookmark {
    #[serde(default = "Vec::default")]
    pub caption: Vec<RichText>,
    #[serde(default = "String::new")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BulletedListItem {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Color::default")]
    pub color: Color,
    #[serde(default = "Vec::default")]
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Callout {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Option::default")]
    pub icon: Option<Icon>,
    #[serde(default = "Color::default")]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChildDatabase {
    #[serde(default = "String::new")]
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChildPage {
    #[serde(default = "String::new")]
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Code {
    #[serde(default = "Vec::default")]
    pub caption: Vec<RichText>,
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "String::new")]
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Embed {
    #[serde(default = "String::new")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Equation {
    #[serde(default = "String::new")]
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Heading {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Color::default")]
    pub color: Color,
    #[serde(default = "bool::default")]
    pub is_toggleable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Image {
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "ExternalUrl::default")]
    pub external: ExternalUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkPreview {
    #[serde(default = "String::new")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumberedListItem {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Color::default")]
    pub color: Color,
    #[serde(default = "Vec::default")]
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pdf {
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "ExternalUrl::default")]
    pub external: ExternalUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Quote {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Color::default")]
    pub color: Color,
    #[serde(default = "Vec::default")]
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paragraph {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Color::default")]
    pub color: Color,
    #[serde(default = "Vec::default")]
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Table {
    #[serde(default = "u32::default")]
    pub table_width: u32,
    #[serde(default = "bool::default")]
    pub has_column_header: bool,
    #[serde(default = "bool::default")]
    pub has_row_header: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableRow {
    #[serde(default = "Vec::default")]
    pub cells: Vec<RichText>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableOfContents {
    #[serde(default = "Color::default")]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToDo {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Color::default")]
    pub color: Color,
    #[serde(default = "bool::default")]
    pub checked: bool,
    #[serde(default = "Vec::default")]
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToggleBlock {
    #[serde(default = "Vec::default")]
    pub rich_text: Vec<RichText>,
    #[serde(default = "Color::default")]
    pub color: Color,
    #[serde(default = "Vec::default")]
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Video {
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "ExternalUrl::default")]
    pub external: ExternalUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // base properties
    #[serde(default = "Parent::default", skip_serializing)]
    pub parent: Parent,
    #[serde(default = "String::new")]
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
    #[serde(rename = "type", default = "String::new")]
    pub type_name: String,
    #[serde(default = "bool::default", skip_serializing)]
    pub has_children: bool,
    #[serde(default = "bool::default", skip_serializing)]
    pub is_togglable: bool,

    // type specific properties
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub bookmark: Option<Bookmark>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub breadcrumb: Option<FxHashMap<String, String>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub bulleted_list_item: Option<BulletedListItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub callout: Option<Callout>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub child_database: Option<ChildDatabase>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub child_page: Option<ChildPage>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub column_list: Option<FxHashMap<String, String>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub column: Option<FxHashMap<String, String>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub divider: Option<FxHashMap<String, String>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub equation: Option<Equation>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub heading_1: Option<Heading>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub heading_2: Option<Heading>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub heading_3: Option<Heading>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub link_preview: Option<LinkPreview>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub numbered_list_item: Option<NumberedListItem>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<Paragraph>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub pdf: Option<Pdf>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Quote>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub table_row: Option<TableRow>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub table_of_contents: Option<TableOfContents>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub to_do: Option<ToDo>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub toggle: Option<ToggleBlock>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
}

impl Default for Block {
    fn default() -> Self {
        let block = Block {
            parent: Parent {
                type_name: ParentType::Block,
                database_id: None,
                page_id: None,
                workspace_id: None,
                block_id: None,
            },
            object: "block".to_string(),
            id: "".to_string(),
            created_time: "".to_string(),
            last_edited_time: "".to_string(),
            created_by: User::default(),
            last_edited_by: User::default(),
            archived: false,
            in_trash: false,
            type_name: "".to_string(),
            has_children: false,
            is_togglable: false,
            bookmark: None,
            breadcrumb: None,
            bulleted_list_item: None,
            callout: None,
            child_database: None,
            child_page: None,
            code: None,
            column_list: None,
            column: None,
            divider: None,
            embed: None,
            equation: None,
            file: None,
            heading_1: None,
            heading_2: None,
            heading_3: None,
            image: None,
            link_preview: None,
            numbered_list_item: None,
            paragraph: None,
            pdf: None,
            quote: None,
            table: None,
            table_row: None,
            table_of_contents: None,
            to_do: None,
            toggle: None,
            video: None,
        };
        return block;
    }
}

impl Block {
    pub fn bookmark(
        parent_type: ParentType,
        parent_id: String,
        caption: String,
        url: String,
    ) -> Self {
        let bookmark = Bookmark {
            caption: vec![RichText::from_str(caption)],
            url: url.to_string(),
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "bookmark".to_string(),
            bookmark: Some(bookmark),
            ..Default::default()
        };
        return block;
    }

    pub fn breadcrumb(parent_type: ParentType, parent_id: String) -> Self {
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "breadcrumb".to_string(),
            breadcrumb: Some(FxHashMap::default()),
            ..Default::default()
        };
        return block;
    }

    pub fn bulleted_list_item(
        parent_type: ParentType,
        parent_id: String,
        texts: Vec<String>,
    ) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let bulleted_list_item = BulletedListItem {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "bulleted_list_item".to_string(),
            bulleted_list_item: Some(bulleted_list_item),
            ..Default::default()
        };
        return block;
    }

    pub fn callout(
        parent_type: ParentType,
        parent_id: String,
        text: String,
        icon: Icon,
        color: Color,
    ) -> Self {
        let rich_text = vec![RichText::from_str(text)];
        let callout = Callout {
            rich_text: rich_text,
            icon: Some(icon),
            color: color,
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "callout".to_string(),
            callout: Some(callout),
            ..Default::default()
        };
        return block;
    }

    pub fn child_database(parent_type: ParentType, parent_id: String, title: String) -> Self {
        let child_database = ChildDatabase {
            title: title.to_string(),
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "child_database".to_string(),
            child_database: Some(child_database),
            ..Default::default()
        };
        return block;
    }

    pub fn child_page(parent_type: ParentType, parent_id: String, title: String) -> Self {
        let child_page = ChildPage {
            title: title.to_string(),
        };
        let block = Block {
            type_name: "child_page".to_string(),
            child_page: Some(child_page),
            parent: Parent::from(parent_type, parent_id),
            ..Default::default()
        };
        return block;
    }

    pub fn code(
        parent_type: ParentType,
        parent_id: String,
        caption: String,
        language: String,
        texts: Vec<String>,
    ) -> Self {
        let caption = vec![RichText::from_str(caption)];
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let code = Code {
            caption: caption,
            rich_text: texts,
            language: language.to_string(),
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "code".to_string(),
            code: Some(code),
            ..Default::default()
        };
        return block;
    }

    pub fn column_list(parent_type: ParentType, parent_id: String) -> Self {
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "column_list".to_string(),
            column_list: Some(FxHashMap::default()),
            ..Default::default()
        };
        return block;
    }

    pub fn column(parent_type: ParentType, parent_id: String) -> Self {
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "column".to_string(),
            column: Some(FxHashMap::default()),
            ..Default::default()
        };
        return block;
    }

    pub fn divider(parent_type: ParentType, parent_id: String) -> Self {
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "divider".to_string(),
            divider: Some(FxHashMap::default()),
            ..Default::default()
        };
        return block;
    }

    pub fn embed(parent_type: ParentType, parent_id: String, url: String) -> Self {
        let embed = Embed {
            url: url.to_string(),
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "embed".to_string(),
            embed: Some(embed),
            ..Default::default()
        };
        return block;
    }

    pub fn equation(parent_type: ParentType, parent_id: String, expression: String) -> Self {
        let equation = Equation {
            expression: expression.to_string(),
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "equation".to_string(),
            equation: Some(equation),
            ..Default::default()
        };
        return block;
    }

    pub fn file(parent_type: ParentType, parent_id: String) -> Self {
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "file".to_string(),
            file: Some(File::default()),
            ..Default::default()
        };
        return block;
    }

    pub fn heading_1(parent_type: ParentType, parent_id: String, texts: Vec<String>) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let heading = Heading {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "heading_1".to_string(),
            heading_1: Some(heading),
            ..Default::default()
        };
        return block;
    }

    pub fn heading_2(parent_type: ParentType, parent_id: String, texts: Vec<String>) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let heading = Heading {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "heading_2".to_string(),
            heading_2: Some(heading),
            ..Default::default()
        };
        return block;
    }

    pub fn heading_3(parent_type: ParentType, parent_id: String, texts: Vec<String>) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let heading = Heading {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "heading_3".to_string(),
            heading_3: Some(heading),
            ..Default::default()
        };
        return block;
    }

    pub fn image(parent_type: ParentType, parent_id: String, url: String) -> Self {
        let external = ExternalUrl {
            url: url.to_string(),
        };
        let image = Image {
            type_name: "external".to_string(),
            external: external,
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "image".to_string(),
            image: Some(image),
            ..Default::default()
        };
        return block;
    }

    pub fn link_preview(parent_type: ParentType, parent_id: String, url: String) -> Self {
        let link_preview = LinkPreview {
            url: url.to_string(),
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "link_preview".to_string(),
            link_preview: Some(link_preview),
            ..Default::default()
        };
        return block;
    }

    pub fn numbered_list_item(
        parent_type: ParentType,
        parent_id: String,
        texts: Vec<String>,
    ) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let numbered_list_item = NumberedListItem {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "numbered_list_item".to_string(),
            numbered_list_item: Some(numbered_list_item),
            ..Default::default()
        };
        return block;
    }

    pub fn paragraph(parent_type: ParentType, parent_id: String, texts: Vec<String>) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let paragraph = Paragraph {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "paragraph".to_string(),
            paragraph: Some(paragraph),
            ..Default::default()
        };
        return block;
    }

    pub fn pdf(parent_type: ParentType, parent_id: String, url: String) -> Self {
        let external = ExternalUrl {
            url: url.to_string(),
        };
        let pdf = Pdf {
            type_name: "external".to_string(),
            external: external,
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "pdf".to_string(),
            pdf: Some(pdf),
            ..Default::default()
        };
        return block;
    }

    pub fn quote(parent_type: ParentType, parent_id: String, texts: Vec<String>) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let quote = Quote {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "quote".to_string(),
            quote: Some(quote),
            ..Default::default()
        };
        return block;
    }

    pub fn table(parent_type: ParentType, parent_id: String) -> Self {
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "table".to_string(),
            table: Some(Table::default()),
            ..Default::default()
        };
        return block;
    }

    pub fn table_row(parent_type: ParentType, parent_id: String, cells: Vec<String>) -> Self {
        let cells = cells
            .iter()
            .map(|cell| RichText::from_str(cell.to_string()))
            .collect::<Vec<RichText>>();
        let table_row = TableRow { cells: cells };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "table_row".to_string(),
            table_row: Some(table_row),
            ..Default::default()
        };
        return block;
    }

    pub fn table_of_contents(parent_type: ParentType, parent_id: String) -> Self {
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "table_of_contents".to_string(),
            table_of_contents: Some(TableOfContents::default()),
            ..Default::default()
        };
        return block;
    }

    pub fn to_do(
        parent_type: ParentType,
        parent_id: String,
        texts: Vec<String>,
        checked: bool,
    ) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let to_do = ToDo {
            rich_text: texts,
            checked: checked,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "to_do".to_string(),
            to_do: Some(to_do),
            ..Default::default()
        };
        return block;
    }

    pub fn toggle_blocks(parent_type: ParentType, parent_id: String, texts: Vec<String>) -> Self {
        let texts = texts
            .iter()
            .map(|text| RichText::from_str(text.to_string()))
            .collect::<Vec<RichText>>();
        let toggle = ToggleBlock {
            rich_text: texts,
            ..Default::default()
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "toggle".to_string(),
            toggle: Some(toggle),
            ..Default::default()
        };
        return block;
    }

    pub fn video(parent_type: ParentType, parent_id: String, url: String) -> Self {
        let external = ExternalUrl {
            url: url.to_string(),
        };
        let video = Video {
            type_name: "external".to_string(),
            external: external,
        };
        let block = Block {
            parent: Parent::from(parent_type, parent_id),
            type_name: "video".to_string(),
            video: Some(video),
            ..Default::default()
        };
        return block;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockBody {
    #[serde(default = "Vec::default")]
    pub children: Vec<Block>,
}
