use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@notionhq/notion-mcp-server";
const SERVER_PATH: &str = "node_modules/@notionhq/notion-mcp-server/bin/cli.mjs";

struct NotionMcpExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct NotionContextServerSettings {
    notion_token: String,
}

impl zed::Extension for NotionMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed_version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-notion", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `notion_token` setting".into());
        };
        let settings: NotionContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        let node_path = zed::node_binary_path()?;
        let server_path = env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: node_path,
            args: vec![server_path],
            env: vec![("NOTION_TOKEN".into(), settings.notion_token)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions = include_str!("../configuration/README.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(NotionContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(NotionMcpExtension);
