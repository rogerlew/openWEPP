# Codex Review - MOFEFID-D6 Infiltration Coupling

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-d6` / `.claude/worktrees/mofefid-d6`
Reviewed commit: `d47f7b24`

## Outcome

Hold as written; do not merge yet.

The direction is sound: D6 is still shadow-first, the precise no-wiring grep
finds no production caller of `run_infiltrated_cascade`, and the 9 D6 tests pass.
I also agree with the physics correction in principle: the D5
supersede-then-compose wording was a misread, and Papanicolaou's routing model
should infiltrate rainfall per OFE while treating the upstream hydrograph as a
surface boundary condition, not as a second infiltration supply.

Four findings need disposition before merge. The substantive blockers are that
the public rainfall-to-excess path still silently normalizes invalid inputs and
the active contract text is internally contradictory after the D6 correction.
The remaining findings are revision-truthfulness cleanup tied to the contract
fix and a numerical-fidelity gap in the claimed unponded-to-ponded Green-Ampt
transition.

## Evidence Classes

Static:
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs`.
- Reviewed stale D5 comments in `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`.
- Reviewed `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- Reviewed `docs/work-packages/20260702-mofefid-d6-infiltration-coupling-001/package.md`.
- Grep-reviewed production/no-wiring surface for D6 symbols.

Ran:
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing::infiltration` -> 9/9 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` -> 32/32 passed.
- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED`.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` -> 2/2 passed.
- Precise no-wiring grep:
  `rg -n "run_infiltrated_cascade|green_ampt_excess_hyetograph|green_ampt_step|GreenAmptSoil|ofe_routing::infiltration|crate::ofe_routing|super::ofe_routing|pub mod infiltration" crates tests -g '!crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs' -g '!target'` -> only `pub mod infiltration`.

Not run:
- `cargo nextest run --workspace --profile full` and `cargo deny check`; stopped because static review found merge-blocking issues.

## Findings

| ID | Verdict | Evidence | Required disposition |
|---|---|---|---|
| `CX-D6-001` invalid rainfall/substep inputs are silently normalized or skipped | Accepted, blocking | Static: `green_ampt_excess_hyetograph` silently replaces `substep_s <= 0` with `1.0` at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs:222` and silently skips intervals with `duration <= 0` or negative rate at `:224-227`. `run_infiltrated_cascade` validates lengths and soils only at `:301-307`, then builds hyetographs at `:310-315`; it does not fail closed on negative/non-finite rainfall rates, non-finite times, reversed intervals, or non-positive/non-finite infiltration substeps. `green_ampt_step` also normalizes negative cumulative state and non-positive rainfall/dt to zero-output behavior at `:101-107`. This conflicts with the repo fail-closed rule and with the package's "config/soil domain fail-closed" claim. | Make the active API fail closed with typed errors for malformed rainfall intervals and substeps, and add tests for negative/non-finite rate, non-finite/reversed interval times, non-positive/non-finite substep, and invalid carried state if that remains public. Do not silently default/skip invalid forcing. |
| `CX-D6-002` `SC-OFEROUTE-001` is contradictory after the D6 physics correction | Accepted, blocking | Static: the package says faithful D6 is SUPERSEDE/no runon re-infiltration, but active contract text still says "Coupling of routed rainfall-excess re-infiltration" at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:52`, says rainfall-excess/infiltration internals are out of scope and owned by SC-RUNOFFPART at `:57`, labels the branch as "downstream re-infiltration" at `:141`, begins `INV-OFEROUTE-009` with "routed inter-OFE excess re-infiltrates" at `:156`, keeps `OBL-OFEROUTE-P-004` as "supersede-then-compose" at `:178`, keeps the D5 test-vector row saying supersede-then-compose at `:248`, and leaves D-val wording as `D5/integration` at `:249`. The stale `cascade.rs` module comment still says routed runon re-infiltrates through downstream hourly infiltration at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:8-15`. | Reconcile the canonical contract and module docs end-to-end: either OFEROUTE owns this Green-Ampt coupling and the scope/obligations/alias/test-vector rows must say so, or the coupling must be grounded in SC-RUNOFFPART with explicit ownership. Remove remaining supersede-then-compose/re-infiltration language from active authority surfaces. |
| `CX-D6-003` contract/package revision truthfulness is stale | Accepted, blocking until fixed with `CX-D6-002` | Static: the D6 package cites `SC-OFEROUTE-001 (rev 4)` at `docs/work-packages/20260702-mofefid-d6-infiltration-coupling-001/package.md:5`, but D5 already added revision 5. The D6 contract adds another version `4` row at `SC-OFEROUTE-001.md:287`, before the existing Codex version `4` row and before the D5 version `5` row at `:289`. | Move D6 to the next unique revision after D5, update the package's contract revision, and make history order match chronology. |
| `CX-D6-004` unponded-to-ponded transition is not implemented as claimed | Accepted candidate; needs fix or narrowed claim | Static: `green_ampt_step` claims to handle the unponded-to-ponded transition at `infiltration.rs:90-93`, but the implementation only checks end-of-step capacity at `:121-132`; once that fails, it applies the ponded implicit equation over the whole step from `f0_eff` at `:134-156`. There is no `F_p`/`t_p` transition calculation and no test that pins the transition point or Newton residual. With substeps this may be a useful approximation, but it is not the stated Green-Ampt-Mein-Larsen transition. | Either implement the transition split explicitly or narrow the package/API documentation to a substep approximation and add a test proving acceptable transition error under the intended D-val substep. For a physics kernel, I recommend implementing the actual transition. |

## Merge Decision

Not merge-ready. The shadow-first/no-wiring evidence is good and the focused
tests pass, but `main` should not take a new process-physics API that silently
normalizes malformed rainfall inputs or a canonical contract that contradicts
itself about whether routed runon is re-infiltrated.
