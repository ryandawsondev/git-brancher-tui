// Append-only clone/reclone audit log at ~/.gitrc-log.jsonl.

import fs from "fs";
import path from "path";
import os from "os";

const LOG_PATH = path.join(os.homedir(), ".gitrc-log.jsonl");

export interface AuditEntry {
  ts: string;
  command: "clone" | "reclone";
  profile: string;
  branch: string;
  mode: string;
  proto: string;
  target: string;
}

export function appendAuditLog(entry: AuditEntry): void {
  fs.appendFileSync(LOG_PATH, JSON.stringify(entry) + "\n", "utf8");
}

export function readAuditLog(): AuditEntry[] {
  if (!fs.existsSync(LOG_PATH)) return [];
  return fs
    .readFileSync(LOG_PATH, "utf8")
    .split("\n")
    .filter(Boolean)
    .map((line) => {
      try {
        return JSON.parse(line) as AuditEntry;
      } catch {
        return null;
      }
    })
    .filter(Boolean) as AuditEntry[];
}
