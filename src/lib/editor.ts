// Opens a file in an editor — tries VS Code first, falls back to $EDITOR/$VISUAL.

import { execa } from "execa";
import { fatal } from "./errors.js";

export async function openInEditor(filePath: string): Promise<void> {
  try {
    // --wait blocks until the editor tab is closed, keeping the process alive
    await execa("code", ["--wait", filePath], { stdio: "inherit" });
    return;
  } catch {
    // code not available
  }

  const editor = process.env.EDITOR ?? process.env.VISUAL ?? null;
  if (editor) {
    try {
      await execa(editor, [filePath], { stdio: "inherit" });
      return;
    } catch {
      // editor failed
    }
  }

  fatal("No editor found. Set $EDITOR or install the VS Code 'code' CLI.");
}

export async function openDirInEditor(dirPath: string): Promise<void> {
  try {
    await execa("code", [dirPath], { stdio: "pipe" });
    return;
  } catch {
    // code not available
  }

  const opener =
    process.platform === "win32" ? "explorer"
    : process.platform === "darwin" ? "open"
    : "xdg-open";

  try {
    await execa(opener, [dirPath], { stdio: "pipe" });
  } catch {
    console.log(`Could not open directory. Open manually: ${dirPath}`);
  }
}
