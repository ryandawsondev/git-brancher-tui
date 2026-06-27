# Re-Architecture Plan: Node.js CLI → Rust Ratatui TUI

## Overview

Port `git-brancher` from a Node.js Commander CLI to a Rust Ratatui TUI. Same core functionality, interactive terminal UI instead of subcommands.

---

## Current CLI Feature Surface

| Command Group | Commands |
|---|---|
| Root | `init`, `config show`, `config set`, `clone`, `log` |
| `profile` | `create`, `list`, `show`, `edit`, `delete`, `rename`, `copy`, `export`, `import`, `diff`, `doctor` |
| `branch` | `list`, `clean`, `status`, `open`, `pull`, `prune`, `reclone`, `diff` |

---

## Rust Crate Dependencies

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"
dirs = "5"
chrono = "0.4"
flate2 = "1"
tar = "0.4"
futures = "0.3"
```

For git operations: spawn `git` subprocess via `tokio::process::Command` (mirrors current `execa` usage, avoids `libgit2` complexity).

---

## Project Structure

```
src/
├── main.rs                  # entry point, event loop bootstrap
├── app.rs                   # App struct, top-level state machine
├── state/
│   ├── mod.rs
│   ├── screen.rs            # Screen enum (all views)
│   ├── config.rs            # GitRc + ProfileSettings types
│   └── data.rs              # BranchEntry, AuditEntry, etc.
├── ui/
│   ├── mod.rs               # render dispatch by screen
│   ├── layout.rs            # shared layout helpers
│   ├── widgets/
│   │   ├── table.rs         # reusable sortable table
│   │   ├── list.rs          # navigable list
│   │   ├── form.rs          # multi-field input form
│   │   ├── confirm.rs       # yes/no dialog
│   │   ├── diff_view.rs     # two-column diff panel
│   │   └── status_bar.rs    # bottom key hint bar
│   └── screens/
│       ├── dashboard.rs
│       ├── profile_list.rs
│       ├── profile_detail.rs
│       ├── profile_create.rs
│       ├── profile_diff.rs
│       ├── profile_doctor.rs
│       ├── branch_list.rs
│       ├── branch_status.rs
│       ├── branch_clean.rs
│       ├── branch_diff.rs
│       ├── clone.rs
│       ├── log.rs
│       └── config.rs
├── actions.rs               # Action enum (all user intents)
├── ops/
│   ├── mod.rs
│   ├── config.rs            # read/write ~/.gitrc
│   ├── profile.rs           # profile CRUD, export/import, diff, doctor
│   ├── branch.rs            # list, delete, status from disk
│   ├── git.rs               # pull, prune, reclone, diff, ls-remote
│   ├── clone.rs             # clone logic + file injection + hooks
│   └── audit.rs             # append/read ~/.gitrc-log.jsonl
└── util/
    ├── age.rs               # parse "30d", "4w", "1m" → days
    ├── format.rs            # "today", "3 days ago"
    ├── paths.rs             # ~ expansion, profile dir resolution
    └── editor.rs            # open in VS Code / OS file manager
```

---

## State Architecture (Elm-like)

```
Event (key/mouse/tick)
    ↓
handle_event() → Action
    ↓
update(state, action) → new state
    ↓
