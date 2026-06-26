import fs from "fs";
import path from "path";
import { execa } from "execa";
import { ProfileSettings } from "../types.js";
import { fatal } from "../errors.js";
import { resolvePath } from "../paths.js";
import { appendAuditLog } from "../audit.js";
import { runHooks } from "../hooks.js";
import { injectFiles } from "./inject.js";

export type CloneMode = "dev" | "pr";
export type Proto = "ssh" | "https";

export interface CloneOptions {
  mode: CloneMode;
  proto: Proto;
  branch: string;
  profileDir: string;
  settings: ProfileSettings;
  depth?: number;
}

export async function cloneBranch(options: CloneOptions): Promise<string> {
  const { mode, proto, branch, profileDir, settings, depth } = options;

  const repo = proto === "https" ? settings.repo.https : settings.repo.ssh;
  const root = resolvePath(mode === "pr" ? settings.paths.pr : settings.paths.dev);

  if (!repo) fatal(`ERROR: repo.${proto} is not set in settings.json`);
  if (!root) fatal(`ERROR: ${mode === "pr" ? "paths.pr" : "paths.dev"} is not set in settings.json`);

  const folder = branch.replace(/[/\\]/g, "-");
  const target = path.join(root, folder);

  console.log(`Protocol : ${proto} -> ${repo}`);
  console.log(`Branch   : ${branch}`);
  console.log(`Target   : ${target}\n`);

  await runHooks(settings.preClone ?? [], "pre-clone");

  fs.mkdirSync(root, { recursive: true });
  if (fs.existsSync(target)) {
    console.log(`Removing existing '${folder}'...`);
    fs.rmSync(target, { recursive: true, force: true });
  }

  console.log(`Cloning '${branch}'...`);

  const cloneArgs: string[] = ["clone", "--branch", branch];
  if (mode === "pr") cloneArgs.push("--single-branch");
  if (depth !== undefined && depth > 0) {
    if (mode !== "pr") console.log(`Note: --depth ${depth} creates a shallow clone (limited history)`);
    cloneArgs.push("--depth", String(depth));
  }
  cloneArgs.push(repo, target);

  await execa("git", cloneArgs, { stdio: "inherit" });

  injectFiles(settings.files ?? [], profileDir, target);

  await runHooks(settings.postClone ?? [], "post-clone", target);

  appendAuditLog({
    ts: new Date().toISOString(),
    command: "clone",
    profile: path.basename(profileDir),
    branch,
    mode,
    proto,
    target,
  });

  return target;
}
