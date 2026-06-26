// 'profile list' — prints a summary of all profiles found in profilesDir.

import { readGitrc } from "../../lib/gitrc.js";
import { listProfiles } from "../../lib/profile/index.js";

export function runProfileList(): void {
  const config = readGitrc();
  const profiles = listProfiles(config);

  if (profiles.length === 0) {
    console.log("\nNo profiles found.");
    console.log("Run: git-brancher profile create <profile-name>\n");
    return;
  }

  console.log(`\nProfiles (${profiles.length}):\n`);

  for (const profile of profiles) {
    if (!profile.settings) {
      console.log(`  ${profile.name}`);
      console.log(`    [!] Missing or invalid settings.json`);
      console.log();
      continue;
    }

    console.log(`  ${profile.name}`);
    console.log(`    SSH   : ${profile.settings.repo.ssh}`);
    console.log(`    HTTPS : ${profile.settings.repo.https}`);
    console.log(`    Dev   : ${profile.settings.paths.dev}`);
    console.log(`    PR    : ${profile.settings.paths.pr}`);
    console.log(`    Files : ${profile.settings.files?.length ?? 0} mapped`);
    console.log();
  }
}
