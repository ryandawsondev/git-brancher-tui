export function parseAgeDays(val: string): number {
  const m = val.trim().match(/^(\d+)(d|w|m)?$/i);
  if (!m) throw new Error(`Invalid --older-than value: '${val}'. Use a number of days (e.g. 30, 30d, 4w, 1m).`);
  const n = parseInt(m[1], 10);
  const unit = (m[2] ?? "d").toLowerCase();
  if (unit === "w") return n * 7;
  if (unit === "m") return n * 30;
  return n;
}
