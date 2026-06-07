# FQ3-DC ET Corn-Engagement Kickoff — annual-crop Ep/canopy closure

Execution mode: package-end-to-end

Autonomy: execute end-to-end — Milestone-1 localization + ownership, contract
amendment, contract-derived red/green tests, pre-implementation gate, production
correction, validation, dual review/verification, disposition, defect-shaped
handoff — without asking for direction on intermediate steps. Ask only if
hard-blocked.

## Item 1 — close defect `FQ3-DC-ET-CORN-ENGAGEMENT-001`

openWEPP produces `Ep`=0, `Interception`=0, `Er`=0 on every **annual-crop (Corn)**
single-OFE hillslope in `/wc1/runs/al/algebraic-radium` (36/36 Corn prefixes),
while legacy `wepp_260606` transpires Corn (p8 legacy `Ep`=1831 mm/7yr) and
openWEPP's own **perennial** path works (p1 Tah legacy `Ep`=5824 ≈ openWEPP 5511).
The transpiration that should occur on Corn is dumped to soil evaporation
(`Es` 4886 vs legacy 2764) — water conserves (rung-1 holds) but the ET partition is
physically wrong. Make openWEPP drive the annual-crop planting → growth → canopy →
transpiration cycle so Corn engages `Ep` (and canopy `Interception` and residue
`Er`), per `SC-PLANT-001` and `SC-EVAP-001`.

This is **Corn-specific** and a **separate root cause** from the runoff DC
(`FQ3-DC-RUNOFFPART-QQOFE-001`, already landed — do NOT touch runoff here).

Primary surfaces: `06_growth_state.rs` (`cancov`/`lai`/`xmxlai`/`dlai`),
`05_pl_phase_dispatch.rs` (annual-crop event projection),
`00_pl_slot_resolution.rs`, `07_decomposition_equations.rs` (`Er`),
`04_kernel_execution.rs` (`run_evapotranspiration`, `run_plant_root_uptake`),
`management.rs` (annual-crop plant parser). Touch `03_kernel_support_01/00` only for
ET-partition wiring that consumes plant state (**merge seam** with the landed runoff
DC — if you edit those files, re-verify runoff Q + conservation).

## Milestone 1 first (localize + ownership)

Reproduce `Ep`=0 on p8 (Corn); pick a growing-season window where legacy
transpires. **Diff the annual (Corn) vs perennial (Tah) path:** trace
`cancov`/`lai`/`xmxlai`/`dlai` growth-state and the `HydrologyEvapotranspiration` /
`PlantRootUptake` phases for a Corn prefix vs a Tah prefix on the same window. Name
the mechanism: annual-crop planting/event not projected (`management.rs` /
`05_pl_phase_dispatch.rs`) / canopy `lai` never develops (`06_growth_state.rs`) /
root uptake not engaged (`04_kernel_execution.rs`) / transpiration computed but not
partitioned into `Ep`. Ownership: legacy transpires Corn on the same daily gridmet
climate AND openWEPP's perennial path works → openWEPP annual-crop defect. **If the
cause is upstream in the management/plant-parser projection, amend `SC-PLANT-001`
PL-projection authority in-package if it's the canopy driver's input, else branch a
defect-shaped parser target.**

## Acceptance authority + constraints

- Conversion rule: root cause in-envelope + `SC-PLANT-001`/`SC-EVAP-001`/physical
  canopy-transpiration authority ⇒ MUST land the contract-first fix.
- `wepp_260606` is a FLAG that Corn should transpire (ADR-0017), NOT a match target.
  Acceptance = contract-correct annual-crop canopy/ET engagement.
- **Do not make ET absorb interception (or vice-versa).** `Ep`/`Es`/`Er` and canopy
  `Interception` are distinct WB terms — fix the shared canopy root cause and let
  each follow; no hand-tuned `Ep`, no folding interception into ET.
- **Interception acceptance is contract-first** (legacy WAT has no interception
  term — it cannot flag it; use `SC-PLANT-001`/`SC-EVAP-001` canopy authority).
- **Conservation must still close** (rung-1 identity + totalwatsed3 audit) after Corn
  transpires.
- **Do NOT regress the perennial path** (Tah p1 `Ep`≈5511 must not move).
- No comparator-match tuning, silent defaults, or downstream compensation.

## Hard constraints (protected boundaries)

- Do NOT touch runoff partition (FQ3-DC-RUNOFFPART, landed) or p11 percolation
  (FQ1-P11). Snow magnitude → Stage-2. 17-OFE MOFE out of scope.
- If you edit a shared kernel-phases file, re-run the runoff closure check
  (Q + conservation) as well as the corn-ET validation.

## Required reading

- `docs/work-packages/20260608-fq3dc-et-corn-engagement-closure-001/package.md`
- FQ-3 `artifacts/fq3-defect-handoff.md` + the FQ-3 review
- FQ3-DC-RUNOFFPART package + review (merge-seam reference)
- `docs/decisions/0011/0017/0018`, `docs/defect_closure_execplans.md`, `AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`,
  `SC-EVAP-001.md`, `SC-WATBAL-001.md`
- Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`; corpora
  `/tmp/fq1_after/outputs`, `/tmp/fq3_exec/legacy/outputs`.
