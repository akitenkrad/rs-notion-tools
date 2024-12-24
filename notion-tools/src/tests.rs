use super::structs::database_properties::*;
use super::*;
use fxhash::FxHashMap;

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
    page.parent.type_name = "database_id".to_string();
    page.parent.database_id = Some(notion.database_id.clone());
    let response = notion.create_a_page(page).await;

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
    page.parent.type_name = "database_id".to_string();
    page.parent.database_id = Some(notion.database_id.clone());

    let response = notion.update_a_page(&page_id, page).await;

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
        .archive_a_page(&page_id, &notion.database_id.clone(), "database_id")
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
