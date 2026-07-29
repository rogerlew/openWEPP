# Terminal Scientific/Code Review A

Status: `PASS — FOLLOW-ON FIGURE PRESENTATION RE-REVIEW`

Evidence class: `Static: complete package, tool, result, figure, roadmap, and
write-set review; Ran: read-only terminal validator, independent inventory and
source-null queries, SVG rasterization, and visual inspection`

## Finding closure

### CAL06-RA-001 — `HIGH` — Harvard observation scales were pooled

`CORRECTED`. The initial operator pooled HF237-01 daily bulk density with
HF237-02 vertical profile layers. Those are not the same measurement scale.
The terminal operator scores WAT aggregate density only against HF237-01 daily
bulk density; profile density is explicitly
`NOT_EVALUATED_SCALE_MISMATCH`. The final complete 261-run execution directly
produced the corrected score table. Harvard deciduous density retains
425--429 matches per member and open retains 400, rather than the inflated
cross-scale counts.

Harvard SWE is separately and correctly excluded as
`INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION` for the 37 deciduous members and
the open control. The retained raw example, provider units, and physical
identity show the approximately tenfold conflict. The installed source was not
relabeled or edited. The unbound Harvard mixed/hemlock comparison remains
`NOT_EVALUATED`, not borrowed as a mixed-stratum validation target.

### CAL06-RA-002 — `HIGH` — terminal summaries were not independently reconstructable

`CORRECTED`. Daily raw traces and WAT objects are intentionally ephemeral, but
the original climatology alone could not reconstruct annual, water-year,
peak, melt-out, or frost summaries. The terminal execution now retains 24,012
data rows in `run-period-operands.csv`: one all-period row, 45 calendar-year
rows, and 46 water-year rows for each of 261 runs.

The validator independently reconstructs every run summary from those
operands, then reconstructs all nine ensemble summaries from `run-results.csv`.
It also checks complete lane membership and the 366-row climatology for every
run. This is sufficient compact retention for the claimed summaries without
promoting the multi-million-row ephemeral objects.

### CAL06-RA-003 — `HIGH` — result figures did not fulfill or bind the figure contract

`CORRECTED`. The initial snow view omitted peak SWE, bulk density, and melt-out
distributions; the litter view omitted residue depth; and downstream/null
labels were not derived from the verdict table. The terminal six-figure set
now includes those snow quantities, residue depth, frost onset/thaw, explicit
source completeness, Harvard SWE exclusion, and visible `NOT_ADVANCED`
consequences. Snow, litter/residue/frost, and downstream status text is derived
from `verdict-matrix.csv`.

Each SVG has an accessible title/description and exact source-table SHA-256
metadata. Independent rasterization and inspection found the required
quantities, units, uncertainty bands, nulls, and advancement labels legible.
The complete 37-member range remains visible; no best member replaces it.

### CAL06-RA-004 — `HIGH` — the validator could accept mutually stale derived evidence

`CORRECTED`. After the Harvard rescore, corrected per-run scores briefly
coexisted with a stale observation summary and a figure bound to that stale
summary. The former validator checked the figure-to-summary digest but not the
score-to-summary relationship.

The terminal validator now independently reconstructs every observation
summary group, member count, match-count range, bias/MAE/RMSE
minimum/median/maximum, and group verdict from `observation-scores.csv`. It
also reconstructs every ensemble-summary metric from the retained run table.
This closes the stale-promotion path rather than merely refreshing the
affected files.

## Terminal assessment

The final retained inventory is complete: 259 forest/member executions cover
the same 37 accepted CAL-04B members in all seven forest lanes, and the two
open controls execute once each. All 261 runs retain 16,437 days. The accepted
ensemble digest, runner digest, source commit, per-run trace/WAT digests, and
retained-output digests provide deterministic identity. No member selection,
refit, downstream compensation, or production-physics edit occurred.

Authority boundaries are appropriately narrow. The 37/37 winter-gradient
ordering is labeled model-response evidence, not empirical canopy-amplitude
validation. Observation residuals have no invented acceptance threshold.
Predictive needle and fine-woody sources are
`NULL_AUTHORITY_MISSING` for all 259 forest runs and `NOT_APPLICABLE` for open
controls. Erosion is `NULL_NOT_EMITTED` for all 261 runs, never zero.
Residue, frost, ET, runoff, and erosion advancement remains `NOT_ADVANCED`
where the named upstream chain does not pass.

I ran the terminal validator against the corrected artifacts. It reported:

> PASS: 261 runs; 259 forest/member executions; 2 open controls; 95,526
> climatology rows; 783 observation scores; 13 verdict cells; 6 SVG figures

The retained gate record also reports the full runner tests (221/221), focused
contracts (19/19), deterministic analysis/renderer repeat, Markdown checks,
and diff hygiene. Current scoped changes are the package, work-package
catalog, canopy roadmap, and CAL-06 figure contract. No production Rust,
canonical contract, fixture, or observation file is in the CAL-06 write set.
The unrelated untracked gate-planner audit is visibly excluded.

## Verdict

`PASS`. CAL06-RA-001 through CAL06-RA-004 are corrected in the reviewed
terminal state. CAL-06 supports bounded canopy-gradient characterization and
the Order-7 handoff only. It does not support CAL-04B transferability,
empirical canopy-amplitude validation, predictive needle/fine-woody source
authority, residue/frost adequacy, or downstream advancement.

