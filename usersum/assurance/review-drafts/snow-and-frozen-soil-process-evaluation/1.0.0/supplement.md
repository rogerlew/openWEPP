# Supplement: Observational Evaluation of openWEPP Snow and Frozen-Soil Processes

*Version 1.0 — 2026-08-05*

This supplement documents the evidence identities, reconstruction, dataset
roles, metric semantics, and claim boundaries behind the
[main report](index.md). It does not extend the report's conclusions.

## S1. Study Structure

The synthesis contains four coupled but separately interpreted substudies:

1. hourly precipitation-phase classification;
2. daily seasonal SWE, depth, density, and timing signatures;
3. frost-tube and soil-temperature frozen-soil response; and
4. production conservation and consumer verification.

They remain in one report because snow phase and snowpack state determine the
insulating boundary required to interpret frozen-soil response. No aggregate
status, accuracy, or validation grade combines them.

## S2. Assessed Realizations and Currency

Drafting occurred at Git
`47c2cf9eae6eef95f0f670d157d2d31df4cbf9cc`. The retained results were
executed under the exact realizations identified in their originating evidence
packages. The integrated production conservation record is frozen at
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.

This source binds the retained bytes rather than claiming all model executions
were repeated at the drafting checkout. Static source availability is not a
fresh release reproduction. Human review may require selected reruns, and
publication requires a separate release transfer to an exact approved source
and release configuration.

The 2026-08-05 authority refresh binds the report to
`SC-SNOWENERGY-001` v7, `SC-SNOWFREEZE-001` v127, and the terminal dispositions
of campaign increments 21K-21N. It does not replace the retained empirical
result with the later diagnostic populations. The retained ten-surface result
still describes its named historical realization; the later evidence changes
currency and interpretation, not those 188 stored values.

## S3. Evidence Identities

| Evidence | SHA-256 | Role |
| --- | --- | --- |
| Jennings phase result | `f5d261379f2aaed862a4ad6734e76e3d8123f56df46ca7739fa2fab86c2d6ef8` | Hourly observed classification and station thresholds |
| Current-default activation/profile result | `f511c11d73b2a0b03cb7ef8f573ddc9309ffd336f2790cd1218514a74565747a` | Production selector/closure and ten-surface snow profile |
| Post-partition residual decomposition | `0225ff80580ef352b2b91720da947f7f16f909a48827fb47e6c207da5b4e8875` | Signed snow residuals and process-family diagnosis |
| Non-SNOTEL frozen-soil baseline | `b3806ced25cf01eb4c7558eee8e9d7f3f486633aa708e93dbe63b115e76a8930` | Frost-tube, isotherm, and paired snow-control metrics |
| Integrated conservation and consumer evidence | `306b96a1d45fca85d5604b16fe8ce4b814df48d2fc15ecb910e198085ee81f18` | Independently reconstructed production WAT ledgers |
| Snow/frost science contract | generated identity lock | v127 formulation, units, obligations, current defaults, Stage 3 ownership seam, and evaluation-only shadow authority |
| Snow-energy science contract | generated identity lock | v7 surface-energy and future sole-melt-owner authority |
| 21K wet-compaction disposition and public-safe extract | generated identity lock | Corrected exact operand lineage and causal-attribution reset |
| 21L warm/mixed disposition and public-safe extract | generated identity lock | Multifactor, chronology-confounded corrected-state signal |
| 21M CoE audit disposition and public-safe extract | generated identity lock | Post-2007 baseline fidelity with an independent-authority gap |
| 21N ownership disposition and public-safe extract | generated identity lock | Stage 3 future ownership and atomic implementation hold |

The exact terminal dispositions are identity-bound inputs. Uniquely named,
report-owned faithful extracts are exposed as public-safe research objects in
the disposable build. The
[archived refresh prompt](research-objects/20260805-codex-execute-assure06-refresh_prompt.md) is
likewise content-identified and staged. Inclusion in staging is not publication
authority.

## S4. Precipitation-Phase Reconstruction

The admitted Jennings result read 17810805 rows of hourly
observations and scored 11711058 rows across
6883 stations. The scorer excluded
6099747 rows
(34.2 %). Eligible rows required complete
numeric inputs, a station threshold, valid temperature and humidity, successful
model evaluation, and an exclusive observed rain or snow label; mixed and
neither-phase rows were excluded. Exclusion-reason counts were not retained.
The reconstruction procedure sums
the confusion matrix independently:

```text
N = rain-as-rain + rain-as-snow + snow-as-rain + snow-as-snow
accuracy = (rain-as-rain + snow-as-snow) / N
```

For Harder-Pomeroy the four operands are
3396066 rows, 498328 rows,
635996 rows, and
7180668 rows. Their total equals the retained scored-row
count, and the reconstructed accuracy equals the source value before it is
accepted into the strict result.

