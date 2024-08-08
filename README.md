# Clip to Notion (ctn)

`ctn` is a command-line tool for clipping web content directly to your Notion database. This tool fetches metadata like the title and Open Graph tags from a URL and stores the information in a specified Notion database.

## Prerequisites

- Notion API key.
- A Notion database set up for clipping, with the following properties:
  - **Name**: Type `title`
  - **URL**: Type `url`
  - **Tags**: Type `multi-select` (only needed if using the `--tags` option)

## Installation

To install `ctn`, ensure you have Rust and Cargo installed, then run:

```sh
make install
```

This will compile the project and install the ctn binary to your Cargo bin path.

## Usage

Before using ctn, make sure to initialize your configuration file:

```sh
ctn init
```

Follow the prompts to enter your Notion API key and database ID.

To clip a URL to your Notion database, use:

```sh
ctn run <url> --tags <tag1,tag2,...>
```

### Example

```
ctn run https://example.com --tags article,tech
```

## Configuration

The configuration file is stored at `~/.config/clip-to-notion/config.toml`. It contains your Notion API key and database ID.
