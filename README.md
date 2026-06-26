# git-brancher

CLI tool for cloning git repo branches into dev/PR directories with profile-based config, file injection, pre/post-clone hooks, and branch management.

---

## Requirements

- Node.js >= 22.0.0
- pnpm
- Git in PATH

---

## Getting Started

### 1. Initialise

Run once on a new machine. Creates `~/.gitrc` and sets up the profiles directory.

```bash
git-brancher init
```

Prompts for:

- **Profiles directory** — where profiles are stored (default: `~/.git-brancher/profiles`)
- **Default profile** — optional; when set, all commands that accept `-p` use it automatically

### 2. Create a profile

```bash
# Interactive — prompts for all fields
git-brancher profile create <profile-name>

# Start from a built-in template (github, gitlab, bitbucket)
git-brancher profile create <profile-name> --template github
```

When no `--template` is given, the wizard shows a template picker first. Selecting a template pre-fills SSH/HTTPS URLs — you only provide org and repo name, then confirm or override paths and post-clone commands. Selecting "None" runs the full manual flow.

Manual prompts:

- Repo SSH URL
- Repo HTTPS URL
- Dev branches directory
- PR branches directory
- Post-clone commands (comma-separated, e.g. `pnpm install, pnpm build`)

Scaffolds the following structure:

```
<profilesDir>/
  <profile-name>/
    settings.json
    files/
      .test-env         ← default placeholder (manual mode only; templates omit this)
```

The `files/` directory is always created. The `.test-env` placeholder is only written in manual mode — template-based creation skips it.

### 3. Clone a branch

```bash
# Dev mode (default) — full history
git-brancher -p <profile> -b <branch>

# PR mode — single-branch, no full history
git-brancher -m pr -p <profile> -b <branch>

# Multiple branches in one invocation (comma-separated)
git-brancher -p <profile> -b main,feat/x,fix/y

# Shallow clone (faster for large repos)
git-brancher -p <profile> -b <branch> --depth 1

# Use HTTPS instead of SSH
git-brancher -p <profile> -b <branch> --https

# Omit -p when defaultProfile is set in ~/.gitrc
git-brancher -b <branch>

# Omit -b to pick branches interactively from remote list
git-brancher -p <profile>
```

After cloning:

1. Files listed in `settings.json#files` are copied into the cloned repo
2. `postClone` commands run in the cloned directory (with `stdio: inherit`)
3. VS Code opens the cloned directory via the `code` CLI (if available); prints a message and continues if not found
4. For multi-branch clones, only the **last** successfully cloned directory is opened in VS Code
5. An entry is appended to the audit log at `~/.gitrc-log.jsonl`

**Dev mode note:** Using `--depth` in dev mode prints `Note: --depth N creates a shallow clone (limited history)` and proceeds. In PR mode, shallow clone is normal and no note is printed.

---

## Profile — settings.json

