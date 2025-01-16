[![CircleCI](https://dl.circleci.com/status-badge/img/circleci/X1fiE4koKU88Z9sKwWoPAH/MVwdZy6GG5cejebNhSpJaw/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/X1fiE4koKU88Z9sKwWoPAH/MVwdZy6GG5cejebNhSpJaw/tree/main) 
![Crates.io Version](https://img.shields.io/crates/v/notion_tools?style=flat-square&color=blue)

# notion-tools

`notion-tools` is a library for interacting with the Notion API. It provides a convenient way to perform various operations such as retrieving databases, querying databases, creating pages, updating pages, archiving pages, and appending block children.

<img src="LOGO.png" alt="LOGO" width="150" height="150">

## Quick Start

### Installation

To start using `notion-tools`, just add it to your project's dependencies in the `Cargo.toml`.

```bash
> cargo add notion-tools
```

Then, import it in your program.

```rust
use notion_tools::Notion;
```

### Usage

To use this library, you need to set the `NOTION_API_KEY` and `NOTION_DATABASE_ID` environment variables. The `NOTION_API_KEY` is required for authentication, while the `NOTION_DATABASE_ID` is optional and can be set later using the `database` method.

### Examples

Examples to be added...

## Implemented endpoints

| Endpoint | Implemented |
|---|:---:|
| [Create a Token](https://developers.notion.com/reference/create-a-token) | - |
| [Append block children](https://developers.notion.com/reference/patch-block-children) | ✅ |
| [Retrieve a block](https://developers.notion.com/reference/retrieve-a-block) | - |
| [Retrieve block children](https://developers.notion.com/reference/get-block-children) | - |
| [Update a block](https://developers.notion.com/reference/update-a-block) | - |
| [Delete a block](https://developers.notion.com/reference/delete-a-block) | - |
| [Create a page](https://developers.notion.com/reference/post-page) | ✅ |
| [Retrieve a page](https://developers.notion.com/reference/retrieve-a-page) | - |
| [Retrieve a page property item](https://developers.notion.com/reference/retrieve-a-page-property-item) | - |
| [Update page properties](https://developers.notion.com/reference/patch-page) | ✅ |
| [Archive a page](https://developers.notion.com/reference/archive-a-page) | ✅ |
| [Create a database](https://developers.notion.com/reference/create-a-database) | - |
| [Query a database](https://developers.notion.com/reference/post-database-query) | ✅ |
| [Retrieve a database](https://developers.notion.com/reference/retrieve-a-database) | ✅ |
| [Update a database](https://developers.notion.com/reference/update-a-database) | - |
| [List all users](https://developers.notion.com/reference/get-users) | - |
| [Retrieve a user](https://developers.notion.com/reference/get-user) | - |
| [Retrieve your token's bot user](https://developers.notion.com/reference/get-self) | - |
| [Create comment](https://developers.notion.com/reference/create-a-comment) | - |
| [Retrieve comments](https://developers.notion.com/reference/retrieve-a-comment) | - |
| [Search by title](https://developers.notion.com/reference/post-search) | - |
