/**
 * TypeScript SDK for the abrasax harness API.
 *
 * ```ts
 * import { AbrasaxClient } from "@abrasax/sdk";
 * const client = await AbrasaxClient.connect({ clientName: "my-app/1.0" });
 * const session = await client.createSession(process.cwd());
 * const turn = await client.run(session.session_id, "hello");
 * console.log(turn.text);
 * client.close();
 * ```
 */

export * from "./protocol.js";
export * from "./sockets.js";
export * from "./framing.js";
export { AbrasaxClient, HarnessError, unixSocketTransport } from "./client.js";
export type { ConnectOptions, Transport, TurnResult } from "./client.js";
