# Codex CLI (Rust Implementation)

We provide Codex CLI as a standalone, native executable to ensure a zero-dependency install.

## Installing Codex

Today, the easiest way to install Codex is via `npm`, though we plan to publish Codex to other package managers soon.

```shell
npm i -g @openai/codex@native
codex
```

You can also download a platform-specific release directly from our [GitHub Releases](https://github.com/openai/codex/releases).

## Config

Codex supports a rich set of configuration options. See [`config.md`](./config.md) for details.

### Azure OpenAI Support

Codex includes a built‑in `azure` model provider. Set `model_provider = "azure"`
in your `config.toml` and define `AZURE_OPENAI_KEY` in the environment. Use your
Azure OpenAI API key as the value or set it to `USE_ENTRA_ID` to authenticate via
Microsoft Entra ID (AAD). When using Entra ID ensure the Azure CLI is logged in
with `az login` so `DefaultAzureCredential` can acquire a token.

You can test the integration with:

```shell
codex --config model_provider="azure" --model <model>
```

## Model Context Protocol Support

Codex CLI functions as an MCP client that can connect to MCP servers on startup. See the [`mcp_servers`](./config.md#mcp_servers) section in the configuration documentation for details.

It is still experimental, but you can also launch Codex as an MCP _server_ by running `codex mcp`. Using the [`@modelcontextprotocol/inspector`](https://github.com/modelcontextprotocol/inspector) is

```shell
npx @modelcontextprotocol/inspector codex mcp
```

## Code Organization

This folder is the root of a Cargo workspace. It contains quite a bit of experimental code, but here are the key crates:

- [`core/`](./core) contains the business logic for Codex. Ultimately, we hope this to be a library crate that is generally useful for building other Rust/native applications that use Codex.
- [`exec/`](./exec) "headless" CLI for use in automation.
- [`tui/`](./tui) CLI that launches a fullscreen TUI built with [Ratatui](https://ratatui.rs/).
- [`cli/`](./cli) CLI multitool that provides the aforementioned CLIs via subcommands.
