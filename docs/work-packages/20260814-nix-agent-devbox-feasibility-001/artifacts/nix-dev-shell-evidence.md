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
operator supplies `OPENWEPP_TASK_ID`. Cross-worktree isolation remains to be
proved after this environment increment is committed.

## Capacity

- Realized development-shell closure: `4.2 GiB`.
- Root/Nix filesystem after realization: `28 GiB` used, `415 GiB` available.
- `/workdir`: approximately `1.3 TiB` available.
- `/tmp`: approximately `503 GiB` available.
