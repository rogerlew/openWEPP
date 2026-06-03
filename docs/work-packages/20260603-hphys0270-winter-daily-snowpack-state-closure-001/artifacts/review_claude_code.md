# Review: Claude Code

Status: complete

Evidence mode: static (package/contract read) + executional (reviewer-run bisection)

Static:

- Reviewer: Claude Code (`claude-opus-4-8`), invoked by user post-execution.
- Scope: HPHYS0270 winter daily snowpack state closure (observability-only trace
  schema v9) plus a reviewer-run bisection of the H1 Snow-Water divergence.
- Continuity: extends the snow arc (HPHYS0268 spring wiring, HPHYS0269
  negative-melt authority) and the 2026-06-03 frost/snow audit.

Executional (Ran by reviewer):

- Bisected H1 candidate vs baseline `Snow-Water` by `sim_day_index` from the
  on-disk run root `/tmp/hphys0270_full_20260603T201051Z`:
  candidate `hillslope_output/H1.wat.parquet` vs baseline
  `/tmp/unpalatable_parity_20260529T192707Z/.../baseline_H1.parquet`.
- Pulled H1 candidate snow trace detail for sim-days 34-37 from
  `H1.hphys0270.trace.jsonl`.

## Assessment of HPHYS0270 (the package)

1. Observability-only and honest: trace schema v9 adds pre/post-day snowpack
   carry state; full-suite metrics are byte-identical to HPHYS0269 (`Snow-Water`
   mean abs `56.63`), correctly labeled as no-physics-change. `HOLD`.
2. The re-localization is directionally right but its framing is slightly off.
   The disposition concludes "candidate day-begin SWE is already ~140-157 mm
   below baseline" and attributes the residual to "seasonal accumulation/
   carry-state lineage." The reviewer bisection shows the mechanism is not
   gradual accumulation — see below.

## Bisection finding (reviewer-run; the actionable result)

- Sim-days 1-35: candidate `Snow-Water` tracks baseline to within ~0.1-0.7 mm;
  both accumulate to ~79 mm by day 35. Accumulation is **not** the defect.
- **Sim-day 36 (julian 36, early February): a discrete spurious melt event.**
  Candidate trace: `swe_before=78.4 mm`, `hourly_melt_raw_sum=54.0 mm`,
  redistributed `melt_sum=27.1 mm`, `swe_after=52.5 mm`, `RM=28.18 mm`. Baseline
  the same day: `RM=0.00`, `Snow-Water` rises 79.1 -> 81.4 mm (no melt). openWEPP
  released ~28 mm where baseline released none.
- The ~29 mm gap never recovers and additional events compound it: first-winter
  (2013) peak SWE is candidate **105 mm** vs baseline **217 mm**.

Interpretation: the early-melt / low-spring-SWE / low-storage / low-`Ep` cascade
traces to openWEPP's **hourly melt energy balance firing when baseline computes
no melt**. Day 36 raw melt is strongly positive (+54 mm), so this is:

- **not** a negative-melt/cold-content case (HPHYS0269's authority decision is
  confirmed correct but **not load-bearing** for this divergence — there is no
  refreeze offset to mishandle on a +54 mm raw-melt day; this is the empirical
  confirmation of the firing concern raised in the HPHYS0269 review);
- **not** an accumulation-rate or daily-settlement defect (both track baseline
  through day 35);
- **the melt magnitude/trigger itself** — the `melt.for` energy-balance terms
  (`amelt/bmelt/cmelt/dmelt`, `winter.for`/`snowd.for` warm-branch entry) or
  their hourly forcing inputs (hourly air temperature in degF, radiation,
  dewpoint, wind) produce ~54 mm melt on an early-Feb day where baseline yields
  zero.

## Findings

1. High — the residual is a spurious hourly-melt computation, not accumulation.
   Re-anchor the continuation diagnosis from "seasonal accumulation/carry-state"
   to the melt energy-balance and its hourly forcing. The decisive next trace is
   H1 sim-day 36: dump the `melt.for` term-by-term values
   (`amelt/bmelt/cmelt/dmelt`) and the hourly temperature/radiation/dewpoint/wind
   the candidate fed them, against the baseline for the same day. A 54 mm
   single-day melt from a 78 mm pack in early February points at a mis-scaled
   term or a wrong hourly forcing (e.g., hourly temperature disaggregation or a
   unit error), not snowpack state.

2. Medium — HPHYS0269's negative-melt authority work is confirmed off the
   critical path for this cohort. The decision to prefer the `03fee455` fix is
   still correct governance, but day 36 shows the parity-relevant defect is melt
   magnitude/trigger, so no further negative-melt redistribution effort should be
   spent until a genuine mixed thaw+refreeze (`class3`) day is shown to drive a
   residual.

3. Low — confirm the carried-forward SIMIMPL18 ET-guard workspace test. HPHYS0269
   reported `cargo test --workspace` red on it; HPHYS0270's disposition does not
   report workspace-test status. Resolve or explicitly track it.

## Notes for disposition owner (Codex)

- The package itself is a clean, honest observability slice; the issue is only
  the continuation framing (accumulation vs melt-trigger). Finding 1 should
  redirect HPHYS0271 to the melt energy balance and hourly forcing on H1 day 36.
- The bisection is reviewer-executed against the HPHYS0270 run root; it is
  reproducible from the parquet pair cited above.
- Disposition `HOLD` at `0/39` is consistent with this review; no overclaim
  observed.
