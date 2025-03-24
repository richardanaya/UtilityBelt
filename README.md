<p align="center">
  <img src="https://github.com/user-attachments/assets/3f8c6127-2c02-4103-a010-0ea5a44dcbe2" alt="Description" width="300">
</p>


A CLI tool for interacting with MCP endpoints for CLI based AI agents (like aider)

## Usage

The purpose of this tool is to load aider with context of how to call tools on an MCP endpoint.

inside aider

```bash
/run utilitybelt describe https://mcp-example.com
```

this should a list of command line calls to aider to use as suggestions for tools it can call.

when tools are suggested, aider should use the following format:

```bash
/run utilitybelt tools call https://mcp-example.com <tool_name> "<tool_params_escaped_json>"
```

## See it work

Using an MCP endpoint for a calculator in aider.

<img width="530" alt="Screenshot 2025-03-24 at 1 02 00 AM" src="https://github.com/user-attachments/assets/3af0d45e-c1e9-4700-abec-d48e395d999f" />
<img width="558" alt="Screenshot 2025-03-24 at 1 02 13 AM" src="https://github.com/user-attachments/assets/32ecbe7e-8102-4b41-a165-b01685e2635d" />
