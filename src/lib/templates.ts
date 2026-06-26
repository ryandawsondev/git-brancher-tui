// Built-in profile templates for common git hosts.

import os from "os";
import path from "path";
import { ProfileSettings } from "./types.js";

export interface ProfileTemplate {
  id: string;
  label: string;
  build(org: string, repo: string, profileName: string): ProfileSettings;
}

export const PROFILE_TEMPLATES: ProfileTemplate[] = [
  {
    id: "github",
    label: "GitHub",
    build: (org, repo, name) => ({
      repo: {
        ssh: `git@github.com:${org}/${repo}.git`,
        https: `https://github.com/${org}/${repo}.git`,
      },
      paths: {
        dev: path.join(os.homedir(), "git-projects", name, "dev", "branches"),
        pr: path.join(os.homedir(), "git-projects", name, "pr", "branches"),
      },
      files: [],
    }),
  },
  {
    id: "gitlab",
    label: "GitLab",
    build: (org, repo, name) => ({
      repo: {
        ssh: `git@gitlab.com:${org}/${repo}.git`,
        https: `https://gitlab.com/${org}/${repo}.git`,
      },
      paths: {
        dev: path.join(os.homedir(), "git-projects", name, "dev", "branches"),
        pr: path.join(os.homedir(), "git-projects", name, "pr", "branches"),
      },
      files: [],
    }),
  },
  {
    id: "bitbucket",
    label: "Bitbucket",
    build: (org, repo, name) => ({
      repo: {
        ssh: `git@bitbucket.org:${org}/${repo}.git`,
        https: `https://${org}@bitbucket.org/${org}/${repo}.git`,
      },
      paths: {
        dev: path.join(os.homedir(), "git-projects", name, "dev", "branches"),
        pr: path.join(os.homedir(), "git-projects", name, "pr", "branches"),
      },
      files: [],
    }),
  },
];
