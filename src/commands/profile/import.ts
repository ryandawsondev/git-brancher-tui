// 'profile import' — extracts a profile archive into the profiles directory.

import path from "path";
import { readGitrc } from "../../lib/gitrc.js";
import { importProfile } from "../../lib/profile/io.js";
import { resolveProfilesDir } from "../../lib/paths.js";

export async function runProfileImport(archivePath: string, profileName: string): Promise<void> {
  const config = readGitrc();
  const profilesDir = resolveProfilesDir(config);

  console.log(`\nImporting profile '${profileName}'...`);
  console.log(`  Archive     : ${archivePath}`);
  console.log(`  Profiles dir: ${profilesDir}\n`);

  await importProfile(archivePath, profilesDir, profileName);

  const destDir = path.join(profilesDir, profileName);
  console.log(`\n✔ Imported '${profileName}' to '${destDir}'\n`);
}
