# Clip to Notion (ctn)

`ctn` is a command-line tool designed to efficiently fetch the title and metadata from any URL and tidily organize them into your Notion workspace. By extracting key information such as titles and Open Graph tags, `ctn` streamlines the process of saving and managing online content directly within your Notion database.

## Prerequisites

- Notion API key.
- A Notion database set up for clipping, with the following properties:
  - **Name**: Type `title`
  - **URL**: Type `url`
  - **Description**: Type `text`
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
