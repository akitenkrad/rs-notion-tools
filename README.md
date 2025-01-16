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

More details to be added...
