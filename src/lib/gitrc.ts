// ~/.gitrc read/write — path constant, existence check, JSON parse, and persist.

import fs from "fs";
import path from "path";
import os from "os";
import { GitRc } from "./types.js";
import { fatal } from "./errors.js";

export const GITRC_PATH = path.join(os.homedir(), ".gitrc");

export function gitrcExists(): boolean {
  return fs.existsSync(GITRC_PATH);
}

export function readGitrc(): GitRc {
  if (!gitrcExists()) {
    fatal("No .gitrc found. Run: git-brancher init");
  }
  const raw = fs.readFileSync(GITRC_PATH, "utf8");
  try {
    return JSON.parse(raw) as GitRc;
  } catch {
    fatal(`Malformed JSON in ${GITRC_PATH}. Fix the file or run: git-brancher init`);
  }
}

export function writeGitrc(config: GitRc): void {
  fs.writeFileSync(GITRC_PATH, JSON.stringify(config, null, 2) + "\n", "utf8");
}