render(state) → terminal frame
```

### App Struct

```rust
pub struct App {
    pub screen: Screen,
    pub prev_screens: Vec<Screen>,   // navigation stack
    pub config: Option<GitRc>,
    pub profiles: Vec<ProfileSummary>,
    pub branches: Vec<BranchEntry>,
    pub audit_log: Vec<AuditEntry>,
    pub async_task: Option<TaskHandle>, // in-flight git op
    pub status_msg: Option<(String, MsgLevel)>,
    pub should_quit: bool,
}
```

### Screen Enum

```rust
pub enum Screen {
    Dashboard,
    ProfileList { selected: usize },
    ProfileDetail { name: String },
    ProfileCreate(FormState),
    ProfileEdit { name: String, form: FormState },
    ProfileDiff { name_a: String, name_b: String, diff: Vec<DiffEntry> },
    ProfileDoctor { name: String, results: Vec<CheckResult> },
    BranchList { profile: String, mode: BranchMode, selected: usize },
    BranchStatus { profile: String, mode: BranchMode, rows: Vec<StatusRow> },
    BranchClean { entries: Vec<BranchEntry>, selected: BTreeSet<usize> },
    BranchDiff { profile: String, branch: String, output: String },
    Clone(CloneWizard),
    AuditLog { entries: Vec<AuditEntry>, offset: usize },
    Config { raw: GitRc },
    FirstRun,   // shown if ~/.gitrc missing (replaces `init`)
}
```

---

## Screen Hierarchy & Navigation

```
FirstRun (init wizard)
    └─→ Dashboard
            ├─→ ProfileList
            │       ├─→ ProfileDetail
            │       │       ├─→ ProfileEdit (form)
            │       │       ├─→ ProfileDoctor
            │       │       └─→ BranchList (for this profile)
            │       │               ├─→ BranchStatus
            │       │               ├─→ BranchClean (multi-select)
            │       │               ├─→ BranchDiff
            │       │               └─→ Clone wizard
            │       ├─→ ProfileCreate (form)
            │       ├─→ ProfileDiff (two-pane)
            │       └─→ [export/import/copy/rename via modal dialogs]
            ├─→ AuditLog
            └─→ Config
```

Back navigation: `Esc` pops `prev_screens` stack.

---

## Key Bindings (global)

| Key | Action |
|---|---|
| `q` / `Ctrl+C` | Quit |
| `Esc` | Back / cancel |
| `↑` / `k` | Up |
| `↓` / `j` | Down |
| `Enter` | Select / confirm |
| `Tab` | Next field (forms) |
| `Shift+Tab` | Prev field (forms) |
| `Space` | Toggle select (multi-select) |
| `?` | Help overlay |
| `r` | Refresh / reload data |

---

## Async Task Pattern

Long-running git ops (clone, pull, prune, status) run in tokio tasks. App holds a `JoinHandle` + progress state. Tick events poll completion.

```rust
pub struct AsyncTask {
    pub label: String,
    pub handle: tokio::task::JoinHandle<anyhow::Result<TaskOutput>>,
}

