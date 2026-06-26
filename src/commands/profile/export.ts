// 'profile export' — archives a profile directory to a .tar.gz file.

import path from "path";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { exportProfile } from "../../lib/profile/io.js";

export async function runProfileExport(profileName: string, outFile?: string): Promise<void> {
  const config = readGitrc();
  const { profileDir } = requireProfile(config, profileName);

  const outPath = outFile ?? path.join(process.cwd(), `${profileName}.tar.gz`);

  console.log(`\nExporting profile '${profileName}'...`);
  console.log(`  Source : ${profileDir}`);
  console.log(`  Output : ${outPath}\n`);

  await exportProfile(profileDir, outPath);

  console.log(`\n✔ Exported to '${outPath}'\n`);
}
