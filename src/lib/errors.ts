// Prints error lines to stderr and exits with code 1.

export function fatal(...lines: string[]): never {
  for (const line of lines) console.error(line);
  process.exit(1);
}
