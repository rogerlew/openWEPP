# Post-Closure Independent Review: Target Feasibility

Status: `post-closure independent review / findings only / no disposition`

Evidence mode: `Ran` plus `Static`

Author: Claude Code. Review date `2026-08-04`, against `b44e75c1`.

## Standing Of This Artifact

This review was authored **after** the package closed at `b44e75c1`. It was not
part of the gated evidence set, did not pass the package's dual-review or
dual-verification gates, and does not alter the package's `disposition.md`,
`integrated-adjudication.md`, or `review-disposition.md`. Its presence changes
the artifact directory contents recorded by the closing write-set
reconciliation; that reconciliation should be read as exact for the closing
commit only.

Nothing here disputes an executed number in this package. Every closure,
identity, alias separation, and exclusion the package reports is accepted as
reported. The findings concern the **candidate set** the verdict ranked within
and the **feasibility of the observational target** the campaign scores against.

Findings are surfaced with evidence and falsifiers. Correction architecture,
successor package design, contract authority, and gate structure are left to
Codex.

## Evidence Base

`Ran`: Python analysis over three checked-in surfaces, no simulation executed.

| Input | sha256 |
|---|---|
| `artifacts/cross-fixture-results.json` | `97b326fdc2851810d17a09345b99733bcb7c7371799b5fa28d91efdb8fdda752` |
| `tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli` | `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7` |
| `tests/fixtures/snotel_observed/observations/sites/snotel_snowbird_ut.csv` | `7fffbfb6e060635c0c5d46835c108022e0386baa94d96e7dabb667b932691992` |

`Static`: read `runoff_reconciliation.rs`, the predecessor arc `EB-04T` through
`SNOW-PREPEAK-LIQUID-EVACUATION`, and
`tests/fixtures/snotel_observed/README.md`.

No openWEPP build, replay, or test execution was performed. Windows are the
package's own `window_start` through `observed_peak_date`, per site-year, with
`right_censored` rows excluded. All ratios are computed per water year before
any median.

## Result Table

| Site | n | fixture/gauge | gauge/pillow | fixture/pillow | achieved peak ratio | attained fraction of ceiling |
|---|---:|---:|---:|---:|---:|---:|
| Mica Creek | 34 | `0.966` | `1.562` | `1.539` | `0.702` | `0.702` |
| Niwot | 44 | `0.963` | `1.237` | `1.198` | `0.504` | `0.504` |
| Paradise | 41 | `0.906` | `1.331` | `1.105` | `0.518` | `0.518` |
| Snowbird | 35 | `0.847` | `0.915` | `0.823` | `0.382` | `0.464` |

`gauge/pillow` uses years with AWDB `PREC` coverage over the window: `33`, `43`,
`31`, and `29` respectively. Ceiling is `min(1.0, fixture/pillow)`.

## Findings

### F-1 The Snowbird observation pair is internally inconsistent

Severity: `high`. Confidence: `high`.

At `766:UT:SNTL` the station's own accumulated precipitation gauge delivers a
median `0.915` of the water its own snow pillow reports as peak SWE over the
package's windows. The gauge total is below the pillow peak in `20` of `29`
covered years; `p10` is `0.833` and `p90` is `1.103`.

A snowpack cannot retain more water than fell on it, and a real pack loses a
nonzero fraction to sublimation and midwinter melt before peak. Gauge and pillow
therefore disagree at this station by roughly `20` to `40` percent. The other
three stations sit at `1.237` to `1.562`, which is physically admissible and
leaves `16.5` to `35.0` percent headroom for pre-peak loss.

Consequence: the maximum peak ratio any correct snow model can attain at
Snowbird is `0.823` under the current fixture and about `0.915` under
gauge-matched forcing. A modeled-versus-observed peak-magnitude score at
Snowbird cannot reach `1.0`, so an "insufficient recovery" verdict at that site
carries no information about the mechanism under test.

Falsifiers: AWDB `PREC` at this station is not a water-year cumulative gauge
total; the window extraction misreads the reset boundary; or a documented
station convention (gauge/pillow separation, catchment scaling, shielding
correction already applied to `WTEQ`) makes the ratio legitimate.

### F-2 The all-phase mass ceiling was never computed in the arc

Severity: `high`. Confidence: `high`.

`EB-04W1`'s `effective_input_ratio` is defined post-phase-partition and
post-retention as `initial + snowfall_swe + rain_retained`
(`tools/run_precipitation_scaling.py:499`, `artifacts/operand-lineage.md:14`).
It is structurally blind to precipitation that arrived and departed as rain, so
it cannot bound what a *different* phase or retention treatment could have
achieved. `EB-04W` computed the post-partition ceiling at Paradise only
(`artifacts/scientific-synthesis.md:21`) and bounded it away from a forcing
claim.

