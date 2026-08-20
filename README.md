# VS-BRAX Aware AI IDE

PRIMAL_HEX: `4f5349524953424c58434b` · merged custom IDE — one tree, one brand: **VS-BRAX**.

## Structure (ide/vsbrax/)

| Path | Role |
|---|---|
| `src/`, `build/`, `extensions/`, `product.json` | Editor core (open-source VS Code base, MIT) — rebranded: all user-visible names are VS-BRAX; functional identifiers (`vsbrax-*`) and MIT copyright headers preserved |
| `agent/` | Agent core (Rust TUI coding agent source, 77 crates) — docs rebranded to VS-BRAX Agent; crate names kept as functional identifiers |
| `vsbrax/harness/abrasax_bus.cs` | C# named-pipe telemetry bridge (`\\.\pipe\abrasax_bus` → stdout → TS host) |
| `vsbrax/harness/pipe_bridge.c` | C driver: `SendManifoldConstraint` / `SendFrameSlice` into ABRASAX pipes |
| `vsbrax/harness/agent_host.ts` | TS agent host: spawns agent shell, streams lines, `/chat` → local LLM (LM Studio :1234) |
| `apps/vscode-abrasax` (ABRASAX repo) | First-party VS-BRAX extension: right-side agent terminal, any-API settings, wire commands |

## Branding policy

- User-visible names: **VS-BRAX / VS-BRAX Aware AI IDE / VS-BRAX Agent** everywhere.
- Kept intentionally: MIT license + copyright headers (legally required), internal functional
  identifiers (`vsbrax-dev`, `jcode-tui`, pipe names) that code depends on.
- No third-party brand appears in any markdown/docs surface.

## Build

Editor: `npm exec --yes --package=yarn@1.22.22 -- yarn install` then `yarn watch` (`.\scripts\code.bat`).
Agent: `cargo build --release` (Rust 1.97 present; built binary also installed as
`%LOCALAPPDATA%\jcode\builds\versions\0.64.2-src\jcode.exe`).
Harness: `dotnet build` for the C# bus; `cl /LD pipe_bridge.c` for the C driver.
