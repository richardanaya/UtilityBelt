# mcpctl

A CLI tool for interacting with MCP endpoints.

## Usage

The purpose of this tool is to load aider with context of how to call tools on an MCP endpoint.


inside aider
```bash
/run mcpctl describe https://api.example.com
```

this should a list of command line calls to aider to use as suggestions for tools it can call.

when tools are suggested, aider should use the following format:

```bash
/run mcpctl tools call https://api.example.com <tool_name> <tool_params>
```
