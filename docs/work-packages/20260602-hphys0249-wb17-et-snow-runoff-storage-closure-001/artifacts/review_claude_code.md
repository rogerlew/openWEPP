# Review: Claude Code

Status: complete

Evidence mode: static (source/contract/test read) + recorded-log read

Static:

- Reviewer: Claude Code (`claude-opus-4-8`), invoked by user post-commit.
- Scope: static correctness review of HPHYS0249 commit `c0b9aa6` — added WB17
  constants, SC-EVAP-001 / SC-WATBAL-001 amendments, `run_plant_root_uptake`
  and WB17 soil-evaporation kernel changes, phase wiring, and the WB17
  contract test suite.
- Baseline cross-check: constants verified directly against
  `/workdir/wepp-forest_260430_baseline/src/evap.for` and `swu.for`.
- Gate evidence: read from committed `artifacts/gate-logs/`. Claude Code ran
  no `cargo` commands in this review; test pass/fail statements below are
  attributed to the recorded logs, not to a run by the reviewer.

## Confirmations (no action)

1. All six new WB17 constants trace to exact baseline source lines and match:
   - `WB17_CANOPY_EAJ_COEFFICIENT 0.5` / `WB17_CANOPY_BARE_SOIL_OFFSET 0.1`
     <- `evap.for:162` `eaj = exp(-0.5*(cv+.1))`.
   - `WB17_SOIL_EVAPORATION_DEPTH_M 0.10` <- `evap.for:624` upper-zone depth
     branch.
   - `WB17_TRANSPIRATION_LAI_FULL_COVER 3.0` <- `evap.for:583-586`
     `if (lai>3.0) ... ep = lai*eo/3.0`.
   - `WB17_SWU_UB 3.065` / `WB17_SWU_UOB 0.953346` <- `swu.for:87`.
   - `WB17_PLTOL_MIN 0.1` / `WB17_PLTOL_MAX 0.4` <- `swu.for:155-159` clamp.
2. Removed `WB17_LAI_PARTITION_COEFFICIENT 0.4` (legacy `exp(-0.4*lai)`
   partition) is correctly retired and named as non-authoritative in the
   contract and lineage map.
3. Contract-first sequence honored: SC-EVAP-001 (v10, `INV-EVAP-015`) and
   SC-WATBAL-001 (v75, `INV-WATBAL-037`) amended before production edits;
   `pre_impl_wb17_contract_test.log` records 2 failed, `post_impl` records 0
   failed — genuine red-green.
4. Phase ordering correct: `PlantRootUptake` (phase 10) executes after
   `Drainage` (8) and `LateralTransfer` (9), satisfying `INV-EVAP-015` /
   `INV-WATBAL-037` "swu after WB19" requirement.

## Findings

1. Low — citation misattribution. In SC-EVAP-001 symbol-source table,
   `WB17_TRANSPIRATION_LAI_FULL_COVER (3.0)` is referenced to
   `REF-EVAP-LEGACY-SWU`, but the LAI<=3 transpiration cap lives at
   `evap.for:583-586`, not `swu.for`. Constant value is correct; the
   reference points at the wrong baseline routine.

2. Medium — `pltol` sourcing gap not surfaced. The kernel hard-guards
   `pltol in [0.1, 0.4]` (faithful to `swu.for`), but
   `runtime_inputs/01_management.rs` seeds `pltol = 0.25` for all crops and
   never reads a per-crop value from the management input. The deficit-scaling
   branch (`pltol * ul`) is therefore real code driven by a single hardcoded
   default. Defensible as the baseline `<=0` default, but it is an unmodeled
   per-crop fidelity gap and is not flagged as such in the diff, contract, or
   gap register. Evidence: `01_management.rs` `("pltol", 0.25)` seed;
   `swu.for:152-159` default/clamp lineage assumes per-crop `pltol(itype)`.

3. Medium — depth-rationing cap is never exercised to bind in tests. The
   partial-layer withdrawal cap
   (`withdrawable = storage*(0.10-prev)/interval`) is the most error-prone new
   code path. In
   `hphys0249_wb17_soil_evaporation_mutates_layer_storage_before_aggregate_writeback`
   the remaining demand (~0.008 m) is below the layer-2 cap (~0.02 m), so the
   cap branch never limits an asserted withdrawal. No vector drives a
   cap-binding case where `withdrawable < remaining_soil_evaporation`.

4. Medium — test suite scope is lineage conformance, not closure/parity. All
   nine WB17 tests assert "kernel matches the contract formula for one seeded
   `BoundaryValue` surface." Expected values are independently recomputed from
   constants (not echoes of the implementation), which is good hygiene, but no
   test asserts a water-balance residual or baseline agreement. A green WB17
   suite confirms the mechanism is wired; it carries no signal about
   convergence. This is consistent with the contract-first model (legacy
   number is not an oracle) and is noted as scope context, not a defect.

5. High (investigation, not fix) — `Ep` fail count is unchanged after the
   transpiration pathway was rewritten. The package added a new `etp` partition
   (`lai*eo/3`) and an entire new `swu` root-uptake phase, yet
   `full-39-suite-metrics.md` reports `Ep` at `0/39` with fail-count sum
   `56834`, described as "unchanged." A literally-unchanged `Ep` after
   rewriting the whole `Ep` computation is a yellow flag: either the
   `PlantRootUptake` phase output is not reaching the `Ep` surface the semantic
   comparator reads, or `Ep` error is dominated by upstream `et_demand`/LAI and
   this layer was never the lever. The disposition treats "Ep unchanged" as an
   expected continuation item; this reviewer recommends it be verified (confirm
   `PlantRootUptake` executes and writes `Ep` in the 39-suite comparator runs)
   rather than assumed, before a follow-on WB17 package builds on this base.
   Evidence: `full-39-suite-metrics.md` `Ep 0/39 ... 56834`; disposition note
   "Ep is unchanged and remains the highest-priority WB17 residual."

## Notes for disposition owner (Codex)

- Findings 1-3 are localized and bounded by existing contract/test scope.
- Finding 4 is scope context; no change implied unless closure-tier tests are
  in scope for a later package.
- Finding 5 is the highest-value follow-on: the contract-faithful fix improved
  `Es` (fail-count sum 56898 -> 3272) and regressed aggregate storage
  (`Total-Soil`/`SoilWaterTotal` worse), per the disposition's own honest
  accounting. The unchanged `Ep` is the item most worth a verification step.
- Disposition `HOLD` at `0/39` is consistent with this review; no overclaim
  observed.

## Codex Disposition

Status: dispositioned

Evidence mode: static + ran

Disposition:

1. Fixed. `WB17_TRANSPIRATION_LAI_FULL_COVER` now cites explicit
   `evap.for:583-586` authority via `REF-EVAP-LEGACY-ETP`.
2. Fixed as disclosure. Added `GAP-EVAP-006` to surface that runtime currently
   seeds default `pltol = 0.25` and does not yet project per-crop
   `pltol(itype)` values.
3. Fixed. Added
   `hphys0249_wb17_soil_evaporation_depth_rationing_cap_limits_partial_layer_withdrawal`
   to exercise a binding partial-layer depth cap.
4. No change. Scope context accepted; HPHYS0249 remains contract-lineage
   validation plus full-suite metrics, not closure/parity proof.
5. Accepted for follow-on. `Ep` non-improvement is retained as the highest-value
   continuation item; handoff now explicitly calls out verifying comparator
   ingestion of `PlantRootUptake` `Ep` output before further WB17 work.

Ran:

- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
  passed `10/10` after the follow-up test addition.
