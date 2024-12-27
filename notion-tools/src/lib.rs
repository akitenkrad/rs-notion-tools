//! # Notion Tools
//!
//! `notion-tools` is a library for interacting with the Notion API. It provides a convenient way to
//! perform various operations such as retrieving databases, querying databases, creating pages,
//! updating pages, archiving pages, and appending block children.
//!
//! ## Usage
//!
//! To use this library, you need to set the `NOTION_API_KEY` and `NOTION_DATABASE_ID` environment
//! variables. The `NOTION_API_KEY` is required for authentication, while the `NOTION_DATABASE_ID`
//! is optional and can be set later using the `database` method.
//!
//! ## Implemented endpoints
//! | Endpoint | Implemented | Code |
//! |---|:---:|---|
//! | [Create a Token](https://developers.notion.com/reference/create-a-token) | - | |
//! | [Append block children](https://developers.notion.com/reference/patch-block-children) | ✅ | [`Notion::append_block_children`](Notion) |
//! | [Retrieve a block](https://developers.notion.com/reference/retrieve-a-block) | - | |
//! | [Retrieve block children](https://developers.notion.com/reference/get-block-children) | - | |
//! | [Update a block](https://developers.notion.com/reference/update-a-block) | - | |
//! | [Delete a block](https://developers.notion.com/reference/delete-a-block) | - | |
//! | [Create a page](https://developers.notion.com/reference/post-page) | ✅ | [`Notion::create_a_page`](Notion) |
//! | [Retrieve a page](https://developers.notion.com/reference/retrieve-a-page) | - | |
//! | [Retrieve a page property item](https://developers.notion.com/reference/retrieve-a-page-property-item) | - | |
//! | [Update page properties](https://developers.notion.com/reference/patch-page) | ✅ | [`Notion::update_a_page`](Notion) |
//! | [Archive a page](https://developers.notion.com/reference/archive-a-page) | ✅ | [`Notion::archive_a_page`](Notion) |
//! | [Create a database](https://developers.notion.com/reference/create-a-database) | - | |
//! | [Query a database](https://developers.notion.com/reference/post-database-query) | ✅ | [`Notion::query_database`](Notion) |
//! | [Retrieve a database](https://developers.notion.com/reference/retrieve-a-database) | ✅ | [`Notion::retrieve_a_database`](Notion) |
//! | [Update a database](https://developers.notion.com/reference/update-a-database) | - | |
//! | [List all users](https://developers.notion.com/reference/get-users) | - | |
//! | [Retrieve a user](https://developers.notion.com/reference/get-user) | - | |
//! | [Retrieve your token's bot user](https://developers.notion.com/reference/get-self) | - | |
//! | [Create comment](https://developers.notion.com/reference/create-a-comment) | - | |
//! | [Retrieve comments](https://developers.notion.com/reference/retrieve-a-comment) | - | |
//! | [Search by title](https://developers.notion.com/reference/post-search) | - | |
//!
//! ## Build a query filter
//! The `QueryFilter` struct is used to build a query filter for querying a database. The `QueryFilter`
//! struct provides methods for building a filter that can be used to query a database.
//! See the [`QueryFilter`] struct for more information.
//!
//! ## Examples
//!
//! ### Create a page
//!
//! ```rust
//! # use anyhow::Result;
//! # use notion_tools::Notion;
//! # use notion_tools::structs::page::*;
//! # use notion_tools::structs::common::*;
//! # use fxhash::FxHashMap;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let notion = Notion::new();
//!
//! // Create a page
//! let mut properties: FxHashMap<String, PageProperty> = FxHashMap::default();
//! properties.insert(
//!     "Name".to_string(),
//!     PageProperty::title(RichText::from_str("Sample Page")),
//! );
//! properties.insert(
//!     "Title".to_string(),
//!     PageProperty::rich_text(vec![RichText::from_str("Sample Page")]),
//! );
//! properties.insert("Status".to_string(), PageProperty::status("ToDo"));
//! let mut page = Page::from_properties(properties);
//! page.parent.type_name = ParentType::Database;
//! page.parent.database_id = Some(notion.database_id.clone());
//!
//! let response = notion.create_a_page(&page).await;
//! println!("{:?}", response);
//! # Ok(())
//! # }
//! ```
//!
//! ### Query a database
//!
//! ```rust
//! # use anyhow::Result;
//! # use notion_tools::Notion;
//! # use notion_tools::structs::query_filter::*;
//! # use notion_tools::structs::page::*;
//! # use notion_tools::structs::common::*;
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut notion = Notion::new();
//! notion.database("your_database_id");
//!
//! // Build a query filter
//! let mut filter = QueryFilter::new();
//! filter.args(FilterItem::status(
//!     "Status",
//!     StatusFilterItem::equals("ToDo"),
//! ));
//! // Query a database
//! let response = notion.query_database(filter).await?;
//! println!("{:?}", response);
//! #     Ok(())
//! # }
//! ```
//!
pub mod structs;

use crate::structs::block::*;
use crate::structs::common::*;
use crate::structs::database::*;
use crate::structs::page::*;
use crate::structs::query_filter::*;
use anyhow::Result;
use dotenvy::dotenv;
use reqwest as request;

/// Notion API client
#[derive(Debug)]
pub struct Notion {
    /// Notion API key: set from the `NOTION_API_KEY` environment variable
    pub api_key: String,
    /// Notion database ID: set from the `NOTION_DATABASE_ID` environment variable
    pub database_id: String,
}