```json
{
  "repo": {
    "ssh": "ssh://git@ssh.bitbucket.example.net:7999/code/repo.git",
    "https": "https://bitbucket.example.net/scm/code/repo.git"
  },
  "paths": {
    "dev": "~/git-projects/my-profile/dev/branches",
    "pr": "~/git-projects/my-profile/pr/branches"
  },
  "files": [
    { "source": "files/.test-env", "dest": ".env" }
  ],
  "preClone": [
    "vpn-check",
    "auth-refresh"
  ],
  "postClone": [
    "pnpm install",
    "pnpm build"
  ]
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `repo.ssh` | string | yes | SSH clone URL |
| `repo.https` | string | yes | HTTPS clone URL |
| `paths.dev` | string | yes | Root directory for dev-mode clones |
| `paths.pr` | string | yes | Root directory for PR-mode clones |
| `files` | array | no | Files to copy from profile dir into cloned repo after clone |
| `preClone` | array | no | Shell commands run before cloning starts |
| `postClone` | array | no | Shell commands run in cloned repo after file injection |

**`files` entries:**
- `source` — path relative to the profile directory (e.g. `files/.test-env`)
- `dest` — path relative to the cloned repo root (e.g. `.env`)
- If a `source` file does not exist at clone time, the clone **fails with a fatal error**

**`preClone` behavior:**
- Commands run before `git clone` with `stdio: inherit`
- If any command exits non-zero, the clone **aborts immediately**
- Useful for VPN checks, auth refresh, environment validation

**`postClone` behavior:**
- Commands run in the cloned repo directory with `stdio: inherit`
- Run after file injection
- If a command fails, subsequent commands still run (non-aborting)

**Path expansion:** `~` and `%USERPROFILE%` in path fields are both resolved to `os.homedir()`.

**Operation order on clone:**
1. Run `preClone` hooks (abort if any fail)
2. Run `git clone`
3. Copy `files` into cloned repo (fatal if source missing)
4. Run `postClone` hooks in cloned dir
5. Append to audit log

---

## ~/.gitrc

```json
{
  "profilesDir": "~/.git-brancher/profiles",
  "defaultProfile": "my-profile"
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `profilesDir` | string | yes | Directory where all profiles are stored |
| `defaultProfile` | string | no | Profile used when `-p` is omitted on any command |

`defaultProfile` applies to **all** commands that accept `-p`: clone, branch list, branch clean, branch status, branch diff, branch open, branch pull, branch prune, branch reclone.

---

## Branch Name Sanitization

Branch names containing `/` or `\` are sanitized to `-` when creating local clone directories.

Example: `feature/my-branch` is cloned into a directory named `feature-my-branch`.

This applies consistently across all commands: clone, reclone, list, status, diff, open, pull, prune, clean. The sanitized name is what appears in `branch list` output.

---

## Audit Log

Every successful clone and reclone appends a JSON line to `~/.gitrc-log.jsonl`.

**Entry fields:**

| Field | Description |
|---|---|
| `ts` | ISO 8601 timestamp |
| `command` | `"clone"` or `"reclone"` |
| `profile` | Profile name |
| `branch` | Branch name |
| `mode` | `"dev"` or `"pr"` |
| `proto` | `"ssh"` or `"https"` |
| `target` | Full path to the cloned directory |

The file is append-only and never truncated. Use `git-brancher log` to read it.

---

## Editor & Directory Opener

**`profile edit`** opens `settings.json`:
1. Tries `code --wait <file>` (VS Code)
2. Falls back to `$EDITOR` environment variable
3. Falls back to `$VISUAL` environment variable

**`branch open`** opens a clone directory:
1. Tries `code <dir>` (VS Code)
2. Falls back to `explorer` (Windows), `open` (macOS), or `xdg-open` (Linux)
3. If all fail, prints "Could not open directory"

---

## CLI Reference

### Clone

```
git-brancher [-p <profile>] [-b <branch>] [-m dev|pr] [--https] [--depth <n>]
```

| Flag | Default | Description |
|---|---|---|
| `-p, --profile <name>` | `defaultProfile` | Profile to use |
| `-b, --branch <branches>` | — | Branch(es) to clone, comma-separated; omit for interactive picker |
| `-m, --mode <mode>` | `dev` | Clone mode: `dev` (full history) or `pr` (single-branch) |
| `--https` | — | Use HTTPS URL instead of SSH |
| `--depth <n>` | — | Shallow clone with N commits of history |

**Interactive picker:** When `-b` is omitted, git-brancher fetches the remote branch list (15-second timeout) and presents a checkbox list. Selecting nothing exits quietly.

**Multi-branch:** When `-b` receives comma-separated values, each branch is cloned sequentially. Per-branch headers print during execution. Only the last branch's directory opens in VS Code.

---

### Init & Log

```
git-brancher init
git-brancher log [-n <count>] [-p <profile>]
```

**`log` flags:**

| Flag | Default | Description |
|---|---|---|
| `-n <count>` | `20` | Number of entries to display |
| `-p, --profile <name>` | — | Filter entries by profile name |

Entries display in reverse chronological order (newest first).

---

### Config

```
git-brancher config show
git-brancher config set <key> <value>
```

Valid keys for `config set`: `profilesDir`, `defaultProfile`. Any other key triggers a fatal error listing the valid options.

---

### Profile

```
git-brancher profile list
git-brancher profile show <name>
git-brancher profile create <name> [--template <id>]
git-brancher profile copy <src> <dest>
git-brancher profile edit <name>
git-brancher profile rename <old> <new>
git-brancher profile delete <name>
git-brancher profile export <name> [outfile]
git-brancher profile import <archive> <name>
git-brancher profile diff <a> <b>
git-brancher profile doctor <name>
```

---

#### `profile list`

Lists all profiles found in `profilesDir`. For each profile, prints: SSH URL, HTTPS URL, dev path, PR path, number of mapped files.

If a profile directory exists but `settings.json` is missing or contains invalid JSON, it prints `[!] Missing or invalid settings.json` and continues — it does not fail.

---

#### `profile create <name> [--template <id>]`

Available templates: `github`, `gitlab`, `bitbucket`.

Templates auto-construct SSH/HTTPS URLs from org + repo name inputs. Only paths and post-clone commands are prompted after template selection.

---

#### `profile export <name> [outfile]`

Archives the profile directory (settings.json + files/) as a `.tar.gz`.

Default output: `<profile-name>.tar.gz` in the current working directory. Pass `[outfile]` to override the path.

---

#### `profile import <archive> <name>`

Extracts a `.tar.gz` profile archive into `profilesDir` under the given name.

Validation requirements:
- Archive must contain exactly **one** top-level directory
- That directory must contain `settings.json`

Fails with a fatal error if target profile name already exists. Temporary directory used during extraction is always cleaned up, even on failure.

---

#### `profile diff <a> <b>`

Compares `settings.json` of two profiles side-by-side. Prints a table with key, value-A, and value-B. Differing rows are highlighted in yellow. Always exits with code 0 — differences are informational only.

---

#### `profile doctor <name>`

Runs a full health check on a profile. Checks:

- `settings.json` exists and parses as valid JSON
- All required fields present (`repo.ssh`, `repo.https`, `paths.dev`, `paths.pr`)
- `paths.dev` and `paths.pr` directories exist on disk
- Each `files[].source` file exists in the profile directory
- `preClone` and `postClone` commands are resolvable on PATH
- Repo reachable via SSH (10-second timeout via `git ls-remote --exit-code`)
- Repo reachable via HTTPS (10-second timeout via `git ls-remote --exit-code`)

Each check prints `✔ pass` or `✘ fail: <reason>`. **Exits with code 1 if any check fails** — composable with CI scripts.

---

### Branch

All `branch` subcommands accept `-p <profile>` (uses `defaultProfile` if omitted) and `-m <mode>` (default: `both`). Mode `both` operates on dev and PR directories combined.

---

#### `branch list`

```
git-brancher branch list [-p <profile>] [-m dev|pr|both] [--older-than <age>]
```

Lists cloned branches sorted by last-modified time (newest first). Age column color coding:

- **Red** — last modified 30+ days ago
- **Yellow** — last modified 7–29 days ago
- Default — modified within 7 days

`--older-than` filters to only show branches older than the given age. Accepts: `30` (days), `30d`, `4w`, `1m`.

---

#### `branch clean`

```
git-brancher branch clean [-p <profile>] [-m dev|pr|both] [--older-than <age>]
```

Interactive checkbox selection for deleting cloned directories.

When `--older-than <age>` is used, branches matching the age threshold are **pre-selected** in the checkbox — the user can deselect before confirming. Confirmation is always required before deletion.

---

#### `branch status`

```
git-brancher branch status [-p <profile>] [-m dev|pr|both]
```

Prints a status table for every cloned branch. Columns:

| Column | Description |
|---|---|
| `NAME` | Sanitized branch directory name |
| `MODE` | `dev` or `pr` |
| `SHA` | Short HEAD commit hash |
| `DIRTY` | Number of modified files; `-` if clean |
| `↑` | Commits ahead of upstream; `?` if no upstream configured |
| `↓` | Commits behind upstream; `?` if no upstream configured |

---

#### `branch diff`

```
git-brancher branch diff -b <branch> [-p <profile>] [-m dev|pr|both] [--ref <ref>]
```

Runs `git diff --stat <ref>` in the branch directory and prints output.

| Flag | Default | Description |
|---|---|---|
| `-b, --branch <branch>` | required | Branch name to diff |
| `--ref <ref>` | `HEAD@{upstream}` | Ref to diff against |

---

#### `branch open`

```
git-brancher branch open [-p <profile>] [-b <branch>] [-m dev|pr|both]
```

Opens a cloned branch directory. If `-b` is omitted, presents an interactive list of branches to choose from. See [Editor & Directory Opener](#editor--directory-opener) for fallback behavior.

---

#### `branch pull`

```
git-brancher branch pull [-p <profile>] [-m dev|pr|both]
```

Runs `git pull` in every cloned branch directory. Prints per-branch result (up to date / fast-forwarded / error message). Summarises total succeeded and failed at the end.

---

#### `branch prune`

```
git-brancher branch prune [-p <profile>] [-m dev|pr|both] [--https]
```

Fetches the remote branch list and deletes local clone directories for branches that no longer exist on the remote.

Remote branch names are sanitized (`/` → `-`) before comparing against local directory names. Prompts for confirmation before deleting. Uses `--https` flag to query via HTTPS URL instead of SSH.

---

#### `branch reclone`

```
git-brancher branch reclone -b <branch> [-p <profile>] [-m dev|pr] [--https] [--depth <n>] [--no-stash]
```

Deletes an existing clone directory and re-clones the branch. Accepts the same flags as the root clone command plus `--no-stash`.

**Stash behavior (default):** Before deleting the directory, git-brancher checks for uncommitted changes. If found:

1. Runs `git stash push`
2. Exports the stash as a `.patch` file saved to the clone's parent directory (e.g. `<paths.dev>/<branch>.patch`)
3. Prints the patch path and instructions: `git apply "<patch-path>"`

Use `--no-stash` to skip stash export and delete immediately.

Accepts comma-separated `-b` for multiple branches: `-b main,feat/x`.

---

## ~/.gitrc — Extended Notes

```json
{
  "profilesDir": "~/.git-brancher/profiles",
  "defaultProfile": "my-profile"
}
```

Managed via `git-brancher init` (first-time) or `git-brancher config set <key> <value>`.

Valid `config set` keys: `profilesDir`, `defaultProfile`.

`~` in `profilesDir` is expanded to `os.homedir()`.

---

## Repo Structure

```
git-brancher/
├── src/
│   ├── index.ts                  # CLI entry
│   ├── cli/                      # Commander registration
│   │   ├── clone.ts
│   │   ├── profile.ts
│   │   └── branch/
│   │       ├── index.ts          # Registers branch subcommand
│   │       ├── read.ts           # list, clean, status, diff
│   │       └── actions.ts        # reclone, open, pull, prune
│   ├── commands/                 # Thin handlers (read opts, call lib, print)
│   │   ├── init.ts
│   │   ├── config.ts
│   │   ├── clone.ts
│   │   ├── log.ts
│   │   ├── profile/
│   │   │   ├── create.ts
│   │   │   ├── copy.ts
│   │   │   ├── delete.ts
│   │   │   ├── edit.ts
│   │   │   ├── export.ts
│   │   │   ├── import.ts
│   │   │   ├── list.ts
│   │   │   ├── rename.ts
│   │   │   ├── show.ts
│   │   │   ├── diff.ts
│   │   │   └── doctor.ts
│   │   └── branch/
│   │       ├── clean.ts
│   │       ├── diff.ts
│   │       ├── list.ts
│   │       ├── open.ts
│   │       ├── prune.ts
│   │       ├── pull.ts
│   │       ├── reclone.ts
│   │       └── status.ts
│   └── lib/                      # Business logic
│       ├── types.ts
│       ├── errors.ts
│       ├── gitrc.ts
│       ├── paths.ts
│       ├── age.ts
│       ├── audit.ts
│       ├── banner.ts
│       ├── doctor-checks.ts
│       ├── editor.ts
│       ├── format.ts
│       ├── hooks.ts
│       ├── reachability.ts
│       ├── templates.ts
│       ├── profile/
│       │   ├── index.ts          # listProfiles, requireProfile
│       │   ├── diff.ts           # diffProfiles
│       │   ├── io.ts             # exportProfile, importProfile, copyProfile
│       │   └── prompts.ts        # collectFromTemplate, collectManually
│       ├── branch/
│       │   ├── index.ts          # listBranches, deleteBranch
│       │   └── ops.ts            # pullBranch, getRemoteBranches, stashAndExport, getBranchStatus
│       └── clone/
│           ├── index.ts          # cloneBranch
│           └── inject.ts         # injectFiles
├── dist/                         # Compiled output — not committed
├── package.json
└── tsconfig.json
```

---

## Development

```bash
pnpm install
pnpm build
node dist/index.js init
```

No test suite. No lint step. Build only (`tsc`).

---

## Files and Secrets

Profiles are stored locally at the `profilesDir` set during `init`. They are never part of the published package and should not be committed to source control — they may contain secrets (e.g. `.env` files injected into clones).
