# Codex Final Re-check - MOFEFID-D4

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-d4` / `.claude/worktrees/mofefid-d4`
Reviewed closure: `a709f99f` plus merge-prep cleanup after merging
`origin/main@f55ba8bb`.

## Outcome

Accepted for merge with one unrelated full-suite caveat.

The two partial closures from `review-codex-recheck.md` are closed:

- `CX-001`: forcing now uses `InvalidForcing` for non-finite or negative
  rainfall excess, rainfall intensity, and upstream inflow. The solver no
  longer silently zeroes finite-negative forcing on the active path.
- `CX-004`: `SC-OFEROUTE-001` now separates D4 physics evidence from D-val/D5
  `Ef` acceptance in `INV-OFEROUTE-011`, the BEI note, and Test-Vector
  Obligations.

I also made merge-prep cleanup for stale non-behavioral text: D4 package test
counts now say 18/18, module headers say ADR-0033 ratified and D4 landed, and
the SC revision history has unique version rows. No solver logic or tests were
changed in that cleanup.

## Evidence Classes

Static:
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.
- Reviewed `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- Reviewed `docs/work-packages/20260702-mofefid-d4-kinematic-wave-solver-001/package.md`.
- Reviewed `docs/work-packages/20260702-mofefid-d4-kinematic-wave-solver-001/artifacts/review-disposition.md`.

Ran:
- `cargo run --quiet` in `/tmp/openwepp-d4-probe` -> `nan_all`,
  `negative_excess`, `negative_inflow`, and `negative_intensity` all returned
  `Err InvalidForcing`.
- `cargo fmt --check` -> pass.
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` -> 18/18 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator` -> 166/166 passed.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED`.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` -> 2/2 passed.
- `cargo deny check` -> pass.
- No-wiring grep: `rg -n "KinematicWaveSolver|ofe_routing::kinematic_wave|kinematic_wave::" crates tests -g '!crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs' -g '!target'` -> no hits.

Full-suite caveat:
- `cargo nextest run --workspace --profile full` was attempted after merging
  `origin/main`; it failed/interrupted with 1195 passed, 10 failed, 1 skipped,
  and 18 not run after interrupt. The run started before the worktree-local
  `.venv` link existed, so several failures were harness setup failures.
- The one non-setup assertion I isolated was
  `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`; it fails on
  this branch and also fails on detached `origin/main@f55ba8bb` with the same
  values (`UpStrmQ 593.7505180774947`, previous `QOFE 118.75010361549894`,
  ratio 2). D4's diff against `origin/main` does not touch that runner test or
  its runner/output surfaces.

## Finding Disposition

| Candidate | Final status | Evidence |
|---|---|---|
| `CX-001` finite-negative forcing still normalized to zero | Closed | `is_valid_forcing(value) = value.is_finite() && value >= 0.0`; `step()` returns `RoutingError::InvalidForcing` for invalid intensity, upstream inflow, or rainfall excess; probe confirms all four invalid channels fail closed. |
| `CX-004` active contract row still assigned `Ef` evidence to D4 | Closed | Test-Vector Obligations now says D4 evidence is conservation/CFL/non-negativity/fail-closed physics vectors, and the separate D-val row assigns `Ef` reproduction to D5/integration. |
| Stale module/package/SC revision text | Closed by Codex merge-prep cleanup | Comment/package/SC history cleanup only; no behavior change. |

## Merge Decision

Merge-ready for D4. The D4-specific gates, workspace clippy, authority guards,
and `cargo deny` pass. Full workspace nextest is not green, but the isolated
behavioral failure reproduces on current `origin/main` and is outside D4's write
set.
