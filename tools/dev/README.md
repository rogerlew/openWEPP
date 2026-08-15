# openWEPP Nix Development Environment

Enter the shell from the checkout or worktree that owns the task:

```bash
cd /path/to/openWEPP-worktree
nix develop
```

The shell shares Cargo downloads, uv downloads, and the future sccache store,
but derives unique Cargo target and `/tmp` paths from the absolute worktree
path. It also holds a nonblocking ownership lock for that target. A second shell
for the same task fails explicitly instead of sharing incremental build state.

Use one worktree per write-capable agent. If two independent tasks must operate
from the same checkout, give each one an explicit identity before entering:

```bash
OPENWEPP_TASK_ID=review-a nix develop
OPENWEPP_TASK_ID=review-b nix develop
```

Do not rely on `nix develop /path/to/flake` to select build identity: Nix keeps
the caller's current directory. Change into the intended worktree first.

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
nix fmt -- flake.nix
nix flake check
tools/dev/check-host
```

`check-host` currently validates the `ow-dev-01` encrypted NVMe layout and is
not expected to pass unchanged on `forest`.
