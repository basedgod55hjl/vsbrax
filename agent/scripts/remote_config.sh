#!/usr/bin/env bash
# Shared loader for Abrasax remote build defaults.
#
# The config file is intentionally a shell fragment so users can write either:
#   ABRASAX_REMOTE_HOST=builder
# or:
#   export ABRASAX_REMOTE_HOST=builder
#
# Explicit environment variables take precedence over values loaded from the
# config file. This lets callers temporarily disable remote builds with, for
# example, `ABRASAX_REMOTE_CARGO=0 scripts/dev_cargo.sh check`.

abrasax_remote_config_path() {
  if [[ -n "${ABRASAX_REMOTE_CONFIG:-}" ]]; then
    printf '%s\n' "$ABRASAX_REMOTE_CONFIG"
  elif [[ -n "${XDG_CONFIG_HOME:-}" ]]; then
    printf '%s\n' "$XDG_CONFIG_HOME/abrasax/remote-build.env"
  elif [[ -n "${HOME:-}" ]]; then
    printf '%s\n' "$HOME/.config/abrasax/remote-build.env"
  fi
}

abrasax_load_remote_config() {
  local config_file
  config_file="$(abrasax_remote_config_path)"
  [[ -n "$config_file" && -f "$config_file" ]] || return 0

  local had_remote_cargo=0 remote_cargo=""
  local had_remote_host=0 remote_host=""
  local had_remote_dir=0 remote_dir=""
  local had_remote_ssh_bin=0 remote_ssh_bin=""
  local had_remote_rsync_bin=0 remote_rsync_bin=""

  if [[ ${ABRASAX_REMOTE_CARGO+x} ]]; then
    had_remote_cargo=1
    remote_cargo="$ABRASAX_REMOTE_CARGO"
  fi
  if [[ ${ABRASAX_REMOTE_HOST+x} ]]; then
    had_remote_host=1
    remote_host="$ABRASAX_REMOTE_HOST"
  fi
  if [[ ${ABRASAX_REMOTE_DIR+x} ]]; then
    had_remote_dir=1
    remote_dir="$ABRASAX_REMOTE_DIR"
  fi
  if [[ ${ABRASAX_REMOTE_SSH_BIN+x} ]]; then
    had_remote_ssh_bin=1
    remote_ssh_bin="$ABRASAX_REMOTE_SSH_BIN"
  fi
  if [[ ${ABRASAX_REMOTE_RSYNC_BIN+x} ]]; then
    had_remote_rsync_bin=1
    remote_rsync_bin="$ABRASAX_REMOTE_RSYNC_BIN"
  fi

  # shellcheck source=/dev/null
  source "$config_file"

  if [[ "$had_remote_cargo" -eq 1 ]]; then
    ABRASAX_REMOTE_CARGO="$remote_cargo"
  fi
  if [[ "$had_remote_host" -eq 1 ]]; then
    ABRASAX_REMOTE_HOST="$remote_host"
  fi
  if [[ "$had_remote_dir" -eq 1 ]]; then
    ABRASAX_REMOTE_DIR="$remote_dir"
  fi
  if [[ "$had_remote_ssh_bin" -eq 1 ]]; then
    ABRASAX_REMOTE_SSH_BIN="$remote_ssh_bin"
  fi
  if [[ "$had_remote_rsync_bin" -eq 1 ]]; then
    ABRASAX_REMOTE_RSYNC_BIN="$remote_rsync_bin"
  fi
}
