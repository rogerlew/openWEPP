# Terminal Verification A

Status: `PASS`

Evidence class: `Ran + Static`

Verification scope: exact staged CAL-06 package, result identities,
inventories, independent reconstruction, observation operators, figure
bindings, terminal reviews/findings, exact write set, and proposed final
disposition. This verification does not rely on either reviewer conclusion.

## Commands and results

Ran:

```text
python3 \
  docs/work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001/tools/validate.py
```

Result:

```text
PASS: 261 runs; 259 forest/member executions; 2 open controls; 95,526
climatology rows; 783 observation scores; 13 verdict cells; 6 SVG figures
```

Ran an independent read-only reconstruction over the retained CSV, JSON, and
SVG files. Result:

```text
PASS independent:
manifest output hashes=5
runs=261; forest/member=259; open controls=2
period operands=24,012
reconstructed run summaries=261
reconstructed ensemble summaries=9
reconstructed observation summaries=13
Harvard deciduous density matches/member=425..429
Harvard open density matches=400
Harvard rescore execution-digest matches=38
Marcell observation matches=31,542
SVG figures=6; exact source-table digest bindings=12
```

Ran:

```text
git diff --name-only
git status --short
git diff --check
```

Result: diff check passed. The CAL-06 diff is confined to the package,
work-package catalog, canopy roadmap, and CAL-06 figure contract. The
untracked
`docs/audits/20260727_gate_planner_demotion_readiness_audit.md` is unrelated
and explicitly excluded. No production Rust, canonical contract, fixture,
observation, or public-schema file is part of this package diff.

## Independent determinations

1. `PASS — manifest and inventory.` All five retained outputs match their
   manifest byte counts and SHA-256 identities. The execution and result
   inventories contain exactly 261 passing runs: seven forest lanes retain
   the same 37 accepted members (`259` executions), and two open controls run
   exactly once. No member selection or replacement was found.

2. `PASS — operand and summary reconstruction.` Each run has one `ALL`, 45
   calendar-year, and 46 water-year operand rows: `92 * 261 = 24,012`.
   Independent arithmetic reconstructs every run-result field within
   `1e-12`, then reconstructs all nine lane ensemble summaries. The 13
   nonzero observation groups independently reproduce their member counts,
   match-count ranges, verdicts, and bias/MAE/RMSE minimum, median, and
   maximum.

3. `PASS — Harvard operator semantics.` Harvard bulk density uses only
   HF237-01 daily bulk measurements: deciduous members retain 425--429 matches
   and the open control retains 400. Profile layers remain
   `NOT_EVALUATED_SCALE_MISMATCH`; bound Harvard SWE remains
   `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION` with zero matches; mixed/
   hemlock comparisons remain unbound and `NOT_EVALUATED`. All 38 retained
   Harvard rescore trace/WAT digests equal the corresponding complete-matrix
   execution-manifest identities.

4. `PASS — cell and figure integrity.` The Marcell-only
   `CAL06-SNOW-001` operator reconstructs exactly 31,542 matches. All six SVG
   files parse with accessible title/description metadata. Their 12 declared
   table bindings independently rehash to the exact embedded SHA-256 values.
   Null, excluded, and `NOT_ADVANCED` semantics remain distinct from zero.

5. `PASS — review and finding closure.` Both terminal reviews are `PASS`.
   Every execution/review finding is accepted and tied to a concrete
   correction; no finding remains undispositioned. The substantive
   corrections are present in the terminal data, validator, figures,
   observation-operator records, and current line-count evidence.

6. `PASS — exact staged disposition.` The kickoff prompt is archived,
   exact-diff reconciliation names only the intended write set, and the
   package/final-disposition status correctly remains
   `ready for terminal verification` until both verifiers pass. The proposed
   disposition,
   `COMPLETE / BOUNDED GRADIENT CHARACTERIZATION / DOWNSTREAM ADVANCEMENT
   WITHHELD`, matches the evidence and preserves every claim limitation.

## Verification disposition

`PASS`. No terminal verification A blocker remains. CAL-06 supports bounded
within-model canopy-gradient characterization and the CAL-07 handoff only. It
does not establish CAL-04B transferability, empirical canopy-amplitude
validity, predictive litter-source authority, residue/frost adequacy,
downstream advancement, Southern Hemisphere robustness, or assurance
publication. Final closure remains contingent on independent Verification B
passing this same staged state and the mechanical final status transition.

## Plot-only Figure/Sidecar Follow-on Re-verification — 2026-07-28

Status: `PASS`

Evidence class: `Ran + Static`

This section verifies the user-directed presentation-only follow-on. It
supersedes only the original figure-integrity inventory and source-binding
count above; the scientific reconstruction and bounded disposition remain
unchanged.

