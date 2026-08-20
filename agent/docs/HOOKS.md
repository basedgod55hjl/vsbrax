# Lifecycle Hooks

VS-BRAX Agent can run external commands at well-defined lifecycle points so other
programs can observe or gate agent behavior without forking VS-BRAX Agent. Hooks
complement the [spawn hook](SPAWN_HOOK.md) (which controls *where headed
sessions appear*); lifecycle hooks tell you *what is happening inside them*.

## Configuration

```toml
# ~/.VS-BRAX Agent/config.toml
[hooks]
turn_end      = "~/bin/VS-BRAX Agent-turn-notify"     # observer
session_start = ""                            # observer
session_end   = ""                            # observer
pre_tool      = "~/bin/VS-BRAX Agent-tool-policy"     # gate
post_tool     = ""                            # observer
pre_tool_timeout_ms = 5000
```

Env overrides (always win; empty value disables a config hook):
`ABRASAX_HOOK_TURN_END`, `ABRASAX_HOOK_SESSION_START`, `ABRASAX_HOOK_SESSION_END`,
`ABRASAX_HOOK_PRE_TOOL`, `ABRASAX_HOOK_POST_TOOL`, `ABRASAX_HOOK_PRE_TOOL_TIMEOUT_MS`.

## Common contract

- The hook command line is parsed shell-style (quotes and backslash escapes
  work) but executed **directly**, not through a shell. A leading `~/` in the
  program path is expanded.
- The hook runs in the session working directory when known.
- Every hook receives:

| Variable | Meaning |
| --- | --- |
| `ABRASAX_HOOK_EVENT` | `turn_end`, `session_start`, `session_end`, `pre_tool`, `post_tool` |
| `ABRASAX_HOOK_SESSION_ID` | Session the event belongs to |
| `ABRASAX_HOOK_CWD` | Session working directory |
| `ABRASAX_HOOK_PAYLOAD` | JSON object mirroring all fields (capped at 16 KB) |
| `ABRASAX_HOOKS_DISABLED` | Always `1`; suppresses hooks in nested VS-BRAX Agent calls (recursion guard) |

## Observer hooks

`turn_end`, `session_start`, `session_end`, and `post_tool` are
**observers**: spawned detached, fire-and-forget. They can never block or slow
the agent; failures are only logged.

### `turn_end`

Fires when an agent turn completes (streaming turn path, which covers TUI,
desktop, swarm workers, and headless sessions).

Extra fields: `ABRASAX_HOOK_STATUS` (`ok`/`error`), `ABRASAX_HOOK_DURATION_MS`,
`ABRASAX_HOOK_MODEL`, `ABRASAX_HOOK_LAST_ASSISTANT_TEXT` (first 4000 chars),
`ABRASAX_HOOK_ERROR` (on failure).

### `session_start` / `session_end`

`session_start` fires when an agent session becomes active, with
`ABRASAX_HOOK_SOURCE` = `create` (brand new), `attach` (existing session object
attached), or `resume` (restored by id). `session_end` fires on normal close
(`ABRASAX_HOOK_SOURCE=close`).

### `post_tool`

Fires after every tool call. Extra fields: `ABRASAX_HOOK_TOOL_NAME`,
`ABRASAX_HOOK_STATUS`, `ABRASAX_HOOK_DURATION_MS`, `ABRASAX_HOOK_OUTPUT_BYTES` (on
success), `ABRASAX_HOOK_ERROR` (on failure).

## Gate hook: `pre_tool`

`pre_tool` runs **synchronously before every tool call** and can block it:

- The hook receives `ABRASAX_HOOK_TOOL_NAME` plus the full tool input JSON on
  **stdin** (and a 16 KB-truncated copy in `ABRASAX_HOOK_TOOL_INPUT`).
- **Exit 0**: allow the call.
- **Exit 2**: block the call. The hook's stderr (trimmed, capped at 2000
  chars) is returned to the model as the tool error, so the model can adapt.
- **Anything else fails open** with a logged warning: other exit codes,
  timeout (`pre_tool_timeout_ms`, default 5s), missing binary, spawn errors.

Fail-open is deliberate: a broken policy script should degrade to "no policy"
rather than brick every session. If you need fail-closed semantics, make the
hook itself robust (it is your trust boundary, not VS-BRAX Agent).

### Example policy script

```bash
#!/usr/bin/env bash
# ~/bin/VS-BRAX Agent-tool-policy
# stdin: tool input JSON. Env: ABRASAX_HOOK_TOOL_NAME, ABRASAX_HOOK_SESSION_ID...
input=$(cat)

case "$ABRASAX_HOOK_TOOL_NAME" in
  bash)
    if grep -qE 'rm -rf /([^a-zA-Z]|$)|mkfs|dd if=' <<<"$input"; then
      echo "blocked: destructive shell command" >&2
      exit 2
    fi
    ;;
  write|edit)
    if grep -q '"file_path":"/etc/' <<<"$input"; then
      echo "blocked: writes to /etc are not allowed" >&2
      exit 2
    fi
    ;;
esac
exit 0
```

## Example: tmux status + desktop notification on turn end

```bash
#!/usr/bin/env bash
# ~/bin/VS-BRAX Agent-turn-notify
if [ "$ABRASAX_HOOK_STATUS" = ok ]; then icon=✅; else icon=❌; fi
tmux display-message "VS-BRAX Agent $icon ${ABRASAX_HOOK_SESSION_ID:0:12}" 2>/dev/null
notify-send "VS-BRAX Agent turn $ABRASAX_HOOK_STATUS" \
  "${ABRASAX_HOOK_LAST_ASSISTANT_TEXT:0:120}" 2>/dev/null
exit 0
```

## Example: JSON event log of all hook activity

Point several hooks at one script and fan out on `ABRASAX_HOOK_EVENT`:

```bash
#!/usr/bin/env bash
# ~/bin/VS-BRAX Agent-event-log
echo "$ABRASAX_HOOK_PAYLOAD" >> ~/.local/state/VS-BRAX Agent-events.jsonl
```

```toml
[hooks]
turn_end      = "~/bin/VS-BRAX Agent-event-log"
session_start = "~/bin/VS-BRAX Agent-event-log"
session_end   = "~/bin/VS-BRAX Agent-event-log"
post_tool     = "~/bin/VS-BRAX Agent-event-log"
```

## Design notes

- Hook lookups are config-driven and re-read on config reload; you can add or
  change hooks without restarting VS-BRAX Agent.
- Hot paths (`pre_tool`/`post_tool`) check whether a hook is configured before
  building any payload, so unconfigured hooks cost ~nothing.
- The recursion guard (`ABRASAX_HOOKS_DISABLED=1`) means a hook may safely call
  `VS-BRAX Agent` CLI commands without re-triggering hooks in that nested process.
