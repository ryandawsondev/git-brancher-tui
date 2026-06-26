// 'branch prune' — delete local cloned dirs for branches no longer on the remote.

import path from "path";
import inquirer from "inquirer";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { listBranches, deleteBranch, BranchMode } from "../../lib/branch/index.js";
import { getRemoteBranches } from "../../lib/branch/ops.js";
import { Proto } from "../../lib/clone/index.js";

export async function runBranchPrune(
  profileName: string | undefined,
  mode: BranchMode,
  proto: Proto,
): Promise<void> {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);
  const label = path.basename(profileDir);

  const repoUrl = proto === "https" ? settings.repo.https : settings.repo.ssh;

  console.log(`\nFetching remote branches from ${repoUrl}...`);
  const remoteBranches = await getRemoteBranches(repoUrl);

  if (remoteBranches.length === 0) {
    console.log("Could not fetch remote branches or repo is empty.\n");
    return;
  }

  // remote branch names use / but local dirs have / replaced with -
  const remoteSanitized = new Set(remoteBranches.map((b) => b.replace(/[/\\]/g, "-")));
  const local = listBranches(settings, mode);
  const orphaned = local.filter((b) => !remoteSanitized.has(b.name));

  if (orphaned.length === 0) {
    console.log(`\nNo orphaned branches found for profile '${label}'.\n`);
    return;
  }

  console.log(`\nFound ${orphaned.length} local branch(es) not on remote:\n`);
  for (const b of orphaned) console.log(`  [${b.mode.toUpperCase()}] ${b.name}`);

  const { confirm } = await inquirer.prompt<{ confirm: boolean }>([
    {
      type: "confirm",
      name: "confirm",
      message: `Delete these ${orphaned.length} branch(es)? This cannot be undone.`,
      default: false,
    },
  ]);

  if (!confirm) { console.log("\nAborted.\n"); return; }

  console.log();
  for (const b of orphaned) {
    try {
      deleteBranch(b);
      console.log(`  ✔ Deleted [${b.mode.toUpperCase()}] ${b.name}`);
    } catch {
      console.error(`  ✘ Failed to delete [${b.mode.toUpperCase()}] ${b.name}`);
    }
  }
  console.log(`\nDone. ${orphaned.length} branch(es) removed.\n`);
}
