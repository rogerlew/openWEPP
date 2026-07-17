# Canopy Phenology GSI Process Kernel

Status: `COMPLETE — PASS-PROCESS-KERNEL`

Package id: `20260717-canopy-phenology-gsi-kernel-001`

Date: `2026-07-17`

Execution mode: `package-end-to-end`

## Objective

Implement the Jolly–Nemani–Running Growing Season Index (GSI) as openWEPP's
first contract-governed forest foliar-phenology process kernel. The kernel must
compute daily minimum-temperature, vapor-pressure-deficit, and signed-latitude
photoperiod indicators, combine them exactly as the published law specifies,
and maintain the published 21-day moving average without fixed calendar dates.

This package closes a process-law claim only. It does not yet map GSI to
openWEPP canopy cover, leaf area, biomass, litter, snow attenuation,
evapotranspiration, erosion, or production defaults.

## Authority And Frozen Intake

- Operator direction on 2026-07-17: complete canopy phenology before
  sublimation/longwave work and before returning ASSURE-06 to review.
- Frozen Git base:
  `45d49090214b4702d11a04aafe5d5ccade7ba440`.
- Jolly, W. M., Nemani, R., and Running, S. W. (2005), “A generalized,
  bioclimatic index to predict foliar phenology in response to climate,”
  *Global Change Biology* 11:619–632,
  <https://doi.org/10.1111/j.1365-2486.2005.00930.x>.
- Canonical accessible paper copy used for equation inspection:
  <https://www.frames.gov/documents/catalog/spa/jolly_nemani_running_2005.pdf>.
- Legacy formulation provenance:
  `/workdir/wepp-forest_260430_baseline/src/grow.for:804-850` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. It is contextual evidence for
  cold/photoperiod sensitivity, not the implementation target.
- Existing contract authority:
  `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`.
- Native forest input foundation:
  `docs/work-packages/20260702-dff-ws1-native-forest-lanuse-mode-001/` and
  `docs/contracts/openwepp-management-lanuse-authority-contract.md`.

## Scientific Boundary

The 2005 paper supplies:

- piecewise-linear daily indicators for minimum temperature, VPD, and
  photoperiod;
- `iGSI = iTmin * iVPD * iPhoto`;
- a 21-day moving average that buffers short weather excursions;
- a 0.5 onset/offset diagnostic threshold; and
- an explicit rationale for using GSI to scale potential LAI.

It does not supply openWEPP's canopy-cover interpolation, deciduous/evergreen
partition, foliar-versus-structural biomass pools, reserve allocation, or litter
transfer law. Those are excluded rather than inferred.

## Included Scope

- Amend `SC-PLANT-001` before production implementation with the GSI equations,
  state, domains, failure behavior, hemisphere invariant, and test vectors.
- Add a small `openwepp-plant-phenology` Rust crate containing:
  - typed parameter and daily-forcing values;
  - a year-aware forcing key and consecutive-day admission guard;
  - signed-latitude astronomical photoperiod;
  - the three daily constraint indicators;
  - instantaneous GSI;
  - exact bounded 21-day moving-window state; and
  - typed domain errors with no silent defaults.
- Test equation breakpoints, intermediate values, moving-window warm-up and
  eviction, polar day/night, Northern/Southern Hemisphere seasonal phase,
  invalid domains, and deterministic replay.
- Record the exact downstream integration contract that the next package must
  satisfy without implementing it here.

## Excluded Scope

- Changes to `cancov`, LAI, live biomass, interception biomass, root state,
  residue/litter pools, frost resistance, snow melt, ET, or erosion.
- Native management/YAML phenology operands or activation.
- A compatibility bridge from cropland-encoded forest managements.
- Calibration to the retained forest sites, empirical accuracy claims, or a
  public assurance-report update.
- Sublimation, longwave radiation, canopy snow interception, or ASSURE-06
  review activity.
- Any fixed Julian leaf-on or leaf-off date.

## Intended Write Set

- `Cargo.toml`
- `Cargo.lock`
- `crates/openwepp-plant-phenology/**`
- `docs/ROADMAP.md`
- `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260717-canopy-phenology-gsi-kernel-001/**`

Everything else is read-only. A downstream integration need discovered during
execution is recorded as a follow-on requirement, not added silently.

