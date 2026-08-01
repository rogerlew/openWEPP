# Seasonal-Phase Diagnostic Protocol

Evidence mode: `Static + Reused Ran`.

## Phase Definitions

| Primary phase | Frozen observed-date boundary | Primary question |
| --- | --- | --- |
| Pre-peak / accumulation | Accepted observed snow-on date through the day before the observed peak anchor | Is snow mass absent before loss processes can explain it? |
| Peak anchor | Earliest accepted date attaining the observed seasonal maximum for the scored observable | Are peak dates separated by input, density timing, or early loss? |
| Post-peak / ablation | Day after the observed peak anchor through accepted observed persistent disappearance | Does a credible peak pack lose mass too rapidly? |
| Melt-out | Accepted persistent-disappearance operator and tie rule from the frozen observation rubric | Is disappearance early because peak mass was deficient or post-peak loss was excessive? |

The primary frame is computed once from observations by water year and
observable, then sealed before any candidate executes. Baseline and every
candidate are sampled on exactly those observed dates. All maximum ties are
reported and the earliest accepted maximum is the deterministic anchor. An
invalid or missing observed peak, inadequate coverage, or missing accepted
snow-on/disappearance boundary makes that phase inconclusive; a modeled date
may never replace it.

Dry-settling, wet-compaction, and model-peak labels are secondary diagnostics.
They may use candidate state, but cannot change dates entering primary efficacy
or promotion operators. Any transition-window width, dry/wet threshold, or
alternative phase operator must be authority-backed and prospectively sealed
by the successor before a result-bearing attempt; none is admitted by EB-04U.

## Cohort Operators

Density work reports KGE correlation, bias ratio, and variability ratio by
the frozen primary frame, plus signed density residuals. Geometry work first reconstructs
`rho_bulk = rho_water * SWE / depth` and requires layer aggregates to close
public SWE and depth. Under-persistence work separates cumulative input through
the observed peak from cumulative vapor and mass loss before and after that
peak. Stage 3 energy may reconstruct cold-content/vapor exchange only; it may
not be interpreted as the authoritative CoE melt-energy ledger.

No phase-conditioned association uniquely identifies a process. EB-04V/04W/04X
must expose the missing process-specific operands named in `operand-lineage.csv`
before selecting a correction.
