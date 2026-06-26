// 'branch pull' — run git pull on every cloned branch for a profile.

import path from "path";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { listBranches, BranchMode } from "../../lib/branch/index.js";
import { pullBranch } from "../../lib/branch/ops.js";

export async function runBranchPull(
  profileName: string | undefined,
  mode: BranchMode,
): Promise<void> {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);
  const label = path.basename(profileDir);
  const branches = listBranches(settings, mode);

  if (branches.length === 0) {
    console.log(`\nNo cloned branches found for profile '${label}'. Nothing to pull.\n`);
    return;
  }

  console.log(`\nPulling ${branches.length} branch(es) for '${label}':\n`);

  let ok = 0;
  let failed = 0;

  for (const entry of branches) {
    process.stdout.write(`  [${entry.mode.toUpperCase()}] ${entry.name.padEnd(48)}`);
    const result = await pullBranch(entry);
    if (result.ok) {
      console.log(`✔ ${result.message}`);
      ok++;
    } else {
      console.log(`✘ ${result.message}`);
      failed++;
    }
  }

  console.log(`\n${ok} succeeded, ${failed} failed.\n`);
}
