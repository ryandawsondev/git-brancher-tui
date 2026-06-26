import { ProfileSettings } from "../types.js";

export interface ProfileDiffEntry {
  key: string;
  a: string;
  b: string;
  changed: boolean;
}

export function diffProfiles(a: ProfileSettings, b: ProfileSettings): ProfileDiffEntry[] {
  const fields: Array<[string, unknown, unknown]> = [
    ["repo.ssh", a.repo.ssh, b.repo.ssh],
    ["repo.https", a.repo.https, b.repo.https],
    ["paths.dev", a.paths.dev, b.paths.dev],
    ["paths.pr", a.paths.pr, b.paths.pr],
    ["files", JSON.stringify(a.files ?? []), JSON.stringify(b.files ?? [])],
    ["preClone", JSON.stringify(a.preClone ?? []), JSON.stringify(b.preClone ?? [])],
    ["postClone", JSON.stringify(a.postClone ?? []), JSON.stringify(b.postClone ?? [])],
  ];
  return fields.map(([key, av, bv]) => ({
    key: key as string,
    a: String(av),
    b: String(bv),
    changed: String(av) !== String(bv),
  }));
}
