# Correctness Re-Anchoring — Keep / Condemn / Re-Derive Map

Author: Claude Code
Date (UTC): 2026-05-31
Status: proposal — first artifact of the correctness re-anchoring reset
Evidence basis: review of the HPARITY01→HPHYS0221 arc (27 commits, 28 packages,
2026-05-29 → 2026-05-31), `git` diff measurement, and the 39-hillslope semantic
runs cited per package. Resolution: **surface-level** (function / module /
contract-section granularity), intended to drive the reset; a line-level audit
follows per surface.

---

## 1. Why now — the parity method has hit its ceiling

HPHYS0221 is the clean confirmation: its disposition records *"latqcc and
total-soil means improved, Dp mean regressed, always-fail columns remain
39/39,"* and it auto-spawns HPHYS0222 to "isolate the Dp regression while
preserving latqcc gains." The big-four water-balance columns
(`Dp`/`latqcc`/`Total-Soil`/`SoilWaterTotal`) have not moved off `39/39` across
the entire 0207→0221 span; threshold tuning on a coupled flux partition can move
means but cannot cross tolerance, because there is **no physics target — only a
legacy-parity target tuned against by trial.**

Root finding (independently verified): the residual is a first-order soil-water
**over-drainage** defect (H1 candidate Total-Soil ~72 mm vs legacy ~645 mm; soil
drains to ~6% of porosity-store rather than ~59%), not the daily-reseed defect
the 0211 ledger named dominant (that was real, was fixed in 0212 — soil now
evolves day-to-day — and did not move the residual). Conservation + bounds pass;
the missing authority is the **constitutive** physics (how much water is
retained, where percolation cuts off). This is why packages can "close their
measures" while release disposition remains HOLD.

## 2. Correctness authority model being adopted

1. **Hard gates (dense, legacy-free, objective):** water-balance closure
   (profile + per-layer, every day/OFE) and physical bounds (`WP ≤ θ ≤
   porosity`, `FC ≥ WP`, fluxes ≥ 0, no water created). Necessary, not
   sufficient — a wrong model can still conserve.
2. **External-authority constitutive suites (the part conservation can't
   decide):**
   - *Level 4 — component physics* (the load-bearing tier for kernel
     correctness): FC = retention θ at −33 kPa and WP at −1500 kPa from the
     declared Rosetta/van Genuchten PTF; `cpm`/`coca` reduce storage by the
     rock/air volume fraction (geometric identity); behavioral laws (relax-to-FC
     between storms, `Dp→0` when `θ≤FC`, monotone recession, `ET ≤ min(PET,
     θ−WP)`); analytic / manufactured solutions.
   - *Level 5 — measured plot/lysimeter data* (system-level validation only).
   - *Level 6 — independent Richards solver* (HYDRUS) for canonical columns.
3. **Legacy is a sanity check and nothing more** (see §3).

Design rules: **test a law, not a number**; each constitutive test traces to an
`INV-<DOMAIN>` in `SC-*` that states the principle, the external authority, and
the tolerance. Each gate must define units and pass/fail thresholds explicitly.
**Level 4 outranks Level 5 for kernel adjudication** — WEPP was
calibrated to plots (effective Ksat is a fit parameter), so measured-match can
certify a broken kernel via compensating errors.

## 3. Legacy demotion policy (explicit)

- Legacy (`wepp-palimpsest` / `wepp-forest` baseline) is a **sanity check / change
  detector** only — per ADR-0011 it is a flagging mechanism, never an acceptance
  oracle. A 39/39 legacy delta triggers a Level-1→4 investigation; it does not
  by itself prove the candidate wrong.
- **End goal: transition completely off legacy comparisons.** No `SC-*` invariant
  and no acceptance gate may be expressed as "match legacy column X."
- **Transition sequencing (no coverage cliff):** keep the dense coverage by
  making the conservation + bounds gates run on every day/layer/OFE (cheap,
  legacy-free); grow the constitutive suites; legacy stays only as a regression
  change-detector until the gates + suites reach coverage, then it is **retired
  on a coverage milestone, not a date.** Any remaining `_legacy`-named code or
  parity clause is debt to be removed (§4).

### 3.1 Legacy retirement milestone (coverage-based, explicit)

Legacy comparison is removed from acceptance gates when all criteria below are
met for a sustained window:

