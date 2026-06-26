// 'clone' command handler — resolves profile, runs clone for each branch, opens result in VS Code.

import path from "path";
import { execa } from "execa";
import { readGitrc } from "../lib/gitrc.js";
import { requireProfile } from "../lib/profile/index.js";
import { printBanner } from "../lib/banner.js";
import { cloneBranch, CloneMode, Proto } from "../lib/clone/index.js";
import pkg from "../../package.json" with { type: "json" };

export interface CloneCommandOptions {
  mode: CloneMode;
  profileName?: string;
  branches: string[];
  proto: Proto;
  depth?: number;
}

export async function runClone(options: CloneCommandOptions): Promise<void> {
  const { mode, profileName, branches, proto, depth } = options;

  const description =
    mode === "pr"
      ? "Clones repo branch into the PR area (single-branch)."
      : "Clones repo branch into the DEV area (with history).";

  printBanner(`Git Brancher (${mode})`, pkg.version, description);

  const config = readGitrc();
  const { profileDir, settings } = requireProfile(config, profileName);

  console.log(`Profile  : ${path.basename(profileDir)}`);

  let lastTarget: string | undefined;

  for (const branch of branches) {
    if (branches.length > 1) console.log(`\n--- Branch: ${branch} ---`);
    const start = Date.now();
    const target = await cloneBranch({ mode, proto, branch, profileDir, settings, depth });
    const elapsed = ((Date.now() - start) / 1000).toFixed(2);
    console.log(`\n✔ ${branches.length > 1 ? `[${branch}] ` : ""}Done in ${elapsed}s`);
    lastTarget = target;
  }

  if (lastTarget) {
    try {
      await execa("code", [lastTarget]);
    } catch {
      console.log("'code' CLI not found — skipping VS Code launch.");
    }
  }
}
