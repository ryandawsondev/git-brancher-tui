// 'profile copy' — duplicates an existing profile directory as a new profile.

import path from "path";
import { readGitrc } from "../../lib/gitrc.js";
import { copyProfile } from "../../lib/profile/io.js";
import { resolveProfilesDir } from "../../lib/paths.js";

export function runProfileCopy(srcName: string, destName: string): void {
  const config = readGitrc();

  copyProfile(config, srcName, destName);

  const destDir = path.join(resolveProfilesDir(config), destName);
  console.log(`\n✔ Copied '${srcName}' -> '${destName}'`);
  console.log(`  Path: ${destDir}`);
  console.log(`\nEdit the new profile's settings:`);
  console.log(`  git-brancher profile edit ${destName}\n`);
}
