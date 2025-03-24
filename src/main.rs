use anyhow::Result;
use clap::{Parser, Subcommand};

mod describe;
mod tools;

#[derive(Parser)]
#[command(author, version, about = "MCP control utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Describe the endpoint at the given URL
    Describe {
        /// URL to describe
        url: String,
    },

    /// Call a tool with the given parameters
    #[command(name = "tools")]
    Tools {
        #[command(subcommand)]
        tool_command: ToolCommands,
    },
}

#[derive(Subcommand)]
enum ToolCommands {
    /// Call a tool with JSON parameters
    Call {
        /// URL of the endpoint
        url: String,

        /// Name of the tool to call
        tool_name: String,

        /// JSON string with parameters
        json_params: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Describe { url } => describe::describe_endpoint(url).await,
        Commands::Tools { tool_command } => match tool_command {
            ToolCommands::Call {
                url,
                tool_name,
                json_params,
            } => tools::call_tool(url, tool_name, json_params).await,
        },
    }
}
