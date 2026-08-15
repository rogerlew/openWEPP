# Nix Development Shell Evidence

Status: `Ran / Phase 1 PASS`

Date: `2026-08-14`

## Identity

- Nix: `2.35.2`, multi-user daemon mode.
- nixpkgs input: `nixos-26.05` at
  `02e08985a27c65ffd33d434eeb2e660a2e4dc84d`.
- Rust/Cargo: `1.95.0`.
- Python: `3.12.14`.
- cargo-nextest: `0.9.136`.
- cargo-deny: `0.19.6`.
- sccache: `0.15.0`.
- mold: `2.41.0`.
- uv: `0.11.21`.
- GitHub CLI in shell: `2.97.0`.

The flake contains no credentials or machine-specific authentication. The Nix
GitHub CLI successfully used the existing host credential helper and keyring.

## Ran

- `nix fmt -- flake.nix` — PASS.
- `nix flake check --print-build-logs` — PASS, including tool entrypoints and
  ShellCheck over both developer scripts.
- `nix develop` shell smoke — PASS.
- `tools/dev/check-host` inside the shell — PASS.
- `cargo metadata --locked --no-deps --format-version 1` — PASS.
- First isolated `cargo check -p openwepp-kernel-contract --lib` — PASS in
  `5.79 s` wall, `8.34 s` user, `1.16 s` system, `347392 KiB` max RSS. This is
  an environment smoke measurement, not an admitted cross-machine benchmark.
- Created the ignored `.venv` with the Nix Python 3.12 interpreter and synced
  `tools/owcmp/requirements.lock.txt` with uv.
- `.venv/bin/python` imported PyArrow `22.0.0` — PASS.
- Secret-safe `gh auth status` inside the Nix shell — PASS.

## Corrected Intake Failure

The first PyArrow import failed because the PyPI wheel could not find
`libstdc++.so.6` in the pure Nix environment. The shell now exposes the pinned
Nix C++ runtime through `LD_LIBRARY_PATH`; the repeated import passed. uv's
cache was also moved to `/workdir/.cache/openwepp/uv` so wheel material and the
repository virtual environment share the NVMe filesystem.

## Isolation

The shell allocated:

```text
CARGO_HOME=/workdir/.cache/openwepp/cargo-home
SCCACHE_DIR=/workdir/.cache/openwepp/sccache
UV_CACHE_DIR=/workdir/.cache/openwepp/uv
CARGO_TARGET_DIR=/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9
TMPDIR=/tmp/openwepp-roger-openWEPP-295c6e060aa9
```

The task identity is derived from the absolute Git worktree path unless the
operator supplies `OPENWEPP_TASK_ID`. Cross-worktree isolation was proved after
the first environment increment was committed.

Post-commit proof used a temporary detached worktree at `e64ee6a3c`, with LFS
smudging disabled to avoid copying payload data. From inside that worktree,
`nix develop` allocated:

```text
CARGO_TARGET_DIR=/workdir/.cache/openwepp/targets/nix-isolation-smoke-3c157e22a9fb
TMPDIR=/tmp/openwepp-roger-nix-isolation-smoke-3c157e22a9fb
```

Both differed from the primary checkout paths and locked Cargo metadata passed.
The temporary worktree and its empty target/scratch directories were removed
after the proof.

An initial probe invoked `nix develop /path/to/secondary/flake` while the
current directory remained the primary checkout. Nix correctly retained the
caller's current directory, so the environment selected the primary identity.
The developer README now requires changing into the intended worktree first.

Same-task collision proof then held one Nix shell open and invoked a second
shell from the primary checkout. The initial hook detected contention but did
not propagate the sourced helper's failure; the hook was corrected to exit on
helper failure. The repeated second invocation exited `1` with:

```text
error: task 'openWEPP-295c6e060aa9' already owns this Cargo target
use a separate worktree or set a unique OPENWEPP_TASK_ID
```

The first implementation kept an advisory lock descriptor open for the shell
lifetime. A background sccache server inherited that descriptor and retained
ownership after the shell exited. The implementation now serializes claim-file
updates with a short advisory lock and records the parent `nix develop` PID plus
Linux process start time. Live PID/start-time identity rejects overlap; stale
claims are replaced atomically; no descriptor is inherited by build daemons.

## Capacity

- Realized development-shell closure: `4.2 GiB`.
- Root/Nix filesystem after realization: `28 GiB` used, `415 GiB` available.
- `/workdir`: approximately `1.3 TiB` available.
- `/tmp`: approximately `503 GiB` available.
