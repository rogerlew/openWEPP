# Claude Code Review Findings — HPHYS0285

Reviewer: Claude Code (independent review; implementation deferred to Codex)
Verdict: **APPROVE as a forward correction under HOLD** (matches the package's
own `executed-hold` status).
Evidence mode: static (diff + contract + baseline) + ran (cross-checks,
conservation / focused / snow-path tests).

## Summary

HPHYS0285 is the first package since the post-0281 re-baseline to move the
dominant `Total-Soil` residual the right way and the first to bring `Es` to
near-parity. The main correction is sound and contract-grounded. One finding —
an unbounded snowpack-exhaustion canonicalization that reverses HPHYS0284's
`A-HIGH-001` fail-closed guard — is weighted higher here than in the package's
own dual review and is recommended for closure in the next package.

## What I verified

Ran (executional, this review):
- Suite metrics match the on-disk report at
  `/tmp/hphys0285_full_release_final_20260604T201242Z/reports/hillslope_semantic_summary.md`
  (`Total-Soil` mean `71.751081` / max `350.397536`; `Ep` `0.759616`; `Es`
  `38/39`; `Dp` `0.043905`). The disposition's numbers are truthful.
- Focused `hphys0285_spring_soil_storage_retention_contract`: 3 passed.
- WB13 conservation-consistency
  (`hphys0203_wb13_soil_water_total_closure_is_conservation_consistent`): passed.
- SIMIMPL18 cold-day partition + multi-day storage mutation: 2 passed. The
  broadened all-liquid ingress does not break the snow-path mass guard.
- I did **not** independently rerun the full ~10 min `cargo test --workspace`;
  I ran the high-risk subset. The package gate-results claims the full workspace
  passed.

Static (read + reasoned):
- Core fix: same-pass WB18 layer ingress generalized from active-snowmelt-only
  (HPHYS0283 scope) to all local liquid (direct rain + routed melt + irrigation),
  gated by `management.initial.params.tillay2_m` rather than snow coupling, and
  applied per substep as `infiltration / lane_substeps` to match baseline hourly
  `xfin = fin / ui_LFtstpF` cadence.
- The localization evidence proves pre-fix direct rain did not reach layer
  storage (`soil_water=10` unchanged), so the broadened ingress fills a real gap
  rather than double-counting. This is corroborated by the conservation test and
  SIMIMPL18 passing and by `Total-Soil` improving rather than ballooning.

## Progress (verified)

| Symbol | HPHYS0284 | HPHYS0285 | Direction |
|---|---:|---:|---|
| Total-Soil mean abs | 89.531529 | 71.751081 | improved (−20%) |
| Ep mean abs | 1.145444 | 0.759616 | improved |
| Dp mean abs | 0.078495 | 0.043905 | improved |
| latqcc mean abs | 0.555122 | 0.476975 | improved |
| Es pass | (failing) | 38/39 | near-closed |
| Er pass | — | 39/39 | closed |
| Q / RM / Snow-Water | — | unchanged | correctly below partition |

Cumulative `Total-Soil` since the post-0281 re-baseline: 149 → 84 → 90 → **72**.

## Findings

### CLAUDE-0285-001 [MEDIUM–HIGH] — Unbounded pack-exhaustion canonicalization reverses HPHYS0284 A-HIGH-001

`crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
(~line 4030):

```rust
let runtime_swe_after_raw = runtime_swe + accumulation_water_m + total_rain_retained_m
    - melt_redistribution.snowpack_state_loss_m;
if !runtime_swe_after_raw.is_finite() { return Err(NonFiniteStateSymbol{..}); }
let runtime_swe_after = if runtime_swe_after_raw <= WB11_ZERO_THRESHOLD { 0.0 }
                        else { runtime_swe_after_raw };
```

- HPHYS0284 `A-HIGH-001` deliberately replaced a silent `.max(0.0)` with a
  fail-closed guard (`runtime_swe_after_raw < -WB11_ZERO_THRESHOLD` →
  `StateSymbolOutOfRange`) precisely to stop silent masking of material SWE
  overdraw. HPHYS0285 removes that guard: only `!is_finite()` now fails, and any
  finite negative — including an arbitrarily large unphysical overdraw —
  silently canonicalizes to zero.
- The triggering value (`snow.runtime_swe = -0.0026918754518707685` at H1
  `2015-106`, per `storage-localization-evidence.md`) is itself evidence of the
  corrected-melt over-depletion flagged in the HPHYS0284 review
  (depth→SWE translation, finding #1). HPHYS0285 masks that symptom rather than
  resolving it.
- Contract `SC-SNOWFREEZE-001#INV-SNOWFREEZE-019` (v20) blesses
  canonicalize-to-zero on pack exhaustion, but neither contract nor code
  **bounds** the overdraw that is canonicalized.
- Disposition note: this is not blocking the package (which is correctly HOLD,
  forward-progress only), but it should not propagate into further layers
  unaddressed. Reviewer B (`review_agent_b.md`) noted the canonicalization as "a
  regression-risk area" but accepted it non-blocking with no bound and no test
  vector; this review weights it higher.

Recommended remediation (next package, Codex to design):
- Cap the carried state-loss at available SWE so the loss is mass-conserving and
  the pack reaches exactly zero, OR canonicalize to zero only when the overdraw
  is within a physical tolerance of available SWE and fail-closed beyond it
  (preserving the HPHYS0284 A-HIGH-001 intent for anomalous overdraw).
- Add a red test proving a large synthetic overdraw fails closed rather than
  silently zeroing.

### CLAUDE-0285-002 [NOTE] — Depth→SWE translation still unresolved

The HPHYS0284 carried-state adjustment is a baseline snow-*depth* term
(`snodpt += ngtvML*1000/densgt`) applied in openWEPP as an *SWE* loss. The
−0.0027 m overdraw indicates the density-conversion equivalence is not exact.
A one-time snow-column mass trace (`P + accum = ΔSWE + routed + ablation`) on H1
spring would close it; the next package will be in this window anyway.

### CLAUDE-0285-003 [POSITIVE] — Scope narrowing handled correctly

Reviewer A's blocking finding (contract claimed carry/runon ingress the code did
not implement; broadening WB12 to runon broke erosion vectors) was resolved by
narrowing the contract scope and deferring carry/runon to a follow-up rather than
forcing an under-tested broadening. This is the right call and avoids
overclaiming.

## Continuation read

The residual has split cleanly:
- H7/H39 still too dry at spring meltout (`Total-Soil` ~278–289 mm vs baseline
  ~580–612 mm near Julian 145–146).
- H1 now too **wet** late-2015 (`Total-Soil` ~340 mm vs baseline ~35–38 mm near
  Julian 222–225).

The late-season wetness points at ET/drainage withdrawal from the now-corrected
storage — i.e., the next package (layer-capacity/retention + WB18/WB17 coupling,
as the disposition recommends) reopens the WB17 `Ep` thread on trustworthy
storage. The bounded-canonicalization fix (CLAUDE-0285-001) should be folded into
that package before more layers build on the snowpack state.
