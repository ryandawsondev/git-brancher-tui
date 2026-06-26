// 'branch reclone' handler — stashes uncommitted work, then re-clones branch(es).

import path from "path";
import inquirer from "inquirer";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { resolvePath } from "../../lib/paths.js";
import { listBranches } from "../../lib/branch/index.js";
import { stashAndExport } from "../../lib/branch/ops.js";
import { cloneBranch, CloneMode, Proto } from "../../lib/clone/index.js";

export interface RecloneOptions {
  profileName?: string;
  branches: string[];
  mode: CloneMode;
  proto: Proto;
  depth?: number;
  noStash?: boolean;
}

export async function runBranchReclone(options: RecloneOptions): Promise<void> {
  const { profileName, branches, mode, proto, depth, noStash } = options;

  const config = readGitrc();
  const { profileDir, settings } = requireProfile(config, profileName);
  const label = path.basename(profileDir);

  const root = resolvePath(mode === "pr" ? settings.paths.pr : settings.paths.dev);
  const allBranches = listBranches(settings, mode);

  for (const branch of branches) {
    const folder = branch.replace(/[/\\]/g, "-");
    const target = path.join(root, folder);
    const existingEntry = allBranches.find((b) => b.name === folder);

    console.log(`\nProfile  : ${label}`);
    console.log(`Branch   : ${branch}`);
    console.log(`Target   : ${target}`);
    console.log(`Mode     : ${mode}\n`);

    const { confirm } = await inquirer.prompt<{ confirm: boolean }>([
      {
        type: "confirm",
        name: "confirm",
        message: `Delete and re-clone '${folder}'? This cannot be undone.`,
        default: false,
      },
    ]);

    if (!confirm) {
      console.log("\nSkipped.\n");
      continue;
    }

    if (existingEntry && !noStash) {
      process.stdout.write("  Checking for uncommitted changes...");
      const patchPath = await stashAndExport(existingEntry, root);
      if (patchPath) {
        console.log(` stashed.\n  Patch saved to: ${patchPath}`);
        console.log(`  To reapply after reclone: git apply "${patchPath}"\n`);
      } else {
        console.log(" clean.\n");
      }
    }

    console.log();
    const start = Date.now();
    await cloneBranch({ mode, proto, branch, profileDir, settings, depth });
    const elapsed = ((Date.now() - start) / 1000).toFixed(2);
    console.log(`\n✔ [${branch}] Done in ${elapsed}s`);
  }
}
