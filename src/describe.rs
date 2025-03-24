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
                name: "mcpctl".into(),
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
            println!(
                "    Schema: {}",
                serde_json::to_string_pretty(&tool.input_schema)
                    .unwrap_or_else(|_| "Unable to format schema".to_string())
            );
            println!();
        }
    }

    Ok(())
}
