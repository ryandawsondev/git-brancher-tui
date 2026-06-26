// Checks whether a profile's repo URLs are reachable via git ls-remote.

import { execa } from "execa";
import { ProfileSettings } from "./types.js";

export interface ReachabilityResult {
  proto: string;
  url: string;
  ok: boolean;
}

export async function checkRepoReachability(
  settings: ProfileSettings,
): Promise<ReachabilityResult[]> {
  const checks: Array<[string, string]> = [
    ["SSH", settings.repo.ssh],
    ["HTTPS", settings.repo.https],
  ];

  const results: ReachabilityResult[] = [];
  for (const [proto, url] of checks) {
    try {
      await execa("git", ["ls-remote", "--exit-code", url, "HEAD"], {
        timeout: 10000,
        reject: true,
        stdio: "pipe",
      });
      results.push({ proto, url, ok: true });
    } catch {
      results.push({ proto, url, ok: false });
    }
  }
  return results;
}
