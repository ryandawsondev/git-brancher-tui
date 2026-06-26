// 'log' command handler — pretty-prints the clone audit log.

import { readAuditLog } from "../lib/audit.js";

export function runLog(count: number, profileFilter?: string): void {
  const all = readAuditLog();
  const entries = all
    .filter((e) => !profileFilter || e.profile === profileFilter)
    .slice(-count)
    .reverse();

  if (entries.length === 0) {
    console.log("\nNo audit log entries found.\n");
    return;
  }

  console.log(`\nAudit log (last ${entries.length} entries):\n`);
  for (const e of entries) {
    const tags = `[${e.mode}/${e.proto}]`;
    console.log(`  ${e.ts}  ${e.command.padEnd(7)}  ${tags.padEnd(12)}  ${e.profile} → ${e.branch}`);
  }
  console.log();
}
