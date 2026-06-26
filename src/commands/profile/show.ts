// 'profile show' — prints the full settings.json for a profile.

import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";

export function runProfileShow(profileName: string): void {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);

  console.log(`\nProfile: ${profileName}`);
  console.log(`Path   : ${profileDir}\n`);
  console.log("--- settings.json ---\n");
  console.log(JSON.stringify(settings, null, 2));
  console.log();
}
