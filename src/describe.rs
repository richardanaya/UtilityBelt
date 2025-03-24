use anyhow::{Context, Result};
use mcp_client::client::{ClientCapabilities, ClientInfo, McpClient, McpClientTrait};
use mcp_client::transport::{SseTransport, Transport};
use mcp_client::McpService;
use std::collections::HashMap;
use std::time::Duration;

pub async fn describe_endpoint(url: &str) -> Result<()> {
    println!("Connecting to MCP endpoint at: {}", url);

    // Create the base transport
    let transport = SseTransport::new(&format!("{}", url), HashMap::new());

    // Start transport
    let handle = transport
        .start()
        .await
        .context("Failed to start transport connection")?;

    // Create the service with timeout middleware
    let service = McpService::with_timeout(handle, Duration::from_secs(5));

    // Create client
    let mut client = McpClient::new(service);

    // Initialize connection
    let server_info = client
        .initialize(
            ClientInfo {
                name: "utilitybelt".into(),
                version: "0.1.0".into(),
            },
            ClientCapabilities::default(),
        )
        .await
        .context("Failed to initialize connection to MCP endpoint")?;

    println!("\nServer Information:");
    println!("  Name:    {}", server_info.server_info.name);
    println!("  Version: {}", server_info.server_info.version);

    // Small delay to ensure server is ready
    tokio::time::sleep(Duration::from_millis(500)).await;

    // List tools
    let tools = client
        .list_tools(None)
        .await
        .context("Failed to list tools")?;

    println!("\nAvailable Tools:");
    if tools.tools.is_empty() {
        println!("  No tools available");
    } else {
        for tool in &tools.tools {
            println!("  {}", tool.name);
            if !tool.description.is_empty() {
                println!("    Description: {}", tool.description);
            }

            // Create a simple example JSON for parameters based on the schema
            let example_params = generate_example_params(&tool.input_schema);
            let escaped_params = example_params.replace("\"", "\\\"");

            // Display example command line call
            println!("    Example call:");
            println!(
                "    utilitybelt tools call {} {} \"{}\"",
                url, tool.name, escaped_params
            );

            println!("    Schema:");
            println!(
                "    {}",
                serde_json::to_string_pretty(&tool.input_schema)
                    .unwrap_or_else(|_| "Unable to format schema".to_string())
                    .lines()
                    .map(|line| format!("    {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            println!();
        }
    }

    Ok(())
}

// Generate a simple example JSON based on the schema structure
fn generate_example_params(schema: &serde_json::Value) -> String {
    if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
        let mut example = serde_json::Map::new();

        for (prop_name, prop_schema) in properties {
            if let Some(example_value) = generate_example_value(prop_schema) {
                example.insert(prop_name.clone(), example_value);
            }
        }

        serde_json::to_string(&example).unwrap_or_else(|_| "{}".to_string())
    } else {
        "{}".to_string()
    }
}

fn generate_example_value(prop_schema: &serde_json::Value) -> Option<serde_json::Value> {
    if let Some(type_str) = prop_schema.get("type").and_then(|t| t.as_str()) {
        match type_str {
            "string" => Some(serde_json::Value::String("example".to_string())),
            "number" => Some(serde_json::Value::Number(serde_json::Number::from(42))),
            "integer" => Some(serde_json::Value::Number(serde_json::Number::from(42))),
            "boolean" => Some(serde_json::Value::Bool(true)),
            "object" => Some(serde_json::Value::Object(serde_json::Map::new())),
            "array" => Some(serde_json::Value::Array(vec![])),
            _ => None,
        }
    } else {
        None
    }
}