For the fixed-threshold baseline the four operands are
3574653 rows,
319741 rows,
1339353 rows, and
6477311 rows. Harder-Pomeroy uses a rain-fraction
cutoff of one-half; the baseline uses air temperature above freezing. Modeled
station transition temperature is the scored event nearest the one-half rain
fraction. Mean station humidity defines lowest and highest deciles of
688 stations each. The exact rules are staged
in the [scoring implementation](research-objects/snowbench_jennings_phase.rs).

The fixed 0 degrees Celsius threshold is a scientifically relevant baseline
because it was the prior production method. It is not a truth target. The
station-threshold and humidity diagnostics test different behavior and are not
pooled with classification accuracy.

## S5. Snowpack Profile Semantics

The current-default profile contains 10 sites: five
SNOTEL and five canopy configurations. Each surface has a consistent set of
forcing-robust rubric cells. Across all surfaces, the available distribution
is:

- 15 cells in the fail band;
- 14 cells in the marginal band;
- 18 cells in the pass band; and
- 43 cells in the strong band.

The ordinal labels use signature-specific bands defined by
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-050`. They must not be averaged into a
probability. The retained source also contains forcing-limited magnitude cells;
those values remain available for inspection but do not carry mechanism
verdicts. The ninety available entries are correlated site-by-signature
diagnostics, not ninety independent observations. Pass begins at KGE at least
six-tenths for KGE cells; marginal spans three-tenths through six-tenths, and
timing pass requires modeled-minus-observed date offset within fourteen days.
Density and magnitude bands are specified separately in the science contract;
none of these thresholds is a calibration objective.

The post-partition decomposition closes 15 cells from the
fail band into 9 cells assigned to densification,
2 cells assigned to depth-SWE geometry, and
4 cells assigned to timing/persistence. The
decomposition
is diagnostic: it identifies the response signature, not a unique parameter or
code defect.

## S6. Frozen-Soil Method Separation

Frost tubes were evaluated at 3 sites. The matched
frost-depth dataset contains 675 rows. The procedure
retains the largest absolute site residual,
0.99 m. Those sites also contain
660 rows of paired snow-depth control dates; the
retained control fails in 493 rows.

Measured soil-temperature profiles came from
2 sites: Mandan and Reynolds Creek. The derived
0 degrees Celsius isotherm is treated as an upper-bound/timing referent rather
than a frost-tube magnitude observation. The retained metrics evaluate a
dataset of 14939 rows against the bound and record
exceedances in
3556 rows. No paired observed snow
depth was available at either site.

The report therefore makes no pooled five-site frost-depth accuracy statement.
It also does not infer that snow-confounded residuals are defects in heat flow,
frozen conductivity, soil-freezing characteristic curves, impedance, or
migration heat.

## S7. Conservation Reconstruction

The production phase trace contains
159986 rows. Among its
53711 rows with active precipitation, the maximum
absolute residual for rain plus snow minus active precipitation is
5.55e-17 m, below the declared
1.0e-10 m allowance.

The integrated validation record reconstructs water from produced WAT fields:

```text
snow residual = precipitation - routed melt - change in Snow-Water
frost residual = initial liquid+frozen water + external inputs
                 - external outputs - final liquid+frozen water
```

Snow-Depth is rejected as a mass operand. Frost depth is rejected as a water
operand. Freeze/thaw exchange is internal to combined liquid-plus-frozen
storage and is not counted as an external flux. The accepted maximum residuals
are 8.88e-16 mm for the reported accumulation row,
0.00e+00 mm for the reported release row, and
8.53e-14 mm for the reported frozen-soil rows.

The frozen-soil freeze-growth spot row retained prior liquid
2.746e+02 mm, prior frozen
3.073e+01 mm, current liquid
2.721e+02 mm, and current
frozen 3.143e+01 mm. The
material-thaw spot row retained prior liquid
3.118e+02 mm, prior frozen
2.247e+00 mm, current liquid
3.129e+02 mm, and current
frozen 2.560e-02 mm. Inputs,
sinks, combined storages, and residuals are shown in the main report table. The
[content-identified source log](research-objects/final-reconstruction-arithmetic.log)
exposes every transcribed operand and row identity.

## S8. Reproduction

From the repository root, reproduce the strict result with Python 3's standard
library:

```console
.venv/bin/python -B assurance/v2/reports/snow-and-frozen-soil-process-evaluation/procedures/reproduce_snow_frost_synthesis.py \
  --jennings docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/artifacts/jennings-validation-report.json \
  --activation docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/artifacts/harder-pomeroy-default-activation.json \
  --residuals docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/artifacts/post-partition-residual-decomposition.json \
  --frost docs/work-packages/20260625-snowfrost-fidelity-i0-non-snotel-rubric-baseline-001/artifacts/non_snotel_rubric_baseline.json \
  --conservation assurance/v2/reports/snow-and-frozen-soil-process-evaluation/inputs/conservation-operands.json \
  --conservation-source docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/final-conservation-and-consumer-evidence.md \
  --conservation-log docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/logs/final-reconstruction-arithmetic.log
