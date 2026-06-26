# SNOWDENSITY-05F Independent Review (Claude)

Reviewer: Claude Code. Date: 2026-06-26.

Evidence class: **Static** (read all 05F artifacts; read the 05E
implementation `snowbench_coe_melt.rs` and `08_snow_albedo.rs`; read the
production melt formula in `infiltration_reconciliation.rs`) **+ Ran** (grep /
source-scan to locate and confirm constants and call sites). I did **not** run
the snowbench or a hillslope sim; the live winter `cancov` value is therefore
**inferred**, not measured (flagged below).

This is an independent review and does not replace the package's own
`review.md` / `verification.md`. Dispositions are recommendations; routing and
contract decisions remain Codex's.

## Verdict

As a **contract/interface freeze**, 05F is well-executed and correctly
conservative on activation. The defect is not in what it activates (nothing) but
in what it **omits from its residual-risk register**: it freezes the 05E melt
boundary and binds it into `SC-SNOWFREEZE-001` v80 while the supporting 05E
evidence was generated in a regime that is unrepresentative of the forested
validation sites. The two specific, material causes are not recorded anywhere in
the package.

Recommendation: **not a revert.** Record the two fidelity caveats on the
boundary, and make the harness-fidelity fix a precondition of SNOWDENSITY-06
rather than something density silently inherits.

## What is sound (credit)

- **Activation confinement is real and verified.** Production day-input builder
  still selects `SnowMeltModel::LegacyCoe`
  (`...day_input_and_helpers/00_builders_and_authority.rs:3003`); `--model` /
  `coe_shortwave_albedo_v1` are confined to `openwepp-snowbench`. No
  parser/runfile/CLI/output/default-activation surface added. A
  contract/default-confinement integration test was added; gates green.
- **Honest about the headline.** The contract's Activation Evidence Baseline
  records both `05E 13->10 / 61->84` and `as-built 9 / 84` side by side, and the
  handoff lists "did not beat H as-built" as a residual risk.
- Cold-start albedo continuity rule ratified; docs-only, low blast radius.

## Findings

### F1 — 05E acceptance evidence was generated at `cancov = 0.0` on forested sites (material)

- The diagnostic harness hardcodes `DEFAULT_CANOPY_COVER_FRACTION = 0.0`
  (`snowbench_coe_melt.rs:22`), passed for all five sites (`:214`, `:543`).
- Production's CoE melt attenuates by the **real** canopy cover
  (`infiltration_reconciliation.rs:221-226`, `:240-243`):
  `amelt = 0.0607 * hrad * shortwave_absorbed_fraction * (1 - cancov * CANOPY_FACTOR)`,
  `cmelt = ... * (1 - 0.8 * cancov * CANOPY_FACTOR)`.
- The fixtures are `luse = forest`; the `.man` canopy coefficients are
  `0.93`/`0.90`, and `cancov = 1 - exp(-bb * canopy_biomass)`
  (`06_growth_state.rs:231`, clamped to `PL_GROWTH_CANCOV_MAX`). So the
  growing-season cover for these sites is high (order ~0.9), not 0.
- Consequence: at `cancov = 0` the radiation melt term runs at full strength
  `(1-0)` versus roughly `(1-0.9) ~ x10` smaller in production. This both
  **depresses the diagnostic `legacy_coe` baseline** (a likely primary driver of
  the diagnostic `13/61` vs as-built `9/84` gap) and **inflates the leverage of
  the modernized shortwave/albedo term** by ~an order of magnitude, since that
  term lives inside `amelt`.
- **Precision:** `cancov = 0` is a property of the diagnostic *harness*, not of
  the `CoeShortwaveAlbedoV1` kernel model (the kernel uses whatever `cancov` it
  is handed). The accepted boundary's *code* is fine; the *acceptance evidence*
  is what is compromised. Fix = re-validate at real `cancov`, not change the
  model.
