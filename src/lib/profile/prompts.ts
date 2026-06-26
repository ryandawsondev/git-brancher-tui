import inquirer from "inquirer";
import { ProfileSettings } from "../types.js";
import { fatal } from "../errors.js";
import { defaultProfileSettings } from "./index.js";
import { PROFILE_TEMPLATES } from "../templates.js";

export async function collectFromTemplate(templateId: string, profileName: string): Promise<ProfileSettings> {
  const template = PROFILE_TEMPLATES.find((t) => t.id === templateId);
  if (!template) {
    fatal(`Unknown template '${templateId}'. Available: ${PROFILE_TEMPLATES.map((t) => t.id).join(", ")}`);
  }

  const { org } = await inquirer.prompt<{ org: string }>([
    { type: "input", name: "org", message: "Organization / username:" },
  ]);
  const { repo } = await inquirer.prompt<{ repo: string }>([
    { type: "input", name: "repo", message: "Repository name:" },
  ]);

  const built = template.build(org, repo, profileName);

  const { devDir } = await inquirer.prompt<{ devDir: string }>([
    { type: "input", name: "devDir", message: "Dev branches directory:", default: built.paths.dev },
  ]);
  const { prDir } = await inquirer.prompt<{ prDir: string }>([
    { type: "input", name: "prDir", message: "PR branches directory:", default: built.paths.pr },
  ]);
  const { postCloneRaw } = await inquirer.prompt<{ postCloneRaw: string }>([
    { type: "input", name: "postCloneRaw", message: "Post-clone commands (comma-separated, or leave blank):", default: "" },
  ]);
  const postClone = postCloneRaw.split(",").map((s) => s.trim()).filter(Boolean);

  return { ...built, paths: { dev: devDir, pr: prDir }, ...(postClone.length > 0 && { postClone }) };
}

export async function collectManually(profileName: string): Promise<ProfileSettings> {
  const defaults = defaultProfileSettings(profileName);

  const { repoSsh } = await inquirer.prompt<{ repoSsh: string }>([
    { type: "input", name: "repoSsh", message: "Repo SSH URL:", default: defaults.repo.ssh },
  ]);
  const { repoHttps } = await inquirer.prompt<{ repoHttps: string }>([
    { type: "input", name: "repoHttps", message: "Repo HTTPS URL:", default: defaults.repo.https },
  ]);
  const { devDir } = await inquirer.prompt<{ devDir: string }>([
    { type: "input", name: "devDir", message: "Dev branches directory:", default: defaults.paths.dev },
  ]);
  const { prDir } = await inquirer.prompt<{ prDir: string }>([
    { type: "input", name: "prDir", message: "PR branches directory:", default: defaults.paths.pr },
  ]);
  const { postCloneRaw } = await inquirer.prompt<{ postCloneRaw: string }>([
    { type: "input", name: "postCloneRaw", message: "Post-clone commands (comma-separated, or leave blank):", default: "" },
  ]);
  const postClone = postCloneRaw.split(",").map((s) => s.trim()).filter(Boolean);

  return {
    repo: { ssh: repoSsh, https: repoHttps },
    paths: { dev: devDir, pr: prDir },
    files: [{ source: "files/.test-env", dest: ".env" }],
    ...(postClone.length > 0 && { postClone }),
  };
}
