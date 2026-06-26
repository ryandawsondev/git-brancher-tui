import fs from "fs";
import path from "path";
import { execa } from "execa";
import { ProfileSettings } from "./types.js";
import { checkRepoReachability } from "./reachability.js";
import { resolvePath } from "./paths.js";

export interface CheckResult { msg: string; pass: boolean }

async function commandOnPath(cmd: string): Promise<boolean> {
  const bin = cmd.trim().split(/\s+/)[0];
  const checker = process.platform === "win32" ? "where" : "which";
  try { await execa(checker, [bin], { stdio: "pipe" }); return true; }
  catch { return false; }
}

export function checkSettingsFields(settings: ProfileSettings): CheckResult[] {
  return [
    { msg: "repo.ssh is set", pass: !!settings.repo.ssh },
    { msg: "repo.https is set", pass: !!settings.repo.https },
    { msg: "paths.dev is set", pass: !!settings.paths.dev },
    { msg: "paths.pr is set", pass: !!settings.paths.pr },
  ];
}

export function checkPaths(settings: ProfileSettings): CheckResult[] {
  const devResolved = resolvePath(settings.paths.dev);
  const prResolved = resolvePath(settings.paths.pr);
  return [
    { msg: `paths.dev exists: '${devResolved}'`, pass: fs.existsSync(devResolved) },
    { msg: `paths.pr exists: '${prResolved}'`, pass: fs.existsSync(prResolved) },
  ];
}

export function checkFiles(settings: ProfileSettings, profileDir: string): CheckResult[] {
  const files = settings.files ?? [];
  if (files.length === 0) return [{ msg: "No file mappings defined", pass: true }];
  return files.map(({ source, dest }) => {
    const srcPath = path.join(profileDir, source);
    const exists = fs.existsSync(srcPath);
    return {
      msg: exists ? `File exists: '${source}' -> '${dest}'` : `File not found: '${source}' (expected at '${srcPath}')`,
      pass: exists,
    };
  });
}

export async function checkHooks(settings: ProfileSettings): Promise<CheckResult[]> {
  const results: CheckResult[] = [];
  for (const [label, hooks] of [["pre-clone", settings.preClone ?? []], ["post-clone", settings.postClone ?? []]] as [string, string[]][]) {
    if (hooks.length === 0) {
      results.push({ msg: `No ${label} hooks defined`, pass: true });
    } else {
      results.push({ msg: `${label} hooks (${hooks.length}): ${hooks.join(", ")}`, pass: true });
      for (const cmd of hooks) {
        const bin = cmd.trim().split(/\s+/)[0];
        results.push({ msg: `  ${label} '${bin}' found on PATH`, pass: await commandOnPath(cmd) });
      }
    }
  }
  return results;
}

export async function checkReachabilityResults(settings: ProfileSettings): Promise<CheckResult[]> {
  const reachability = await checkRepoReachability(settings);
  return reachability.map(({ proto, url, ok }) => ({
    msg: ok ? `${proto} reachable: ${url}` : `${proto} unreachable: ${url}`,
    pass: ok,
  }));
}