impl Notion {
    /// Create a new Notion API client.  
    /// environment variables are read from the `.env` file.
    pub fn new() -> Self {
        dotenv().ok();
        let api_key = std::env::var("NOTION_API_KEY").expect("NOTION_API_KEY must be set");
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or("".to_string());

        Notion {
            api_key,
            database_id,
        }
    }

    /// Set your database ID
    pub fn database(&mut self, database_id: &str) -> &mut Self {
        self.database_id = database_id.to_string();
        return self;
    }

    /// # Retrieve a database properties  
    /// ## Return
    /// - [`Database`] struct
    pub async fn retrieve_a_database(&self) -> Result<Database> {
        let url = format!("https://api.notion.com/v1/databases/{}", self.database_id);
        let client = request::Client::new();
        let content = client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .send()
            .await?
            .text()
            .await?;

        let mut database = serde_json::from_str::<Database>(&content)?;
        if database.status == 0 {
            database.status = 200;
        }

        return Ok(database);
    }

    /// # Query a database  
    /// ## Arguments:  
    /// - filter: [`QueryFilter`]
    /// ## Return:  
    /// - [`PageResponse`] struct
    pub async fn query_database(&self, filter: QueryFilter) -> Result<PageResponse> {
        let url = format!(
            "https://api.notion.com/v1/databases/{}/query",
            self.database_id
        );
        let query = filter.build();
        let client = request::Client::new();
        let content = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        let mut response = serde_json::from_str::<PageResponse>(&content)?;
        if response.status == 0 {
            response.status = 200;
        }

        return Ok(response);
    }

    /// # Create a page
    /// ## Arguments:
    /// - page: [`Page`] struct
    /// ## Return:
    /// - [`Page`] struct
    pub async fn create_a_page(&self, page: &Page) -> Result<Page> {
        let url = "https://api.notion.com/v1/pages";
        let client = request::Client::new();
        let data = serde_json::to_string(page)?;
        let content = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .body(data)
            .send()
            .await?
            .text()
            .await?;

        let mut page = serde_json::from_str::<Page>(&content)?;
        if page.status == 0 {
            page.status = 200;
        }
        return Ok(page);
    }

    /// # Update a page
    /// ## Arguments:
    /// - page_id: &str
    /// - page: [`Page`] struct
    /// ## Return:
    /// - [`Page`] struct
    pub async fn update_a_page(&self, page_id: &str, page: &Page) -> Result<Page> {
        let url = format!("https://api.notion.com/v1/pages/{}", page_id);
        let client = request::Client::new();
        let data = serde_json::to_string(page)?;
        let content = client
            .patch(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .body(data)
            .send()
            .await?
            .text()
            .await?;

        let mut page = serde_json::from_str::<Page>(&content)?;
        if page.status == 0 {
            page.status = 200;
        }
        return Ok(page);
    }

    /// # Archive a page
    /// ## Arguments:
    /// - page_id: &str
    /// - parent_id: &str
    /// - parent_type: [`ParentType`]
    /// ## Return:
    /// - [`Page`] struct
    /// ## Note:
    /// - The page will be archived by updating the page with the `archived` field set to `true`.
    pub async fn archive_a_page(
        &self,
        page_id: &str,
        parent_id: &str,
        parent_type: ParentType,
    ) -> Result<Page> {
        let mut page = Page {
            archived: true,
            ..Default::default()
        };

        match parent_type {
            ParentType::Database => {
                page.parent.type_name = parent_type;
                page.parent.database_id = Some(parent_id.to_string());
            }
            ParentType::Page => {
                page.parent.type_name = parent_type;
                page.parent.page_id = Some(parent_id.to_string());
            }
            ParentType::Workspace => {
                page.parent.type_name = parent_type;
                page.parent.workspace_id = Some(parent_id.to_string());
            }
            ParentType::Block => {
                page.parent.type_name = parent_type;
                page.parent.block_id = Some(parent_id.to_string());
            }
        }
        let page = self.update_a_page(page_id, &page).await?;
        return Ok(page);
    }

    /// # Append block children
    /// Because the Notion API only allows appending 100 blocks at a time, this method will split the
    /// blocks into chunks of 100 and append them to the parent block.
    /// ## Arguments:
    /// - parent_id: &str
    /// - blocks: [`BlockBody`]
    /// ## Return:
    /// - [`BlockResponse`] struct
    pub async fn append_block_children(
        &self,
        parent_id: &str,
        blocks: Vec<Block>,
    ) -> Result<BlockResponse> {
        let url = format!("https://api.notion.com/v1/blocks/{}/children", parent_id);
        let client = request::Client::new();
        let mut res_blocks: Vec<Block> = Vec::new();

        for i in (0..blocks.len()).step_by(100) {
            let end_index = std::cmp::min(i + 100, blocks.len());
            let block_body = BlockBody {
                children: blocks[i..end_index].to_vec(),
            };
            let data = serde_json::to_string(&block_body)?;
            let content = client
                .patch(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Notion-Version", "2022-06-28")
                .body(data)
                .send()
                .await?
                .text()
                .await?;
            let _bby = serde_json::from_str::<BlockResponse>(&content)?;
            res_blocks.extend(_bby.results);
        }

        let res_block = BlockResponse {
            object: "list".to_string(),
            results: res_blocks,
            status: 200,
            ..Default::default()
        };
        return Ok(res_block);
    }
}

#[cfg(test)]
mod tests;
