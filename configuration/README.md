# Notion MCP Server Setup

This extension requires a Notion integration token to connect to your Notion workspace.

## Prerequisites

- **Node.js** must be installed on your system (required for the MCP server)

## Getting Your Notion Integration Token

1. Go to [Notion Integrations](https://www.notion.so/my-integrations)
2. Click **"+ New integration"**
3. Give your integration a name (e.g., "Zed MCP")
4. Select the workspace you want to connect
5. Click **"Submit"**
6. Copy the **Internal Integration Secret** (starts with `ntn_`)

## Connecting Your Integration to Pages

After creating the integration, you need to share specific pages/databases with it:

1. Open a Notion page or database you want to access
2. Click the **"..."** menu in the top-right corner
3. Scroll down and click **"+ Add connections"**
4. Select your integration from the list

Repeat this for each page or database you want the AI to access.

## Configuration

Add the following to your Zed settings (`settings.json`):

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

Replace `ntn_your_integration_secret_here` with your actual integration token.

## Verifying the Connection

After configuration, check the Agent Panel in Zed. The indicator dot next to "mcp-server-notion" should be green, indicating the server is active.

## More Information

- [Notion MCP Documentation](https://developers.notion.com/docs/mcp)
- [Creating Notion Integrations](https://developers.notion.com/docs/create-a-notion-integration)
