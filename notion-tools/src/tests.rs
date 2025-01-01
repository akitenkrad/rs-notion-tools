use super::*;
use fxhash::FxHashMap;

#[tokio::test]
async fn test_retrieve_database() {
    let notion = Notion::new();
    let response = notion.retrieve_a_database().await;

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
    let mut filter = QueryFilter::new();
    filter.args(FilterItem::status(
        String::from("Status"),
        StatusFilterItem::equals(String::from("Deep Dive")),
    ));
    let response = notion.query_database(filter).await;

    match response {
        Ok(response) => {
            println!("Number of data: {}", response.results.len());
            assert!(response.results.len() > 0);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "{}", e.to_string());
        }
    }
}

#[tokio::test]
async fn test_query_database_with_pagenation() {
    let notion = Notion::new();
    let mut filter = QueryFilter::new();
    filter.args(FilterItem::status(
        String::from("Status"),
        StatusFilterItem::equals(String::from("Deep Dive")),
    ));

    let mut has_more = true;
    let mut num_papers = 0;

    while has_more {
        let response = notion.query_database(filter.clone()).await;

        match response {
            Ok(response) => {
                assert!(response.results.len() > 0);
                num_papers += response.results.len();
                has_more = response.has_more.unwrap_or(false);
                filter.start_cursor = response.next_cursor.unwrap_or(String::new());
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "Error: {}", e.to_string());
            }
        }
    }
    assert!(num_papers > 0);
    println!("Number of papers: {}", num_papers);
}

#[tokio::test]
async fn test_crud_a_page() {
    let notion = Notion::new();
    let mut page_id = "".to_string();

    // Create a page
    let mut properties: FxHashMap<String, PageProperty> = FxHashMap::default();
    properties.insert(
        "Name".to_string(),
        PageProperty::title(RichText::from_str(String::from("Notion API Test"))),
    );
    properties.insert(
        "Title".to_string(),
        PageProperty::rich_text(vec![RichText::from_str(String::from("Notion API Test"))]),
    );
    properties.insert(
        "Status".to_string(),
        PageProperty::status(String::from("Ready")),
    );
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
    let mut properties: FxHashMap<String, PageProperty> = FxHashMap::default();
    properties.insert(
        "Title".to_string(),
        PageProperty::rich_text(vec![RichText::from_str(String::from(
            "Notion API Test Updated",
        ))]),
    );
    properties.insert(
        "Status".to_string(),
        PageProperty::status(String::from("Deep Dive")),
    );
    let mut page = Page::from_properties(properties);
    page.id = page_id.clone();
    page.parent.type_name = ParentType::Database;
    page.parent.database_id = Some(notion.database_id.clone());

    let response = notion.update_a_page(page_id.clone(), &page).await;

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
        .archive_a_page(
            page_id.clone(),
            notion.database_id.clone(),
            ParentType::Database,
        )
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
    let mut properties: FxHashMap<String, PageProperty> = FxHashMap::default();
    properties.insert(
        "Name".to_string(),
        PageProperty::title(RichText::from_str(String::from("Notion API Test"))),
    );
    properties.insert(
        "Title".to_string(),
        PageProperty::rich_text(vec![RichText::from_str(String::from("Notion API Test"))]),
    );
    properties.insert(
        "Status".to_string(),
        PageProperty::status(String::from("Ready")),
    );
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
    let mut blocks: Vec<Block> = Vec::new();
    blocks.push(Block::heading_1(
        ParentType::Page,
        page_id.clone(),
        vec![String::from("Test Heading 1")],
    ));
    blocks.push(Block::heading_2(
        ParentType::Page,
        page_id.clone(),
        vec![String::from("Test Heading 2")],
    ));
    blocks.push(Block::heading_3(
        ParentType::Page,
        page_id.clone(),
        vec![String::from("Test Heading 3")],
    ));
    blocks.push(Block::paragraph(
        ParentType::Page,
        page_id.clone(),
        vec![String::from("Test Paragraph")],
    ));
    blocks.push(Block::paragraph(
        ParentType::Page,
        page_id.clone(),
        vec![String::from("Original Text")],
    ));

    let response = notion.append_block_children(page_id.clone(), blocks).await;
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
    let mut blocks: Vec<Block> = Vec::new();
    blocks.push(Block::paragraph(
        ParentType::Page,
        original_text_block_id.clone(),
        vec![String::from("Updated Text 1")],
    ));
    blocks.push(Block::paragraph(
        ParentType::Page,
        original_text_block_id.clone(),
        vec![String::from("Updated Text 2")],
    ));

    let response = notion
        .append_block_children(original_text_block_id.clone(), blocks)
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
        .archive_a_page(
            page_id.clone(),
            notion.database_id.clone(),
            ParentType::Database,
        )
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
async fn test_crud_blocks_many() {
    let notion = Notion::new();
    let mut page_id = "".to_string();

    // Create a page
    let mut properties: FxHashMap<String, PageProperty> = FxHashMap::default();
    properties.insert(
        "Name".to_string(),
        PageProperty::title(RichText::from_str(String::from("Notion API Test"))),
    );
    properties.insert(
        "Title".to_string(),
        PageProperty::rich_text(vec![RichText::from_str(String::from("Notion API Test"))]),
    );
    properties.insert(
        "Status".to_string(),
        PageProperty::status(String::from("Ready")),
    );
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
    let mut blocks: Vec<Block> = Vec::new();
    for i in 0..500 {
        blocks.push(Block::paragraph(
            ParentType::Page,
            page_id.clone(),
            vec![format!("Test Paragraph {}", i)],
        ));
    }
    let response = notion.append_block_children(page_id.clone(), blocks).await;
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
                        == "Test Paragraph 0"
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
    let mut blocks: Vec<Block> = Vec::new();
    blocks.push(Block::paragraph(
        ParentType::Page,
        original_text_block_id.clone(),
        vec![String::from("Updated Text 1")],
    ));
    blocks.push(Block::paragraph(
        ParentType::Page,
        original_text_block_id.clone(),
        vec![String::from("Updated Text 2")],
    ));

    let response = notion
        .append_block_children(original_text_block_id, blocks)
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
        .archive_a_page(
            page_id.clone(),
            notion.database_id.clone(),
            ParentType::Database,
        )
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
