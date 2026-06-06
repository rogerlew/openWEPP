# WBVAL01 Rocky Mountain Daily-Climate Single-OFE Water-Balance Closure Validation

Status: executed-hold

## Objective

Establish an empirical **water-balance conservation-closure** baseline for
openWEPP on a real CLIGEN daily (non-breakpoint) Rocky Mountain run, and produce
a per-hillslope closure ledger that names concrete single-OFE WB closure targets
for the next roadmap rung (frost). This is a **validation/characterization**
package: it measures where openWEPP stands and classifies conservation, it does
not implement physics corrections.

Execution summary (`2026-06-06`): WBVAL01 ran the release
`openwepp-cli-hill` path against generated TOML wrappers for all `22`
single-OFE hillslopes discovered in the run directory. `12` hillslopes emitted
complete WAT ledgers and all `12` are `conservation-break` for years `2..6`
against the package tolerance. `10` single-OFE hillslopes failed closed before
WAT publication (`6` climate hourly-radiation domain guards, `4` WB11
percolation domain guards). The package remains `executed-hold` because the
acceptance ledger cannot truthfully cover every single-OFE hillslope and year 1
has no pre-day-1 initial storage row in the published WAT surface.

Run under validation: `/wc1/runs/in/indispensable-presenter` — DRIGGS ID
(Teton valley, Rocky Mountains, elev. `1859 m`), CLIGEN v`5.323` daily climate,
`23` hillslopes (`wepp/runs/p10..pNN`), watershed structure present (`chan.inp`).

## Rationale

After HPHYS0298->0320 (22 packages, one real forcing fix at the very end, all on
the synthetic `unpalatable-rind` H1..H39 lane), the program needs an independent,
real-world conservation signal before committing to frost work. This run is the
right grounding surface for three reasons:

1. **Independent forcing path.** It is **non-breakpoint** (CLIGEN daily,
   `ibrkpt=0`). The entire 0298->0320 arc was breakpoint/hourly SIMIMPL28
   forcing. Daily disaggregation exercises a different forcing lane and does not
   inherit the breakpoint-specific assumptions the synthetic lane stressed.
2. **Exercises the HPHYS0320 fix on real data.** The non-breakpoint random
   start-hour path feeds the just-landed `wnttim < 1.0 -> 1.0` normalization
   (`INV-CLIMATE-018`), so this run validates that fix outside its source key.
3. **Single-OFE, snow-bearing, real.** Sampled hillslopes are single-OFE
   (`.slp` OFE count `= 1`), and the cold high-elevation site keeps the winter
   path active — so it is squarely **rung-1 territory** (single-OFE vertical
   conservation) with live winter forcing, on real inputs rather than a fixture.

Per ADR-0017, fixed-baseline comparator agreement is an investigation flag, not
a target. The acceptance authority here is **conservation closure**, not
comparator match and not snow magnitude.

## Roadmap position

This is rung-1 of the agreed sequence `single -> frost -> MOFE -> snow`
(`docs/work-packages/README.md` "Current roadmap"). On completion, the
`worker-handoff.md` MUST name **frost** as the next rung (item 1) and MUST keep
the HPHYS0298->0320 snow/`RM` comparator route suspended behind
`docs/backlog/20260605-snow-code-deferred-science-review.md`. Do not reopen the
snow route from this package.

## Included Scope

- Enumerate all `23` hillslope prefixes under
  `/wc1/runs/in/indispensable-presenter/wepp/runs/` and confirm per-hillslope OFE
  count from each `.slp` (record single-OFE vs multi-OFE; multi-OFE hillslopes
  are observed-only here and routed to rung-3/MOFE).
- Run `openwepp-cli-hill` (`crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`,
  precedent: `tests/integration/cli01_runner_hillslope_integration.rs`) on each
  single-OFE hillslope using its WEPP-format inputs (`pNN.run/.cli/.slp/.sol/.man`).
- Compute a per-hillslope, per-year **conservation residual**:
  `R = Σinputs - Σoutputs - ΔStorage`, where
  - inputs = precipitation (+ irrigation if present),
  - outputs = runoff (`Q`) + ET (`Ep`+`Es`+`Er`) + deep percolation (`Dp`) +
    lateral (`latqcc`),
  - ΔStorage = Δ(soil water, `Total-Soil`/`SoilWaterTotal`) + Δ(snow,
    `Snow-Water`).
  Declare the term set and units explicitly; fail closed on any missing term
  rather than imputing zero.
- Classify each single-OFE hillslope against a named closure tolerance:
  `conservation-clean` (|R| within tolerance every year) vs
  `conservation-break` (|R| exceeds tolerance) and, for breaks, the first year
  and dominant non-closing term.
- Produce the closure ledger and a prioritized list of `conservation-break`
  hillslopes as the concrete rung-2/frost entry targets.
- Optionally run `openwepp-cli-watershed` for routing **observation only**
  (clearly labeled non-acceptance), to preview rung-3 surfaces.

## Excluded Scope

- No production Rust kernel or forcing edits. This package characterizes; it does
  not correct.
- No comparator-match acceptance gate; any fixed-baseline comparison is
  observe-only under ADR-0017.
- No snow-magnitude adjudication — that is the suspended route behind the backlog
  science review, judged at rung-4.
- No frost (`ksflag`/`ksatadj`) work — that is rung-2 and begins only after this
  package names its targets.
- No MOFE routing closure work beyond observe-only watershed preview.
- No empirical compensation in any WB13/WB14/WB17/WB18/WB19 term to force closure.

## Deliverables

- `artifacts/single-ofe-closure-ledger.md` — per-hillslope, per-year conservation
  residual `R`, term breakdown, units, and `conservation-clean` /
  `conservation-break` classification with the named tolerance.
- `artifacts/run-manifest.md` — the `23` hillslope inventory with OFE counts,
  exact `openwepp-cli-hill` invocations, binary build hash, and input paths;
  evidence mode (**Ran**) for each invocation.
- `artifacts/rung2-frost-target-handoff.md` — prioritized `conservation-break`
  targets for frost, plus the explicit roadmap continuation (frost next; snow
  suspended).
- `artifacts/watershed-routing-observation.md` — *(optional)* observe-only
  watershed routing preview, labeled non-acceptance.
- Standard gate, dual review (`review_agent_a.md`/`review_agent_b.md`), dual
  verification, disposition, and `worker-handoff.md` artifacts per the
  work-package convention.

## Acceptance / Exit Criteria

- A complete `single-ofe-closure-ledger.md` covering every single-OFE hillslope
  in the run, with the conservation residual computed from explicitly declared,
  fully-populated terms (no imputed-zero terms).
- Each single-OFE hillslope classified `conservation-clean` or
  `conservation-break` against a named, justified tolerance.
- `rung2-frost-target-handoff.md` names the frost entry targets and conveys the
  roadmap continuation (frost next; snow route suspended).
- Evidence mode is truthful per artifact (**Ran** for actual `openwepp-cli-hill`
  invocations; **Static** for any reasoned classification).
- Dual review findings dispositioned; no undispositioned findings at closure.

This is a characterization package: a population of `conservation-break`
hillslopes is a **valid and expected** outcome — it is the rung-2 input, not a
package failure. Package failure is an incomplete/untruthful ledger, not the
discovery of non-closure.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/work-packages/README.md` (Current roadmap)
- `/workdir/openWEPP/docs/backlog/20260605-snow-code-deferred-science-review.md`
- Run inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`
