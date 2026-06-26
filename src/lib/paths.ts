// Path utilities — expands home dir shortcuts and derives profile directory paths.

import path from "path";
import os from "os";
import { GitRc } from "./types.js";

export function resolvePath(p: string): string {
  const home = os.homedir();
  // handles both Unix (~) and Windows (%USERPROFILE%) conventions
  return p.replace(/^~/, home).replace(/%USERPROFILE%/g, home);
}

export function resolveProfilesDir(config: GitRc): string {
  return resolvePath(config.profilesDir);
}

export function getProfileDir(config: GitRc, profileName: string): string {
  return path.join(resolveProfilesDir(config), profileName);
}
