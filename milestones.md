# Code Quality Milestones

---

## Decoupling & File Separation Milestones

### 7. Split `lib/config.ts` god module into focused files
**Problem:** `lib/config.ts` holds 4 unrelated concerns — type definitions, error utility, path helpers, and gitrc I/O. Every module imports from it even when it only needs one thing (e.g. just `fatal` or just a type).
- [x] Create `lib/types.ts` — move `GitRc`, `ProfileSettings`, `ResolvedProfile` interfaces
- [x] Create `lib/errors.ts` — move `fatal` helper
- [x] Create `lib/gitrc.ts` — move `GITRC_PATH`, `gitrcExists`, `readGitrc`, `writeGitrc`
- [x] Keep `lib/config.ts` for path utilities (`resolvePath`, `resolveProfilesDir`, `getProfileDir`)
- [x] Update all import paths across `commands/` and `lib/`

### 8. Move `requireProfile` from `lib/config.ts` → `lib/profile.ts`
**Problem:** `requireProfile` reads and validates a profile directory. It belongs with `listProfiles` in `lib/profile.ts`, not in the gitrc/config module.
- [x] Move `requireProfile` to `lib/profile.ts`
- [x] Remove it from `lib/config.ts`
- [x] Update all callers in `commands/`

### 9. Move business logic out of `commands/profile.ts` into `lib/`
**Problem:** Two pieces of lib-level logic live in the command layer:
- `defaultSettings()` (lines 15–39) builds a `ProfileSettings` template — business logic, not command orchestration
- `runProfileValidate` calls `execa("git", ["ls-remote", ...])` directly — external I/O belongs in lib
- [x] Extract `defaultProfileSettings(profileName: string): ProfileSettings` → `lib/profile.ts`
- [x] Extract `checkRepoReachability(settings: ProfileSettings): Promise<{proto, url, ok}[]>` → `lib/profile.ts`
- [x] `runProfileValidate` in `commands/profile.ts` calls lib function and only handles output
- [x] `runProfileCreate` uses `defaultProfileSettings` from lib instead of local function

### 10. Fix `commands/init.ts` manual path expansion
**Problem:** Line 41 re-implements `~` expansion inline (`profilesDir.replace(/^~/, os.homedir())`) instead of calling the centralised `resolvePath`. Milestone 1 addressed this everywhere else but missed `init.ts`.
- [x] Replace manual expand with `resolvePath(profilesDir)` from `lib/config.ts`
- [x] Remove `os` import from `commands/init.ts` if no longer needed (`os` retained — still used for default path)

### 11. Separate file injection from git clone in `lib/clone.ts`
**Problem:** `cloneBranch` does two distinct operations — git clone and file injection — in one function. They have different failure modes and are independently useful.
- [x] Extract `injectFiles(files: Array<{source: string; dest: string}>, profileDir: string, target: string): void`
- [x] `cloneBranch` calls `injectFiles` after clone completes
- [x] Export `injectFiles` so it can be called independently if needed

### 12. Move `formatLastModified` to a shared utility module
**Problem:** `lib/branch.ts:54–65` contains a date formatting utility that has zero branch-domain knowledge. It is only in `lib/branch.ts` because that was where it was first needed.
- [x] Create `lib/format.ts` and move `formatLastModified` there
- [x] Update `lib/branch.ts` and `commands/branch.ts` imports
- [x] `lib/format.ts` can grow to hold future display formatters (file sizes, durations, etc.)

### 13. Move branch deletion out of `commands/branch.ts` into `lib/branch.ts`
**Problem:** `commands/branch.ts:111` calls `fs.rmSync(branch.dir, { recursive: true, force: true })` directly. Raw filesystem operations in the command layer bypass the lib boundary — the lib owns all branch directory operations.
- [x] Add `deleteBranch(entry: BranchEntry): void` to `lib/branch.ts`
- [x] `runBranchClean` in `commands/branch.ts` calls `deleteBranch(branch)` instead of `fs.rmSync`
- [x] Remove `import fs from "fs"` from `commands/branch.ts` if no longer needed

---

## 1. Extract shared `resolvePath` to `lib/config.ts`
- [x] Remove duplicate `resolvePath` from `lib/clone.ts` and `lib/branch.ts`
- [x] Export single `resolvePath` from `lib/config.ts`
- [x] Update all call sites

## 2. Add `fatal()` helper to `lib/config.ts`
- [x] Add exported `fatal(msg: string): never` that calls `console.error` + `process.exit(1)`
- [x] Replace all scattered `console.error(...); process.exit(1)` pairs across `lib/config.ts`, `lib/clone.ts`, `index.ts`

