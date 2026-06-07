# FQ1 Soil Corrected-Layer Coverage Closure (HS-RUNTIME-E-062)

Status: queued

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `FQ1-HS-RUNTIME-E-062-SOIL-CORRECTED-LAYER-COVERAGE` end-to-end:
`37/43` single-OFE hillslopes on `/wc1/runs/al/algebraic-radium` fail closed with
`HS-RUNTIME-E-062` (`CorrectedLayerMappingIncomplete`) / surfaced as `CLIHILL-E-011`
before any hydrology output, blocking the frost rung (FROSTVAL01 could only reach
`6/43`). Make openWEPP's corrected soil-layer runtime mapping cover the full
profile for valid SURGO/disturbed soils so all 43 single-OFE hillslopes parse soil
and produce `H.wat.parquet` + `H.hbp` — or reclassify any genuinely-invalid soil as
typed invalid input with evidence.

This package owns correction inside the soil corrected-layer mapping envelope. If
the root cause is in-envelope and authority-backed, it must land the contract-first
fix.

## Rationale

FROSTVAL01 (rung-2) was blocked at population scale by this soil-runtime defect.
The typed failures are concrete (`/tmp/frostval01/full/run_status.tsv`):

- `p1`: "soil OFE 1 layer 6 corrected-lineage mapping coverage incomplete
  (1100..2000 mm, covered 700 mm)".
- `p2`/`p4`: "layer 4 … (760..2000 mm, covered 1040 mm)".
- `p3`: "layer 4 … (1270..2000 mm, covered 530 mm)".

