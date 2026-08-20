// VS-BRAX agent_host.ts — TS agent host: spawns the agent runtime and streams its
// output line-by-line to the VS-BRAX right-side terminal (extension webview).
// PRIMAL_HEX: 4f5349524953424c58434b
// This file mirrors the extension host logic in apps/vscode-abrasax so the fork
// can run the harness standalone during development (node agent_host.ts).
import { spawn, ChildProcess } from 'node:child_process';
import { createInterface } from 'node:readline';

const PRIMAL = '4f5349524953424c58434b';
const ROOT = process.env.VSBRAX_ROOT ?? process.cwd();
const LLM_ENDPOINT = process.env.VSBRAX_API_ENDPOINT ?? 'http://127.0.0.1:1234/v1';
const LLM_MODEL = process.env.VSBRAX_API_MODEL ?? 'dolphin3.0-llama3.1-8b';
const LLM_KEY = process.env.VSBRAX_API_KEY ?? 'lm-studio';

interface Emitter {
	emit(line: string): void;
}

export function startAgentHost(emitter: Emitter): ChildProcess {
	const proc = spawn('powershell.exe', ['-NoProfile', '-NoLogo', '-Command', '-'], {
		cwd: ROOT,
		stdio: ['pipe', 'pipe', 'pipe'],
	});
	const stream = (d: Buffer, tag: string): void => {
		for (const line of String(d).split(/\r?\n/)) {
			if (line.trim().length > 0) {
				emitter.emit(`${tag}${line}`);
			}
		}
	};
	proc.stdout?.on('data', (d) => stream(d, ''));
	proc.stderr?.on('data', (d) => stream(d, '[err] '));
	proc.on('close', (code) => emitter.emit(`─ agent exited (${code}) — /start to relaunch ─`));
	proc.on('error', (e) => emitter.emit(`─ spawn error: ${e.message} ─`));
	emitter.emit(`VS-BRAX agent host live — PRIMAL ${PRIMAL}`);
	emitter.emit(`LLM: ${LLM_MODEL} @ ${LLM_ENDPOINT}`);
	return proc;
}

export async function askLocalLlm(prompt: string, system: string): Promise<string> {
	const res = await fetch(`${LLM_ENDPOINT}/chat/completions`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${LLM_KEY}` },
		body: JSON.stringify({
			model: LLM_MODEL,
			messages: [
				{ role: 'system', content: system },
				{ role: 'user', content: prompt },
			],
			max_tokens: 400,
		}),
		signal: AbortSignal.timeout(120000),
	});
	const j = await res.json() as { choices?: Array<{ message?: { content?: string } }> };
	return j.choices?.[0]?.message?.content ?? '';
}

// Standalone REPL mode: node agent_host.ts
if (import.meta.url === `file://${process.argv[1]?.replace(/\\/g, '/')}`) {
	const rl = createInterface({ input: process.stdin, output: process.stdout });
	const proc = startAgentHost({ emit: (line) => console.log(line) });
	rl.on('line', async (line) => {
		const t = line.trim();
		if (t === '/exit') {
			proc.kill();
			process.exit(0);
		}
		if (t.startsWith('/chat ')) {
			console.log('… thinking …');
			console.log(await askLocalLlm(t.slice(6), 'VS-BRAX agent. Answer concisely.'));
			return;
		}
		proc.stdin?.write(t + '\n');
	});
}
