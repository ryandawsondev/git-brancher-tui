import { execa } from "execa";

export async function runHooks(cmds: string[], label: string, cwd?: string): Promise<void> {
  if (cmds.length === 0) return;
  console.log(`\nRunning ${label} hooks...`);
  for (const cmd of cmds) {
    console.log(`\n> ${cmd}`);
    const [bin, ...args] = cmd.split(" ");
    await execa(bin, args, { ...(cwd ? { cwd } : {}), stdio: "inherit" });
  }
}
