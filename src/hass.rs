use std::sync::Arc;

use anyhow::Result;
use rmcp::{
    model::*, schemars, service::RequestContext, tool, Error as McpError, RoleServer, ServerHandler,
};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use serde_json::json;

use crate::prompts::{get_all_prompts, get_prompt_by_name};


#[derive(Clone, Debug)]
pub struct Entities {
    entities: Arc<Mutex<Vec<Entity>>>,
}
#[derive(Deserialize, Debug)]
pub struct EntitiesData {
    _entities: Vec<Entity>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Entity {
    attributes: serde_json::Value,
    entity_id: String,
    last_changed: String,
    state: String,
}

#[tool(tool_box)]
impl Entities {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(Mutex::new(Vec::new()))
        }
    }
    #[tool(description = "Get all available Ligths, Switches, Sensors and their current states")]
    async fn get_entities(&self) -> Result<CallToolResult, McpError> {

        let prefixes = ["light.", "switch.", "sensor."];
        let client = reqwest::Client::new();
        let token = std::env::var("HASS_TOKEN").map_err(|_| {
            McpError::internal_error("HASS_TOKEN environment variable has not been set", None)
        })?;
        let endpoint = std::env::var("HASS_ENDPOINT").map_err(|_| {
            McpError::internal_error("HASS_ENDPOINT environment variable has not been set", None)
        })?;

        let response = client.get(format!("http://{}:8123/api/states", endpoint))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to fetch data from Home Assistant: {}", e), None)
            })?;

        // Check if the request was a success
        if !response.status().is_success() {
            return Err(McpError::internal_error(format!(
                "Home Assistant API returned error status: {}",
                response.status()
            ), None));
        }

        let fetched_data: Vec<Entity> = response.json().await.map_err(|e| {
            McpError::internal_error(format!("Failed to parse entities: {}", e), None)
        })?;


        // Filter only lights
        let filtered_entities: Vec<Entity> = fetched_data
            .into_iter()
            .filter(|entity|{
                prefixes.iter().any(|&prefix| entity.entity_id.starts_with(prefix))
            })
            .collect();
        // Update the stored entities - stores it in the main Entities Struct
        {
            let mut entities = self.entities.lock().await;
            *entities = filtered_entities.clone();
        }
        Ok(
            CallToolResult::success(vec![Content::json(
                &filtered_entities,
            )?])
        )
    }
    #[tool(description = "Turn on a given entity")]
    async fn turn_on_entitity(
        &self,
        #[tool(param)]
        #[schemars(description = "The id of the entity")]
        entity_id: String,
    ) -> Result<CallToolResult, McpError> {

        let client = reqwest::Client::new();
        let token = std::env::var("HASS_TOKEN").map_err(|_| {
            McpError::internal_error("HASS_TOKEN environment variable has not been set", None)
        })?;
        let endpoint = std::env::var("HASS_ENDPOINT").map_err(|_| {
            McpError::internal_error("HASS_ENDPOINT environment variable has not been set", None)
        })?;

        let body = json!({
            "entity_id": entity_id
        });

        tracing::debug!("REQUEST SENT: {:?}", entity_id);

        let response = client.post(format!("http://{}:8123/api/services/light/turn_on", endpoint))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to send request to Home Assistant: {}", e), None)
            })?;

        // Check if the request was a success
        if !response.status().is_success() {
            return Err(McpError::internal_error(format!(
                "Home Assistant API returned error status: {}",
                response.status()
            ), None));
        }

        Ok(
            CallToolResult::success(vec![Content::text(
                format!("Succesfully turned on light: {}", entity_id)
            )])
        )

    }
    #[tool(description = "Turn off a given entity")]
    async fn turn_off_entitity(
        &self,
        #[tool(param)]
        #[schemars(description = "The id of the entity")]
        entity_id: String,
    ) -> Result<CallToolResult, McpError> {

        let client = reqwest::Client::new();
        let token = std::env::var("HASS_TOKEN").map_err(|_| {
            McpError::internal_error("HASS_TOKEN environment variable has not been set", None)
        })?;
        let endpoint = std::env::var("HASS_ENDPOINT").map_err(|_| {
            McpError::internal_error("HASS_ENDPOINT environment variable has not been set", None)
        })?;

        let body = json!({
            "entity_id": entity_id
        });

        tracing::debug!("REQUEST SENT: {:?}", entity_id);

        let response = client.post(format!("http://{}:8123/api/services/light/turn_off", endpoint))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to send request to Home Assistant: {}", e), None)
            })?;

        // Check if the request was a success
        if !response.status().is_success() {
            return Err(McpError::internal_error(format!(
                "Home Assistant API returned error status: {}",
                response.status()
            ), None));
        }

        Ok(
            CallToolResult::success(vec![Content::text(
                format!("Succesfully turned on light: {}", entity_id)
            )])
        )

    }
}

#[tool(tool_box)]
impl ServerHandler for Entities {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server is used to retrieve entities from my home assistant. It will get you lights and other smart home devices. It can also be used to control lights with the turn_on_entitity and turn_off_entitity tools. When running get_entities tool you will get the light, switch, and sensor entitites.".to_string()),
        }
    }


    // Shortened version thanks to prompts.rs
    async fn list_prompts(
        &self,
        // NOTE: These are required by the Trait implementation, even though I dont use them. In
        // this example the _request parameter is used for pagination, if I have a lot of prompts
        // and want to paginate the return.
        _request: Option<PaginatedRequestParamInner>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: get_all_prompts()
        })
    }

    async fn get_prompt(
        &self,
        // NOTE: This is parameter destructuring. Because we are passing an Struct that has the
        // fields `name` and `arguments`. We are just pulling them in as `name` and `arguments`.
        // This is a conciser way of doing `param.name` and `param.arguments`.
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let prompt_def = get_prompt_by_name(&name)
            .ok_or_else(||McpError::invalid_params("prompt_not_found", None))?;

        //process arguments and fill template
        let filled_prompt = prompt_def.process(arguments)?;
        Ok(
            GetPromptResult {
                description: None,
                messages: vec![
                    PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(filled_prompt)
                    }
                ]
            }
        )
    }

    async fn initialize(
        &self, 
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(rmcp::ServerHandler::get_info(self))
    }
}
