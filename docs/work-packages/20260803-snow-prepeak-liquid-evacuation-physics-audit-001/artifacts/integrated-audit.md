# Integrated Audit

Status: `executed / reviewed / verified / HOLD-EVIDENCE`

Evidence mode: `Static + Ran`

## Outcome First

Snowbird's modeled pre-peak loss is generated upstream of Stage 3. In the fresh
same-binary baseline, the uncensored Snowbird median contains `0.7690 m` of
snowfall accumulation, `0.5296 m` of pack loss, and `0.5379 m` of gross-positive
hourly CoE melt. The active positive-parts/capacity path exports that generated
liquid, while downstream Stage-3 cold content and refreeze are snow-neutral.

This interaction has the correct sign, timing, and order-one magnitude. The
audit does not establish which physical correction is right. Temperature,
dewpoint, and wind forcing bias; empirical `B/C` transferability; signed-hour
thermal interpretation; and the modern export boundary remain competing
owners. The predecessor's tested `rst` family was insufficient, but broader
phase and precipitation-forcing error remain unresolved rather than excluded.

## Accepted Evidence Population

Review rejected two earlier result surfaces. V1 had an incompletely frozen
event operator and included right-censored WY2025 in primary summaries. V2
rebuilt the release binary but compared its cells with older predecessor
reference traces. V3 prospectively froze the complete analysis contract and
ran a fresh same-binary baseline plus three operators: 16 cells total.

Primary results exclude WY2025 and contain 154 site-years. The 158-window
all-year surface is retained only as a right-censor sensitivity. Independent
mass reconstruction over the primary windows closes within `8.84e-13 m` at
the endpoints; the all-row daily maximum is `1.00e-12 m`. The routed-liquid
alias closes within `1.56e-17 m`.

Snowbird's primary 35-window medians are:

| Quantity | Median |
|---|---:|
| Snowfall accumulation | `0.7690 m` |
| Pack SWE loss | `0.5296 m` |
| Gross-positive hourly CoE melt | `0.5379 m` |
| Negative hourly CoE melt | `-0.1243 m` |
| Stage-3 refreeze | `0.00112 m` |
| Modeled/observed peak-SWE ratio | `0.382` |
| Modeled peak timing | `47 d` early |

The signed raw-term scale is heterogeneous. Snowbird medians are `A=0.0883`,
`B=0.1473`, `C=0.1990`, and `D=0.00443 m`; `B+C` are the largest signed scale,
but those sums are not causal shares of gross-positive melt. Niwot is more
shortwave-weighted. Snowbird loses 79.7% of its primary-window mass on days
without snowfall or rain, so rain-on-snow is not the systemic dominant cause.

The largest retained Snowbird event, 2011-05-02 through 2011-05-22, loses
`0.3730 m`, 46.8% of that year's loss. It contains `0.3666 m` gross-positive
CoE melt and only `0.000194 m` Stage-3 refreeze. At the primary threshold, all
eligible Snowbird events capture 99.7% of loss; the stored top three per year
capture 56.2%, so event truncation is not hidden.

## Same-Binary Causal Bounds

- Disabling Stage 3 changes median Snowbird loss, peak SWE, and peak ratio by
  exactly zero; maximum loss differences are roundoff (`3.33e-16 m`). Median
  refreeze falls by `0.00112 m`.
- Enabling explicit longwave changes authoritative mass and peak metrics by
  exactly zero while increasing Snowbird median diagnostic refreeze by
  `0.01287 m`. Cooling is visible but cannot delay melt in the current
  snow-neutral architecture.
- Legacy CoE routing reduces Snowbird median loss by `0.4032 m`, increases peak
  SWE by `0.2786 m`, raises peak ratio by `0.2893`, and moves peak timing 31 days
  later. Comparable order-one responses occur at every site. This is a rollback
  bound on the coupled modern export/state trajectory, not proof that the
  legacy density gate is correct or an isolated capacity effect.

## Reachability And Authority Gaps

The accepted reference traces contain 6,716 primary-window days with both
positive and negative hourly applied CoE terms, including 1,510 at Snowbird.
They contain 1,031 mixed-sign days with positive routed liquid and positive
Stage-3 refreeze, including 298 at Snowbird. That invalidates the empirical
reachability premise in INV-SNOWFREEZE-015 and activates its own requirement
for re-adjudication. It does not prove that negative empirical melt should be
added back to SWE or interpreted directly as refreeze energy.

Physical-density wet compaction receives `pack_loss + routed_melt`; the trace
reconstructs this as `2*pack_loss + rain_released` within `2.78e-17 m`. This is
a confirmed duplicate data-flow alias. The active multilayer wet-compaction
authority does not establish that `routed_melt` alone is the correct complete
driver, so the physical-defect verdict remains `UNRESOLVED`. The separate CoE
boundary excludes this alias as the direct source of authoritative SWE loss.

The bounded Stage-3 energy identity independently closes within
`1.87e-8 J m^-2`. Exact Stage-3 liquid closure still cannot be reconstructed:
the runtime diagnostic owns incoming, routed, retained, refrozen, and residual
liquid operands, but the real JSONL consumer omits four of them. Internal guards
are not an independent downstream closure.

## Disposition And Next Action

The package closes `HOLD-EVIDENCE`, not `complete`. The smallest justified next
package is behavior-neutral and contract-first:

1. publish all Stage-3 liquid operands through the real JSONL consumer;
2. publish the hourly forcing, branch, cold-content, retained-liquid, and
   pre/post state operands needed to adjudicate mixed signed hours;
3. establish active multilayer wet-compaction operand authority before calling
   the duplicate alias a physical defect; and
4. rerun a no-tuning event cohort to separate forcing bias, empirical `B/C`
   structure, signed-hour thermal handling, and modern export behavior.

Stateful shortwave/cloud effects remain unresolved and rank below those paths;
the retained horizontal-daily ERA diagnostics do not substitute for a
slope-aware hourly replay.
