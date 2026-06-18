# STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE - Base Lateral/Percolation Conductivity Adjudication

Status: scaffolded 2026-06-18 (Stage-2 physics-magnitude; the *actual* driver of the H2637 71%,
after REFINTENT001 proved `ksatadj` is off for H2637)

Package type: **Stage-2 physics-magnitude adjudication — produces a VERDICT, not a fix.** Same
shape as STAGE2-LATQCC: judge a magnitude against physics + contract authority, legacy a flag
(ADR-0017). Output: `CORRECT` (FARPOINT01 resolves), `OPENWEPP-DEFECTIVE` (defect-closure ExecPlan),
or `CONTRACT-GAP`/`UNRESOLVED`.

## Why this package exists (the corrected localization)

REFINTENT001 fixed openWEPP's `ksatadj` `sat_frac` (a real defect) — **but it was byte-inert on
H2637 because `ksatadj = 0` there**: the post-fix WAT was byte-identical to pre-fix. So the H2637
71% lateral magnitude is **not** driven by `ksatadj`. STAGE2-LATQCC already showed it: "the lateral
conductivity is the soil conductivity exposed to WB19, **not** a `ksatadj` override" — H2637's
lateral conductivity is the **base soil conductivity** (`Ke`/`ssc`), and *that* is the
un-adjudicated driver of the 71%. This package adjudicates it.

## The lesson baked in — verify the driver FIRST (do not repeat the `ksatadj` detour)

**Before adjudicating the base conductivity, prove it actually drives the H2637 magnitude.** A
sensitivity probe: perturb the base lateral conductivity for H2637 and confirm the lateral
magnitude / `runvol` **changes** (byte-sensitivity). If a candidate driver is byte-inert, it is the
wrong driver — stop and re-localize. (The `ksatadj` detour cost a full fix package because the model
flag was never confirmed active for the fixture.)

## The question

Is openWEPP's H2637 **base lateral/percolation conductivity** (`Ke`/`ssc`) — derived from the
soil-file `ksat` through the WEPP 200 mm runtime-layer normalization and the layer averaging —
correct under `SC-SUBHYD-001` / `SC-INFILE-SOIL-001` / `SC-PERC-001` and the source-intent soil
lineage? I.e. does the conductivity that produces the 71% lateral magnitude **follow from** the
soil inputs correctly, or is a step inflated?

## Governing authority

- **`SC-INFILE-SOIL-001`** — the soil-file → runtime conductivity lineage (`ksat`, the WEPP 200 mm
  runtime-layer normalization at `input.for:748-928`, the FC/WP/theta terms).
- **`SC-PERC-001`** — the percolation conductivity `ssc` (which equals the lateral `ssh` at
  anisotropy 1.0 on H2637 — confirm that wiring is intended).
- **`SC-SUBHYD-001`** — the lateral flow consuming the conductivity (`INV-SUBHYD-003/012`, the
  `Ke = (86400/substeps)·Σk_depth/Σsat_depth` lineage STAGE2-LATQCC recomputed).
- **Source intent** (`wepp-forest_260430_baseline/src/{input,infpar}.for`): the 200 mm
  normalization (`input.for:748-928`) and the harmonic-mean layer conductivity
  (`infpar.for:~286`: `avks = solthk(2)/(solthk(1)/ssc(1) + (solthk(2)-solthk(1))/ssc(2))`).
- H2637 soil-file `ksat`: layers ≈ `60, 330, 33, 33 mm h⁻¹` (`p2637.sol`) — the raw inputs the
  conductivity lineage starts from.

## Method — extract the lineage, verify it drives, then judge

From the closed H2637 run, **with no `ksatadj` involvement** (it is off):

1. **Sensitivity probe (gate before adjudication):** perturb the base lateral conductivity; confirm
   `runvol`/`latqcc` changes (byte-sensitivity). Proves the base conductivity is the driver.
2. **Lineage extraction:** trace the H2637 base conductivity from soil-file `ksat` → the 200 mm
   runtime-layer normalization → `wb18_perc_ssc` → the `wb19` `Ke`. Record each transform + the
   values (the ~`9.2e-5 m/s` peak STAGE2-LATQCC observed).
3. **Source-intent + contract check:** does openWEPP's normalization + layer averaging match the
   legacy `input.for:748-928` / `infpar.for` intent and `SC-INFILE-SOIL-001`? Is the layer averaging
   the intended harmonic mean? Is `ssh == ssc` (lateral = percolation) the intended wiring, or
   should an anisotropy/lateral factor apply?
4. **Plausibility:** is the resulting `Ke` physically defensible for H2637's forest soil (WEPP Ch.5/6
   + the forest-hydrology authority MAGPARITY01 cited), or inflated by a normalization/averaging step?
