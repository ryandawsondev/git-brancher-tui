// 'branch clean' handler — interactive checkbox selection and deletion of stale branch directories.

import path from "path";
import inquirer from "inquirer";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { listBranches, deleteBranch, BranchEntry, BranchMode } from "../../lib/branch/index.js";
import { formatLastModified } from "../../lib/format.js";
import { parseAgeDays } from "../../lib/age.js";

export async function runBranchClean(
  profileName: string | undefined,
  mode: BranchMode,
  olderThan?: string,
): Promise<void> {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);
  const label = path.basename(profileDir);
  const branches = listBranches(settings, mode);

  if (branches.length === 0) {
    console.log(`\nNo cloned branches found for profile '${label}'. Nothing to clean.\n`);
    return;
  }

  let thresholdMs: number | undefined;
  if (olderThan) {
    const days = parseAgeDays(olderThan);
    thresholdMs = Date.now() - days * 24 * 60 * 60 * 1000;
    console.log(`\nSelect branches to delete for profile '${label}' (pre-selected: older than ${days} day(s)):\n`);
  } else {
    console.log(`\nSelect branches to delete for profile '${label}':\n`);
  }

  const choices = branches.map((branch) => ({
    name: `[${branch.mode.toUpperCase()}] ${branch.name.padEnd(48)} ${formatLastModified(branch.lastModified)}`,
    value: branch,
    checked: thresholdMs !== undefined && branch.lastModified.getTime() < thresholdMs,
  }));

  const { selected } = await inquirer.prompt<{ selected: BranchEntry[] }>([
    { type: "checkbox", name: "selected", message: "Space to select, Enter to confirm:", choices, pageSize: 20 },
  ]);

  if (selected.length === 0) {
    console.log("\nNothing selected. Aborted.\n");
    return;
  }

  console.log(`\nAbout to delete ${selected.length} branch(es):`);
  for (const branch of selected) console.log(`  [${branch.mode.toUpperCase()}] ${branch.name}`);

  const { confirm } = await inquirer.prompt<{ confirm: boolean }>([
    { type: "confirm", name: "confirm", message: "Confirm deletion? This cannot be undone.", default: false },
  ]);

  if (!confirm) { console.log("\nAborted.\n"); return; }

  console.log();
  for (const branch of selected) {
    try {
      deleteBranch(branch);
      console.log(`  ✔ Deleted [${branch.mode.toUpperCase()}] ${branch.name}`);
    } catch {
      console.error(`  ✘ Failed to delete [${branch.mode.toUpperCase()}] ${branch.name}`);
    }
  }

  console.log(`\nDone. ${selected.length} branch(es) removed.\n`);
}
