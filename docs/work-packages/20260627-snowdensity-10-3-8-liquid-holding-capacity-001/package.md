# SNOWDENSITY-10.3.8 Liquid Holding-Capacity Drainage

Status: complete

Evidence mode: Static/Ran.

## Objective

Implement and adjudicate the §10.3 step-6 durable snow liquid drainage fix:
replace the opt-in low-density thaw override with an opt-in physical liquid-water
holding-capacity drainage candidate, grounded only in in-repo authority.

The candidate must preserve `legacy_coe` as the default/rollback path. Closure
requires both event-window improvement and real coupled direct-production WAT
evidence, with conservation/routing proof from produced artifacts.

## Correction Authority Envelope

Defect family:

- `SNOWDENSITY-10.3.6`: winter-thaw under-ablation is defect-eligible.
- `SNOWDENSITY-10.3.7`: positive thaw melt routed as state loss improves the
  defect but remains incomplete (`978/1415` coupled paired WAT snow-control
  failures).
- §10.3 step 6: the remaining durable lever is the legacy `350 kg m^-3` density
  gate proxy in `INV-SNOWFREEZE-002`, which traps liquid in low-density maritime
  snowpacks.

Authority:

- `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 6.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.
- `references/copyrighted/noaa_6392_DS1.md` Anderson 1976 NWS-19 liquid-water
  retention/transmission notes.
- `references/annotated_bibliography.md` R-55 Marks et al. 1998 and R-36
  Anderson 2006 SNOW-17 notes.
- Existing in-repo SNOBAL-lineage constant
  `max_liquid_water_volume_fraction = 0.01` in
  `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs`.

Authorized edit classes:

- Amend `SC-SNOWFREEZE-001` before production edits.
- Add an opt-in CoE melt model ID `coe_liquid_holding_capacity_v1`.
- Add package-bound diagnostic direct-production selector support for the new
  model through `OPENWEPP_SNOWDENSITY1038_MELT_MODEL`.
- Add persistent retained-liquid typed snow-lane state for the opt-in candidate;
  daily scratch storage is not sufficient closure for this package.
- Add independent operand evidence for liquid capacity, liquid retained,
  excess liquid released, SWE loss, routed melt, and final state closure.
- Add snowbench and coupled WAT diagnostic tooling/evidence as needed.
- Add or update focused contract/integration tests.

Protected boundaries:

- No default activation.
- No parser/runfile/user CLI activation selector.
- No public output-schema change.
- No fixture input edits.
- No melt coefficient, radiation, canopy, phase partition, density-compaction,
  frost, sub-canopy longwave, rain-heat, Qwet/frzftp, or compatibility-runtime
  behavior changes.
- No site-calibrated holding-capacity constants.

## Intended Write Set

- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs` if CSV schema
  compatibility requires it
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
- `tests/integration/snowdensity10_3_8_liquid_holding_capacity.rs`
- Related snowdensity guard tests only for contract-version or source-location
  reconciliation.
- `tools/snowfreeze_observed/winter_thaw_melt_response_correction.py`
- `tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py`

## Phase Plan

1. Scaffold package artifacts and required-reading map.
2. Amend `SC-SNOWFREEZE-001` with `INV-SNOWFREEZE-067`,
   `OBL-SNOWFREEZE-P-042`, and a 10.3.8 addendum.
3. Add contract-derived tests for authority, selector boundaries, default
   identity, holding-capacity drainage, conservation/routing, and diagnostic
   reports.
4. Implement the opt-in candidate at the typed CoE snow partition seam.
5. Extend snowbench and coupled WAT diagnostic tooling to compare
   `legacy_coe`, `coe_winter_thaw_state_loss_v1`, and
   `coe_liquid_holding_capacity_v1`.
6. Run snowbench event-window and direct-production WAT gates; update package
   artifacts.
7. Complete dual review, finding disposition, verification, line-count
   governance, final disposition, and worker handoff.

## Exit Criteria

Closure may be `complete` only if all criteria pass:

- Contract amended before production code.
- `legacy_coe` absent/default behavior is byte/field-identical on focused
  fixtures.
- Unknown diagnostic selector values fail closed.
- Candidate uses the in-repo physical holding-capacity default, not fixture
  tuning.
- Candidate drains excess liquid above capacity and retains only bounded liquid.
- Independent reconstruction proves no net SWE/routed-liquid creation:
  available storage, liquid retained, excess released, SWE loss, routed melt,
  and final state close from produced artifacts.
- Paired Sleepers/Harvard event-window evidence improves both under-ablation
  count and aggregate depth-loss deficit relative to `legacy_coe`.
- Real direct-production WAT snow-control gate improves or is at least no worse
  than `legacy_coe`; if it worsens or cannot run, close `HOLD`.
- Direct snow trace proves the selected model reached the coupled path.
- No protected boundary changes are detected.
- Final gates pass: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, and docs lint for package docs.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to review and verification subagents for read-only package review, evidence
review, and gate legitimacy checks. Expected outputs are package-local review
and verification artifacts. Write access is not authorized for subagents.

## Closeout

Disposition: `COMPLETE-OPT-IN-IMPROVEMENT`.

The package implemented `coe_liquid_holding_capacity_v1` behind typed/snowbench
and package-bound direct-production selectors only. `legacy_coe` remains the
default and rollback path. No parser/runfile/user CLI selector, public output
schema, fixture edit, coefficient tuning, radiation/canopy/phase/density/frost
change, rain-heat/sub-canopy-longwave/Qwet change, or compatibility-runtime
change was made.

Event-window gate:

- Report: `artifacts/liquid-holding-capacity-event-window.json`.
- Disposition: `WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`.
- Under-ablation windows: `132 -> 94`.
- Aggregate depth-loss deficit: `24.105059374337998 m -> 15.506372398659 m`.
- Conservation: SWE balance residual `0`, routed state-loss residual `0`, and
  storage margin non-negative for the candidate.

Coupled WAT gate:

- Report: `artifacts/liquid-holding-capacity-coupled-wat.json`.
- Disposition: `WINTER-THAW-COUPLED-WAT-IMPROVES`.
- Paired snow-control failures: `1147 -> 761`.
- Paired row count: `1415`.
- No paired surface worsened.
- Residual blocker: `SNOW-CONTROL-NOT-CLEARED`; the opt-in improvement is not
  default activation and does not unblock frost attribution by itself.

Validation gates:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract`: pass.
- `markdown-doc lint --path docs/planning/snow-frost-fidelity-strategy.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/work-packages/README.md --path docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001 --format json`:
  pass, `10` files scanned, `0` errors, `0` warnings.
- `git diff --check`: pass.

Full-test fixups:

- The 10.3.1a snowbench CSV schema guard now includes the new liquid-capacity
  diagnostic columns.
- The direct-runtime layout guard now records the measured
  `DirectLaneFrame <= 1216` bound, reflecting the authorized retained-liquid
  lane state.
