#!/usr/bin/env bash
set -euo pipefail

# Publishable Terminal-Bench 2.1 campaign for abrasax + Opus 4.8 (native Anthropic
# API). Runs with effectively-uncapped agent execution time so no task is lost
# to an agent timeout, while keeping verifier/build timeouts at their default
# (deterministic grading). Captures full provenance for publication.
#
# Env knobs:
#   ABRASAX_TB_JOBS_DIR   output dir (default /tmp/abrasax-tb21-pub)
#   ABRASAX_TB_K          attempts per task (default 1)
#   ABRASAX_TB_NCONC      concurrent containers (default 3)
#   ABRASAX_TB_AGENT_MULT agent-timeout multiplier (default 1000 ~ uncapped)

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
REPO_ROOT=$(cd -- "$SCRIPT_DIR/.." && pwd)

JOBS_DIR=${ABRASAX_TB_JOBS_DIR:-/tmp/abrasax-tb21-pub}
K=${ABRASAX_TB_K:-1}
NCONC=${ABRASAX_TB_NCONC:-3}
AGENT_MULT=${ABRASAX_TB_AGENT_MULT:-1000}
JOB_NAME=${ABRASAX_TB_JOB_NAME:-tb21-opus48-uncapped-k${K}}
TB_PATH=${ABRASAX_TB_PATH:-/tmp/terminal-bench-2.1}
MODEL=${ABRASAX_TB_MODEL:-anthropic-api/claude-opus-4-8}

mkdir -p "$JOBS_DIR"

# Provenance manifest.
{
  echo "# abrasax Terminal-Bench 2.1 publishable run"
  echo "timestamp_utc: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "git_commit: $(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
  echo "git_describe: $(git -C "$REPO_ROOT" describe --tags --always --dirty 2>/dev/null || echo unknown)"
  echo "abrasax_binary: ${ABRASAX_HARBOR_BINARY:-/tmp/abrasax-compat-dist/abrasax-linux-x86_64.bin}"
  echo "abrasax_version: $(/tmp/abrasax-compat-dist/abrasax-linux-x86_64.bin --no-update --no-selfdev version 2>/dev/null | head -1 || echo unknown)"
  echo "harbor_version: $(harbor --version 2>/dev/null | head -1 || echo unknown)"
  echo "dataset: terminal-bench/terminal-bench-2-1 (local: $TB_PATH)"
  echo "n_tasks: $(ls "$TB_PATH" | wc -l)"
  echo "model: $MODEL"
  echo "reasoning_effort: ${ABRASAX_ANTHROPIC_REASONING_EFFORT:-high}"
  echo "k_attempts: $K"
  echo "n_concurrent: $NCONC"
  echo "agent_timeout_multiplier: $AGENT_MULT"
  echo "verifier_timeout: dataset default (unchanged)"
} > "$JOBS_DIR/RUN_MANIFEST.txt"
cat "$JOBS_DIR/RUN_MANIFEST.txt"

exec "$REPO_ROOT/scripts/run_terminal_bench_claude.sh" \
  --path "$TB_PATH" \
  --model "$MODEL" \
  --n-concurrent "$NCONC" \
  -k "$K" \
  --agent-timeout-multiplier "$AGENT_MULT" \
  --jobs-dir "$JOBS_DIR" \
  --job-name "$JOB_NAME" \
  --yes
