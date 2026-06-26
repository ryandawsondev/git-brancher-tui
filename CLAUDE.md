# git-brancher

TypeScript CLI tool. Clones git repo branches into dev/PR directories with profile-based config and file injection.

## Stack

- Node.js >= 22, ESM (`"type": "module"`)
- TypeScript 5, target ES2022, NodeNext module resolution
- pnpm (package manager)
- commander (CLI), inquirer (prompts), execa (shell), chalk (color)
- @changesets/cli for versioning

## Commands

```bash
pnpm build          # tsc → dist/
node dist/index.js  # run locally
```

No test suite. No lint step. Build only.

## Architecture

```
src/
  index.ts              # CLI entry — wires commander subcommands
  cli/                  # commander registration (maps flags/args to command handlers)
    profile.ts
    clone.ts
    branch/
      index.ts          # registers branch subcommand, delegates to read/actions
      read.ts           # list, clean, status, diff
      actions.ts        # reclone, open, pull, prune
  commands/             # thin command handlers (read opts, call lib, print)
    init.ts
    config.ts
    clone.ts
    log.ts
    profile/
      create.ts
      delete.ts
      list.ts
      show.ts
      edit.ts
      rename.ts
      copy.ts
      export.ts
      import.ts
      diff.ts
      doctor.ts
    branch/
      list.ts
      clean.ts
      status.ts
      reclone.ts
      diff.ts
      open.ts
      pull.ts
      prune.ts
  lib/                  # business logic
    types.ts            # GitRc, ProfileSettings, ResolvedProfile
    gitrc.ts            # ~/.gitrc read/write (GITRC_PATH, readGitrc, writeGitrc)
    paths.ts            # resolvePath, resolveProfilesDir, getProfileDir
    errors.ts           # fatal() — prints to stderr and exits 1
    editor.ts           # openInEditor() — VS Code then $EDITOR/$VISUAL
    format.ts           # formatLastModified() for branch timestamps
    age.ts              # parseAgeDays() — parses age strings (30, 4w, 1m)
    audit.ts            # appendAuditLog() — writes clone audit entries
    hooks.ts            # runHooks() — executes preClone/postClone shell commands
    templates.ts        # PROFILE_TEMPLATES — built-in profile templates
    reachability.ts     # checkRepoReachability() via git ls-remote
    doctor-checks.ts    # checkSettingsFields, checkPaths, checkFiles, checkHooks, checkReachabilityResults
    banner.ts           # chalk banner
    profile/
      index.ts          # listProfiles, requireProfile, defaultProfileSettings
      diff.ts           # diffProfiles()
      io.ts             # exportProfile, importProfile, copyProfile
      prompts.ts        # collectFromTemplate, collectManually
    branch/
      index.ts          # listBranches, deleteBranch, BranchEntry, BranchMode
      ops.ts            # pullBranch, getRemoteBranches, stashAndExport, getBranchStatus
    clone/
      index.ts          # cloneBranch(), CloneMode, Proto, CloneOptions
      inject.ts         # injectFiles()
dist/                   # compiled output — not committed
```

## Key concepts

- `~/.gitrc` — JSON global config: `{ profilesDir, defaultProfile? }`
- Profile — directory under `profilesDir/<name>/` containing `settings.json` and `files/`
- `settings.json` shape: `{ repo: { ssh, https }, paths: { dev, pr }, files?: [{ source, dest }], postClone?: string[] }`
- Clone modes: `dev` (full history) | `pr` (single-branch)
- Proto: `ssh` (default) | `https` — selected via `--https` flag
- Depth: `--depth <n>` for shallow clones (works in both modes)
- File injection: after clone, files listed in `settings.json#files` are copied from profile dir into cloned repo
- Post-clone hooks: `settings.json#postClone` — array of shell commands run in cloned dir after injection
- Branch/PR folder names: branch name with `/` and `\` replaced by `-`
- Path expansion: `~` and `%USERPROFILE%` both resolved to `os.homedir()`
- Default profile: `~/.gitrc#defaultProfile` used when `-p` flag omitted

## CLI surface

```
git-brancher init
git-brancher config show
git-brancher config set <key> <value>
git-brancher log
git-brancher -p <profile> -b <branch> [-m dev|pr] [--https] [--depth <n>]

git-brancher profile create <name>
git-brancher profile delete <name>
git-brancher profile list
git-brancher profile show <name>
git-brancher profile edit <name>
git-brancher profile rename <old> <new>
git-brancher profile copy <src> <dest>
git-brancher profile export <name> [outfile]
git-brancher profile import <archive> <name>
git-brancher profile diff <a> <b>
git-brancher profile doctor <name>

git-brancher branch list [-p <profile>] [-m dev|pr|both] [--older-than <age>]
git-brancher branch clean [-p <profile>] [-m dev|pr|both] [--older-than <age>]
git-brancher branch status [-p <profile>] [-m dev|pr|both]
git-brancher branch diff -b <branch> [-p <profile>] [-m dev|pr|both] [--ref <ref>]
git-brancher branch open [-p <profile>] [-b <branch>] [-m dev|pr|both]
git-brancher branch pull [-p <profile>] [-m dev|pr|both]
git-brancher branch prune [-p <profile>] [-m dev|pr|both] [--https]
git-brancher branch reclone -b <branch> [-p <profile>] [-m dev|pr] [--https] [--depth <n>] [--no-stash]
```

## Patterns

- `src/lib/types.ts` owns all types (`GitRc`, `ProfileSettings`, `ResolvedProfile`)
- `src/lib/gitrc.ts` owns `~/.gitrc` I/O (`readGitrc`, `writeGitrc`, `gitrcExists`)
- `fatal()` from `src/lib/errors.ts` — all user-facing errors; no thrown errors reach top level
- Commands call `readGitrc()` then `requireProfile()` to get validated config before doing work
- `requireProfile()` falls back to `config.defaultProfile` when no name passed
- CLI layer (`src/cli/`) only registers commander options/actions; all logic lives in `commands/` and `lib/`
- `src/lib/profile/`, `src/lib/branch/`, `src/lib/clone/` are domain subfolders; import via `index.js` (e.g. `../../lib/profile/index.js`)

## Versioning

Uses changesets. Run `pnpm changeset` to create a changeset, `pnpm changeset:version` to bump.
