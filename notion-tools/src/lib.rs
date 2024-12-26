pub mod structs;

use crate::structs::block::*;
use crate::structs::common::*;
use crate::structs::database_filter::*;
use crate::structs::notion::*;
use anyhow::Result;
use dotenvy::dotenv;
use reqwest as request;

#[derive(Debug)]
pub struct Notion {
    pub api_key: String,
    pub database_id: String,
}

impl Notion {
    pub fn new() -> Self {
        dotenv().ok();
        let api_key = std::env::var("NOTION_API_KEY").expect("NOTION_API_KEY must be set");
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or("".to_string());

        Notion {
            api_key,
            database_id,
        }
    }

    pub async fn retrieve_database(&self) -> Result<NotionDatabase> {
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
        let database = serde_json::from_str::<NotionDatabase>(&content)?;
        return Ok(database);
    }

    pub async fn query_database(&self, filter: DatabaseFilter) -> Result<PageResponse> {
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
            _ => {}
        }
        let page = self.update_a_page(page_id, &page).await?;
        return Ok(page);
    }

    pub async fn append_block_children(
        &self,
        parent_id: &str,
        blocks: BlockBody,
    ) -> Result<BlockResponse> {
        let url = format!("https://api.notion.com/v1/blocks/{}/children", parent_id);
        let client = request::Client::new();
        let data = serde_json::to_string(&blocks)?;
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

        let mut res_blocks = serde_json::from_str::<BlockResponse>(&content)?;
        if res_blocks.status == 0 {
            res_blocks.status = 200;
        }

        return Ok(res_blocks);
    }
}

#[cfg(test)]
mod tests;
