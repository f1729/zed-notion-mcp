# Notion MCP Server for Zed

A Zed extension that wraps the official [Notion MCP server](https://github.com/makenotion/notion-mcp-server), enabling AI-powered interaction with your Notion workspace directly from Zed's Agent Panel.

## Features

- Access and search Notion pages, databases, and blocks
- Create and modify Notion content from your editor
- Query databases and retrieve structured data
- Seamless integration with Zed's AI assistant

## Prerequisites

- [Zed Editor](https://zed.dev) (with Agent Panel support)
- [Node.js](https://nodejs.org) installed on your system

## Installation

### From Zed Extensions

1. Open Zed
2. Go to **Extensions** (Cmd+Shift+X on macOS)
3. Search for "Notion MCP Server"
4. Click **Install**

### As a Dev Extension

1. Clone this repository
2. In Zed, go to **Extensions** > **Install Dev Extension**
3. Select the cloned directory

## Configuration

### 1. Create a Notion Integration

1. Go to [Notion Integrations](https://www.notion.so/my-integrations)
2. Click **"+ New integration"**
3. Name your integration (e.g., "Zed MCP")
4. Select your workspace
5. Click **"Submit"**
6. Copy the **Internal Integration Secret** (starts with `ntn_`)

### 2. Share Pages with Your Integration

1. Open a Notion page or database you want to access
2. Click the **"..."** menu in the top-right corner
3. Click **"+ Add connections"**
4. Select your integration

Repeat for each page/database you want the AI to access.

### 3. Configure Zed

Add to your Zed `settings.json`:

```json
{
  "context_servers": {
    "mcp-server-notion": {
      "settings": {
        "notion_token": "ntn_your_integration_secret_here"
      }
    }
  }
}
```

## Verifying the Connection

1. Open the Agent Panel in Zed
2. Check the indicator dot next to "mcp-server-notion"
3. A green indicator means the server is active

## Usage

Once configured, you can interact with Notion through Zed's AI assistant. Example prompts:

- "Search my Notion workspace for project documentation"
- "Create a new page in my Tasks database"
- "Show me the contents of my meeting notes"
- "Update the status of my project to 'In Progress'"

## Troubleshooting

### Server not starting

- Ensure Node.js is installed and accessible in your PATH
- Verify your `notion_token` is correct and starts with `ntn_`
- Check Zed's logs: **View** > **Open Log** or run `zed --foreground`

### Can't access certain pages

- Make sure the page/database is shared with your integration
- The integration only has access to explicitly shared content

## Resources

- [Notion MCP Documentation](https://developers.notion.com/docs/mcp)
- [Notion API Documentation](https://developers.notion.com)
- [Zed MCP Extensions Guide](https://zed.dev/docs/extensions/mcp-extensions)
- [Official Notion MCP Server](https://github.com/makenotion/notion-mcp-server)

## License

Apache-2.0
