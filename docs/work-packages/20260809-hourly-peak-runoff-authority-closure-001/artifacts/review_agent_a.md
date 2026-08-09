# Independent Hydrology / Science Re-Review A

Status: `executed`

Reviewed identity: `7820953c1b5258564200bd167e0c4994a69b3065`

Reviewed range: `a65cc3973..7820953c1b5258564200bd167e0c4994a69b3065`

Evidence class:

- `Static`: terminal base-to-implementation diff, implementation, tests,
  `SC-WATBAL-001` v168, `SC-RUNOFFPART-001`, `SC-INFILE-HBP-001`, and package
  evidence/claim artifacts.
- `Ran`: `git diff --check
  a65cc3973..7820953c1b5258564200bd167e0c4994a69b3065`; PASS.
- `Ran`: `cargo nextest run --test erosion_single_ofe_p61_sediment
  --test erosion_multi_ofe_p102_chain --profile quick`; 2 passed, 0 skipped,
  nextest run `b4a186f1-4be4-4a34-a3c3-5574c8bba7a8`.
- `Ran`: focused six-test source-custody/frost/tolerance expression in
  `openwepp-hillslope-orchestrator`; 6 passed, 466 skipped, nextest run
  `85991a95-4ede-48b1-9063-67e74c96a951`.
- `Static/package-recorded`: other gate and census evidence was inspected but
  was not independently rerun by this reviewer.

Verdict: `PASS`

## Findings

No remaining Critical, Major, or Minor hydrology/science finding at the exact
reviewed implementation commit.

## Authority Assessment

### Hourly post-partition peak definition

The peak operand is the closing 24-bin runoff-depth series: WB14 excess after
infiltration, depression storage, same-pass correction, and frost
reconciliation, plus WB19 surface-saturation return in its produced hour.
`closing_hourly_runoff_depths_m` independently assembles that series and
`ensure_hourly_runoff_source_closure` requires its sum to close to daily `Q`.
The production peak is then the maximum bin depth divided by 3,600 seconds;
normalized weights are derived only after closure and are not the peak
authority (`direct_runtime/runoff.rs:1523-1665`). This implements
`SC-WATBAL-001#INV-WATBAL-102..104` without an instantaneous or subhourly peak
claim.

### Melt and runon custody

Routed melt/liquid is admitted through the WB14 additional-supply ledger.
Area-scaled surface and lateral/subsurface upstream carry is resolved in R4J,
kept on its explicit hourly transfer shape, and admitted to that same WB14
supply before infiltration and depression-storage partitioning
(`direct_runtime/runoff.rs:157-195,577-615,651-742,1743-1896`). Positive runon
without a producer or positive surface/lateral carry without its hourly shape
hard-fails. There is no uniform fallback and neither melt nor runon is appended
as a raw runoff limb after partition.

The lateral-carry strategy is consistent with the pinned baseline authority:
`SC-RUNOFFPART-001#REF-RUNOFFPART-BASELINE-RUNON-FIN` records baseline hourly
`xfin` admission of both upstream surface and lateral carry, while
`INV-RUNOFFPART-028/031` requires separated, area-scaled transfer and same-pass
WB14 re-infiltration. The implementation preserves that separation through R4J
before merging the two lawful liquid supplies for WB14.

### Same-pass and frost fail-closed behavior

The live R4K path calls `ensure_hourly_same_pass_source_custody` before applying
a daily-only additional same-pass infiltration debit. A positive local-only
debit combined with positive runon therefore hard-fails because source-tagged
hourly debit timing is absent; a merged source series is not proportionally or
residually edited (`direct_runtime/runoff.rs:230-247,1441-1456`).

For frost, a daily debit may clear the complete hourly WB14 series only when no
positive partition runoff remains. Partial frost retention that leaves positive
runoff hard-fails for a missing hourly frost producer, and positive daily
runoff with an empty hourly ledger also hard-fails
(`direct_runtime/runoff.rs:1466-1521`). The focused exact-commit run exercised
complete clearing, partial-retention rejection, empty-ledger rejection, and
mixed-source rejection.

### Tolerance provenance

`TOL-WATBAL-009` is bounded to 24 times the existing `1e-9 m` WB14
interval-infiltration allowance in
`SC-RUNOFFPART-001#TOL-RUNOFFPART-007`, with scale awareness. It adjudicates two
present independently accumulated ledgers only: it cannot supply a source,
invent timing, or mutate a positive hourly bin. The exact aggregate boundary is
accepted and a value `1e-12 m` beyond it is rejected in the focused test. The
hourly ledger remains authoritative within tolerance.

### Area conversion and downstream consumer

Publication validates positive area, basis-adjusts the depth-rate peak only to
the published runoff-depth basis, and multiplies by area exactly once to obtain
`m3/s` (`direct_runtime/01_publication.rs:576-640`). `SC-INFILE-HBP-001` defines
the public peak as `max(V_h)/3600`, requires `sum(V_h) = runvol`, and keeps
rectangular-equivalent duration at `runvol/peak`. The independently rerun p61
single-OFE and p102 routed multi-OFE consumers both passed at the exact commit,
including reconstruction from the real HBP/pass-Parquet hourly series.

## Claim Boundary

This PASS is an implementation hydrology/science review. It supports only a
hillslope-scale, non-calibrated maximum-hourly-mean runoff peak with the stated
WB14/WB19 source custody. It does not support an instantaneous peak,
subhourly timing, watershed/channel routing, observed-flow validation, or
legacy numerical parity claim.

Package closure is not claimed by this artifact. At review time,
`implementation-test-evidence.md` still says terminal gates are pending,
`mutation-study.md` and `summary.md` identify implementation anchor
`949349e7055c5d19277eeb708401c4614a52cd77`, `gate-results.md` is queued, and
`disposition.md` remains executing. Although the committed full census log ends
successfully, it does not itself embed sufficient binary/commit provenance to
promote that run to exact-commit closure evidence. Those artifacts must be
reconciled to the terminal identity before any package-complete claim; this is
an evidence/disposition follow-up, not a defect in the reviewed science
implementation.

The exact implementation commit therefore passes hydrology/science review,
with terminal package disposition correctly remaining outside this review's
claim.