This pass does not replace Review B, finding disposition, dual independent
verification, terminal Markdown/diff reconciliation after review artifacts,
or final package disposition.

## Follow-on figure presentation re-review

Status: `PASS`

Evidence class: `Static: current figure contract, renderer, validator, six
SVGs, and six same-basename Markdown sidecars; Ran: terminal validator,
independent SVG text/digest inspection, SVG rasterization and visual
inspection, package Markdown lint, and diff hygiene`

This section reviews the user-directed plot-only presentation refinement and
supersedes only the presentation-specific wording in CAL06-RA-003. Scientific
results and its `CORRECTED` disposition are unchanged.

All six SVGs are plot-only in their visible content:

- chronology, snow, litter/residue/frost, and downstream figures contain only
  legends, panel/axis labels, units, tick values, and month ticks;
- seasonal panels contain only metric/axis labels, tick values, plotted
  site/stratum categories, bars, and whiskers. The irrelevant month labels
  have been removed from these categorical panels; and
- the congruence matrix contains only its plotted categorical dimensions:
  cell identifier, status, and advancement.

Independent XML text extraction found no visible caption, verdict rationale,
source-completeness narrative, observation-residual narrative, or gated-chain
prose. Accessible SVG `title` and `desc` elements remain nonvisual metadata.
Rasterized inspection found all six layouts legible, with uncertainty bands,
line patterns, units, categories, and matrix symbols distinguishable.

Each SVG has exactly one same-basename Markdown sidecar. Every sidecar contains
a caption, an `Ancillary information` section, and a `Source data` section
that names the paired SVG, linked tidy tables, and deterministic renderer.
Together they preserve the moved scientific boundaries:

- the complete 37-member ensemble and model-response-only canopy claim;
- the 37/37 bounded seasonal ordering and no empirical amplitude promotion;
- Harvard SWE
  `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION`, bulk/profile scale separation,
  unbound mixed/hemlock comparison, and no invented snow threshold;
- predictive needle/fine-woody `NULL_AUTHORITY_MISSING`, null-not-zero
  semantics, and residue/frost `NOT_ADVANCED`;
- downstream model-response-only status, no compensation, and erosion
  `NULL_NOT_EMITTED`; and
- the distinction among `BOUNDED`, `NOT_EVALUATED`, `NOT_ADVANCED`,
  authority-missing, nonapplicable, unbound, and measured zero.

Independent SHA-256 reconstruction matched every SVG
`metadata#source-bindings` entry to its plotted source table. Current SVG
digests are:

- canopy chronology:
  `fb1b17fd375a447446c99748bf56213a3a3c461bfcb7625e82072cf5da12cf80`;
- congruence matrix:
  `5997213bb954863465caf5e77a537eafd9cf65a7eac690986e829797c1b202a4`;
- downstream consequences:
  `250156290ffe27a13f661a77b7640b3734433b15526111e3a7f4a9ece3d47b08`;
- litter/residue/frost:
  `0382387b6635bd95ee55f54dc9522cc13754a57a3375636acf409b598ca723c8`;
- seasonal ordering/amplitude:
  `40757569f3f6282ea423f1ff7c8a7e6624dbb68edfbabcd9c5074e713a851f68`;
  and
- snow response:
  `5ac1f9e3fa13913c7aff76db6267c3dcca67bb3ec18329c1a2371fd2c7c38264`.

The retained result-manifest identities for the accepted ensemble, source
commit, runner, run results, period operands, observation scores, execution
manifest, and daily climatology are unchanged from the original Review A.
The presentation refinement therefore changes figure composition and
documentation only; it does not change a run, summary, score, cell verdict, or
science claim.

The current terminal validator reported:

> PASS: 261 runs; 259 forest/member executions; 2 open controls; 95,526
> climatology rows; 783 observation scores; 13 verdict cells; 6 SVG figures; 6
> Markdown sidecars

Package Markdown lint scanned 30 files with zero errors or warnings, and
`git diff --check` passed.

Follow-on verdict: `PASS`. The plot-only SVG/sidecar split fulfills the amended
figure contract without weakening CAL-04B uncertainty, CAL-05 source-null
semantics, Harvard observation exclusions, or downstream advancement gates.

## Harvard downstream legend-correction re-check

Status: `PASS`

The downstream renderer now supplies the legend with exactly `open`,
`deciduous`, and `mixed`, matching the three plotted Harvard lanes. Independent
SVG text extraction and raster inspection confirm that `conifer` is absent
from the legend and plots. The sidecar explicitly records that no Harvard
conifer lane is available, and the validator both rejects `conifer` and
requires all three available strata.

The regenerated downstream SVG digest is
`6303080140c882d2bad216aafe8b0da1b506ca2a981dbd18774e1e5b2d995157`,
which supersedes the earlier downstream digest recorded in the follow-on
re-review. Its `daily-climatology.csv` source binding independently matches
`e3b973d51d93b0dc804dc06feb7021cf5e4f39c85fc034da3f615e63a81dc410`.
The terminal validator passes all 261 runs, six SVGs, and six sidecars. This is
a presentation correction only; no science result or authority boundary
changed.
