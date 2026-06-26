// 'branch status' handler — prints git status summary for each cloned branch.

import path from "path";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { listBranches, BranchMode } from "../../lib/branch/index.js";
import { getBranchStatus } from "../../lib/branch/ops.js";

export async function runBranchStatus(profileName: string | undefined, mode: BranchMode): Promise<void> {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);
  const label = path.basename(profileDir);
  const branches = listBranches(settings, mode);

  if (branches.length === 0) {
    console.log(`\nNo cloned branches found for profile '${label}'`);
    if (mode !== "both") console.log(`(mode: ${mode})`);
    console.log();
    return;
  }

  console.log(`\nBranch status for '${label}':\n`);
  console.log(
    `  ${"NAME".padEnd(40)} ${"MODE".padEnd(5)} ${"SHA".padEnd(8)} ${"DIRTY".padEnd(6)} ${"↑".padEnd(5)} ${"↓".padEnd(5)}`,
  );
  console.log(`  ${"-".repeat(75)}`);

  const statuses = await Promise.all(branches.map((b) => getBranchStatus(b)));

  for (let i = 0; i < branches.length; i++) {
    const b = branches[i];
    const s = statuses[i];
    const ahead = s.ahead === null ? "?" : String(s.ahead);
    const behind = s.behind === null ? "?" : String(s.behind);
    const dirty = s.dirty === 0 ? "-" : String(s.dirty);
    console.log(
      `  ${b.name.padEnd(40)} ${b.mode.padEnd(5)} ${s.headSha.padEnd(8)} ${dirty.padEnd(6)} ${ahead.padEnd(5)} ${behind.padEnd(5)}`,
    );
  }

  console.log();
}