openWEPP normalizes the soil into a corrected FC/WP layer lineage (scon.for-derived
`compute_normalized_corrected_layer_runtime_symbols_from_legacy_seed`) and maps it
back to the parser layers
(`map_corrected_layer_runtime_symbols_to_parser_layers`,
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:246`,
`:649-672`). For the 6 runnable soils the mapping reaches full coverage; for the 37
blocked (shallower / restricting-layer SURGO/disturbed profiles) it leaves a gap to
the 2000 mm target and fails closed. The error is real and fail-closed (good — no
silent bad state), but it blocks valid soils.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `FQ1-HS-RUNTIME-E-062-SOIL-CORRECTED-LAYER-COVERAGE`
  - Observable failure: `37/43` single-OFE hillslopes return rc=1 with
    `CLIHILL-E-011 ... HS-RUNTIME-E-062: soil OFE 1 layer <N>
    corrected-lineage mapping coverage incomplete (<lo>..2000 mm, covered <c> mm)`
    before hydrology/WAT output.
  - Fixture: `/wc1/runs/al/algebraic-radium/wepp/runs/` (the blocked prefixes per
    `/tmp/frostval01/full/run_status.tsv`).
  - The 6 currently-runnable prefixes (`p8,p13,p22,p23,p26,p28`) are the
    non-regression control.

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
    (soil profile / corrected FC-WP layer lineage authority).
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` only as a
    downstream consumer of the corrected-layer storage symbols.
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
    (corrected-layer normalization + parser-layer mapping — the primary surface).
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
    (the `CorrectedLayerMappingIncomplete` error type).
  - The soil parser (`crates/openwepp-input-contract/src/parsers/` soil) only if
    Milestone 1 proves layer depths/restricting-layer are mis-parsed.
  - `tests/integration/**soil**.rs`, `**fq1**.rs` for contract-derived regressions.
  - `docs/work-packages/20260608-fq1-soil-corrected-layer-coverage-closure-001/**`
  - `docs/work-packages/README.md`

### Allowed Edit Classes

- Amend canonical `SC-SOIL-001` for the proven corrected-layer coverage/extension
  behavior before production code.
- Correct the corrected-layer normalization/mapping so it covers the full profile
  for valid soils per legacy authority (e.g. correct profile-depth target,
  bottom-layer extension, or restricting-layer handling — whichever Milestone 1
  proves).
- Add contract-derived tests over the blocked-soil shapes and the 6-runnable
  non-regression control.
- Improve typed `HS-RUNTIME-E-062` evidence for genuinely-invalid soils.

### Protected Boundaries (do not cross)

- **Do not loosen the coverage guard to silently proceed on genuinely-invalid
  soil state.** `HS-RUNTIME-E-062` must still fail closed for real invalid input;
  the fix makes valid (shallow/restricting-layer) soils map correctly, it does not
  blanket-accept incomplete coverage.
- No frost, ET, runoff, snow, or hydrology-kernel changes — this is soil
  parse/runtime mapping only. (FQ-3/FQ-4 own ET/runoff and frost.)
- No downstream WB compensation for soil mapping.
- Snow magnitude remains a Stage-2 protected boundary.

### Acceptance Criteria

- All 43 single-OFE hillslopes either produce `H.wat.parquet` + `H.hbp` without
  `HS-RUNTIME-E-062`, or are reclassified as genuinely-invalid soil input with
  typed evidence (and `wepp_260606_hill` also failing on the same soil).
- The 6 currently-runnable prefixes still run (no regression) and their corrected
  FC/WP layer symbols are unchanged within tolerance.
- Any in-envelope correction is backed by canonical `SC-SOIL-001` text,
  contract-derived red/green tests, pre-implementation failing evidence, and
  post-fix validation.
- No silent coverage-guard loosening, defaults, or unbounded clamping.

### Branch-out Boundaries

- If Milestone 1 proves the gap is a soil-PARSER defect (layer depths /
  restricting layer mis-read) rather than the normalization/mapping, fix the
  proven surface (still in-envelope) or branch with a defect-shaped target.
- If a specific soil is genuinely invalid (legacy `wepp_260606_hill` also fails),
  reclassify it as invalid input with evidence rather than forcing coverage.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared soil
corrected-layer mapping envelope and the corrected behavior is supported by
canonical `SC-SOIL-001` authority, pinned-baseline soil-profile provenance, or a
contract-authorized physical invariant, it must proceed through contract
amendment, contract-derived tests, pre-implementation gate evidence, production
correction, validation, review, and disposition in this package. It may not close
as `HOLD` merely because more investigation is possible.

## Seven-Gate Bar

All seven true ⇒ `HOLD` is invalid and the package must land the fix:

1. Reproduction: at least one blocked prefix's `HS-RUNTIME-E-062` reproduced.
2. Mechanism: reduced to a named mechanism (normalization target / bottom-layer
   extension / restricting-layer handling / parser layer-depth), not "inspect the
   next soil field".
3. Ownership: the mechanism is in the declared soil write-set, and legacy
   `wepp_260606_hill` runs the same soils (so it is an openWEPP defect, not invalid
   input).
4. Authority: corrected behavior traces to `SC-SOIL-001` / legacy soil-profile
   provenance, not comparator matching.
5. Safety: no coverage-guard loosening for invalid soils; no silent default.
6. Testability: a contract-derived regression fails before and passes after, over
   the blocked-soil shapes + 6-runnable control.
7. Validation: the 43-of-43 run-through is measurable before/after.

## Symptom-Existence + Ownership Gate (Milestone 1, first)

1. Reproduce a blocked prefix (e.g. `p1`) and capture the exact corrected-layer
   mapping state: parser layer depths, the normalized corrected-layer increments,
   and where the `lo..2000 mm` coverage gap arises.
2. Determine the mechanism: is the 2000 mm target wrong (should map to actual
   profile depth / restricting layer), is bottom-layer extension missing, or are
   layer depths mis-parsed?
3. Ownership: confirm `wepp_260606_hill` runs the blocked soils (it should — they
   are standard SURGO/disturbed soils). If legacy runs them and openWEPP does not,
   it is an openWEPP defect to fix; if legacy also fails on a soil, reclassify that
   soil as invalid input.

## Legitimate HOLD Conditions

- The mechanism is outside the declared soil envelope.
- Canonical authority is missing/contradictory for the corrected coverage behavior.
- A soil is proven genuinely invalid (legacy also fails) — reclassify, do not force.
- Required evidence cannot be generated in the environment.

Grind-HOLD (forbidden): "inspect the next soil field," "trace the mapping one layer
deeper," "root cause in `02_soil_slope.rs` but implementation deferred."

## Milestones

1. Symptom-existence + ownership gate (above).
2. Contract: amend `SC-SOIL-001` for the proven corrected-layer coverage behavior.
3. Contract-derived red tests over the blocked-soil shapes + 6-runnable control.
4. Pre-implementation gate evidence.
5. Production correction in the corrected-layer mapping (or proven parser surface).
6. Validation: rerun all 43 single-OFE; confirm WAT emitted (or invalid-with-evidence)
   and no regression on the 6.
7. Dual review, finding disposition, dual verification, defect-shaped handoff
   (returns the unblocked substrate to FROSTVAL01 / the frost queue).

## Deliverables

- `artifacts/corrected-layer-coverage-localization.md` (Milestone-1 mechanism +
  ownership: legacy-runs-these-soils evidence).
- `artifacts/fq1-validation-ledger.md` (43-of-43 before/after run-through, 6-prefix
  non-regression).
- Standard contract, gate, dual-review, verification, disposition, worker-handoff
  artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- FROSTVAL01 package + `artifacts/frostval01-followon-queue.md`
- Frost-run blocked taxonomy: `/tmp/frostval01/full/run_status.tsv`
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
- Run inputs: `/wc1/runs/al/algebraic-radium/wepp/runs/`

## Autonomy

Execute end-to-end for the declared scope — Milestone-1 localization + ownership,
contract amendment, red tests, pre-impl gate, production correction, 43-of-43
validation, dual review/verification, disposition, defect-shaped handoff — without
asking for direction on intermediate diagnostic steps. Ask only if hard-blocked by
a proven boundary (authority missing, a genuinely-invalid soil, or a mechanism
outside the soil envelope).