1. Hard-gate coverage runs daily/per-layer/per-OFE on CI for all kernel PRs.
2. Level-4 constitutive suite exists for FC/WP, storage fractions, and
   lateral/drain constitutive behavior with explicit tolerances.
3. Level-4 behavioral suite exists for relax-to-FC, Dp cutoff near FC, ET
   bounds, and monotone inter-storm recession.
4. Analytic/manufactured suite exists for at least one infiltration and one
   recession primitive.
5. The new gate stack passes in CI for 30 consecutive days without a legacy-only
   blocker.

## 4. The map

Legend: **KEEP** = load-bearing physics, survives the reset · **CONDEMN** =
parity scaffold, remove once the condemning gate exists · **RE-DERIVE** = genuine
physical principle, parity-tuned implementation — keep the principle, re-author
against the gate.

### KEEP — physics foundation

| Surface | Location | Why keep |
|---|---|---|
| Soil retention corrections (`scon` path) | `02_soil_slope.rs:843 legacy_correct_layer_moisture` | Real soil physics (cpm/coca/sm20c/0.83 cap/floors). *Caveat:* the `cpm` term is the prime suspect for FC-too-low — kept, but gated by FC=θ(−33 kPa) (see RE-DERIVE). |
| Normalized-grid profile aggregation | `02_soil_slope.rs:633 compute_wb13_profile_symbols_from_legacy_seed` | Single correct grid; ProfileDepth/PorosityCap match legacy to 7 digits and pass on physics. This is the grid everything should use. |
| WB11 daily state-carry | `mod.rs` `ExecutionLane::Daily` (preserves mutable WB18/WB11 state) | Correct time-stepping; verified live (soil evolves day-to-day). |
| WB12 conservation/closure | `mod.rs` WB12 reconciliation | Budget must close — a hard gate, already objective. |
| Kernel phase architecture | WB11–WB20 phase structure | Sound decomposition; the reset re-anchors authority, not topology. |
| ProfileDepth / porosity | normalized aggregation | Physically correct, closed (0/39). |

### CONDEMN — parity scaffold to remove

| Surface | Location | Condemning gate / rationale |
|---|---|---|
| Dual-grid remap | `02_soil_slope.rs:730 compute_normalized_corrected_layer_runtime_symbols_from_legacy_seed` + `:754 map_corrected_layer_runtime_symbols_to_parser_layers` + `CorrectedLayer*` error types in `00_core_types.rs` | Reconciles two representations (normalized seed vs parser-grid) of one physical quantity — an artifact of the 0202↔0207 authority flip. FC=θ(−33 kPa) on a single grid removes the need for two grids and the depth-overlap weighting. |
| Legacy-pool `max()` in lateral flux | `kernel_phases.rs:906 drainable_storage_legacy`, `:1086 available_pool = layer_pool.max(drainable_storage_legacy + recharge_pe)` | A parity-reconciliation `max()` against a legacy-named quantity. Lateral flux must equal the physical drainable pool (saturated thickness above the restrictive layer, `θ−FC`), not be max'd with a legacy term. |
| FC/WP authority flip-flop residue | `SC-WATBAL-001` (~21 authority-language lines; HPHYS0202 "layer-authoritative / non-authoritative" vs HPHYS0207 "storage-symbol authoritative" marked "historical") | Re-state as one physics-derived publication authority; delete superseded/parity-derived clauses. |
| Parity-tuned WB19 threshold knobs | `kernel_phases.rs` WB19 region (fcdep/coca/cpm tuning added 0218–0221) | To the extent these were tuned to move parity columns rather than derived — condemned by the behavioral gates (relax-to-FC, Dp-cutoff). See RE-DERIVE for the physical core. |
| Adjudication/diagnostic process scaffold | WP packages 0204, 0210, 0211, 0214, 0217, 0220 (docs) | ≥6 of 28 packages assessed the residual without moving it; the assessment cadence is replaced by gate-driven adjudication. |

### 4.1 Condemnation exit triggers (required before deletion)

- **Dual-grid remap:** remove after `field_capacity_equals_retention_theta_at_minus_33kpa`
  passes on the unified grid and no parser-grid fallback is required.
- **Legacy-pool `max()`:** remove after `lateral_flow_responds_to_saturated_thickness`
  and profile/layer closure gates pass without the reconciliation term.
