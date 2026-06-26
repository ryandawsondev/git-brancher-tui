#!/usr/bin/env node
// CLI entry point — wires all commander subcommands and parses argv.
import { program } from "commander";
import pkg from "../package.json" with { type: "json" };
import { runInit } from "./commands/init.js";
import { runConfigShow, runConfigSet } from "./commands/config.js";
import { registerProfileCommands } from "./cli/profile.js";
import { registerBranchCommands } from "./cli/branch/index.js";
import { registerCloneCommand } from "./cli/clone.js";
import { runLog } from "./commands/log.js";

program
  .name("git-brancher")
  .description("Clone repo branches into dev/pr areas with profile-based configuration.")
  .version(pkg.version)
  .enablePositionalOptions();

program.command("init").description("One-time global setup — writes ~/.gitrc").action(runInit);

registerProfileCommands(program);
registerBranchCommands(program);

const configCmd = program.command("config").description("View and update ~/.gitrc");
configCmd.command("show").description("Print current config").action(() => runConfigShow());
configCmd.command("set <key> <value>").description("Update a config value (e.g. profilesDir)")
  .action((key: string, value: string) => runConfigSet(key, value));

registerCloneCommand(program);

program.command("log").description("Show clone audit log (~/.gitrc-log.jsonl)")
  .option("--n <count>", "Number of entries to show", (v) => parseInt(v, 10), 20)
  .option("-p, --profile <name>", "Filter by profile name")
  .action((opts: { n: number; profile?: string }) => runLog(opts.n, opts.profile));

program.parseAsync(process.argv);
