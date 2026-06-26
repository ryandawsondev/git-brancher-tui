// 'branch open' — open a cloned branch directory in VS Code or OS file manager.

import path from "path";
import inquirer from "inquirer";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { listBranches, BranchEntry, BranchMode } from "../../lib/branch/index.js";
import { openDirInEditor } from "../../lib/editor.js";
import { fatal } from "../../lib/errors.js";

export async function runBranchOpen(
  profileName: string | undefined,
  branchName: string | undefined,
  mode: BranchMode,
): Promise<void> {
  const config = readGitrc();
  const { settings, profileDir } = requireProfile(config, profileName);
  const label = path.basename(profileDir);
  const branches = listBranches(settings, mode);

  if (branches.length === 0) {
    fatal(`No cloned branches found for profile '${label}'.`);
  }

  let entry: BranchEntry;

  if (branchName) {
    const sanitized = branchName.replace(/[/\\]/g, "-");
    const found = branches.find((b) => b.name === branchName || b.name === sanitized);
    if (!found) fatal(`Branch '${branchName}' not found for profile '${label}'.`);
    entry = found;
  } else {
    const { selected } = await inquirer.prompt<{ selected: BranchEntry }>([
      {
        type: "list",
        name: "selected",
        message: "Select branch to open:",
        choices: branches.map((b) => ({
          name: `[${b.mode.toUpperCase()}] ${b.name}`,
          value: b,
        })),
        pageSize: 20,
      },
    ]);
    entry = selected;
  }

  console.log(`\nOpening '${entry.name}' (${entry.mode})...\n`);
  await openDirInEditor(entry.dir);
}
