# VS-BRAX — Build

PRIMAL_HEX: `4f5349524953424c58434b`

## Prereqs (verified on the ABRASAX workstation)

- Node 22.x recommended (24 works for install), Yarn **1.x classic**:
  `npm exec --yes --package=yarn@1.22.22 -- yarn <cmd>`
- VS 2022 Build Tools (VC x64 toolset), Python, Rust 1.97+, .NET 10
- ~25 GB free disk for a full packaged build

## Editor (repo root)

```
npm exec --yes --package=yarn@1.22.22 -- yarn install
npm exec --yes --package=yarn@1.22.22 -- yarn watch        # dev compile
.\scripts\code.bat                                          # run dev build
npm exec --yes --package=yarn@1.22.22 -- yarn gulp vscode-win32-x64   # packaged VS-BRAX
```

## Agent (agent/)

```
cargo build --release          # 77 crates, ~74 min first build
cargo run --release -- repl    # smoke test
```

## Harness (vsbrax/harness/)

```
dotnet build abrasax_bus.cs    # C# bus (net10.0)
cl /LD pipe_bridge.c           # C driver (MSVC)
node agent_host.ts             # TS agent host REPL
```

## Validation

- Editor: `node -e "JSON.parse(require('fs').readFileSync('product.json','utf8'))"` after any rebrand edit.
- Agent: `cargo check` after any .rs change (docs-only rebrands need no check).
- Push safety: GitHub push protection runs on every push — never commit credentials.
