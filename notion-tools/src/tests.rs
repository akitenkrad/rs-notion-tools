use super::structs::common::*;
use super::structs::database_properties::*;
use super::*;
use fxhash::FxHashMap;
use structs::block;

#[tokio::test]
async fn test_retrieve_database() {
    let notion = Notion::new();
    let response = notion.retrieve_database().await;

    match response {
        Ok(database) => {
            println!("{:?}", database);
            assert!(database.properties.len() > 0);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_query_database() {
    let notion = Notion::new();
    let mut filter = DatabaseFilter::new();
    filter.args(DatabaseFilterItems::status(
        "Status",
        StatusFilterItem::default().equals("Deep Dive"),
    ));
    let response = notion.query_database(filter).await;

    match response {
        Ok(response) => {
            println!("{:?}", response.results.first().unwrap());
            assert!(response.results.len() > 0);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_crud_a_page() {
    let notion = Notion::new();
    let mut page_id = "".to_string();

    // Create a page
    let mut properties: FxHashMap<String, DatabaseProperty> = FxHashMap::default();
    properties.insert(
        "Name".to_string(),
        DatabaseProperty::title(RichText::from_str("Notion API Test")),
    );
    properties.insert(
        "Title".to_string(),
        DatabaseProperty::rich_text(vec![RichText::from_str("Notion API Test")]),
    );
    properties.insert("Status".to_string(), DatabaseProperty::status("Ready"));
    let mut page = Page::from_properties(properties);
    page.parent.type_name = ParentType::Database;
    page.parent.database_id = Some(notion.database_id.clone());
    let response = notion.create_a_page(&page).await;

    match response {
        Ok(page) => {
            println!("Created a page: {}", page.id);
            assert_eq!(page.status, 200);
            page_id = page.id;
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }

    // Update a page
    let mut properties: FxHashMap<String, DatabaseProperty> = FxHashMap::default();
    properties.insert(
        "Title".to_string(),
        DatabaseProperty::rich_text(vec![RichText::from_str("Notion API Test Updated")]),
    );
    properties.insert("Status".to_string(), DatabaseProperty::status("Deep Dive"));
    let mut page = Page::from_properties(properties);
    page.id = page_id.clone();
    page.parent.type_name = ParentType::Database;
    page.parent.database_id = Some(notion.database_id.clone());

    let response = notion.update_a_page(&page_id, &page).await;

    match response {
        Ok(page) => {
            println!("Updated the page: {}", page.id);
            assert_eq!(page.status, 200);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }

    // Delete a page
    let response = notion
        .archive_a_page(&page_id, &notion.database_id.clone(), ParentType::Database)
        .await;

    match response {
        Ok(page) => {
            println!("Deleted the page: {:?}", page);
            assert_eq!(page.status, 200);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_crud_blocks() {
    let notion = Notion::new();
    let mut page_id = "".to_string();

    // Create a page
    let mut properties: FxHashMap<String, DatabaseProperty> = FxHashMap::default();
    properties.insert(
        "Name".to_string(),
        DatabaseProperty::title(RichText::from_str("Notion API Test")),
    );
    properties.insert(
        "Title".to_string(),
        DatabaseProperty::rich_text(vec![RichText::from_str("Notion API Test")]),
    );
    properties.insert("Status".to_string(), DatabaseProperty::status("Ready"));
    let mut page = Page::from_properties(properties);
    page.parent.type_name = ParentType::Database;
    page.parent.database_id = Some(notion.database_id.clone());
    let response = notion.create_a_page(&page).await;

    match response {
        Ok(page) => {
            println!("Created a page: {}", page.id);
            assert_eq!(page.status, 200);
            page_id = page.id;
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }

    // Create blocks
    let mut blocks = BlockBody::default();
    blocks.children.push(Block::heading_1(
        ParentType::Page,
        &page_id,
        vec!["Test Heading 1"],
    ));
    blocks.children.push(Block::heading_2(
        ParentType::Page,
        &page_id,
        vec!["Test Heading 2"],
    ));
    blocks.children.push(Block::heading_3(
        ParentType::Page,
        &page_id,
        vec!["Test Heading 3"],
    ));
    blocks.children.push(Block::paragraph(
        ParentType::Page,
        &page_id,
        vec!["Test Paragraph"],
    ));
    blocks.children.push(Block::paragraph(
        ParentType::Page,
        &page_id,
        vec!["Original Text"],
    ));

    let response = notion.append_block_children(&page_id, blocks).await;
    let mut original_text_block_id = String::new();
    match response {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status, 200);

            response.results.iter().for_each(|block| {
                if block.type_name == "paragraph"
                    && block
                        .paragraph
                        .clone()
                        .unwrap()
                        .rich_text
                        .first()
                        .unwrap()
                        .plain_text
                        == "Original Text"
                {
                    original_text_block_id = block.id.clone();
                }
            });
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }

    // Update a block
    let mut blocks = BlockBody::default();
    blocks.children.push(Block::paragraph(
        ParentType::Page,
        &original_text_block_id,
        vec!["Updated Text 1"],
    ));
    blocks.children.push(Block::paragraph(
        ParentType::Page,
        &original_text_block_id,
        vec!["Updated Text 2"],
    ));

    let response = notion
        .append_block_children(&original_text_block_id, blocks)
        .await;

    match response {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status, 200);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }

    // Delete a page
    let response = notion
        .archive_a_page(&page_id, &notion.database_id.clone(), ParentType::Database)
        .await;

    match response {
        Ok(page) => {
            println!("Deleted the page: {:?}", page);
            assert_eq!(page.status, 200);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}