## Applicable Instructions

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/work-packages/AGENTS.md`

The chain was resolved with `tools/agents/find-agents --for` before edits.

## Plan

1. Freeze the literature equations, constants, units, and explicit limits.
2. Amend `SC-PLANT-001` with the process-kernel contract and reviewable vectors.
3. Implement the independent Rust kernel and its state machine.
4. Run focused tests, formatting, Clippy, and documentation checks.
5. Run full workspace gates and fresh adjudicated CRAP closure.
6. Complete dual independent review, disposition every finding, and complete
   dual terminal verification on the amended source.
7. Close with a precise handoff for native canopy/biomass/litter integration.

## Acceptance Criteria

| ID | Criterion |
| --- | --- |
| `CP-GSI-001` | `SC-PLANT-001` contains the exact published indicator/GSI/window law, typed domains, provenance, and fail-closed behavior before implementation is accepted. |
| `CP-GSI-002` | At the published default thresholds, temperature indicators are 0 at/below -2 °C and 1 at/above 5 °C; VPD indicators are 1 at/below 900 Pa and 0 at/above 4100 Pa; photoperiod indicators are 0 at/below 10 h and 1 at/above 11 h. |
| `CP-GSI-003` | Instantaneous GSI is the product of the three indicators and remains finite in `[0,1]`. |
| `CP-GSI-004` | The state reports the arithmetic mean of exactly the available trailing samples up to 21 days, then evicts the oldest sample deterministically. No exponential or calendar approximation is allowed. |
| `CP-GSI-005` | Photoperiod uses signed latitude and runtime day, remains finite in `[0,24]` hours including polar day/night, and exhibits opposite seasonal phase at equal-magnitude Northern/Southern latitudes. |
| `CP-GSI-006` | Invalid latitude, runtime day, threshold ordering, non-finite forcing, or negative VPD returns a typed error; no default, clamp-and-proceed, or hidden normalization masks invalid input. |
| `CP-GSI-007` | Deterministic replay of identical parameters and forcing produces bit-identical daily outputs and state. |
| `CP-GSI-007A` | Stateful admission rejects repeated, skipped, reversed, and year-invalid dates before mutation; restart state includes the ordered FIFO and newest date. |
| `CP-GSI-008` | No production canopy/biomass/litter consumer or assurance claim changes in this package. The integration boundary is documented explicitly. |
| `CP-GSI-009` | Required Rust gates, touched contract checks, documentation lint, line-count governance, and fresh CRAP closure pass on terminal source. |
| `CP-GSI-010` | Dual independent review and dual terminal verification close with every finding dispositioned. |

## Required Gates

Focused:

```bash
cargo nextest run -p openwepp-plant-phenology --profile quick
cargo clippy -p openwepp-plant-phenology --all-targets -- -D warnings
markdown-doc lint --path docs/ROADMAP.md
markdown-doc lint --path docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md
markdown-doc lint --path docs/work-packages/20260717-canopy-phenology-gsi-kernel-001
git diff --check
```

Terminal:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
```

Run the canonical adjudicated CRAP workflow from
`docs/work-packages/AGENTS.md`; touched production files must be at most 30 and
the workspace actionable set must remain empty. Record source line counts and
apply the 2,000-line warning policy.

## Subagent Authorization And Review Independence

Subagent authorization: this package explicitly authorizes two independent
read-only implementation reviewers and two independent read-only terminal
verifiers after the producer work and required gates are complete. Reviewers
must separately assess equation fidelity, numeric/domain behavior, hemisphere
logic, state determinism, scope containment, tests, and evidence truthfulness,
and must not read each other's initial review. Verifiers assess the fully
dispositioned terminal source. No subagent may change production code, broaden
the scientific claim, or authorize canopy integration.

## Progress

- [x] (2026-07-17) Reconciled the landed native forest/YAML foundation, current
  crop-style perennial growth path, legacy rangeland-only decline, and retained
  litter-date limitation.
- [x] (2026-07-17) Selected the published GSI as the bounded first process
  kernel and separated it from unratified biomass/canopy integration.
- [x] (2026-07-17) Scaffolded the package and made it the active roadmap item.
- [x] (2026-07-17) Amended `SC-PLANT-001` revision 21 with the process law,
  invariants, vectors, and integration hold before implementation.
- [x] (2026-07-17) Implemented the standalone typed kernel; post-review
  focused Nextest passes 13/13 across two binaries and strict package Clippy
  passes.
- [x] (2026-07-17) Dispositioned both independent reviews: added contract
  traceability, separated published window authority from inferred cold-start
  policy, added year-aware chronology/restart state, and strengthened the
  independent equation/FIFO/daylight vectors.
- [x] (2026-07-17) Passed focused and terminal gates: final full workspace
  Nextest 2,085/2,085, dependency policy clean, and fresh adjudicated CRAP with
  zero actionable rows in or outside the touched production file.
- [x] (2026-07-17) Accepted initial terminal-verification findings and added an
  explicit first-admission assertion plus public-API anchored-restart
  equivalence through the next forcing day; refreshed all gates afterward.
- [x] (2026-07-17) Completed dual independent review, finding disposition,
  two recorded HOLD verification cycles, and dual final terminal verification
  with independent `PASS` dispositions.

## Decision Log

- Decision: implement one continuous GSI kernel before separate leaf-off and
  leaf-on production branches.
  Rationale: the published law produces both limbs from the same signed-
  latitude physical constraints and has global/Harvard evaluation, while a
  leaf-off-only production state would leave the annual canopy cycle and
  interannual drift unresolved.
  Date/Author: 2026-07-17 / Codex.
- Decision: stop this package before canopy and biomass integration.
  Rationale: Jolly et al. supplies foliar-phenology and LAI-scaling authority,
  but not the biomass allocation, evergreen floor, litter transfer, or canopy-
  cover mapping required by openWEPP's existing conservation contracts.
  Date/Author: 2026-07-17 / Codex.

## Terminal Disposition

`PASS-PROCESS-KERNEL` (2026-07-17). The Jolly–Nemani–Running GSI equations,
FAO-56 signed-latitude photoperiod, year-aware chronology, exact 21-sample FIFO,
cold-start inference, typed failures, and anchored restart are contract-governed
and verified. Final dual terminal verification passes, full workspace gates
pass, and fresh adjudicated CRAP has zero actionable rows.

This is not integrated canopy phenology, empirical validation, or a snow-model
improvement. No production canopy, biomass, litter, snow, ET, erosion, native
YAML, or assurance consumer reads the new result. `CANOPY-PHENOLOGY-02` owns
that separately ratified integration.
