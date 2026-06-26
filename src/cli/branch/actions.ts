import { Command, Option } from "commander";
import { BranchMode } from "../../lib/branch/index.js";
import { CloneMode, Proto } from "../../lib/clone/index.js";
import { runBranchReclone } from "../../commands/branch/reclone.js";
import { runBranchOpen } from "../../commands/branch/open.js";
import { runBranchPull } from "../../commands/branch/pull.js";
import { runBranchPrune } from "../../commands/branch/prune.js";

export function registerBranchActionCommands(cmd: Command): void {
  cmd.command("reclone").description("Delete and re-clone existing branch(es)")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .requiredOption("-b, --branch <branches>", "Branch(es) to re-clone (comma-separated)")
    .addOption(new Option("-m, --mode <mode>", "Clone mode").choices(["dev", "pr"]).default("dev"))
    .option("--https", "Use HTTPS instead of SSH")
    .option("--depth <n>", "Shallow clone with limited commit history", (v) => parseInt(v, 10))
    .option("--no-stash", "Skip stashing uncommitted changes before reclone")
    .action(async (opts: { profile?: string; branch: string; mode: CloneMode; https?: boolean; depth?: number; stash: boolean }) => {
      await runBranchReclone({
        profileName: opts.profile,
        branches: opts.branch.split(",").map((b) => b.trim()).filter(Boolean),
        mode: opts.mode,
        proto: opts.https ? ("https" as Proto) : ("ssh" as Proto),
        depth: opts.depth,
        noStash: !opts.stash,
      });
    });

  cmd.command("open").description("Open a cloned branch directory in VS Code")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .option("-b, --branch <branch>", "Branch to open (prompts if omitted)")
    .addOption(new Option("-m, --mode <mode>", "Mode").choices(["dev", "pr", "both"]).default("both"))
    .action(async (opts: { profile?: string; branch?: string; mode: BranchMode }) => {
      await runBranchOpen(opts.profile, opts.branch, opts.mode);
    });

  cmd.command("pull").description("Run git pull on all cloned branches for a profile")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .addOption(new Option("-m, --mode <mode>", "Mode").choices(["dev", "pr", "both"]).default("both"))
    .action(async (opts: { profile?: string; mode: BranchMode }) => {
      await runBranchPull(opts.profile, opts.mode);
    });

  cmd.command("prune").description("Delete local branches no longer on the remote")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .addOption(new Option("-m, --mode <mode>", "Mode").choices(["dev", "pr", "both"]).default("both"))
    .option("--https", "Use HTTPS to query remote branch list")
    .action(async (opts: { profile?: string; mode: BranchMode; https?: boolean }) => {
      await runBranchPrune(opts.profile, opts.mode, opts.https ? ("https" as Proto) : ("ssh" as Proto));
    });
}