- **Inference caveat (truthfulness):** ~0.9 is inferred from the `.man`
  coefficients and the growth formula. The value that actually matters is the
  **winter / snow-season** `cancov` from the live growth state, which I did not
  run to confirm. A senescing perennial could sit below the peak; an evergreen
  conifer stays high. Either way the harness ignores the real per-day state.

### F2 — 05E radiation source is the PySnobal bridge, not native gridded shortwave (material)

- The harness drives melt from the PySnobal/SMRF export `net_solar`,
  reconstructed to incident via `net_solar / 0.80`
  (`snowbench_coe_melt.rs:20`, `:350-352`) with a flat underlying albedo `0.2`
  (`:23`), then the Brock albedo is re-applied.
- This is **not** openWEPP's native gridded shortwave (DAYMET/GRIDMET) that the
  strategy specified, and **not** the field ET consumes — so the radiation-source
  binding and the ET-coupling guardrail are not exercised; they are stood in by
  the export.
- The flat `/0.80` is self-consistent only if the export's `net_solar` was netted
  with a constant `0.20` albedo. If the export used a varying snow albedo, the
  constant inversion mis-scales incident shortwave. A round factor on a radiation
  cut-point is the dimensional-mismatch class that has produced false signals
  before; it should be proven like-for-like.

### F3 — Residual-risk register records the symptom, not the causes (the 05F-specific gap)

- `worker-handoff.md:32-36` notes "05E improved diagnostic legacy but did not
  beat H as-built," but nowhere in 05F (greped) are F1 or F2 acknowledged; the
  only "radiation source" mentions are Non-Scope deferrals
  (`package.md:39`, `:71`; `review.md:15`).
- The contract's Activation Evidence Baseline enshrines the `13->10 / 61->84`
  deltas as "necessary context" without the regime caveat that they were
  produced at `cancov = 0` on a PySnobal-bridge radiation source.
- A density-facing handoff that omits known defects from its risk register hands
  the next package a partial truth.

### F4 — Harness-fidelity propagates to the density rung (sequencing)

- `cancov = 0` lives in the shared snowbench harness, so SNOWDENSITY-06's rubric
  evidence will inherit the same unrepresentative regime unless the harness is
  fixed first.
- Combined with `OBL-SNOWFREEZE-P-031` ("do not retune melt for density"), this
  risks the melt/density conflation **in reverse**: density is evaluated on top
  of a melt model whose own validation is compromised, and is forbidden from
  touching it.

### F5 — Carryover verification debt (cheap, now closeable)

- The Brock-2000 albedo constants (`08_snow_albedo.rs:12-16`:
  `0.713`, `0.112`, `0.442`, `0.058`, `0.024 m`) were flagged unverified-from-
  paper in the 05-series research sweep (esp. `0.713`/`0.442`). `brock2000.pdf`
  is now in `references/copyrighted/`; the five constants should be confirmed
  against it (notably the `0.024 m` depth-transition scale vs Brock's ~0.5 cm
  w.e. switch). Guardrail: constants from cited authority.

## Recommended dispositions (for Codex)

1. **Amend the 05F handoff + contract addendum** to record F1 and F2 as explicit
   residual risks / caveats on the accepted boundary, and to mark the
   `13->10 / 61->84` deltas as regime-limited (produced at `cancov = 0`,
   PySnobal-bridge radiation).
2. **Make harness fidelity a SNOWDENSITY-06 entry-gate (item 1):** drive `cancov`
   from the real per-day growth state, and resolve the radiation provenance
   (native gridded shortwave, or prove the `/0.80` bridge like-for-like).
3. **Re-run the 05E adjudication at real `cancov`** and confirm `legacy_coe`
   reconciles toward the as-built `9/84`; only then is the
   `coe_shortwave_albedo_v1` acceptance scientifically earned.
4. **Close F5** by reading the five Brock constants from the in-repo paper.

None of these require reverting 05F's contract/interface freeze; they make the
frozen boundary's acceptance honest and keep the density rung's evidence
trustworthy.