The all-phase test — total `.cli` precipitation over the accumulation window
against observed peak SWE — appears nowhere in the arc. Its values are in the
result table above. At Snowbird total precipitation of all phases is below the
observed peak in `34` of `35` primary years.

The extraction cross-validates: the `fixture/gauge` ratio computed here for
Snowbird, `0.847`, reproduces the `0.848` independently computed by
`20260803-snowbird-snotel-climate-forcing-diagnostic-001`.

Falsifier: an operand in the schema-v4 trace shows precipitation reaching the
column that the raw `.cli` daily `prcp` field does not carry.

### F-3 The `2.0` multiplier partly buys down an infeasible target

Severity: `high`. Confidence: `medium-high`.

`20260803-snowbird-snotel-climate-forcing-diagnostic-001` measured a fixture dry
bias of `0.848` against the gauge and recorded the unresolved tension that
`EB-04W2` required `2.0` times precipitation for Snowbird peak parity. The
chain `gauge/pillow 0.915` × `fixture/gauge 0.847` = `fixture/pillow 0.823`
closes against the directly computed `0.823` and supplies the missing factor.
The multiplier decomposes as `2.0 ~= 1.22 x 1.65`, where `1.22 = 1/0.823` lifts
the fixture only to the zero-loss ceiling and the residual `1.65` compensates
for modeled pre-peak loss. About a fifth of the multiplier is therefore buying
down target infeasibility rather than correcting model physics.

This does not exonerate the melt path. Decomposing the Snowbird peak-ratio gap:
about `0.18` is forcing bias plus observation-pair inconsistency, and about
`0.44` is modeled pre-peak loss. Modeled loss remains the single largest share.
What the finding invalidates is the *scoring*, not the ranking of melt as a
contributor.

Falsifier: an independent gauge-versus-pillow reconciliation at this station
returning a ratio at or above unity.

### F-4 Feasibility normalization removes Snowbird's outlier status

Severity: `medium`. Confidence: `medium-high`.

Normalized by achievable ceiling, attainment is `0.702` at Mica Creek, `0.504`
at Niwot, `0.518` at Paradise, and `0.464` at Snowbird. Snowbird is not a
distinct failure mode requiring its own explanation; it is the same failure seen
through an unreachable target. Mica Creek is the only site whose forcing matches
its own gauge closely (`0.966`) while retaining substantial loss headroom
(`35.0` percent), and it also shows the best attainment.

The campaign's site-selection and anchoring assumptions predate this
normalization.

Falsifier: F-1 falsified, which removes the Snowbird ceiling.

### F-5 The verdict's affirmative evidence is near-tautological

Severity: `medium`. Confidence: `high`.

The load-bearing positive fact is `gross_positive_to_solid_loss_ratio` at
`0.9995` to `1.0176`. Applied CoE melt is the mechanism that *produces* solid
pack loss on this code path, so a ratio near unity is a closure identity between
a mechanism and the state it mutates. `integrated-adjudication.md:42` states
this directly ("adjacent-ledger localization, not independent proof").

Net of that identity, `UPSTREAM_GENERATION_PRIORITY` rests on elimination. The
eliminations are sound and hard-won — in particular the static real-consumer
proof that Stage-3 routed, retained, and refrozen outcomes are bypassed by
hydrologic publication and never restore runtime SWE is an architectural result
that holds regardless of magnitude. The concern is that elimination over a set
that excluded forcing and the observation operator by scope yields a verdict
whose confidence exceeds its information content.

Falsifier: an operand that separates CoE melt magnitude from pack loss without
the closure identity, or a demonstration that the ratio could have materially
departed from unity under a defect.

### F-6 The forcing branch closed on a budget stop, then ranking proceeded

Severity: `medium`. Confidence: `high`.

`EB-04W2` disposed `FORCING_BRANCH_CLOSED` and directed "Close the forcing
branch and proceed to EB-04X". Its own review disposition records that no `W3`
follows because `2.0` was a prospectively frozen experiment-budget "stop, not
because physical saturation was proven"
(`20260802-...-04w2-.../artifacts/review-disposition.md:21`), and its verifier
independently records the `2.0` ceiling as "consistently described as an
experiment-budget stop rather than a physical upper bound"
(`.../verification-agent-a.md:87-88`). Both `W1` and `W2` carry
`identifiability_status = PARTIALLY_IDENTIFIABLE` with forcing confounded
against phase, representativeness, retention, and modeled loss.

