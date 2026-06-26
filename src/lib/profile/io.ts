import fs from "fs";
import path from "path";
import * as tar from "tar";
import { GitRc } from "../types.js";
import { fatal } from "../errors.js";
import { resolveProfilesDir } from "../paths.js";

export async function exportProfile(profileDir: string, outPath: string): Promise<void> {
  const name = path.basename(profileDir);
  const parent = path.dirname(profileDir);
  await tar.c({ gzip: true, file: outPath, cwd: parent }, [name]);
}

export async function importProfile(archivePath: string, profilesDir: string, name: string): Promise<void> {
  const destDir = path.join(profilesDir, name);
  if (fs.existsSync(destDir)) fatal(`Profile '${name}' already exists at '${destDir}'.`);

  fs.mkdirSync(destDir, { recursive: true });
  const tmpDir = path.join(profilesDir, `__tmp_import_${Date.now()}`);
  fs.mkdirSync(tmpDir, { recursive: true });

  try {
    await tar.x({ file: archivePath, cwd: tmpDir });
    const entries = fs.readdirSync(tmpDir);
    if (entries.length !== 1) {
      fatal(`Archive must contain exactly one top-level directory. Found: ${entries.join(", ")}`);
    }
    const extracted = path.join(tmpDir, entries[0]);
    if (!fs.existsSync(path.join(extracted, "settings.json"))) {
      fatal("Archive does not contain settings.json — not a valid profile archive.");
    }
    fs.cpSync(extracted, destDir, { recursive: true });
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
}

export function copyProfile(config: GitRc, srcName: string, destName: string): void {
  const profilesDir = resolveProfilesDir(config);
  const srcDir = path.join(profilesDir, srcName);
  const destDir = path.join(profilesDir, destName);
  if (!fs.existsSync(srcDir)) fatal(`Profile '${srcName}' not found.`);
  if (fs.existsSync(destDir)) fatal(`Profile '${destName}' already exists.`);
  fs.cpSync(srcDir, destDir, { recursive: true });
}
