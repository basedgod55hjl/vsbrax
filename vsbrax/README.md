# VS-BRAX Aware AI IDE — fork of microsoft/vscode

PRIMAL_HEX: `4f5349524953424c58434b` · fork of https://github.com/microsoft/vscode
owned by @basedgod55hjl. Rebrand: **VS-BRAX Aware AI IDE** (VS-BRAX).

## What is in-tree

| Path | Role |
|---|---|
| `vsbrax/harness/abrasax_bus.cs` | C# named-pipe telemetry bridge (`\\.\pipe\abrasax_bus` → stdout → TS host) |
| `vsbrax/harness/pipe_bridge.c` | C driver: `SendManifoldConstraint` / `SendFrameSlice` into ABRASAX pipes |
| `vsbrax/harness/agent_host.ts` | TS agent host: spawns agent shell, streams lines, `/chat` → local LLM (LM Studio :1234) |
| `vsbrax/extension/` | First-party VS-BRAX extension (right-side agent terminal, any-API settings, wire commands) — source of truth: `apps/vscode-abrasax` in the ABRASAX repo |
| `product.json` | Rebranded identity: VS-BRAX names, mutexes, registry value `VSBRAX` |

## Build (Windows)

Prereqs (verified on this machine): Node 22.x (have 24 — works for install, pin 22 for full builds),
Yarn **1.x classic** (`npm exec --yes --package=yarn@1.22.22 -- yarn`), VS 2022 Build Tools (present),
Python, ~25 GB disk for a full packaged build.

```
npm exec --yes --package=yarn@1.22.22 -- yarn install     # classic lockfile install
npm exec --yes --package=yarn@1.22.22 -- yarn watch        # dev compile (long)
.\scripts\code.bat                                         # run dev build
```

Full distro: `yarn gulp vscode-win32-x64` (packaged VS-BRAX .exe). The heavy full build is
deliberately NOT run in-tree by default — see `.github/copilot-instructions.md` ("minimize
large-scale builds"); the deliverable layer is the rebrand + harness + first-party extension.

## ABRASAX wiring

- Right-side panel: VS-BRAX Agent Terminal (extension view in the **auxiliary bar** = right side).
- Providers: any OpenAI-compatible API (settings `vsbrax.api.*`) — defaults to local LM Studio
  `http://127.0.0.1:1234/v1`, 14 local models (dolphin3.0-llama3.1-8b, deepseek-r1-distill-qwen-7b,
  qwen2.5-7b/14b, mixtral, llama-3.3-70b, gemma, codegemma, phi-4-mini, nemotron, lfm2.5, nomic).
- Self-growing AI: every interaction logs to the extension's `memory.jsonl` (`vsbrax.selfGrow`).
- Wire channel: 16 audited commands (status/vram/vlchp/vlcfull/regget/regset/regdel/reglist/gpuset/scan/build/…)
  streamed through `tools/wire_control.mjs` (hex-encoded envelopes).

## macOS/Linux identity

`product.json` still carries the vsbrax darwin/linux identifiers — rebrand those when a
non-Windows build is needed (Windows-first fork, per the ABRASAX workstation).
