// Registers the default clone command on the root program — supports multi-branch and interactive picker.

import { Command, Option } from "commander";
import inquirer from "inquirer";
import { fatal } from "../lib/errors.js";
import { CloneMode, Proto } from "../lib/clone/index.js";
import { getRemoteBranches } from "../lib/branch/ops.js";
import { readGitrc } from "../lib/gitrc.js";
import { requireProfile } from "../lib/profile/index.js";
import { runClone } from "../commands/clone.js";

export function registerCloneCommand(program: Command): void {
  program
    .addOption(new Option("-m, --mode <mode>", "Clone mode").choices(["dev", "pr"]).default("dev"))
    .option("-p, --profile <name>", "Profile name")
    .option("-b, --branch <branches>", "Branch(es) to clone (comma-separated; omit for interactive picker)")
    .option("--https", "Use HTTPS instead of SSH")
    .option("--depth <n>", "Shallow clone with limited commit history", (v) => parseInt(v, 10))
    .action(async (options: { mode: CloneMode; profile?: string; branch?: string; https?: boolean; depth?: number }) => {
      const proto: Proto = options.https ? "https" : "ssh";
      let branches: string[];

      if (options.branch) {
        branches = options.branch.split(",").map((b) => b.trim()).filter(Boolean);
      } else {
        const config = readGitrc();
        const { settings } = requireProfile(config, options.profile);
        const repoUrl = proto === "https" ? settings.repo.https : settings.repo.ssh;
        console.log(`\nFetching remote branches from ${repoUrl}...`);
        const remote = await getRemoteBranches(repoUrl);
        if (remote.length === 0) {
          fatal("Could not fetch remote branches. Pass -b <branch> to clone directly.");
        }
        const { selected } = await inquirer.prompt<{ selected: string[] }>([
          {
            type: "checkbox",
            name: "selected",
            message: "Select branch(es) to clone:",
            choices: remote,
            pageSize: 20,
          },
        ]);
        if (selected.length === 0) { console.log("Nothing selected.\n"); return; }
        branches = selected;
      }

      await runClone({
        mode: options.mode,
        profileName: options.profile,
        branches,
        proto,
        depth: options.depth,
      });
    });
}