## 3. Use commander `.choices()` for mode validation
- [x] Replace manual `["dev","pr","both"].includes(mode)` guards in `index.ts` with `.addOption(new Option(...).choices([...]))`
- [x] Same for clone mode (`"dev" | "pr"`)

## 4. Read version from `package.json`
- [x] Remove hardcoded `"0.1.0"` from `index.ts` and `commands/clone.ts`
- [x] Import version from `package.json` (or read it once at startup)

## 5. Mark `ProfileSettings.files` as optional
- [x] Change `files: Array<...>` to `files?: Array<...>` in `lib/config.ts`
- [x] Verify `clone.ts` `?? []` null-coalesce still works

## 6. Wrap JSON.parse in try/catch
- [x] Wrap `JSON.parse` in `readGitrc()` with try/catch — print helpful error on malformed JSON
- [x] Same for `requireProfile()` settings parse

---

## Feature Milestones

### F1. Post-clone hooks
**Goal:** Run arbitrary commands in the cloned repo after file injection (e.g. `pnpm install`, `npm run build`).
- [x] Add `postClone?: string[]` field to `ProfileSettings` in `lib/types.ts`
- [x] `lib/clone.ts` — after `injectFiles`, iterate `postClone` commands and run each via `execa` in the cloned dir with `stdio: "inherit"`
- [x] `profile create` wizard prompts for optional post-clone commands
- [x] `profile validate` reports presence/absence of `postClone` entries

### F2. Default profile in `~/.gitrc`
**Goal:** Skip the `-p <profile>` flag when only one profile is used regularly.
- [x] Add `defaultProfile?: string` to `GitRc` in `lib/types.ts`
- [x] `config set defaultProfile <name>` wires through existing `runConfigSet`
- [x] `requireProfile` falls back to `config.defaultProfile` when no `-p` flag passed
- [x] `init` wizard optionally prompts for default profile after setting `profilesDir`
- [x] All commands that accept `-p` mark it optional (not required) when default is set

### F3. `branch reclone <branch>`
**Goal:** Delete an existing clone and re-clone in one command — common "get latest" workflow.
- [x] Add `reclone` subcommand under `branch` (registered in `cli/branch.ts`)
- [x] Handler in `commands/branch/reclone.ts` — resolves profile, finds existing dir, deletes it, calls `cloneBranch`
- [x] Accepts same flags as `clone` (`--mode`, `--proto`)
- [x] Prompts for confirmation before delete if dir exists

### F4. `branch status`
**Goal:** Show git status summary for each cloned branch (dirty file count, ahead/behind upstream).
- [x] Add `getBranchStatus(entry: BranchEntry): Promise<BranchStatus>` to `lib/branch.ts`
- [x] `BranchStatus` shape: `{ dirty: number; ahead: number; behind: number; headSha: string }`
- [x] Uses `git status --porcelain`, `git rev-list --count HEAD@{upstream}..HEAD` and reverse
- [x] `commands/branch/status.ts` handler — lists all branches then prints status table
- [x] Graceful degradation when no upstream configured (show `?` for ahead/behind)

### F5. `profile copy <src> <dest>`
**Goal:** Duplicate an existing profile as a starting point for a new one.
- [x] Add `copyProfile(src: string, dest: string, config: GitRc): void` to `lib/profile.ts`
- [x] Copies entire profile dir (settings.json + files/) via `fs.cpSync`
- [x] Handler in `commands/profile/copy.ts` — validates src exists, dest does not, calls lib
- [x] Registered in `cli/profile.ts`

### F6. `--depth N` flag on clone
**Goal:** Support shallow clones for large repos.
- [x] Add `depth?: number` to `CloneOptions` in `lib/clone.ts`
- [x] Pass `--depth <N>` to `git clone` args when set
- [x] Add `--depth <n>` option to `clone` command in `cli/clone.ts`
- [x] Dev mode only — warn (but allow) if `--depth` used with `mode: pr` since `--single-branch` already limits fetch

### F7. Profile export/import
**Goal:** Share profiles across machines via a zip archive.
- [x] Add dependency: `archiver` (export) + `unzipper` (import) — or use native `tar` via execa to keep zero extra deps
- [x] `lib/profile.ts` — `exportProfile(profileDir: string, outPath: string): Promise<void>` (tar.gz the profile dir)
- [x] `lib/profile.ts` — `importProfile(archivePath: string, profilesDir: string, name: string): Promise<void>` (extract, validate settings.json exists)
- [x] Commands: `profile export <name> [outfile]`, `profile import <file> <name>`
- [x] Registered in `cli/profile.ts`

