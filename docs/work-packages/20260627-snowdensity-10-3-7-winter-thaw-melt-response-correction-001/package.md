# SNOWDENSITY-10.3.7 - Winter-Thaw Melt Response Correction

Status: complete  
Type: defect-closure execplan  
Owner: Codex  
Created: 2026-06-27

## Objective

Close the `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE` handoff from
SNOWDENSITY-10.3.6 by authoring a contract-first, opt-in winter-thaw melt
state-loss correction and proving whether it improves the paired Sleepers /
Harvard thaw-ablation evidence without tuning coefficients, forcing, canopy,
phase partition, density constants, rain heat, sub-canopy longwave, frost, or
production defaults.

## Final Disposition

`WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`.

`WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES` with coupled WAT improvement.

The opt-in candidate met the revised v94 package gates. It reduced paired
Sleepers/Harvard thaw-window under-ablation count (`132 -> 108`) and aggregate
depth-loss deficit (`24.105 m -> 17.629 m`), passed active-ledger
conservation/routing reconstruction, and improved the real direct-production WAT
snow-control gate (`1147 -> 978` failures) with no paired surface worsening and
trace proof that `coe_winter_thaw_state_loss_v1` reached the direct snow
partition. Snow control remains failed, so this is an opt-in improvement and not
full fix, default activation, frost-unblock, or snow-control closure.

## Correction Authority Envelope

In scope:

- Amend `SC-SNOWFREEZE-001` before production edits with:
  - `coe_winter_thaw_state_loss_v1` as an opt-in `snow_melt_model` candidate.
  - explicit default/rollback isolation.
  - the authorized exception to the legacy `rho_snew < 350 kg m^-3` density gate.
  - thaw-window operand reconstruction, conservation/routing acceptance, and
    coupled WAT snow-control gates.
- Add contract-derived tests for the new invariant, selector, default identity,
  and conservation behavior.
- Add the opt-in selector to typed CoE melt dispatch and snowbench diagnostic
  replay.
- Add a package-bound diagnostic direct-production selector,
  `OPENWEPP_SNOWDENSITY1037_MELT_MODEL`, only for coupled WAT adjudication;
  absent/empty preserves `legacy_coe`, unknown values fail closed, and no
  parser/runfile/user CLI activation is added.
- Correct only the low-density positive-thaw application branch: positive CoE
  `wmelt` may leave the snowpack as state loss for the opt-in candidate instead
  of being entirely absorbed as density-only compaction below `350 kg m^-3`.
- Rerun the 10.3.6 paired thaw-ablation diagnosis for `legacy_coe` versus
  `coe_winter_thaw_state_loss_v1`.

Out of scope:

- Default activation, parser/runfile/user CLI selectors, compatibility-runtime
  changes, public WAT/HBP/PASS schema changes, fixture input edits, or output
  publication changes.
- Melt coefficient fitting, radiation scaling, canopy tuning, phase-partition
  changes, density-constant changes, frost changes, sub-canopy longwave, rain
  heat, Qwet/frzftp, or site constants.
- Reopening `coe_shortwave_albedo_v1` activation; this package may coexist with
  it but must not promote it.

## Required Reading

- `docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/artifacts/worker-handoff.md`
- `docs/planning/snow-frost-fidelity-strategy.md` Section 10.3
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `tools/snowfreeze_observed/winter_thaw_melt_response.py`

## Intended Write Set

- `docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`
- `tools/snowfreeze_observed/winter_thaw_melt_response_correction.py`
- `tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py`
- `tests/integration/snowdensity10_3_7_winter_thaw_melt_response_correction.rs`
- `Cargo.toml`

## Phase Plan

1. Contract first:
   - Amend `SC-SNOWFREEZE-001` to authorize exactly one opt-in correction
     selector and qualify the legacy density gate.
   - Add package and contract-derived tests before production code.
2. Production implementation:
   - Add the typed selector to `SnowMeltModel` and `CoeMeltModel`.
   - Keep `legacy_coe` default and `coe_shortwave_albedo_v1` albedo behavior
     unchanged.
   - Apply the state-loss correction only when
     `snow_melt_model = coe_winter_thaw_state_loss_v1`.
3. Diagnostic rerun:
   - Compare `legacy_coe` and `coe_winter_thaw_state_loss_v1` on the 10.3.6
     paired Sleepers/Harvard thaw-ablation surfaces.
   - Publish JSON and Markdown reports into both `target/` and package artifacts.
   - Reconstruct daily SWE conservation and routed state-loss closure from
     emitted snowbench rows.
4. Coupled WAT rerun:
   - Run the real direct-production WAT path for the 10.3.5c snow-depth
     surfaces with absent `legacy_coe` selector versus
     `OPENWEPP_SNOWDENSITY1037_MELT_MODEL=coe_winter_thaw_state_loss_v1`.
   - Prove the selected melt model reached direct snow partition trace rows.
   - Classify whether coupled snow-control improves, is neutral, worsens, or
     remains blocked.
5. Review, verification, and closure:
   - Run focused tests and package diagnostic.
   - Run workspace closure gates.
   - Produce dual reviews, finding disposition, dual verification, line-count
     governance, owned-file manifest, worker handoff, and final disposition.

## Exit Criteria

- Contract includes the new selector, invariant, producer obligation, addendum,
  guard-map entry, and revision-history row.
- Focused test proves:
  - `legacy_coe` remains default/rollback and preserves low-density legacy gate.
  - `coe_winter_thaw_state_loss_v1` emits positive state loss under the isolated
    low-density thaw condition.
  - the candidate does not require or consume albedo state.
  - snowbench parses and reports the selector.
- Diagnostic report records paired thaw-ablation deltas and independent operand
  totals for raw melt, routed melt, SWE loss, depth loss, under-ablation counts,
  and aggregate depth-loss deficit.
- Diagnostic report proves daily emitted-row conservation:
  - prior SWE + snow input + retained rain - SWE loss - after SWE closes within
    tolerance;
  - routed melt - released rain - SWE loss closes within tolerance;
  - state loss never exceeds prior SWE plus same-day snow/rain input.
- Coupled WAT report records direct-production default-vs-opt-in paired
  snow-depth control deltas, trace proof for the selected melt model, and
  no-worse/improves/worse classification.
- Closure may be `complete` only if the opt-in candidate improves both paired
  under-ablation count and aggregate depth-loss deficit without violating
  isolation, conservation/routing, or coupled WAT gates. If the coupled WAT gate
  worsens snow control, or if conservation/routing evidence is missing/failing,
  close with `HOLD` and a concrete next blocker.
- Required gates have current-run evidence:
  - `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_correction.py`
  - `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py`
  - `cargo test --test snowdensity10_3_7_winter_thaw_melt_response_correction`
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
  - `wctl doc-lint --path docs/work-packages`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for package artifacts, contract
amendments, code changes, diagnostic evidence, and gate evidence. Expected
outputs are `artifacts/review_agent_*.md`, `artifacts/verification_agent_*.md`,
and `artifacts/finding-disposition.md`. Write access remains bounded to this
package's artifact files unless the operator explicitly authorizes otherwise.

## Security / Safety Impact

No secrets, network credentials, parser/runfile/user selectors, or public output
schemas are in scope. The candidate is opt-in only and must fail closed on any
invalid state rather than silently falling back or mutating the default path.