Successor packages then ranked within the remaining candidate set. A branch
closed for budget is not a branch excluded on evidence, and the distinction is
not visible in the current adjudication's candidate framing.

### F-7 Chronology co-closes with magnitude under one input scalar

Severity: `medium-high`. Confidence: `medium-high`.

From `20260802-...-04w2-.../artifacts/precipitation-grid-extension-summary.csv`,
`primary_peak_ratio` and `chronology_abs_error_days` against the multiplier:

| Site | `1.0` | selected multiplier | `2.0` |
|---|---|---|---|
| Mica Creek | `0.619` / `35.0 d` | `1.4` `TRADEOFF_BRACKETED`: `0.968` / `21.0 d` | `1.529` / `8.0 d` |
| Niwot | `0.495` / `46.5 d` | `1.7` `TRADEOFF_BRACKETED`: `1.029` / `13.5 d` | `1.270` / `9.5 d` |
| Paradise | `0.473` / `37.0 d` | `1.8` `BRACKETED_CANDIDATE`: `0.989` / `0.0 d` | `1.125` / `0.0 d` |
| Snowbird | `0.390` / `44.5 d` | `2.0` `EXPERIMENT_BUDGET_BOUNDARY` | `0.977` / `23.0 d` |

At Paradise a single scalar precipitation multiplier drives peak ratio to
`0.989` and peak-date offset to exactly `0.0` days simultaneously. Both error
modes vanish together under one parameter that touches only input mass. Mica
Creek and Niwot show the same joint collapse less completely.

A melt-rate or melt-coefficient defect would not generally be repaired by a pure
input scalar; a mass-input deficit would produce exactly this signature, because
too little snow both lowers the peak and exhausts the pack early. This is
evidence bearing against upstream melt-generation dominance at those three
sites, it lives in the corpus already, and it is not represented in this
package's candidate framing.

Snowbird is the only site that fails to co-close, remaining `23` days early at
correct magnitude. That non-closure is confounded by F-1: reaching magnitude
parity at Snowbird requires roughly twice the site's actual precipitation, which
independently distorts accumulation chronology because any given SWE is reached
far too early. The residual there is therefore not clean evidence of a melt
chronology defect either.

This does not make the multiplier physical. `EB-04W1` and `EB-04W2` correctly
refused that, and the finding concerns the *shape* of the response rather than
the value of any multiplier.

Two chronology-relevant forcing signals from
`20260803-snowbird-snotel-climate-forcing-diagnostic-001` remain unpropagated
into any snow response: a wet-winter `Tmax` bias of `+0.617 C` (`+1.055 C` snow
season, `+1.206 C` annual, against a `-0.375 C` `Tmin` bias), and a truncated
event tail with `p99` at `37.04` mm fixture against `50.88` mm SNOTEL, with
`1120` against `1298` wet days. Source keys in that package's
`artifacts/comparison-results.json` are
`results/{all,snow_season,wet_winter}/tmax_c/fixture_minus_snotel_bias`,
`results/wet_winter/tmin_c/fixture_minus_snotel_bias`, and
`results/wet_winter/precipitation/{positive_event_quantiles_mm,fixture_wet_days,snotel_wet_days}`.

Falsifier: a melt-side scalar that produces the same joint magnitude and
chronology closure at Paradise, which would remove the discriminating power of
the input-scalar result.

### F-8 A confirmed production duplicate alias remains open

Severity: `low-medium`. Confidence: `high`.

`runoff_reconciliation.rs:2309` passes
`liquid_for_compaction_m: snow_coupling.snowpack_state_loss + routed_melt_m`,
while `resolve_snow_partition_terms` (`:234-273`) builds `routed_melt_m` as
`signed_s + accumulation + rain_retained + rain_released`, which under the
package's own storage identity equals state loss plus released rain. The wet
compaction forcing therefore receives `2 * state_loss + rain_released`. The
predecessor audit confirmed the alias and quantified `73.123` m of duplicated
state-loss over primary windows
(`20260803-snow-prepeak-liquid-evacuation-physics-audit-001/artifacts/mechanism-matrix.md:18`),
disposing the alias `SUPPORTED` with physical-defect verdict `UNRESOLVED`
pending active multilayer wet-compaction operand authority.

It has been open in production since `2026-08-03` and this package lists it as
unresolved (`integrated-adjudication.md:88`). Layer geometry produced under the
duplicated forcing feeds Stage-3 depth and cold content, which the recommended
successor intends to reason about. The materiality question and the operand
authority question are separable; only the second is currently gating.

## Cross-Cutting Gap: The Observation Operator Was Never Audited

