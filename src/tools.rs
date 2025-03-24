use anyhow::{Context, Result};
use mcp_client::client::{ClientCapabilities, ClientInfo, McpClient, McpClientTrait};
use mcp_client::transport::{SseTransport, Transport};
use mcp_client::McpService;
use std::collections::HashMap;
use std::time::Duration;

pub async fn call_tool(url: &str, tool_name: &str, json_params: &str) -> Result<()> {
    println!("Calling tool '{}' at URL: {}", tool_name, url);

    // Parse JSON parameters
    let params: serde_json::Value =
        serde_json::from_str(json_params).context("Failed to parse JSON parameters")?;

    // Create the base transport
    let transport = SseTransport::new(
        &format!("{}/sse", url.trim_end_matches("/sse")),
        HashMap::new(),
    );

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
    let _server_info = client
        .initialize(
            ClientInfo {
                name: "utilitybelt".into(),
                version: "0.1.0".into(),
            },
            ClientCapabilities::default(),
        )
        .await
        .context("Failed to initialize connection to MCP endpoint")?;

    // Small delay to ensure server is ready
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Call the tool
    let result = client
        .call_tool(tool_name, params)
        .await
        .context(format!("Failed to call tool '{}'", tool_name))?;

    println!("\nTool Result:");
    println!(
        "{}",
        serde_json::to_string_pretty(&result).unwrap_or_else(|_| format!("{:?}", result))
    );

    Ok(())
}
