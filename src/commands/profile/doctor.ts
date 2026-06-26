import { readGitrc } from "../../lib/gitrc.js";
import { requireProfile } from "../../lib/profile/index.js";
import {
  CheckResult,
  checkSettingsFields,
  checkPaths,
  checkFiles,
  checkHooks,
  checkReachabilityResults,
} from "../../lib/doctor-checks.js";

function printResult(r: CheckResult): void {
  console.log(r.pass ? `  ✔ ${r.msg}` : `  ✘ ${r.msg}`);
}

export async function runProfileDoctor(profileName: string): Promise<void> {
  const config = readGitrc();
  const { profileDir, settings } = requireProfile(config, profileName);

  console.log(`\nRunning doctor for profile '${profileName}'...\n`);

  const mainResults: CheckResult[] = [
    ...checkSettingsFields(settings),
    ...checkPaths(settings),
    ...checkFiles(settings, profileDir),
    ...(await checkHooks(settings)),
  ];

  mainResults.forEach(printResult);

  console.log("\n  Checking repo reachability...");
  const reachResults = await checkReachabilityResults(settings);
  reachResults.forEach(printResult);

  const all = [...mainResults, ...reachResults];
  const passed = all.filter((r) => r.pass).length;
  const failed = all.filter((r) => !r.pass).length;

  console.log(`\n  ${passed} passed, ${failed} failed`);
  if (failed > 0) { console.log(); process.exit(1); }
  console.log();
}
