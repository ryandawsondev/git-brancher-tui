// 'config show' and 'config set' — prints and updates ~/.gitrc values.

import { GitRc } from "../lib/types.js";
import { fatal } from "../lib/errors.js";
import { readGitrc, writeGitrc, GITRC_PATH } from "../lib/gitrc.js";

const VALID_KEYS: Array<keyof GitRc> = ["profilesDir", "defaultProfile"];

// ── show ──────────────────────────────────────────────────────────────────────

export function runConfigShow(): void {
  const config = readGitrc();

  console.log(`\nGit Brancher config (${GITRC_PATH}):\n`);
  for (const [key, value] of Object.entries(config)) {
    console.log(`  ${key}: ${value}`);
  }
  console.log();
}

// ── set ───────────────────────────────────────────────────────────────────────

export function runConfigSet(key: string, value: string): void {
  if (!VALID_KEYS.includes(key as keyof GitRc)) {
    fatal(`Unknown config key '${key}'.`, `Valid keys: ${VALID_KEYS.join(", ")}`);
  }

  const config = readGitrc();
  const oldValue = config[key as keyof GitRc];

  (config as unknown as Record<string, string>)[key] = value;
  writeGitrc(config);

  console.log(`\n✔ Updated '${key}'`);
  console.log(`  ${oldValue} -> ${value}\n`);
}
