// 'profile delete' — confirms then removes the profile directory.

import fs from "fs";
import inquirer from "inquirer";
import { fatal } from "../../lib/errors.js";
import { readGitrc } from "../../lib/gitrc.js";
import { getProfileDir } from "../../lib/paths.js";

export async function runProfileDelete(profileName: string): Promise<void> {
  const config = readGitrc();
  const profileDir = getProfileDir(config, profileName);

  if (!fs.existsSync(profileDir)) {
    fatal(`Profile '${profileName}' not found at '${profileDir}'`);
  }

  const { confirm } = await inquirer.prompt<{ confirm: boolean }>([
    {
      type: "confirm",
      name: "confirm",
      message: `Delete profile '${profileName}' at '${profileDir}'? This cannot be undone.`,
      default: false,
    },
  ]);

  if (!confirm) {
    console.log("Aborted.");
    process.exit(0);
  }

  fs.rmSync(profileDir, { recursive: true, force: true });
  console.log(`\n✔ Profile '${profileName}' deleted.\n`);
}
