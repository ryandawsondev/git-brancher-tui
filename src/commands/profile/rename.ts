// 'profile rename' — moves the profile directory from old name to new.

import fs from "fs";
import inquirer from "inquirer";
import { fatal } from "../../lib/errors.js";
import { readGitrc } from "../../lib/gitrc.js";
import { getProfileDir } from "../../lib/paths.js";

export async function runProfileRename(oldName: string, newName: string): Promise<void> {
  const config = readGitrc();
  const oldDir = getProfileDir(config, oldName);
  const newDir = getProfileDir(config, newName);

  if (!fs.existsSync(oldDir)) {
    fatal(`Profile '${oldName}' not found at '${oldDir}'`);
  }

  if (fs.existsSync(newDir)) {
    fatal(`Profile '${newName}' already exists at '${newDir}'`);
  }

  const { confirm } = await inquirer.prompt<{ confirm: boolean }>([
    {
      type: "confirm",
      name: "confirm",
      message: `Rename profile '${oldName}' to '${newName}'?`,
      default: true,
    },
  ]);

  if (!confirm) {
    console.log("Aborted.");
    process.exit(0);
  }

  fs.renameSync(oldDir, newDir);
  console.log(`\n✔ Profile renamed '${oldName}' -> '${newName}'\n`);
}
