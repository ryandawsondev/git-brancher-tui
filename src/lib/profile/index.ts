import fs from "fs";
import path from "path";
import os from "os";
import { GitRc, ProfileSettings, ResolvedProfile } from "../types.js";
import { fatal } from "../errors.js";
import { resolveProfilesDir, getProfileDir } from "../paths.js";

export interface ProfileSummary {
  name: string;
  profileDir: string;
  settings: ProfileSettings | null;
  settingsPath: string;
}

export function defaultProfileSettings(profileName: string): ProfileSettings {
  return {
    repo: {
      ssh: "ssh://git@ssh.bitbucket.example.net:7999/code/repo.git",
      https: "https://bitbucket.example.net/scm/code/repo.git",
    },
    paths: {
      dev: path.join(os.homedir(), "git-projects", profileName, "dev", "branches"),
      pr: path.join(os.homedir(), "git-projects", profileName, "pr", "branches"),
    },
    files: [{ source: "files/.test-env", dest: ".env" }],
  };
}

export function listProfiles(config: GitRc): ProfileSummary[] {
  const profilesDir = resolveProfilesDir(config);
  if (!fs.existsSync(profilesDir)) return [];

  return fs
    .readdirSync(profilesDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => {
      const profileDir = path.join(profilesDir, entry.name);
      const settingsPath = path.join(profileDir, "settings.json");
      let settings: ProfileSettings | null = null;
      if (fs.existsSync(settingsPath)) {
        try { settings = JSON.parse(fs.readFileSync(settingsPath, "utf8")) as ProfileSettings; }
        catch { /* leave null */ }
      }
      return { name: entry.name, profileDir, settings, settingsPath };
    });
}

export function requireProfile(config: GitRc, profileName?: string): ResolvedProfile {
  const resolved = profileName ?? config.defaultProfile;
  if (!resolved) {
    fatal(
      "No profile specified and no defaultProfile set in ~/.gitrc.",
      "Pass -p <profile> or run: git-brancher config set defaultProfile <name>",
    );
  }
  const profileDir = getProfileDir(config, resolved);
  const settingsPath = path.join(profileDir, "settings.json");

  if (!fs.existsSync(profileDir)) {
    fatal(`Profile '${resolved}' not found.`, `Run: git-brancher profile create ${resolved}`);
  }
  if (!fs.existsSync(settingsPath)) {
    fatal(`Profile '${resolved}' is missing settings.json.`, `Run: git-brancher profile create ${resolved}`);
  }

  let settings: ProfileSettings;
  try {
    settings = JSON.parse(fs.readFileSync(settingsPath, "utf8")) as ProfileSettings;
  } catch {
    fatal(`Malformed JSON in ${settingsPath}. Fix the file or re-create the profile.`);
  }
  return { profileDir, settings };
}
