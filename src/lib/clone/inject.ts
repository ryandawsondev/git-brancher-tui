import fs from "fs";
import path from "path";
import { fatal } from "../errors.js";

export function injectFiles(
  files: Array<{ source: string; dest: string }>,
  profileDir: string,
  target: string,
): void {
  for (const { source, dest } of files) {
    const src = path.join(profileDir, source);
    const dst = path.join(target, dest);
    if (!fs.existsSync(src)) fatal(`ERROR: Source file not found: '${src}'`);
    fs.mkdirSync(path.dirname(dst), { recursive: true });
    fs.copyFileSync(src, dst);
    console.log(`Copied  '${source}' -> '${dest}'`);
  }
}
