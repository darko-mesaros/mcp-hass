use rmcp::model::*;
use rmcp::Error as McpError;
use anyhow::Result;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// Local prompt information
pub struct PromptDefinition {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<PromptArgument>,
    pub template: String,
}

impl PromptDefinition {
    pub fn new(
        name: &str,
        description: Option<&str>,
        arguments: Vec<PromptArgument>,
        template: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            arguments,
            template: template.to_string(),
        }
    }

    // Helper function that convers the prompt to the SDK Prompt type
    pub fn to_mcp_prompt(&self) -> Prompt {
        Prompt::new(
            &self.name,
            self.description.clone(),
            Some(self.arguments.clone()),
        )
    }


    // Processes the prompt by adding the necessary arguments to the right spots
    pub fn process(&self,
        arguments: Option<serde_json::Map<String, serde_json::Value>>
    ) -> Result<String, McpError> {
        let mut result = self.template.clone();

        if let Some(args) = arguments {
            // Check required arguments
            for arg in &self.arguments {
                if arg.required.unwrap_or(false) && !args.contains_key(&arg.name) {
                    return Err(McpError::invalid_params(
                       format!("Required argument '{}' missing", arg.name),
                       None
                    ));
                }
            }

            // Replace placeholders
            for (key, value) in args {
                let placeholder = format!("{{{}}}", key);
                if let Some(value_str) = value.as_str() {
                    result = result.replace(&placeholder, value_str);
                } else if value.is_object() || value.is_array() {
                    // For complex values use JSON representation
                    result = result.replace(&placeholder, &value.to_string());
                } else {
                    // For other scalar values
                    result = result.replace(&placeholder, &value.to_string());
                }
            }

        }
        Ok(result)

    }
    
}

// Store all prompts in a static HashMap for efficient lookup
pub static PROMPTS: Lazy<HashMap<String, PromptDefinition>> = Lazy::new(||{
    let mut map = HashMap::new();

    // Add prompts here:

    // Example prompt
    map.insert(
        "example_prompt".to_string(),
        PromptDefinition::new(
            "example_prompt",
            Some("This is an example prompt that takes one argument, message"),
            vec![PromptArgument {
                name: "message".to_string(),
                description: Some("A message to put in the prompt".to_string()),
                required: Some(true),
            }],
            "This is an example prompt with your message here: '{message}'"
        )
    );

    // Code review prompt
    map.insert(
        "code_review".to_string(),
        PromptDefinition::new(
            "code_review",
            Some("This is an example prompt that takes one argument, message"),
            vec![
                PromptArgument {
                    name: "language".to_string(),
                    description: Some("Programming language".to_string()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "code".to_string(),
                    description: Some("Code to review".to_string()),
                    required: Some(true),
                },
            ],
            "Please review this {language} code, and provide me the top 3 things I need to do to improve it:\n\n{language}\n{code}\n"
        )
    );

    map
});

// Helper functions to work with prompts
pub fn get_all_prompts() -> Vec<Prompt> {
    PROMPTS.values()
        .map(|p| p.to_mcp_prompt())
        .collect()
}

pub fn get_prompt_by_name(name: &str) -> Option<&PromptDefinition> {
    PROMPTS.get(name)
}
