// Registers all 'profile' subcommands — maps commander actions to runProfile* handlers.

import { Command } from "commander";
import { runProfileCreate } from "../commands/profile/create.js";
import { runProfileDelete } from "../commands/profile/delete.js";
import { runProfileList } from "../commands/profile/list.js";
import { runProfileShow } from "../commands/profile/show.js";
import { runProfileEdit } from "../commands/profile/edit.js";
import { runProfileRename } from "../commands/profile/rename.js";
import { runProfileDoctor } from "../commands/profile/doctor.js";
import { runProfileCopy } from "../commands/profile/copy.js";
import { runProfileExport } from "../commands/profile/export.js";
import { runProfileImport } from "../commands/profile/import.js";
import { runProfileDiff } from "../commands/profile/diff.js";

export function registerProfileCommands(program: Command): void {
  const cmd = program.command("profile").description("Manage profiles");

  cmd.command("create <profile-name>").description("Scaffold a new profile")
    .option("--template <id>", "Start from a template (github, gitlab, bitbucket)")
    .action((name: string, opts: { template?: string }) => runProfileCreate(name, opts.template));

  cmd.command("delete <profile-name>").description("Delete a profile")
    .action((name: string) => runProfileDelete(name));

  cmd.command("list").description("List all profiles with a summary")
    .action(() => runProfileList());

  cmd.command("show <profile-name>").description("Print full settings for a profile")
    .action((name: string) => runProfileShow(name));

  cmd.command("edit <profile-name>").description("Open a profile's settings.json in your editor")
    .action((name: string) => runProfileEdit(name));

  cmd.command("rename <old-name> <new-name>").description("Rename a profile")
    .action((oldName: string, newName: string) => runProfileRename(oldName, newName));

  cmd.command("doctor <profile-name>").description("Health check — settings, paths, files, hooks, reachability")
    .action((name: string) => runProfileDoctor(name));

  cmd.command("copy <src-name> <dest-name>").description("Duplicate an existing profile as a new one")
    .action((src: string, dest: string) => runProfileCopy(src, dest));

  cmd.command("export <profile-name> [outfile]").description("Archive a profile to a .tar.gz file")
    .action(async (name: string, outFile?: string) => runProfileExport(name, outFile));

  cmd.command("import <archive> <profile-name>").description("Import a profile from a .tar.gz archive")
    .action(async (archive: string, name: string) => runProfileImport(archive, name));

  cmd.command("diff <profile-a> <profile-b>").description("Compare two profiles' settings side-by-side")
    .action(async (a: string, b: string) => runProfileDiff(a, b));
}
