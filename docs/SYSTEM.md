# VS-BRAX SYSTEM — Architecture

PRIMAL_HEX: `4f5349524953424c58434b` · This is a NEW system, built on open-source (MIT) code.

## The system in one diagram

```
┌─────────────────────────────────────────────────────────────┐
│ VS-BRAX Aware AI IDE  (ide/vsbrax)                          │
│                                                             │
│  editor/  (this repo root: src/, build/, extensions/)       │
│    │  Electron workbench — rebranded VS-BRAX                │
│    ├── right-side agent terminal (auxiliary bar panel)      │
│    ├── any-API provider settings (vsbrax.api.*)             │
│    └── self-growing memory (memory.jsonl)                   │
│                                                             │
│  agent/   (77-crate Rust TUI agent — rebranded)             │
│    └── multi-model loop, swarm core, 30+ tools              │
│                                                             │
│  vsbrax/  (ABRASAX harness — ours)                          │
│    ├── harness/abrasax_bus.cs   C# named-pipe bus           │
│    ├── harness/pipe_bridge.c    C pipe driver               │
│    └── harness/agent_host.ts    TS agent host               │
│                                                             │
│  wire channel → tools/wire_control.mjs (hex envelopes,      │
│    16 audited commands: status/vram/vlchp/regget/gpuset/…)  │
└─────────────────────────────────────────────────────────────┘
             │                    │                    │
        LM Studio :1234      gateway :9000        \\.\pipe\abrasax
       (14 local models)    (GPU telemetry)       (mesh bus)
```

## Layers

| Layer | What it does | Source base |
|---|---|---|
| Editor core | text editing, workbench, panels | open-source VS Code (MIT) — rebranded |
| Agent core | multi-model coding agent, TUI, swarm | open-source jcode (MIT) — rebranded |
| Harness | wire envelopes, pipe bus, agent host | ABRASAX (ours) |
| Providers | any OpenAI-compatible API incl. local LM Studio | config-driven |
| Memory | self-growing interaction log + mesh memory | ours |

## Branding rules

- Everything user-visible is **VS-BRAX**.
- MIT copyright headers are preserved (license requirement).
- Internal identifiers (`vscode` in module paths) remain — functional plumbing, not branding.

## Repos

- System: https://github.com/basedgod55hjl/vsbrax (this repo, fresh history)
- Upstream fork (dev mirror): https://github.com/basedgod55hjl/vscode
