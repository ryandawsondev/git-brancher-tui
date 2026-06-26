// 'profile diff <a> <b>' — compare two profiles' settings side-by-side.

import chalk from "chalk";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { diffProfiles } from "../../lib/profile/diff.js";

export async function runProfileDiff(nameA: string, nameB: string): Promise<void> {
  const config = readGitrc();
  const { settings: settingsA } = requireProfile(config, nameA);
  const { settings: settingsB } = requireProfile(config, nameB);

  const diffs = diffProfiles(settingsA, settingsB);

  console.log(`\nProfile diff: '${nameA}' vs '${nameB}'\n`);

  const keyW = 20;
  const valW = 50;
  console.log(`  ${"Key".padEnd(keyW)}  ${"Profile A".padEnd(valW)}  Profile B`);
  console.log(`  ${"─".repeat(keyW)}  ${"─".repeat(valW)}  ${"─".repeat(valW)}`);

  for (const { key, a, b, changed } of diffs) {
    const row = `  ${key.padEnd(keyW)}  ${a.slice(0, valW).padEnd(valW)}  ${b.slice(0, valW)}`;
    console.log(changed ? chalk.yellow(row) : row);
  }

  const changedCount = diffs.filter((d) => d.changed).length;
  console.log(`\n  ${changedCount} field(s) differ.\n`);
}
