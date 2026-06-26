// 'branch list' handler — prints cloned branches grouped by mode with age highlighting.

import path from "path";
import chalk from "chalk";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { listBranches, BranchEntry, BranchMode } from "../../lib/branch/index.js";
import { formatLastModified } from "../../lib/format.js";
import { parseAgeDays } from "../../lib/age.js";

export function runBranchList(
  profileName: string | undefined,
  mode: BranchMode,
  olderThan?: string,
): void {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);
  const label = path.basename(profileDir);
  let branches = listBranches(settings, mode);

  if (olderThan) {
    const days = parseAgeDays(olderThan);
    const threshold = Date.now() - days * 24 * 60 * 60 * 1000;
    branches = branches.filter((b) => b.lastModified.getTime() < threshold);
  }

  if (branches.length === 0) {
    console.log(`\nNo cloned branches found for profile '${label}'`);
    if (mode !== "both") console.log(`(mode: ${mode})`);
    if (olderThan) console.log(`(older-than filter: ${olderThan})`);
    console.log();
    return;
  }

  console.log(`\nCloned branches for '${label}' (${branches.length}):\n`);

  const groups: Array<{ label: string; entries: BranchEntry[] }> =
    mode === "both"
      ? [
          { label: "DEV", entries: branches.filter((b) => b.mode === "dev") },
          { label: "PR", entries: branches.filter((b) => b.mode === "pr") },
        ]
      : [{ label: mode.toUpperCase(), entries: branches }];

  for (const group of groups) {
    if (group.entries.length === 0) continue;
    console.log(`  [${group.label}]`);
    for (const branch of group.entries) {
      const age = formatLastModified(branch.lastModified);
      const diffDays = Math.floor((Date.now() - branch.lastModified.getTime()) / (1000 * 60 * 60 * 24));
      const coloredAge = diffDays >= 30 ? chalk.red(age) : diffDays >= 7 ? chalk.yellow(age) : age;
      console.log(`    ${branch.name.padEnd(50)} ${coloredAge}`);
    }
    console.log();
  }
}