### F8. Age-based pre-selection in `branch clean`
**Goal:** `branch clean --older-than <days>` pre-checks branches not modified in N days.
- [x] Add `--older-than <days>` option to `branch clean` command
- [x] `listBranches` already returns `lastModified` — filter at choice-build time in `commands/branch/clean.ts`
- [x] Pre-checked entries still require user confirmation (don't auto-delete)
- [x] Accept both integer days (`30`) and shorthand (`30d`, `4w`, `1m`)

### F9. `branch open`
**Goal:** Open a cloned branch directory in VS Code (or OS file explorer as fallback).
- [x] Add `open` subcommand under `branch` (registered in `cli/branch.ts`)
- [x] Accept `-p <profile>` and `-b <branch>` flags; `-m dev|pr` to disambiguate when same branch exists in both
- [x] Handler in `commands/branch/open.ts` — resolves branch dir via `listBranches`, calls `openInEditor(dir)` (reuse `lib/editor.ts`)
- [x] If `-b` omitted, prompt user to select from branch list via inquirer

### F10. `branch pull`
**Goal:** Run `git pull` across all cloned branches for a profile in one command.
- [x] Add `pull` subcommand under `branch` (registered in `cli/branch.ts`)
- [x] Handler in `commands/branch/pull.ts` — calls `listBranches`, runs `git pull` in each dir via execa
- [x] Add `pullBranch(entry: BranchEntry): Promise<{ ok: boolean; output: string }>` to `lib/branch.ts`
- [x] Print per-branch result table (branch name, status: up to date / fast-forwarded / conflict / error)
- [x] Accept `-p <profile>` and `-m dev|pr|both` flags; skip non-git dirs gracefully

### F11. `branch prune`
**Goal:** Delete local cloned dirs for branches that no longer exist on the remote.
- [x] Add `prune` subcommand under `branch` (registered in `cli/branch.ts`)
- [x] Handler in `commands/branch/prune.ts` — fetches remote branch list via `git ls-remote --heads <repo>`, cross-references `listBranches` output
- [x] Add `getRemoteBranches(repoUrl: string): Promise<string[]>` to `lib/branch.ts` (uses execa git ls-remote)
- [x] Shows diff: local branches missing from remote, prompts confirmation before delete
- [x] Accept `-p <profile>` and `--https` flags; proto determines which repo URL to query

### F12. `branch diff`
**Goal:** Show a condensed git diff summary for a cloned branch vs its upstream or a target ref.
- [x] Add `diff` subcommand under `branch` (registered in `cli/branch.ts`)
- [x] Required `-b <branch>` flag; optional `--ref <ref>` (default: `HEAD@{upstream}` or `origin/main`)
- [x] Handler in `commands/branch/diff.ts` — resolves branch dir, runs `git diff --stat <ref>` via execa with `stdio: inherit`
- [x] Add `getBranchDiff(entry: BranchEntry, ref: string): Promise<void>` to `lib/branch.ts`
- [x] Accept `-p <profile>` and `-m dev|pr` flags

### F13. `branch list` age column
**Goal:** Flag stale branches in `branch list` output without a separate command.
- [x] Add `age` column to `branch list` table output using existing `formatLastModified` from `lib/format.ts`
- [x] Add `--older-than <age>` filter flag to `branch list` (same parsing logic as `branch clean`)
- [x] Highlight stale entries in a distinct chalk colour (e.g. yellow for >7d, red for >30d)
- [x] No new lib functions needed — `BranchEntry.lastModified` already populated

### F14. `profile diff <a> <b>`
**Goal:** Compare two profiles' `settings.json` side-by-side to spot differences.
- [x] Add `diff <a> <b>` subcommand under `profile` (registered in `cli/profile.ts`)
- [x] Handler in `commands/profile/diff.ts` — loads both profiles' settings via `requireProfile`, diffs all fields
- [x] Add `diffProfiles(a: ProfileSettings, b: ProfileSettings): Array<{ key: string; left: unknown; right: unknown }>` to `lib/profile.ts`
- [x] Print as two-column table: key | value-A | value-B; highlight differing rows in chalk yellow
- [x] Show `(missing)` for keys present in one but not the other

### F15. Profile templates
**Goal:** Let `profile create` start from a named preset (GitHub, Bitbucket, GitLab) instead of blank placeholders.
- [x] Add `lib/templates.ts` — exports `PROFILE_TEMPLATES: Record<string, ProfileSettings>` with presets for GitHub, Bitbucket, GitLab
- [x] Each preset fills in correct SSH/HTTPS URL patterns, sensible default path structures, and empty `files`/`postClone`
- [x] Add `--template <name>` option to `profile create` command
- [x] If `--template` omitted, `profile create` wizard offers a template picker (inquirer list) before the existing prompts
- [x] `profile create` with template skips prompts for fields the template already populates (only asks for repo name/org to substitute into URLs)

### F16. `profile doctor`
**Goal:** Single command that verifies everything about a profile is healthy.
- [x] Add `doctor <profile-name>` subcommand under `profile` (registered in `cli/profile.ts`)
- [x] Handler in `commands/profile/doctor.ts` — runs all checks and prints a pass/fail checklist
- [x] Checks to run: settings.json exists and parses, all required fields present, `paths.dev` and `paths.pr` directories exist, each `files[].source` exists in profile dir, repo reachable via SSH and HTTPS (reuse `checkRepoReachability` from `lib/reachability.ts`), `postClone` commands resolvable on PATH
- [x] Print each check as `✓ pass` / `✗ fail: <reason>` using chalk green/red
- [x] Exit code 1 if any check fails — composable with CI/scripts

### F17. `preClone` hooks
**Goal:** Run commands before cloning starts (e.g. VPN check, auth refresh) — parallel to existing `postClone`.
- [x] Add `preClone?: string[]` to `ProfileSettings` in `lib/types.ts`
- [x] `lib/clone.ts` — before `git clone`, iterate `preClone` commands via execa with `stdio: inherit`; abort clone if any command exits non-zero
- [x] `profile validate` reports presence/absence of `preClone` entries (same as `postClone`)
- [x] `profile doctor` checks `preClone` commands are resolvable on PATH (see F16)

### F18. Multi-branch clone
**Goal:** Clone multiple branches in one invocation — `-b main,feat/x,fix/y`.
- [x] Update `-b` flag to accept comma-separated list (`-b <branches...>` or parse split on `,`)
- [x] `cli/clone.ts` — split value on `,`, iterate and call `cloneBranch` for each branch sequentially
- [x] Print per-branch result summary after all clones complete
- [x] Same for `branch reclone` — accept `-b branch1,branch2`
- [x] If any branch fails, continue remaining clones and report failures at end (don't abort early)

### F19. Audit log
**Goal:** Append-only log of every clone/reclone so you can trace what was cloned when and from which profile.
- [x] Add `lib/audit.ts` — exports `appendAuditLog(entry: AuditEntry): void`
- [x] `AuditEntry` shape: `{ ts: string; command: "clone" | "reclone"; profile: string; branch: string; mode: string; proto: string; target: string }`
- [x] Log file path: `~/.gitrc-log.jsonl` (one JSON object per line, never truncated)
- [x] Call `appendAuditLog` at end of `cloneBranch` success path in `lib/clone.ts`
- [x] Add `git-brancher log` command — reads and pretty-prints last N entries (default 20); accepts `--n <count>` and `--profile <name>` filter

### F20. Interactive branch picker on clone
**Goal:** When `-b` is omitted, fetch remote branches and let the user fuzzy-search/select instead of erroring.
- [x] Add `fetchRemoteBranches(repoUrl: string): Promise<string[]>` to `lib/branch.ts` — runs `git ls-remote --heads <url>` via execa, parses `refs/heads/<name>` lines
- [x] Install `@inquirer/search` (or `inquirer-autocomplete-prompt`) for fuzzy filtering
- [x] `cli/clone.ts` — when `-b` is absent, resolve profile, call `fetchRemoteBranches`, present autocomplete prompt, use selected value as branch
- [x] Same fallback in `cli/branch.ts` for `branch reclone` when `-b` omitted
- [x] `--proto` flag (or default SSH) determines which repo URL is queried for branch list

### F21. Stash-before-reclone
**Goal:** Preserve uncommitted work before `branch reclone` deletes the clone directory.
- [x] Add `stashAndExport(entry: BranchEntry, outDir: string): Promise<string | null>` to `lib/branch.ts` — runs `git stash` then `git stash show -p > <outDir>/<branch>.patch`; returns patch path or null if working tree was clean
- [x] `commands/branch/reclone.ts` — before deleting dir, call `stashAndExport`; print patch path to user if stash was created
- [x] Save patch alongside the clone's parent dir (e.g. `<paths.dev>/<branch>.patch`) so it survives deletion
- [x] After reclone completes, inform user how to re-apply: `git apply <patch-path>`
- [x] Add `--no-stash` flag to skip this behaviour for users who want the original hard-delete
