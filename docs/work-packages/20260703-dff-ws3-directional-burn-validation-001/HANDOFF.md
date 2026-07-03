# WS-3 Handoff — Directional Burn-Ordering Validation + Peakflow Magnitude Adjudication

From: Claude Code (campaign continuity). To: Codex (scaffold `package.md` from
this, then execute). Date: 2026-07-03. This is a **handoff brief**, not the
work-package — author `package.md` + artifacts under this dir per
`docs/work-packages/AGENTS.md`.

Campaign: [disturbed-forest-fidelity](../../planning/disturbed-forest-fidelity-strategy.md)
WS-3 (the acceptance gate for WS-1 + WS-2 together). Read the strategy's WS-3
section and `wepppy/tests/disturbed/analysis_results.md` before scaffolding.

## Objective
Two coupled deliverables:
1. **Directional-ordering validation harness** — the openWEPP analogue of the
   wepppy 80-cell burn matrix (4 textures × 5 veg × 4 burn severities): assert
   the burn-ordering **laws**, not magnitudes — burned total > matched-unburned
   for runoff, sediment, and peak; burned > unburned in the majority of matched-
   day events; texture/veg ordering.
2. **Peakflow magnitude adjudication** — resolve the physically impossible legacy
   burned-forest peak. Locate where the legacy peak-runoff calc diverges to
   river-scale under burn hydrophobicity and establish the sensible openWEPP
   magnitude **envelope**; confirm openWEPP's WS-2 `ksatadj` + WB14 path does
   **not** reproduce the artifact.

## Dependencies + current repo state (read before scaffolding)
- **WS-2 (`ksatadj`) is on `main`** (commit `2ae7fa13`): the source-intent
  effective-conductivity port + `min(Keff_ksatadj, frost_infcap)` frost
  composition. WS-3 validates *its* burn effect. `SC-SUBHYD-001` v34.
- **WS-1 (native forest `lanuse`) is UNMERGED** on branch
  `dff-ws1-inc2-native-forest-lanuse` (`f91399c0`, origin). **You likely do NOT
  need it merged for WS-3's runoff/sediment/peak laws:** `ksatadj` is *soil-side*
  (the burn effect flows through the `.sol` `DisturbedPolicy` + WB14 conductivity,
  independent of whether the `.man` is native-forest or the cropland masquerade).
  The 80-cell matrix can run on cropland-encoded managements + `ksatadj=1`
  disturbed soils. Confirm this at scaffold time; if a specific law needs the
  native lanuse authority, flag WS-1 merge as a prerequisite.
- **Existing anchor fixtures** (`tests/fixtures/disturbed_burn/`):
  - `forest_high_severity_clay_loam/` — the **magnitude-adjudication anchor**:
    McKenzie Bridge OR, ~1194 mm/yr, `ksatadj=1`, legacy `peakro` up to
    **380,150 m³/s** (mean 3,128/event over 480 events) vs unburned ~0.008 m³/s
    from a ~201 m hillslope. See its `manifest.md`.
  - `forest_high_severity_loam/` — real hillslope 313, `ksatadj=1`, **but
    MORAN WY (cold/dry): output-inert for `ksatadj`** (no infiltration-excess
    runoff → burned == unburned outputs). Do NOT use it as a directional-effect
    fixture (see below).

## CRITICAL — fixture climate selection (the load-bearing insight)
The directional laws (burned > unburned runoff/peak) only bite where there is
**infiltration-excess (Hortonian) runoff**. WS-2's own p313 anchor is
byte-identical burned-vs-unburned precisely because MORAN WY is cold/dry and
generates none. **WS-3's matrix must be built on a runoff-generating climate**
(warm, intense rain-on-unfrozen-ground storms) — the **McKenzie Bridge OR**
climate (the wepppy `analysis_results.md` matrix climate; the `clay_loam` anchor
`.cli`) — not MORAN WY. Verify each generated cell actually produces burned-vs-
unburned runoff divergence before asserting an ordering law on it; a cell with
no runoff on either arm is not evidence.

## Suggested scope shape (Codex decides the increments)
- **Fixture generation** is the bulk of the work: 80 cells = 80 disturbed run
  dirs (4 textures × 5 veg × 4 burn) on the McKenzie Bridge climate, each with a
  `ksatadj`-carrying `.sol` from the authoritative `(texture × class)` lookup +
  a management + the openWEPP TOML runfile. This is large — consider a
  representative subset first (e.g. the high-severity row + one gradient per
  texture) with a documented path to the full matrix, and note any silent
  reduction (don't imply full coverage from a subset).
  - Old CLIGEN `.cli` inputs may trip `CLIM-RUNTIME-E-017` (radly out of domain);
    normalize with `tools/clamp_cli_radly.py` and record it (see
    `tests/fixtures/AGENTS.md`).
- **Directional harness**: a test/harness that runs matched burned/unburned
  pairs and asserts the ordering laws on the *runoff-separated* event component
  where relevant (quickflow, not total — cf. `INV-SUBHYD-033`'s QUICKFLOW note),
  after conservation/closure holds.
- **Magnitude adjudication**: run the `clay_loam` anchor through openWEPP; record
  the openWEPP peak vs the legacy 380,150 m³/s; establish the plausible envelope
  for a ~201 m hillslope; attribute the legacy blow-up to the peak-runoff model
  (not `ksatadj`). Likely needs a new magnitude invariant/envelope (analogous to
  `INV-SUBHYD-033`'s investigation-flag posture) — decide contract-first.

## Contract touchpoints
- `SC-SUBHYD-001` `INV-SUBHYD-032` (the WS-2 `ksatadj` conductivity — the
  mechanism under test), `INV-SUBHYD-033` (forest lateral/water-yield
  observed-authority envelope, investigation-flag posture — the model for
  magnitude judgement).
- Erosion/sediment contract for the sediment-ordering law.
- `ADR-0011` (validate a law, not a number) + `ADR-0017` (legacy-as-flag): the
  legacy peakflow is forensic evidence, never a parity target.

## Guardrails (non-negotiable)
- **Directional ordering is acceptance; magnitudes are NOT targets.** The
  190,000× / 380,150 m³/s peak is explicitly *not* a target — it is the
  adjudication subject.
- **Frost stays ON** (the WS-2 decoupled posture; `min(Keff_ksatadj,
  frost_infcap)`).
- **Area duality**: pair `QOFE` with outlet-OFE area and `Q` with hillslope area;
  mixing them over-scales `runvol` 2–3× (prior MOFEFID error — see the memory /
  `reference_qofe_q_area_duality`). Relevant to any peak/runoff magnitude check.
- **Conservation first**: assert closure (WB/mass) before any ordering/magnitude
  verdict; a non-conserving cell is not evidence.

## Open questions to resolve at scaffold time
1. Full 80 cells vs a justified representative subset (and how to source the
   `(texture × class)` disturbed soils + managements — from wepppy, or the
   lookup).
2. Whether any law needs WS-1's native lanuse authority (else WS-3 proceeds on
   `main` + WS-2 without the WS-1 merge).
3. The magnitude-envelope invariant: extend `SC-SUBHYD-001`, or a dedicated
   peakflow-plausibility contract? Decide contract-first before the adjudication
   test.
