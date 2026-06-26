import fs from "fs";
import path from "path";
import inquirer from "inquirer";
import { fatal } from "../../lib/errors.js";
import { readGitrc } from "../../lib/gitrc.js";
import { getProfileDir } from "../../lib/paths.js";
import { PROFILE_TEMPLATES } from "../../lib/templates.js";
import { collectFromTemplate, collectManually } from "../../lib/profile/prompts.js";

export async function runProfileCreate(profileName: string, templateId?: string): Promise<void> {
  const config = readGitrc();
  const profileDir = getProfileDir(config, profileName);

  if (fs.existsSync(profileDir)) {
    fatal(`Profile '${profileName}' already exists at '${profileDir}'`);
  }

  console.log(`\nCreating profile '${profileName}'...\n`);

  let resolvedTemplate = templateId;

  if (!resolvedTemplate) {
    const { choice } = await inquirer.prompt<{ choice: string | null }>([
      {
        type: "list",
        name: "choice",
        message: "Start from a template?",
        choices: [
          { name: "None (manual setup)", value: null },
          ...PROFILE_TEMPLATES.map((t) => ({ name: t.label, value: t.id })),
        ],
      },
    ]);
    resolvedTemplate = choice ?? undefined;
  }

  const settings = resolvedTemplate
    ? await collectFromTemplate(resolvedTemplate, profileName)
    : await collectManually(profileName);

  fs.mkdirSync(path.join(profileDir, "files"), { recursive: true });
  fs.writeFileSync(path.join(profileDir, "settings.json"), JSON.stringify(settings, null, 2) + "\n", "utf8");

  if (!resolvedTemplate) {
    fs.writeFileSync(path.join(profileDir, "files", ".test-env"), "test=...\n", "utf8");
  }

  console.log(`\n✔ Profile '${profileName}' created at '${profileDir}'`);
  console.log(`\nNext steps:`);
  console.log(`  1. Edit ${path.join(profileDir, "settings.json")}`);
  console.log(`  2. Add files to ${path.join(profileDir, "files")}`);
  console.log(`  3. Run: git-brancher -p ${profileName} -b <branch>\n`);
}
