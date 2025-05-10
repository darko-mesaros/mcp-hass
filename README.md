# Home Assistant MCP Integration

A Rust-based Model Context Protocol (MCP) server that integrates with Home Assistant to control smart home devices.

## Overview

This project provides a bridge between AI assistants and Home Assistant using the Model Context Protocol (MCP). It allows AI assistants to:

- Retrieve the status of lights, switches, and sensors from Home Assistant
- Control smart home devices (turn lights/switches on and off)
- Use predefined prompts for common interactions

## Features

- **Entity Management**: Get status information for lights, switches, and sensors
- **Device Control**: Turn devices on and off through simple commands
- **MCP Integration**: Implements the Model Context Protocol for seamless AI assistant integration
- **Prompt System**: Includes a flexible prompt template system for standardized interactions

## Prerequisites

- Rust (2021 edition or newer)
- Home Assistant instance
- Home Assistant long-lived access token
- Network connectivity between this service and your Home Assistant instance

## Usage

Build the server:

```
cargo build --release
```

And add the following `mcp.json` to `~.aws/amazonq/`:
```json
{
  "mcpServers": {
    "hass": {
      "command": "<PATH OF THE BINARY IF NOT IN $PATH>",
      "args": [],
      "env": {
        "HASS_ENDPOINT": "<IP ADDRESS OF HASS>",
        "HASS_TOKEN": "<LONG LIVED TOKEN?"
      }
    }
  }
}

```

The server will start and listen for MCP requests. AI assistants that support MCP can then use the following tools:

- `hass___get_entities`: Retrieve all available lights, switches, and sensors with their current states
- `hass___turn_on_entitity`: Turn on a specific entity (e.g., a light)
- `hass___turn_off_entitity`: Turn off a specific entity

## Prompt System <BETA>

The project includes a flexible prompt system that allows for templated interactions. Example prompts:

- `example_prompt`: A simple example that takes a message parameter
- `code_review`: A prompt for code review that takes language and code parameters

## Development

### Project Structure

- `src/main.rs`: Entry point that initializes the MCP server
- `src/hass.rs`: Core functionality for Home Assistant integration
- `src/prompts.rs`: Prompt template system implementation

### Adding New Features

To add new Home Assistant capabilities:
1. Add new tool methods to the `Entities` struct in `hass.rs`
2. Implement the necessary API calls to Home Assistant
3. Register the tools using the `#[tool]` attribute

To add new prompts:
1. Add prompt definitions to the `PROMPTS` lazy static in `prompts.rs`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
