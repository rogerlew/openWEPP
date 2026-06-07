# FQ3-DC ET Corn-Engagement Closure

Status: complete

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

Closure note: execution closed the observed annual Corn `Ep` and canopy
`Interception` engagement defect. The kickoff objective also listed `Er`, but
upstream FQ-3 evidence classified `Er=0` as expected-config-zero with legacy
`Er=0`; this is dispositioned in `artifacts/disposition.md` as an objective
overclaim, not an unresolved defect.

## Objective

Close defect `FQ3-DC-ET-CORN-ENGAGEMENT-001` end-to-end: openWEPP produces zero
transpiration (`Ep`=0), zero canopy interception, and zero residue evaporation
(`Er`=0) on every **annual-crop (Corn)** single-OFE hillslope in
`/wc1/runs/al/algebraic-radium`, while the legacy comparator transpires materially
(p8 corn legacy `Ep`=1831 mm over 7 yr) and the perennial (Tah_4899) path works in
openWEPP (p1 legacy `Ep`=5824 ≈ openWEPP 5511). Make openWEPP drive the annual-crop
planting → growth → canopy → transpiration cycle so Corn engages ET and
interception per `SC-PLANT-001` and `SC-EVAP-001`, validated on the
algebraic-radium Corn population, **without regressing the working perennial path**.

This package owns correction inside the plant-growth / canopy / ET-engagement
envelope. If the root cause is in-envelope and authority-backed, it must land the
contract-first fix.

## Run / sibling context

Parallel to `FQ3-DC-RUNOFFPART-QQOFE-001` (runoff partition — **landed**, see
`docs/work-packages/20260608-fq3dc-runoffpart-q-qofe-closure-001/`). The two are
distinct root causes:

- This defect (`Ep`/`Interception`/`Er`=0) is **Corn-specific** — 36/36 Corn
  prefixes; perennial Tah (6/6) transpires fine. It is a crop-canopy engagement
  defect, not a water-balance defect.
- `Q`=0 was **universal across managements** (hit Tah too) and is closed by the
  runoff DC.

Both come from FQ-3
(`docs/work-packages/20260608-fq3-et-runoff-zero-term-characterization-001/`).

**Merge seam:** the runoff DC edited
`03_kernel_support_01_kernel_phases.rs` / `03_kernel_support_00_support_helpers.rs`.
If this package edits ET-partition wiring in the same files, re-run **both** the
runoff closure (Q + conservation) **and** the corn-ET check after this lands.

## Rationale (FQ-3 evidence, comparator-flag confirmed)

FQ-3 + Claude verification (legacy `wepp_260606` WAT vs openWEPP post-FQ1):

| | legacy `Ep` | openWEPP `Ep` | legacy `Es` | openWEPP `Es` |
|---|---:|---:|---:|---:|
| p8 (Corn) | 1831 | **0** | 2764 | 4886 |
| p1 (Tah_4899) | 5824 | 5511 | — | — |

- Corn `Ep`=0 on 36/36 Corn prefixes; correctly **not** flagged on Tah.
- Corn `Interception`=0 and `Er`=0 (Tah p1 `Interception`=643): no canopy → no
  interception, no transpiration, no residue evaporation. Almost certainly the
  **same root cause**.
- Water still **conserves** (rung-1 closure holds): the transpiration that should
  occur on Corn is instead dumped to soil evaporation (`Es` 4886 vs legacy 2764).
  This is a **partition/magnitude defect**, not a conservation defect.

