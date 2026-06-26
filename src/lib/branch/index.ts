import fs from "fs";
import path from "path";
import { ProfileSettings } from "../types.js";
import { resolvePath } from "../paths.js";

export type BranchMode = "dev" | "pr" | "both";

export interface BranchEntry {
  name: string;
  mode: "dev" | "pr";
  dir: string;
  lastModified: Date;
}

function readBranchesFromDir(dir: string, mode: "dev" | "pr"): BranchEntry[] {
  const resolved = resolvePath(dir);
  if (!fs.existsSync(resolved)) return [];

  return fs
    .readdirSync(resolved, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => {
      const branchDir = path.join(resolved, entry.name);
      const stat = fs.statSync(branchDir);
      return { name: entry.name, mode, dir: branchDir, lastModified: stat.mtime };
    })
    .sort((a, b) => b.lastModified.getTime() - a.lastModified.getTime());
}

export function listBranches(settings: ProfileSettings, mode: BranchMode): BranchEntry[] {
  const results: BranchEntry[] = [];
  if (mode === "dev" || mode === "both") results.push(...readBranchesFromDir(settings.paths.dev, "dev"));
  if (mode === "pr" || mode === "both") results.push(...readBranchesFromDir(settings.paths.pr, "pr"));
  return results;
}

export function deleteBranch(entry: BranchEntry): void {
  fs.rmSync(entry.dir, { recursive: true, force: true });
}
