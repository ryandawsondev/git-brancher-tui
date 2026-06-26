// 'profile edit' — opens the profile's settings.json in the user's editor.

import path from "path";
import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import { openInEditor } from "../../lib/editor.js";

export async function runProfileEdit(profileName: string): Promise<void> {
  const config = readGitrc();
  const { profileDir } = requireProfile(config, profileName);
  const settingsPath = path.join(profileDir, "settings.json");

  console.log(`\nOpening '${settingsPath}'...\n`);
  await openInEditor(settingsPath);
}
