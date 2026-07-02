# Codex Re-check - MOFEFID-D4 Kinematic-Wave Solver

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-d4` / `.claude/worktrees/mofefid-d4`
Reviewed disposition range: `a2ca5747..e5ab64de`

## Outcome

Hold as written. Do not merge yet.

The disposition closes two of the four blockers: all three positivity-clamp
sites now contribute to the conservation ledger, and `sample_dt_s <= 0` now
fails closed instead of hanging. The focused and full orchestrator crate gates
are green.

Two closures remain partial:

- `CX-001`: non-finite forcing now fails closed, but finite negative forcing is
  still silently converted to zero.
- `CX-004`: `INV-OFEROUTE-011` and the BEI note now assign formal `Ef` to
  D-val/D5, but the active Test-Vector Obligations row still says the D4
  single-OFE solver must provide `Ef` evidence.

## Evidence Classes

Static:
- Reviewed `docs/work-packages/20260702-mofefid-d4-kinematic-wave-solver-001/package.md`.
- Reviewed `docs/work-packages/20260702-mofefid-d4-kinematic-wave-solver-001/artifacts/review-disposition.md`.
- Reviewed `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.

Ran:
- `cargo fmt --check` -> pass.
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` -> 17/17 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator` -> 165/165 passed.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` -> pass.
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED`.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> pass.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` -> 2/2 passed.
- `/tmp/openwepp-d4-probe`: a local probe crate against this checkout. Output:
  `nan_all: Err NonFiniteForcing`; `negative_excess: Ok inflow=0 rain=0 peak=0 samples=2`; `negative_inflow: Ok inflow=0 rain=0 peak=0 samples=2`; `negative_intensity: Ok inflow=0 rain=0 peak=0 samples=2`.

I did not run full workspace nextest or `cargo deny check` because the package
is still held on source/contract-level blockers.

## Findings

| Candidate | Verdict | Evidence | Disposition |
|---|---|---|---|
| `CX-001` partial closure: finite negative forcing still normalizes to zero instead of failing closed. | Accepted, blocker | Static: `step` now rejects non-finite upstream inflow and intensity at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:370`-`:376`, and rejects non-finite rainfall excess at `:384`-`:388`, but still applies `.max(0.0)` to upstream inflow and rainfall excess at `:376` and `:388`. The friction path also clamps negative rainfall intensity at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs:49`. Contract posture allows bounded zero for no-flow/component-absent states, but hard-fails invalid active operands and active routing domains at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:133`-`:137`, `:148`-`:154`, and `:175`-`:176`. Ran: `/tmp/openwepp-d4-probe` showed all three finite negative forcing channels return `Ok` with zero rain/inflow/peak. | Add a typed invalid-forcing path for finite negative rainfall excess, upstream inflow, and rainfall intensity before the `.max(0.0)` normalizations, or amend the contract with an explicit bounded-normalization rule and tests. The former matches the original review disposition wording: finite/non-negative forcing validation. |
| `CX-002` all clamp sites included in the ledger. | Closed | Static: predictor clamp accumulation at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:408`-`:412`, corrector accumulation at `:421`-`:425`, final clamp accumulation at `:447`-`:458`, and ledger publication at `:478`. The convergence test asserts zero clamp mass at both resolutions at `:833`-`:839`. Ran: focused `ofe_routing` suite 17/17 and full orchestrator crate suite 165/165 passed. | No remaining blocker on this finding. |
| `CX-003` `sample_dt_s <= 0` hang fixed. | Closed | Static: `run()` now rejects non-finite/non-positive `sample_dt_s` at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:493`-`:503`; regression test at `:901`-`:917`. Ran: focused `ofe_routing` suite 17/17 passed. | No remaining blocker on this finding. |
| `CX-004` partial closure: one active contract row still assigns `Ef` evidence to D4. | Accepted, blocker | Static: `INV-OFEROUTE-011` now clearly assigns formal `Ef` acceptance to D-val/D5 at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:158`, and the BEI note now says D4 is physics validation only at `:267`. But the Test-Vector Obligations row still says `Single-OFE solver ... D4 KWE/TVD/CFL closure, non-negativity, and Ef evidence` at `:246`-`:248`. That preserves the old gate in an active contract section, so the package-local deferral still conflicts with canonical authority. Static: the revision history also has two `Version 2` rows at `:284`-`:285`; this is secondary but should be repaired with the same contract cleanup. | Update the Test-Vector Obligations row so D4 requires physics evidence only, and formal `Ef` evidence is explicitly D-val/D5/integration. Make the revision-history versioning unambiguous. |
| `CX-006` stale module headers. | Still open, minor | Static: `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs:1`-`:5` still says `ADR-0033 Proposed`, "later stages", and "D3 lands the friction-factor kernels only" despite D4 landing `kinematic_wave`; `friction.rs:1` still says `ADR-0033 Proposed`. Package heading also still says "Validation (13 committed tests)" while the suite is now 17 after the fail-closed tests. | Non-blocking cleanup, but fix before final close to avoid stale public module and package prose. |

## Merge Decision

Not merge-ready. I did not merge or fast-forward `main`.

Once the two partial closures are corrected, note that this branch is no longer
a fast-forward of current `origin/main` (`origin/main` has advanced with
`f55ba8bb`), so the final merge path should first reconcile `worktree-mofefid-d4`
with current main before pushing `main`.