pub enum TaskOutput {
    BranchList(Vec<BranchEntry>),
    StatusRows(Vec<StatusRow>),
    CloneDone { branch: String },
    PullResults(Vec<PullResult>),
    // ...
}
```

Spinner widget shown on current screen while task in flight. Result dispatched as `Action::TaskComplete(TaskOutput)` on completion.

---

## Data Type Mapping (TS → Rust)

| TypeScript | Rust |
|---|---|
| `GitRc` interface | `struct GitRc` (serde Deserialize) |
| `ProfileSettings` interface | `struct ProfileSettings` |
| `ResolvedProfile` | `struct ProfileSummary` |
| `BranchEntry` | `struct BranchEntry` |
| `BranchMode = "dev" \| "pr" \| "both"` | `enum BranchMode { Dev, Pr, Both }` |
| `CloneOptions` | `struct CloneOptions` |
| `AuditEntry` | `struct AuditEntry` |
| `BranchStatus` | `struct BranchStatus` |
| `CheckResult` | `struct CheckResult` |
| `ProfileDiffEntry` | `struct DiffEntry` |
| `ReachabilityResult` | `struct ReachabilityResult` |

---

## Ops Layer Mapping (TS → Rust)

### `ops/config.rs`
- `read_gitrc() → Result<GitRc>` — reads `~/.gitrc` JSON
- `write_gitrc(config: &GitRc) → Result<()>`
- `gitrc_exists() → bool`

### `ops/profile.rs`
- `list_profiles(config: &GitRc) → Result<Vec<ProfileSummary>>`
- `require_profile(config: &GitRc, name: Option<&str>) → Result<ProfileSummary>`
- `default_profile_settings(name: &str) → ProfileSettings`
- `export_profile(profile_dir: &Path, out: &Path) → Result<()>` — tar.gz
- `import_profile(archive: &Path, profiles_dir: &Path, name: &str) → Result<()>`
- `copy_profile(config: &GitRc, src: &str, dest: &str) → Result<()>`
- `diff_profiles(a: &ProfileSettings, b: &ProfileSettings) → Vec<DiffEntry>`

### `ops/branch.rs`
- `read_branches_from_dir(dir: &Path, mode: BranchMode) → Result<Vec<BranchEntry>>`
- `list_branches(settings: &ProfileSettings, mode: BranchMode) → Result<Vec<BranchEntry>>`
- `delete_branch(entry: &BranchEntry) → Result<()>`

### `ops/git.rs`
- `pull_branch(entry: &BranchEntry) → Result<()>`
- `get_remote_branches(repo_url: &str) -> Result<Vec<String>>`
- `stash_and_export(entry: &BranchEntry, out_dir: &Path) -> Result<()>`
- `get_branch_status(entry: &BranchEntry) -> Result<BranchStatus>`
- `branch_diff(entry: &BranchEntry, ref_: &str) -> Result<String>`

### `ops/clone.rs`
- `clone_branch(opts: CloneOptions) -> Result<()>` — spawns git clone, runs hooks, injects files
- `inject_files(files: &[String], profile_dir: &Path, target: &Path) -> Result<()>`
- `run_hooks(cmds: &[String], label: &str, cwd: &Path) -> Result<()>`

### `ops/audit.rs`
- `append_audit_log(entry: &AuditEntry) -> Result<()>`
- `read_audit_log() -> Result<Vec<AuditEntry>>`

---

## Widgets to Build

| Widget | Purpose | Replaces |
|---|---|---|
| `NavigableList` | arrow-key selection, wraps ratatui `List` | `inquirer.select` |
| `MultiSelectList` | space-toggle checkboxes | `inquirer.checkbox` |
| `FormWidget` | multi-field text input, tab navigation | `inquirer.input` prompts |
| `ConfirmDialog` | modal yes/no | `inquirer.confirm` |
| `DataTable` | sortable, scrollable table | `chalk` + `console.log` tables |
| `DiffPane` | two-column key/value comparison | `profile/diff.ts` output |
| `SpinnerOverlay` | covers current screen during async ops | none (ops were sync before) |
| `StatusBar` | bottom bar: key hints + status messages | none |
| `LogViewer` | scrollable text area | `runLog()` output |

---

## Migration Phases

### Phase 1 — Scaffold & Core Types
- Init Rust project (`cargo init`)
- Define all `state/` types (GitRc, ProfileSettings, all enums/structs)
- Implement `ops/config.rs` (read/write `~/.gitrc`)
- Implement `util/` helpers (age, format, paths, editor)
- Bootstrap ratatui event loop with crossterm backend

### Phase 2 — Profile Ops + Screens
- `ops/profile.rs` — full CRUD, export/import, diff
- `ops/audit.rs`
- Screens: Dashboard, ProfileList, ProfileDetail, ProfileCreate, ProfileDiff, ProfileDoctor, AuditLog, Config, FirstRun

### Phase 3 — Branch Ops + Screens
- `ops/branch.rs` + `ops/git.rs`
- Async task runner
- Screens: BranchList, BranchStatus, BranchClean, BranchDiff

### Phase 4 — Clone Wizard
- `ops/clone.rs` — clone, inject, hooks
- Clone wizard screen (profile picker → branch picker → mode/proto → progress)
- Profile templates (GitHub/GitLab/Bitbucket) ported from `src/lib/templates.ts`

### Phase 5 — Polish
- Help overlay (`?`)
- Error handling + status bar messages
- `--profile` / `--mode` CLI args to deep-link into TUI at specific screen
- Cross-platform paths (Windows `%USERPROFILE%` vs `~`)
- Audit log on all mutating ops

---

## Notes

- Keep `ops/` layer pure functions with no TUI dependency — same separation as current `src/lib/` vs `src/commands/`.
- `editor.rs` spawns `code` or `$EDITOR` as subprocess, suspends TUI, restores terminal on return (same pattern as vim-mode editors).
- Profile `doctor` runs all checks concurrently via `tokio::join!`.
- `branch clean` uses `MultiSelectList` widget — user marks branches then hits `d` to delete selected.
- `clone` wizard is a multi-step form: step 1 = profile, step 2 = branch (fetched async from remote), step 3 = mode + proto, step 4 = progress log.