### Commands and results

Ran:

```text
python3 \
  docs/work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001/tools/validate.py
```

Result:

```text
PASS: 261 runs; 259 forest/member executions; 2 open controls; 95,526
climatology rows; 783 observation scores; 13 verdict cells; 6 SVG figures; 6
Markdown sidecars
```

Copied the package to a fresh temporary root, ran its current
`tools/plot_results.py`, and independently compared SHA-256 identities.
Result:

```text
PASS: temporary-root renderer reproduced all 6 SVGs byte-for-byte
```

Ran an independent read-only XML/Markdown/hash check. Result:

```text
PASS independent follow-on:
SVG figures=6
same-basename Markdown sidecars=6
exact plotted-source bindings=6
resolvable sidecar links=22
unchanged manifest-bound scientific objects=5
```

### Follow-on determinations

1. `PASS — exact paired inventory.` The figure directory contains exactly six
   canonical `cal06-*.svg` files and six `cal06-*.md` sidecars with identical
   basename sets. No plot lacks a caption/ancillary sidecar and no orphan
   sidecar exists.

2. `PASS — plot-only visible content.` Independent XML text extraction found
   only legends, panel/axis labels, units, ticks, plotted site/stratum
   categories, and the congruence matrix's plotted cell/status/advancement
   values. Caption prose, verdict rationale, source-completeness narrative,
   residual narrative, and gated-consequence prose are absent from visible
   SVG `<text>`. The seasonal categorical panels contain no irrelevant month
   ticks.

3. `PASS — source identity and accessibility.` Every SVG retains
   `role="img"`, `aria-labelledby="title desc"`, and nonvisual accessible
   `title`/`desc` elements. Each SVG has one exact
   `metadata#source-bindings` entry, and every embedded SHA-256 value
   independently rehashes to its plotted source table. The six SVG file
   digests also match the identities recorded by follow-on Review A.

4. `PASS — sidecar semantics and links.` Every sidecar contains `Caption`,
   `Ancillary information`, and `Source data`, names its paired SVG, and links
   to existing local tables and the deterministic renderer. The sidecars
   preserve the complete ensemble/model-response boundary, 37/37 bounded
   ordering, Harvard SWE and density-scale exclusions, predictive
   needle/fine-woody authority-missing nulls, residue/frost/downstream
   `NOT_ADVANCED`, erosion `NULL_NOT_EMITTED`, and distinctions among
   nonapplicable, unbound, missing, and measured-zero states.

5. `PASS — deterministic presentation only.` A clean temporary-root render
   reproduced every SVG byte-for-byte. The five manifest-bound scientific
   objects retain their original exact hashes:
   `daily-climatology.csv`, `execution-manifest.csv`,
   `observation-scores.csv`, `run-period-operands.csv`, and
   `run-results.csv`. The validator still independently reconstructs all run,
   ensemble, observation, and verdict evidence. No run, score, cell, or
   science claim changed.

6. `PASS — follow-on closure records and write set.` Both independent
   follow-on re-reviews pass. The finding ledger accepts and closes the
   plot/sidecar request, and exact-diff/final-disposition records describe the
   same bounded claim. The write set remains the CAL-06 package, work-package
   catalog, canopy roadmap, and fulfilled figure contract; the unrelated
   gate-planner audit remains excluded. `git diff --check` passes.

### Follow-on verification disposition

`PASS`. The plot-only SVG/Markdown-sidecar split is accessible,
source-bound, deterministic, and scientifically lossless. It does not alter
the supported disposition:
`COMPLETE / BOUNDED GRADIENT CHARACTERIZATION / DOWNSTREAM ADVANCEMENT
WITHHELD`. No follow-on Verification A blocker remains.

## Downstream Legend Correction Re-verification — 2026-07-28

Status: `PASS`

Evidence class: `Ran + Static`

Ran the current terminal validator; it passed all 261 runs, 24,012 period
operands, 95,526 climatology rows, 783 observation scores, 13 verdict cells,
six SVGs, and six Markdown sidecars.

Independent SVG text inspection found the downstream visible legend is
exactly `open`, `deciduous`, and `mixed`; `conifer` is absent. The paired
sidecar truthfully states that Harvard has no conifer lane and that conifer is
intentionally absent from both plots and legend.

A fresh temporary-root render reproduced all six SVGs byte-for-byte. The five
manifest-bound scientific objects retain their previously verified SHA-256
identities, so the correction changes only the Harvard downstream legend and
its explanatory sidecar. The finding ledger accepts the correction and
`git diff --check` passes.

Targeted legend-correction disposition: `PASS`. No Verification A blocker
remains.
