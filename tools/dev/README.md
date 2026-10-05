# openWEPP Nix Development Environment

Enter the shell from the checkout or worktree that owns the task:

```bash
cd /path/to/openWEPP-worktree
tools/dev/develop
```

`tools/dev/develop` stages the versioned `nix-toolchain/flake.nix`, lock, and
canonical `openwepp-env` shell helper under the user cache, then enters that staged flake. It never gives
Nix the checkout as a flake input, so source and evidence edits do not create
repository-sized Nix snapshots. The staged identity changes only when one of
those three toolchain inputs changes. Pass a command after the wrapper to run
it in the shell while retaining the checkout as the current directory:

```bash
tools/dev/develop cargo check --workspace
```

The wrapper creates a persistent profile for its content identity, which is a
Nix GC root. Do not use `nix develop` at the repository root: the root flake
was deliberately removed to prevent it from importing the checkout.

## Toolchain lifecycle

The lifecycle command manages only the wrapper's own staged inputs and profiles.
It does not run `nix gc` or inspect/delete arbitrary store paths.

```bash
tools/dev/nix-lifecycle list
tools/dev/nix-lifecycle pin <toolchain-id> pre-upgrade
tools/dev/nix-lifecycle release pre-upgrade
tools/dev/nix-lifecycle prune --keep 2 --older-than-days 30
```

Pins are separate persistent profiles. `prune` retains the newest two inactive,
unpinned staged environments by default and only retires additional ones after
30 days since their last wrapper use. Active or pinned environments always win. It removes matching files
below the wrapper-owned cache and state directories. It deliberately does not
follow a profile link into `/nix/store` without a recorded owned-output identity,
and never invokes global garbage collection.

The optional user-systemd templates in `tools/dev/systemd/` run that same scoped
command weekly. Set `OPENWEPP_NIX_LIFECYCLE` to the absolute path of the chosen
checkout's `tools/dev/nix-lifecycle` in
`~/.config/openwepp/nix-lifecycle.env`, then install the two templates as user
units and enable `openwepp-nix-prune.timer`.

The shell shares Cargo downloads, uv downloads, and the future sccache store,
but derives unique Cargo target and `/tmp` paths from the absolute worktree
path. It also records an atomic live-process ownership claim for that target. A
second shell for the same task fails explicitly instead of sharing incremental
build state.

Use one worktree per write-capable agent. If two independent tasks must operate
from the same checkout, give each one an explicit identity before entering:

```bash
OPENWEPP_TASK_ID=review-a tools/dev/develop
OPENWEPP_TASK_ID=review-b tools/dev/develop
```

Do not rely on `nix develop /path/to/flake` to select build identity: Nix keeps
the caller's current directory. Change into the intended worktree first.

## Heavy Commands

Use the host-wide admission wrapper for full-workspace Clippy, nextest,
release, comparator, and similar closure commands:

```bash
tools/dev/heavy cargo nextest run --workspace --profile full
```

Only one wrapped heavy command can run across all worktrees on the host. A
collision exits with status `75`. The concurrency intake selected 8 Cargo build
jobs and 8 nextest test threads while focused agents remain active. An
exclusive run may override either value deliberately:

```bash
OPENWEPP_HEAVY_JOBS=16 \
OPENWEPP_HEAVY_TEST_THREADS=16 \
tools/dev/heavy cargo nextest run --workspace --profile full
```

## Python

Create the ignored repository virtual environment with the pinned Nix Python:

```bash
uv venv .venv --python "$(command -v python3.12)"
uv pip sync tools/owcmp/requirements.lock.txt
```

## Optimization Experiments

Cargo incremental compilation remains the default. Shared sccache and mold are
available but are intentionally opt-in until the feasibility package selects
the winning settings.

Example sccache arm:

```bash
CARGO_INCREMENTAL=0 RUSTC_WRAPPER=sccache cargo check --workspace
sccache --show-stats
```

Example mold arm:

```bash
RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold" cargo check --workspace
```

## Validation

```bash
tools/dev/develop nixfmt --check tools/dev/nix-toolchain/flake.nix
tools/dev/develop sh -c 'nix flake check "path:$OPENWEPP_NIX_STAGED_INPUT"'
tools/dev/develop tools/dev/check-nix-tooling
tools/dev/tests/nix-lifecycle-test
tools/dev/check-host
```

`check-host` currently validates the `ow-dev-01` encrypted NVMe layout and is
not expected to pass unchanged on `forest`.
