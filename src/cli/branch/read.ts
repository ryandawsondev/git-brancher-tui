import { Command, Option } from "commander";
import { BranchMode } from "../../lib/branch/index.js";
import { runBranchList } from "../../commands/branch/list.js";
import { runBranchClean } from "../../commands/branch/clean.js";
import { runBranchStatus } from "../../commands/branch/status.js";
import { runBranchDiff } from "../../commands/branch/diff.js";

export function registerBranchReadCommands(cmd: Command): void {
  cmd.command("list").description("List cloned branches for a profile")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .addOption(new Option("-m, --mode <mode>", "Mode").choices(["dev", "pr", "both"]).default("both"))
    .option("--older-than <age>", "Only show branches not modified in N days (e.g. 30, 4w, 1m)")
    .action((opts: { profile?: string; mode: BranchMode; olderThan?: string }) => {
      runBranchList(opts.profile, opts.mode, opts.olderThan);
    });

  cmd.command("clean").description("Interactively delete stale cloned branches")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .addOption(new Option("-m, --mode <mode>", "Mode").choices(["dev", "pr", "both"]).default("both"))
    .option("--older-than <age>", "Pre-select branches not modified in N days (e.g. 30, 4w, 1m)")
    .action(async (opts: { profile?: string; mode: BranchMode; olderThan?: string }) => {
      await runBranchClean(opts.profile, opts.mode, opts.olderThan);
    });

  cmd.command("status").description("Show git status summary for all cloned branches")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .addOption(new Option("-m, --mode <mode>", "Mode").choices(["dev", "pr", "both"]).default("both"))
    .action(async (opts: { profile?: string; mode: BranchMode }) => {
      await runBranchStatus(opts.profile, opts.mode);
    });

  cmd.command("diff").description("Show git diff --stat for a cloned branch vs a ref")
    .option("-p, --profile <name>", "Profile name (default from ~/.gitrc if set)")
    .requiredOption("-b, --branch <branch>", "Branch to diff")
    .addOption(new Option("-m, --mode <mode>", "Mode").choices(["dev", "pr", "both"]).default("both"))
    .option("--ref <ref>", "Ref to diff against", "HEAD@{upstream}")
    .action(async (opts: { profile?: string; branch: string; mode: BranchMode; ref: string }) => {
      await runBranchDiff(opts.profile, opts.branch, opts.mode, opts.ref);
    });
}