```

Parse the output and retained `snow-frost-synthesis.json` as JSON and require
exact structural/value equality. The procedure checks confusion-matrix closure,
reported accuracies, snow-label closure, residual-family closure, site/method
aggregation, finite values, and the conservation source digest. It does not run
openWEPP, choose a method, assign an ordinal band, or interpret a conclusion.

The portable research package contains the
[strict synthesis result](research-objects/snow-frost-synthesis.json),
[deterministic reproduction procedure](research-objects/reproduce_snow_frost_synthesis.py),
[conservation operands](research-objects/conservation-operands.json),
[phase-evaluation result](research-objects/jennings-validation-report.json),
[production activation evidence](research-objects/harder-pomeroy-default-activation.json),
[snow residual decomposition](research-objects/post-partition-residual-decomposition.json),
[frozen-soil baseline](research-objects/non_snotel_rubric_baseline.json), and
[integrated conservation record](research-objects/final-conservation-and-consumer-evidence.md).
Exact phase rules, conservation rows, and dataset provenance are carried by the
[phase-scoring implementation](research-objects/snowbench_jennings_phase.rs),
[selected-row reconstruction log](research-objects/final-reconstruction-arithmetic.log),
and [dataset provenance extract](research-objects/dataset-provenance.md).
Method and provenance context is retained in the
[science contract](research-objects/SC-SNOWFREEZE-001.md),
[study protocol](research-objects/study-protocol.md),
[evidence inventory](research-objects/evidence-inventory.md).
Agent involvement is disclosed through the retained
[execution prompt](research-objects/20260716-codex-execute-assure06_prompt.md) and
[assistance packet](research-objects/agent-assistance-packet.json).

Build a disposable reader surface with:

```console
stage="$(mktemp -d)"
mkdir -p "$stage/usersum"
cp usersum/snow-frost-modeling-and-validation.md "$stage/usersum/"
cargo run --quiet -p openwepp-assurance -- build \
  --report snow-and-frozen-soil-process-evaluation --staging-root "$stage"
cargo run --quiet -p openwepp-assurance -- check \
  --report snow-and-frozen-soil-process-evaluation --staging-root "$stage"
```

The build copies authored prose and mechanically resolves typed values, tables,
figures, links, and research objects. It does not reproduce the science or
authorize publication.

## S9. Claim-to-Evidence Map

| Claim | Primary evidence | What it establishes | What it does not establish |
| --- | --- | --- | --- |
| SF-P01 precipitation phase | Jennings result; activation trace; Harder-Pomeroy authority | Retrospective observed classification, humidity pattern, active selector, partition closure | Untouched held-out validation, precipitation amount, snowpack accuracy |
| SF-P02 seasonal snowpack | Current-default profile; residual decomposition; admitted SNOTEL/canopy observations | Cross-regime response profile and named residual families | Universal score, forcing-independent magnitude accuracy, untested-site fitness |
| SF-P05 authority and implementation currency | v7/v127 contracts; identity-bound 21K-21N dispositions and public-safe extracts | Corrected operand authority, current CoE compatibility ownership, future Stage 3 sole ownership, evaluation-only shadow authority, and implementation hold | Empirical efficacy, noninferiority, default change, runtime cutover, causation, or warm-maritime conifer transfer |
| SF-P03 frozen-soil response | Non-SNOTEL baseline; observation manifests | Method-specific coupled residuals and snow-control status | Isolated frost-physics error or transferable frost-depth accuracy |
| SF-P04 production verification | Activation trace; integrated WAT reconstruction | Selected real consumer and water-accounting identities | Empirical predictive accuracy |

## S10. Agent Assistance and Human Boundary

Codex assembled the evidence inventory, authored the draft, and implemented the
deterministic reconstruction under the ASSURE-06 protocol. The exact source and
input identities are retained in the machine descriptor and agent-assistance
packet. Hidden model/runtime sampling configuration is unavailable, so agent
provenance supports audit and review rather than bitwise prose regeneration.

Internal coding-agent reviewers may identify defects and reproduce arithmetic.
They cannot supply accountable human report leadership, scientific approval,
external peer review, assurance stewardship, release ownership, or application
fitness. The current structured attribution and lifecycle projections are:

**Authorship and accountability.** Draft authors: Codex (AI coding agent). Accountable report lead: Roger Lew. Material producers: None recorded.


**Assurance status.** This report is `DRAFT`. Independent scientific, reproduction/publication, and assurance-steward approval remain pending; no approval lock exists. It does not authorize public export, vendoring, or an application-fitness determination.


## Revision Log

| Version | Date | Change |
| --- | --- | --- |
| 1.0 authority refresh | 2026-08-06 | Bound v7/v127, 21K-21N, and evaluation-shadow authority; separated current CoE implementation from future Stage 3 authority; and retained the unchanged 188-value empirical result and human-review boundary. |
| 1.0 review entry | 2026-07-16 | Recorded Roger Lew as report lead and material producer; entered independent human review without changing scientific claims, methods, results, tables, or figures. |
| 1.0 draft | 2026-07-16 | First manuscript-first synthesis of the retained phase, seasonal snowpack, frozen-soil, and conservation evidence. |
