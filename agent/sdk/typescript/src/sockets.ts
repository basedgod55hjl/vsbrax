/**
 * Socket path resolution, mirroring `abrasax-harness-api::sockets`.
 *
 * The bridge and every client must resolve the same directory or nothing can
 * connect, so the rules here follow the Rust module exactly.
 */

import os from "node:os";
import path from "node:path";

export function runtimeDir(): string {
  const explicit = process.env.ABRASAX_RUNTIME_DIR;
  if (explicit) return explicit;
  const xdg = process.env.XDG_RUNTIME_DIR;
  if (xdg) return xdg;
  if (process.platform === "darwin" && process.env.TMPDIR) {
    return process.env.TMPDIR;
  }
  return path.join(os.tmpdir(), `abrasax-${userDiscriminator()}`);
}

function userDiscriminator(): string {
  const raw =
    process.platform === "win32"
      ? (process.env.USERNAME ?? process.env.USER)
      : (process.env.UID ?? process.env.USER);
  return sanitize(raw ?? "user");
}

function sanitize(raw: string): string {
  const out = raw
    .split("")
    .filter((ch) => /[A-Za-z0-9\-_]/.test(ch))
    .slice(0, 64)
    .join("");
  return out === "" ? "user" : out;
}

/** Path of the versioned harness API socket. `ABRASAX_API_SOCKET` overrides. */
export function apiSocketPath(): string {
  return process.env.ABRASAX_API_SOCKET ?? path.join(runtimeDir(), "abrasax-api.sock");
}

/** Path of the internal daemon socket. `ABRASAX_SOCKET` overrides. */
export function legacySocketPath(): string {
  return process.env.ABRASAX_SOCKET ?? path.join(runtimeDir(), "abrasax.sock");
}