- **FC/WP authority residue:** remove after one canonical FC/WP publication
  authority is asserted in `SC-WATBAL-001` and covered by contract tests.
- **WB19 parity knobs:** remove or re-derive each knob once a constitutive or
  behavioral gate proves the physical replacement path.

### RE-DERIVE — genuine principle, parity-tuned application

| Principle (keep) | Current impl (re-author) | Gate to derive against |
|---|---|---|
| Rock-fragment (`cpm`) & entrapped-air (`coca`) reduce storage | applied in `legacy_correct_layer_moisture`; prime suspect for FC at ~10% of porosity (implausibly low for silt loam at 17–54% rock) | `cpm`/`coca` must equal the rock/air **volume fraction** (geometric identity); resulting FC must equal θ(−33 kPa). |
| Lateral/drainage flux from Darcy + saturated thickness over a restrictive layer | WB19 `run_lateral_transfer` fcdep/coca/anisotropy coupling | Darcy lateral flux; `relax-to-FC`; `Dp→0` at FC; conservation of the partitioned budget. |

## 5. The gates that do the condemning (first suites to build)

Build these before/with removal so condemnation is evidence-based:

- **Hard gates (dense):** `profile_and_layer_water_balance_closes_each_day`;
  `theta_within_wilting_and_porosity`; `fc_ge_wp`; `fluxes_nonnegative`.
- **Constitutive (Level 4):** `field_capacity_equals_retention_theta_at_minus_33kpa`;
  `wilting_equals_retention_theta_at_minus_1500kpa`;
  `cpm_coca_reduce_storage_by_rock_air_volume_fraction`.
- **Behavioral (Level 4):** `profile_relaxes_to_field_capacity_between_storms`
  (**the over-drainage adjudicator** — would have caught 72-vs-645 legacy-free);
  `deep_percolation_ceases_at_field_capacity`;
  `inter_storm_soil_water_recession_is_monotone`;
  `et_bounded_by_pet_and_available_water`;
  `lateral_flow_responds_to_saturated_thickness`.
- **Analytic (Level 4):** `bucket_drains_to_fc_and_stops`;
  `green_ampt_single_storm_matches_closed_form`;
  `linear_reservoir_recession_matches_time_constant`.

`field_capacity_equals_retention_theta_at_minus_33kpa` +
`profile_relaxes_to_field_capacity_between_storms` are the two that adjudicate the
live over-drainage and condemn the dual-grid / `cpm` path; they are the first to
stand up.

### 5.1 Gate specification minimums

Each gate above must include:

1. Explicit authority citation (`SC-*` invariant + external reference).
2. Explicit units and tolerance (`abs`, `rel`, or mixed).
3. Fixture class (`unit`, `component`, `integration`) and expected runtime cost.
4. CI lane (`required` vs `periodic`) and failure class (`hard-fail` vs
   investigation).

## 6. Sequence

1. Stand up the dense hard gates (legacy-free coverage floor).
2. Stand up the two FC/relax-to-FC constitutive tests; run them on the current
   tree to confirm-or-refute the `cpm`-over-correction / over-drainage diagnosis
   with physics, not legacy.
3. Re-derive the RE-DERIVE surfaces against those gates.
4. Remove the CONDEMN surfaces once a gate covers their function; collapse the
   dual grid to one; delete the legacy-pool `max()` and the flip-flop contract
   clauses.
5. Demote legacy to change-detector; retire on the coverage milestone (§3).

Execution checkpoint after each step:
- record gate outcomes,
- record changed surfaces,
- record whether any CONDEMN surface is now eligible for deletion.

## 7. Recommendation on rollback

Do **not** wholesale-roll-back the code: the KEEP surfaces are genuine physics a
rewrite would re-author identically, and rollback re-incurs that cost while
leaving the authority gap (the actual cause) unsolved. The debt is concentrated
in the contracts (parity-derived authority) and three code surfaces (dual grid,
legacy-pool `max()`, WB19 parity knobs) — all localized and listed above. Reset
the **contracts**, build the **gates**, let them **condemn** the scaffold for
surgical removal. If targeted rollback is taken, scope it to FC/WP authority
churn and WB19 threshold knobs only, not the kernel foundation.