ADR-0017 external-authority discipline correctly treats SNOTEL as authority
rather than as a tunable. No package in the arc has verified that the two
instruments at a single SNOTEL station are mutually consistent, which is a
different question from whether the source is authoritative. The data required
is already checked in under
`tests/fixtures/snotel_observed/observations/sites/`, and
`tests/fixtures/snotel_observed/README.md` already warns that these are
high-relief sites where the CLIGEN station sits well below the alpine hillslope
and directs the reader to "verify the winter temperature/precip lapse there
before trusting magnitudes."

## Open Questions For Disposition

These are not recommendations. They are the questions the findings leave open.

1. What target does a magnitude verdict score against when a station's gauge and
   pillow disagree, and does magnitude authority survive at such a site at all?
2. Do existing `EB-04` "insufficient" verdicts change when re-scored against
   achievable rather than observed peaks?
3. Given that one input scalar closes magnitude and chronology jointly at
   Paradise, what discriminates a mass-input deficit from a melt-generation
   defect on these fixtures, and does the current candidate set contain that
   discriminant?
4. Is the wet-compaction alias materiality bounded independently of the operand
   authority question that currently gates it?
5. Does the `Tmax` warm bias and truncated event tail belong to forcing
   ownership, phase ownership, or both?

## Reproduction

Read-only, from repository root at `b44e75c1`. Requires only Python 3 and the
checked-in surfaces named above.

```python
import csv, datetime as dt, json, os, statistics as st

WP = 'docs/work-packages/20260804-snow-prepeak-mass-transition-physics-adjudication-001'
BASE = 'tests/fixtures/snotel_observed'
d = json.load(open(f'{WP}/artifacts/cross-fixture-results.json'))

def cli_daily(site):
    p = os.path.join(BASE, site)
    f = [x for x in os.listdir(p) if x.endswith('.cli')][0]
    out = {}
    for line in open(os.path.join(p, f)):
        c = line.split()
        if len(c) < 12:
            continue
        try:
            da, mo, yr, pr = int(c[0]), int(c[1]), int(c[2]), float(c[3])
        except ValueError:
            continue
        if 1900 < yr < 2100 and 1 <= mo <= 12 and 1 <= da <= 31:
            out[dt.date(yr, mo, da)] = pr
    return out

for site in sorted({r['site'] for r in d['annual']}):
    gauge = {}
    for r in csv.DictReader(open(f'{BASE}/observations/sites/{site}.csv')):
        if r['observed_precip_mm']:
            gauge[dt.date.fromisoformat(r['date'])] = float(r['observed_precip_mm'])
    cli = cli_daily(site)
    fg, gp, fp = [], [], []
    for r in [x for x in d['annual'] if x['site'] == site and not x['right_censored']]:
        w0 = dt.date.fromisoformat(r['window_start'])
        pk = dt.date.fromisoformat(r['observed_peak_date'])
        fix = sum(v for k, v in cli.items() if w0 <= k <= pk) / 1000.0
        fp.append(fix / r['observed_peak_swe_m'])
        # AWDB PREC is a water-year cumulative total; difference the endpoints.
        s = [v for k, v in gauge.items() if w0 <= k <= w0 + dt.timedelta(days=10)]
        e = [v for k, v in gauge.items() if pk - dt.timedelta(days=5) <= k <= pk]
        if s and e and (max(e) - min(s)) > 0:
            g = (max(e) - min(s)) / 1000.0
            fg.append(fix / g)
            gp.append(g / r['observed_peak_swe_m'])
    ach = st.median([r['peak_swe_ratio'] for r in d['annual']
                     if r['site'] == site and not r['right_censored']])
    print(site, len(fp), round(st.median(fg), 3), round(st.median(gp), 3),
          round(st.median(fp), 3), round(ach, 3),
          sum(1 for x in gp if x < 1.0), len(gp))
```

Expected output reproduces the result table, with `20/29` years below unity for
`gauge/pillow` at Snowbird.

## Claim Limits

- No simulation was executed for this review; every modeled quantity is read
  from the package's frozen result set.
- `gauge/pillow` is an observation-consistency ratio. It bounds what a model can
  attain; it does not attribute the inconsistency to undercatch, pillow bias,
  site representativeness, or preferential deposition, and it does not authorize
  a correction to either instrument.
- The `0.823` and `0.915` ceilings are algebraic upper bounds under zero
  pre-peak loss. A physically correct model must lose some mass before peak, so
  the attainable value is strictly lower and is not quantified here.
- No verdict in this package is disputed, no correction is proposed, no
  parameter is fitted, and no forcing, phase, or observation correction is
  authorized.
- `n = 4` sites. The rank association between forcing headroom and attained peak
  ratio is descriptive and is not offered as a statistical result.
