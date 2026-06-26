import fs from "fs";
import path from "path";
import { execa } from "execa";
import { BranchEntry } from "./index.js";

export interface BranchStatus {
  headSha: string;
  dirty: number;
  ahead: number | null;
  behind: number | null;
}

export async function pullBranch(entry: BranchEntry): Promise<{ ok: boolean; message: string }> {
  const result = await execa("git", ["-C", entry.dir, "pull"], { reject: false, stdio: "pipe" });
  if (result.exitCode === 0) {
    const out = (result.stdout as string).trim();
    return { ok: true, message: out.split("\n")[0] || "up to date" };
  }
  const err = (result.stderr as string).trim();
  return { ok: false, message: err.split("\n")[0] || "error" };
}

export async function getRemoteBranches(repoUrl: string): Promise<string[]> {
  const result = await execa("git", ["ls-remote", "--heads", repoUrl], {
    reject: false,
    stdio: "pipe",
    timeout: 15000,
  });
  if (result.exitCode !== 0) return [];
  return (result.stdout as string)
    .split("\n")
    .filter(Boolean)
    .map((line) => line.split("\t")[1]?.replace("refs/heads/", "") ?? "")
    .filter(Boolean)
    .sort();
}

export async function stashAndExport(entry: BranchEntry, outDir: string): Promise<string | null> {
  const git = (...args: string[]) =>
    execa("git", ["-C", entry.dir, ...args], { reject: false, stdio: "pipe" });

  const statusResult = await git("status", "--porcelain");
  const isDirty = (statusResult.stdout as string).trim().length > 0;
  if (!isDirty) return null;

  await git("stash", "push", "-m", "git-brancher pre-reclone stash");
  const patchResult = await git("stash", "show", "-p");
  if (patchResult.exitCode !== 0 || !(patchResult.stdout as string).trim()) return null;

  const patchPath = path.join(outDir, `${entry.name}.patch`);
  fs.writeFileSync(patchPath, patchResult.stdout as string, "utf8");
  return patchPath;
}

export async function getBranchStatus(entry: BranchEntry): Promise<BranchStatus> {
  const git = (...args: string[]) =>
    execa("git", ["-C", entry.dir, ...args], { reject: false, stdio: "pipe" });

  const [shaResult, statusResult, aheadResult, behindResult] = await Promise.all([
    git("rev-parse", "--short", "HEAD"),
    git("status", "--porcelain"),
    git("rev-list", "--count", "HEAD@{upstream}..HEAD"),
    git("rev-list", "--count", "HEAD..HEAD@{upstream}"),
  ]);

  const headSha = shaResult.exitCode === 0 ? (shaResult.stdout as string).trim() : "?";
  const dirty = statusResult.exitCode === 0
    ? (statusResult.stdout as string).split("\n").filter(Boolean).length
    : 0;
  const ahead = aheadResult.exitCode === 0 ? parseInt((aheadResult.stdout as string).trim(), 10) : null;
  const behind = behindResult.exitCode === 0 ? parseInt((behindResult.stdout as string).trim(), 10) : null;

  return { headSha, dirty, ahead, behind };
}
