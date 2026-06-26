// Shared type contracts — GitRc, ProfileSettings, ResolvedProfile.

export interface GitRc {
  profilesDir: string;
  defaultProfile?: string;
}

export interface ProfileSettings {
  repo: {
    ssh: string;
    https: string;
  };
  paths: {
    dev: string;
    pr: string;
  };
  files?: Array<{
    source: string;
    dest: string;
  }>;
  preClone?: string[];
  postClone?: string[];
}

export interface ResolvedProfile {
  profileDir: string;
  settings: ProfileSettings;
}
