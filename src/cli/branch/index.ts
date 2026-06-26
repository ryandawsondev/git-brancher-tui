import { Command } from "commander";
import { registerBranchReadCommands } from "./read.js";
import { registerBranchActionCommands } from "./actions.js";

export function registerBranchCommands(program: Command): void {
  const cmd = program.command("branch").description("Manage cloned branches").enablePositionalOptions();
  registerBranchReadCommands(cmd);
  registerBranchActionCommands(cmd);
}
