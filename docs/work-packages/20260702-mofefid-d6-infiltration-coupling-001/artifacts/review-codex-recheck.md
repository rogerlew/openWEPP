# Codex Re-check - MOFEFID-D6 Infiltration Coupling

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-d6` / `.claude/worktrees/mofefid-d6`
Reviewed commit: `37598d32`

## Outcome

Not merge-ready yet.

The main direction remains acceptable and the focused gates are green. The
Green-Ampt transition was materially improved, and the active contract body is
much closer to the D6 SUPERSEDE decision. Two review findings are closed. Two
remain partially closed:

- The public `green_ampt_step` / `InfiltrationState` API still silently
  normalizes invalid direct inputs, despite the original finding requiring
  carried-state coverage if that API remained public.
- The package and module comments still carry stale revision/compose wording.

## Evidence Classes

Static:
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`.
- Reviewed `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- Reviewed `docs/work-packages/20260702-mofefid-d6-infiltration-coupling-001/package.md`.
- Reviewed `docs/work-packages/20260702-mofefid-d6-infiltration-coupling-001/artifacts/review-disposition.md`.
- Grep-reviewed D6 no-wiring surface and stale `re-infiltration` / `supersede-then-compose` text.

Ran:
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing::infiltration` -> 11/11 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` -> 34/34 passed.
- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED`.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` -> 2/2 passed.
- Precise no-wiring grep:
  `rg -n "run_infiltrated_cascade|green_ampt_excess_hyetograph|green_ampt_step|GreenAmptSoil|InfiltrationState|ofe_routing::infiltration|crate::ofe_routing|super::ofe_routing|pub mod infiltration" crates tests -g '!crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs' -g '!target'` -> only `pub mod infiltration`.

Not run:
- `cargo nextest run --workspace --profile full` and `cargo deny check`; stopped because re-check still found merge-blocking issues.

## Finding Status

| ID | Re-check status | Evidence | Required disposition |
|---|---|---|---|
| `CX-D6-001` invalid rainfall/substep inputs silently normalized or skipped | Partially closed | `green_ampt_excess_hyetograph` now returns `Result` and fails closed on invalid soil, non-finite/non-positive substep, and malformed rainfall intervals (`infiltration.rs:238-259`), and `run_infiltrated_cascade` propagates those errors (`:349-355`). However, `green_ampt_step` remains `pub` and still normalizes direct invalid inputs: `state.cumulative_m.max(0.0)` at `:132`, negative/non-finite rainfall and non-positive/non-finite `dt_s` through zero-output/NaN-prone paths at `:133-140`, and invalid soil only indirectly tolerated by branch math. The original review explicitly called for invalid carried-state tests if this API remained public. | Either make `green_ampt_step` fail closed with `Result` and tests for invalid state/rainfall/dt/soil, or make it private/internal and document that `green_ampt_excess_hyetograph` / `run_infiltrated_cascade` are the public fail-closed boundary. |
| `CX-D6-002` contract contradictory after D6 physics correction | Mostly closed, one stale module comment remains | The active `SC-OFEROUTE-001` invariant and guard text now state SUPERSEDE/no runon re-infiltration. The remaining non-historical contradiction is in `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:8-15`, which still says routed runon re-infiltrates through downstream hourly infiltration and composes with SC-RUNOFFPART. | Update the D5 cascade module comment to match the D6 correction: cascade routes surface runon; infiltration is applied to rainfall in D6, and production activation disables DC01 daily-lump admission. |
| `CX-D6-003` contract/package revision truthfulness stale | Partially closed | `SC-OFEROUTE-001` now has D6 as revision 6 at `SC-OFEROUTE-001.md:291`. The package still says `SC-OFEROUTE-001 (rev 4)` at `package.md:5` and says D6 corrects it in "contract rev 4" at `package.md:31`. | Update package references to rev 6. |
| `CX-D6-004` unponded-to-ponded transition not implemented as claimed | Closed | `green_ampt_step` now computes `Fp = s/(r/Ks - 1)`, splits mid-step ponding via `t_p = (Fp - F0)/r`, and integrates only the post-ponded remainder with `green_ampt_integrate_ponded` (`infiltration.rs:117-185`). The new test `explicit_ponding_split_conserves_and_delays_excess` covers a transition-crossing step. | No further action required. |

## Merge Decision

Hold. This is close, but `main` should not take D6 until the public
Green-Ampt step boundary is either made fail-closed or made private/internal,
and the remaining stale package/module text is reconciled.
