// 'init' command — one-time setup that writes ~/.gitrc with a chosen profiles directory.

import fs from "fs";
import path from "path";
import os from "os";
import inquirer from "inquirer";
import { GITRC_PATH, gitrcExists, writeGitrc } from "../lib/gitrc.js";
import { resolvePath } from "../lib/paths.js";

export async function runInit(): Promise<void> {
  console.log("\nGit Brancher — Initial Setup\n");

  if (gitrcExists()) {
    const { overwrite } = await inquirer.prompt<{ overwrite: boolean }>([
      {
        type: "confirm",
        name: "overwrite",
        message: `.gitrc already exists at '${GITRC_PATH}'. Overwrite?`,
        default: false,
      },
    ]);

    if (!overwrite) {
      console.log("Aborted.");
      process.exit(0);
    }
  }

  const defaultProfilesDir = path.join(
    os.homedir(),
    ".git-brancher",
    "profiles",
  );

  const { profilesDir } = await inquirer.prompt<{ profilesDir: string }>([
    {
      type: "input",
      name: "profilesDir",
      message: "Profiles directory:",
      default: defaultProfilesDir,
    },
  ]);

  const { defaultProfile } = await inquirer.prompt<{ defaultProfile: string }>([
    {
      type: "input",
      name: "defaultProfile",
      message: "Default profile name (leave blank to require -p flag each time):",
      default: "",
    },
  ]);

  const resolved = resolvePath(profilesDir);
  fs.mkdirSync(resolved, { recursive: true });

  const gitrc = defaultProfile.trim()
    ? { profilesDir, defaultProfile: defaultProfile.trim() }
    : { profilesDir };

  writeGitrc(gitrc);

  console.log(`\n✔ Written to '${GITRC_PATH}'`);
  console.log(`  profilesDir: ${profilesDir}`);
  if (gitrc.defaultProfile) console.log(`  defaultProfile: ${gitrc.defaultProfile}`);
  console.log(`\nNext: git-brancher profile create <profile-name>\n`);
}
