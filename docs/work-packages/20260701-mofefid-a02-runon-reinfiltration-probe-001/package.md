# MOFEFID-A02 — Runon Re-Infiltration Probe

Status: **EXECUTED — VERDICT: CONFIRMED (material)** (2026-07-01). Probe moved `runvol_pct_precip` 72.33% → 62.23% (−10.1 pp, 2× the CONFIRMED threshold, lower-bound probe); ET +34%, outlet latqcc +133%, Dp flat; default path byte-identical; closure green. See `artifacts/verdict.md`.
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) §7
(operator-directed). Owner: Claude Code. Worktree: `mofefid-a02-probe`.

## Hypothesis under test

A01's F-A2 finding (runon excluded from the WB14 infiltration supply,
opposite to the pinned-baseline `fin`/`xfin` semantics at
`wepp-forest_260430_baseline/src/watbal_hourly.for:361-363`, `:471-473`)
**explains a material share of the FARPOINT01 magnitude gap** (openWEPP
H2637 `runvol_pct_precip = 71.0036550031206` vs legacy 55.5%). Mechanism:
the baseline re-infiltrates upstream carry — surface *and lateral* — into
downslope soil hourly; openWEPP routes it to the surface partition, so
water legacy re-soaks becomes exported runoff.

## Probe design

Env-gated opt-in (`OPENWEPP_MOFEFID_A02_RUNON_INFILTRATION=1`), default
no-op. At R4K (after R4J has resolved the day's area-scaled runon), the
probe adds `runon_input_m + subsurface_carry_m` into the WB14 producer
hyetograph as uniform added intensity (the same distribution the builder
uses for routed melt). The R4A partition subtracts the enlarged cumulative
infiltration from the same liquid+runon supply, so all conservation
identities and closure guards remain enforced. Sites:
`direct_runtime/runoff.rs` (`apply_mofefid_a02_runon_infiltration_probe`,
`mofefid_a02_augment_hyetograph_with_uniform_depth`).

**Known approximations (probe, not production semantics):**
1. Daily-lump augmentation over the existing hyetograph, not the
   baseline's hourly `xfin` distribution.
2. Dry-runon days (no positive-duration hyetograph) are skipped — Green–
   Ampt has no time base — so the measured effect is a **lower bound**.
3. The melt-only same-pass reconstruction retains max semantics (it can
   only raise infiltration), so no interference.

## Acceptance gates

1. Probe unit tests green; workspace suite green; fmt/clippy clean.
2. **Default path byte-identical** on H2637 (all five protected outputs)
   with the env unset.
3. Probe run: exit 0, all closure guards green (the guards stay on — a
   closure failure is a probe-design defect, not a finding).
4. Metric: `runvol_pct_precip` (canonical volume recipe,
   `QOFE(outlet) × Area(outlet)` vs `P × A_total`) — baseline must
   reproduce `71.0036550031206` as the calculation sanity check.

## Verdict rubric (declared before results)

- **CONFIRMED (material):** probe moves `runvol_pct_precip` by ≥ 5
  percentage points toward legacy's 55.5% — the seam is a first-order
  driver; F-A2's contract decision escalates ahead of Lane D.
- **PARTIAL:** movement 1–5 pp — contributing mechanism; record magnitude,
  keep Lane D disposition with elevated priority.
- **REFUTED (immaterial):** movement < 1 pp — hypothesis dies; F-A2 stays
  a Lane D contract item on fidelity grounds alone.

The probe is diagnostic-only: no default-path change, no contract
amendment, no production adoption regardless of outcome.