Legacy transpires Corn on the **same daily gridmet climate**, so the annual-crop
ET mechanism is reachable on this forcing and openWEPP is failing to engage it —
an openWEPP defect, not invalid input.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `FQ3-DC-ET-CORN-ENGAGEMENT-001`
  - Observable: `Ep`=0, `Interception`=0, `Er`=0 on all Corn single-OFE prefixes;
    `Es` over-inflated. Fixture `/wc1/runs/al/algebraic-radium/wepp/runs/`,
    post-FQ1 corpus `/tmp/fq1_after/outputs`, legacy
    `/tmp/fq3_exec/legacy/outputs`.
  - Per-prefix classification in FQ-3 `per_prefix_term_classification.csv`.

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` (**primary** —
    cropland plant-state evolution; PL transition-control runtime projection
    authority for **annual/perennial** management event payloads; LAI / canopy /
    root-depth / biomass descriptors).
  - `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` (potential /
    actual ET partition and the plant-state consumers that drive transpiration
    `Ep` and residue evaporation `Er`).
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` as the
    downstream consumer (closure must stay closed; interception is a distinct
    water-balance term, not folded into ET).
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/06_growth_state.rs`
    (plant growth state: `cancov`, `lai`, `xmxlai`, `dlai`).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/05_pl_phase_dispatch.rs`
    (PL phase dispatch — annual-crop event projection).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs`
    (PL slot resolution).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`
    (residue / decomposition surfaces feeding `Er`).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`
    (`run_evapotranspiration`, `run_plant_root_uptake` at the
    `HydrologyEvapotranspiration` / `PlantRootUptake` phases).
  - `crates/openwepp-input-contract/src/parsers/management.rs` (annual-crop
    management/plant parser projection).
  - `03_kernel_support_01_kernel_phases.rs` / `03_kernel_support_00_support_helpers.rs`
    **only** for ET-partition wiring that consumes plant state (merge seam with the
    runoff DC — see above).
  - `tests/integration/**plant**.rs`, `**evap**.rs`, `**et**.rs`, `**fq3dc**.rs`,
    `**watbal**.rs`.
  - `docs/work-packages/20260608-fq3dc-et-corn-engagement-closure-001/**`,
    `docs/work-packages/README.md`.

### Allowed Edit Classes

- Amend canonical `SC-PLANT-001` (and/or `SC-EVAP-001`) for the corrected
  annual-crop canopy/transpiration-engagement behavior **before** production code.
- Correct the annual-crop plant-growth → canopy (`cancov`/`lai`) → root-uptake →
  transpiration path so Corn engages `Ep` (and the canopy-driven `Interception`
  and `Er`) when the perennial path already does.
- Add contract-derived tests (a Corn growing-season day producing nonzero `Ep` /
  canopy; a perennial non-regression case; the 36 Corn prefixes).
- Add bounded diagnostics to localize where the annual-crop canopy/LAI/transpiration
  path diverges from the working perennial path.

### Protected Boundaries (do not cross)

- **No comparator-match acceptance.** `wepp_260606` is a flag that Corn should
  transpire (ADR-0017); acceptance is contract-correct annual-crop canopy/ET
  engagement, not matching legacy `Ep` magnitude.
- **Do not make ET absorb interception (or vice-versa).** `Ep`/`Es`/`Er` and
  canopy `Interception` are **distinct** water-balance terms; fix the shared
  canopy-engagement root cause and let each term follow. Do not hand-tune `Ep`
  values or fold interception into ET to make a number move.
- **Interception acceptance is contract-first.** Legacy WAT exposes no interception
  flux term, so it cannot flag interception — define interception acceptance from
  `SC-PLANT-001`/`SC-EVAP-001` canopy authority, not from the comparator. Do not let
  the comparator-availability gap stall the canopy fix.
- Conservation must stay closed — the rung-1 identity + totalwatsed3 audit (incl.
  interception, snow) must still hold after Corn transpires.
- **Do not regress the perennial path** (Tah p1 `Ep`≈5511 must not move).
- Do not touch runoff partition (`FQ3-DC-RUNOFFPART-QQOFE-001`, landed) or `p11`
  percolation (`FQ1-P11`). Snow magnitude remains a Stage-2 protected boundary.
- The 17-OFE hillslope (MOFE) is out of scope (rung-3).

### Acceptance Criteria

- openWEPP produces nonzero, canopy-driven `Ep` (and canopy `Interception` and
  residue `Er`) on the affected Corn single-OFE hillslopes across the growing
  season, consistent with `SC-PLANT-001`/`SC-EVAP-001` (legacy nonzero as a flag,
  not a target), with the over-inflated soil-evaporation share (`Es`) correspondingly
  corrected.
- The perennial Tah path is **non-regressed** (`Ep`≈5511).
- The water-balance closure (rung-1 identity + totalwatsed3 audit) still closes
  after Corn transpires; the landed runoff closure (Q + conservation) is
  re-verified if a shared kernel-phases file was touched.
- Contract-derived red/green tests; pre-implementation failing evidence; post-fix
  validation over the 36 Corn prefixes + non-regression on perennial + correct terms.
- No conservation break, comparator-target tuning, silent default, ET↔interception
  absorption, or downstream compensation.

### Branch-out Boundaries

- If Milestone 1 proves the cause is upstream in the **management/plant-parser
  projection** of annual-crop events (rather than the growth/canopy kernel), amend
  `SC-PLANT-001` PL-projection authority in-package if it is the canopy driver's
  input, or branch a defect-shaped parser target if it is a broader input-contract
  defect.
- If a subset of Corn prefixes legitimately produce ~0 `Ep` (e.g. fallow / no
  growing season in the window — legacy also ~0), exclude them with evidence.

## Conversion Rule

If a reproducible root cause is established inside the declared
plant-growth/canopy/ET-engagement envelope and the corrected behavior is supported
by canonical `SC-PLANT-001`/`SC-EVAP-001` authority, pinned-baseline provenance, or
a contract-authorized physical invariant (annual-crop canopy growth → transpiration),
the package must proceed through contract amendment → tests → pre-implementation
gate → production correction → validation → disposition. It may not close `HOLD`
because more investigation is possible.

## Seven-Gate Bar

All seven true ⇒ `HOLD` invalid, must land the fix: (1) reproduce `Ep`=0 on an
affected Corn prefix; (2) named mechanism (annual-crop event not projected /
canopy/`lai` never grows / root-uptake not engaged / transpiration not partitioned),
not "trace deeper"; (3) ownership in the plant-growth/ET write-set (legacy
transpires Corn on same climate; perennial works in openWEPP); (4) authority
`SC-PLANT-001`/`SC-EVAP-001`/physical canopy-transpiration law, not comparator
match; (5) safety — no conservation break, no ET↔interception absorption, no silent
default, no perennial regression; (6) testability — red/green on a Corn
growing-season day; (7) validation — Corn-population `Ep`/canopy measurable
before/after, perennial non-regressed.

## Symptom-Existence + Ownership Gate (Milestone 1, first)

1. Reproduce `Ep`=0 (and `Interception`=0/`Er`=0) on an affected Corn prefix
   (e.g. p8) and pick a growing-season window where legacy transpires.
2. **Localize the divergence between the annual (Corn) and perennial (Tah) path:**
   trace `cancov`/`lai`/`xmxlai`/`dlai` growth-state evolution and the
   `HydrologyEvapotranspiration` / `PlantRootUptake` phases for a Corn prefix vs a
   Tah prefix on the same window. Name the mechanism — annual-crop planting/event
   not projected (`management.rs` / `05_pl_phase_dispatch.rs`), canopy/`lai` never
   develops (`06_growth_state.rs`), root uptake not engaged
   (`04_kernel_execution.rs` `run_plant_root_uptake`), or transpiration computed but
   not partitioned into `Ep`.
3. Ownership: legacy `wepp_260606_hill` transpires Corn on the same daily climate,
   and openWEPP's own perennial path works → openWEPP annual-crop defect.

## Legitimate HOLD Conditions

- Mechanism outside the declared plant-growth/canopy/ET envelope (branch with a
  defect-shaped target — e.g. a broader management input-contract defect).
- Canonical authority missing/contradictory.
- A subset of Corn prefixes legitimately produces ~0 `Ep` (legacy also ~0) — exclude
  with evidence.
- Required evidence cannot be generated in the environment.

Grind-HOLD (forbidden): "inspect the next growth variable," "trace canopy one step
deeper," "root cause in `lai` projection but implementation deferred."

## Milestones

1. Symptom-existence + ownership gate (above) — annual-vs-perennial divergence
   localized to a named mechanism.
2. Contract: amend `SC-PLANT-001` (and/or `SC-EVAP-001`) for corrected annual-crop
   canopy/transpiration engagement; define interception acceptance contract-first.
3. Contract-derived red tests (Corn growing-season day → nonzero `Ep`/canopy;
   perennial non-regression; correct-term non-regression).
4. Pre-implementation gate evidence.
5. Production correction in the proven plant-growth/canopy/ET-engagement surface.
6. Validation: Corn-population `Ep`/`Interception`/`Er` engaged; `Es` share
   corrected; perennial non-regressed; WB closure still holds; runoff closure
   re-verified if a shared kernel-phases file was touched.
7. Dual review, finding disposition, dual verification, defect-shaped handoff.

## Deliverables

- `artifacts/corn-et-engagement-localization.md` (M1 annual-vs-perennial mechanism +
  ownership).
- `artifacts/fq3dc-et-validation-ledger.md` (before/after Corn `Ep`/`Interception`/
  `Er`/`Es` + perennial non-regression + closure preservation + runoff
  re-verification if applicable).
- Standard contract, gate, dual-review, verification, disposition, handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/defect_closure_execplans.md`
- `docs/decisions/0011-...`, `0017-...`, `0018-...`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`,
  `SC-EVAP-001.md`, `SC-WATBAL-001.md`
- FQ-3 package + `artifacts/fq3-defect-handoff.md`,
  `per_prefix_term_classification.csv`
- FQ3-DC-RUNOFFPART package (merge-seam reference, landed)
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
- Run inputs: `/wc1/runs/al/algebraic-radium/wepp/runs/`; corpora
  `/tmp/fq1_after/outputs`, `/tmp/fq3_exec/legacy/outputs`

## Autonomy

Execute end-to-end for the declared scope — M1 localization + ownership, contract
amendment, red tests, pre-impl gate, production correction, validation, dual
review/verification, disposition, defect-shaped handoff — without asking for
direction on intermediate steps. Ask only if hard-blocked by a proven boundary
(annual-crop projection outside the plant-growth envelope, missing authority, or a
legitimately zero-transpiration Corn subset).
