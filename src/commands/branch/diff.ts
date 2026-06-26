// 'branch diff' — show git diff --stat for a cloned branch vs a ref.

import path from "path";
import { execa } from "execa";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { listBranches, BranchMode } from "../../lib/branch/index.js";
import { fatal } from "../../lib/errors.js";

export async function runBranchDiff(
  profileName: string | undefined,
  branchName: string,
  mode: BranchMode,
  ref: string,
): Promise<void> {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);
  const label = path.basename(profileDir);

  const branches = listBranches(settings, mode);
  const sanitized = branchName.replace(/[/\\]/g, "-");
  const entry = branches.find((b) => b.name === branchName || b.name === sanitized);

  if (!entry) {
    fatal(`Branch '${branchName}' not found in [${mode}] for profile '${label}'.`);
  }

  console.log(`\nDiff for '${entry.name}' (${entry.mode}) vs ${ref}:\n`);

  const result = await execa("git", ["-C", entry.dir, "diff", "--stat", ref], {
    stdio: "inherit",
    reject: false,
  });

  if (result.exitCode !== 0) {
    console.error(`\ngit diff exited with code ${result.exitCode}`);
    process.exit(1);
  }
  console.log();
}