5. **Verdict** (ADR-0017 taxonomy), each citing the transform/invariant/authority, legacy a flag.

## Leads to test (surface, do not pre-decide)

- **The 200 mm runtime-layer normalization** (`input.for:748-928`) — the most likely place a
  conductivity could be inflated/mis-distributed vs the source intent.
- **The layer-conductivity averaging** (`avks` harmonic mean) — does openWEPP average layer `ksat`
  the intended way?
- **The `ssh == ssc` (lateral = percolation) wiring** at anisotropy 1.0 — intended, or should
  lateral conductivity differ from vertical percolation?
- For any term lacking external/source authority, the **REFIMPL-intent / ADR-0024** route applies
  (legacy code intent as authority) — but **behavior stays a flag**; confirm the intent is clear.

## Verdict + handoff

- `CORRECT` — the base conductivity follows correctly from the soil lineage: the H2637 71% is
  correct forest hydrology → **resolve the FARPOINT01 flag**.
- `OPENWEPP-DEFECTIVE` — a normalization/averaging/wiring step is wrong → **Defect-Closure ExecPlan**
  item-1 (ADR-0018). No fix here.
- `CONTRACT-GAP` / `UNRESOLVED` — name the authority/evidence needed (or the ADR-0024 source-intent
  anchor to author).

## Scope

In scope: the H2637 base lateral/percolation conductivity lineage; the sensitivity probe; the
source-intent + contract check; the verdict + handoff.

Out of scope:

- **No `ksatadj`** (off for H2637; REFINTENT001 already corrected it).
- **No code/physics fix** (defects route to a defect-closure follow-on).
- **No `SC-*` change** (gap flagged); no legacy 55.5% parity target; no conservation/transfer rework.
- Irrigation deferred.

## Acceptance Criteria

- **Sensitivity probe** proving the base conductivity drives the H2637 `runvol`/`latqcc` (not inert).
- **Conductivity lineage** extracted (soil `ksat` → 200 mm normalization → `ssc` → `Ke`), values + transforms.
- **Per-step source-intent + contract verdict** (normalization, averaging, lateral=percolation wiring),
  citing `SC-INFILE-SOIL-001`/`SC-PERC-001`/`SC-SUBHYD-001` + source intent — not the legacy delta.
- **Verdict** (ADR-0017 taxonomy) + handoff (FARPOINT01 resolution / defect-closure ExecPlan / gap).
- Evidence Static / Ran. Markdown lint clean. (No Rust gates unless code touched — none expected.)

## Deliverables

- `artifacts/base-cond-sensitivity-probe.md` (proves the driver — the anti-detour gate)
- `artifacts/base-cond-lineage.md` (soil `ksat` → 200 mm normalization → `ssc` → `Ke`)
- `artifacts/base-cond-source-intent-check.md` (normalization/averaging/wiring vs intent + contracts)
- `artifacts/base-cond-plausibility.md` (forest-hydrology plausibility of the `Ke`)
- `artifacts/base-cond-per-step-verdict.md`
- `artifacts/base-cond-handoff.md` (flag resolution / defect-closure ExecPlan / gap)
- `artifacts/base-cond_disposition.md`

## Dependencies

- `docs/work-packages/20260618-refintent001-ksatadj-satfrac-defect-closure-001/artifacts/{review-claude-independent,refintent001_disposition}.md` (the corrected localization)
- `docs/work-packages/20260618-stage2-latqcc-h2637-magnitude-001/artifacts/{latqcc-equation-correctness,latqcc-operand-plausibility}.md` (the `Ke` lineage + the "base conductivity, not ksatadj override" signal)
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`, `SC-PERC-001.md`, `SC-SUBHYD-001.md`
- `docs/decisions/0024-reference-implementation-intent-authority.md`, `0017-...comparator-is-flag-not-target.md`, `0018-defect-closure-execplans-conversion-rule.md`
- `wepp-forest_260430_baseline/src/{input,infpar}.for` (200 mm normalization `input.for:748-928`; `avks` `infpar.for`)
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs` (openWEPP soil → conductivity projection)
- `docs/numerics/README.md`; `AGENTS.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the per-layer lineage trace + the
source-intent comparison are parallelizable. Record evidence.

## Autonomy

Execute end-to-end through the sensitivity probe (the anti-detour gate), the conductivity-lineage
extraction, the source-intent + contract check, plausibility, the verdict, and the handoff. **If the
sensitivity probe shows the base conductivity is *also* not the driver, STOP and re-localize** — do
not adjudicate an inert quantity. The verdict resolves or re-routes the FARPOINT01 71% flag.
