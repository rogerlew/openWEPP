# Terminal Scientific Review B

Status: `PASS`

Evidence class: `Ran: independent read-only reconstruction + Static:
science, source-authority, runtime-lineage, figure, and closure review`

## Finding closure

| Finding | State | Evidence |
| --- | --- | --- |
| CAL06-RB-001 | `CORRECTED` | The first score table pooled Harvard vertical density-profile layers with modeled bulk snow density. The terminal executor and full 261-run rerun use only HF237-01 daily bulk density for the bound Harvard open and deciduous lanes. Deciduous density retains 425--429 exact-date matches per member and open retains 400. Profile layers remain `NOT_EVALUATED_SCALE_MISMATCH`; Harvard SWE remains `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION`; unbound hemlock remains `NOT_EVALUATED`. |
| CAL06-RB-002 | `CORRECTED` | `CAL06-SNOW-001` initially reported the global bound-snow match count even though its prospective cell is Marcell-only. Independent reconstruction now returns 31,542 Marcell metric/member matches, the verdict reports that count, and the terminal validator asserts the cell-scoped value. |
| CAL06-RB-003 | `CORRECTED` | Terminal line-count evidence initially described stale package-tool sizes. It now reports all five current Python tools exactly and preserves the applicable result: no Rust file changed, so no Rust line-count threshold is implicated. |

## Independent evidence

- The terminal validator passes: 261 runs, 259 forest/member executions, two
  open controls, 95,526 climatology rows, 783 observation-score rows, 13
  prespecified verdict cells, and six SVG figures.
- The retained CAL-04B ensemble digest is
  `83e749a3961604e4592f2a2217db30965c8bbb59f4752d0ff6d85fbac61fd986`.
  It matches the accepted CAL-04B artifact, and all seven forest lanes retain
  the same 37 member identities. The release-runner digest also matches the
  current exact binary.
- All five manifest-bound result objects rehash exactly. The 24,012 retained
  period-operand rows provide one `ALL`, 45 calendar-year, and 46 water-year
  rows for every run. Independent arithmetic reproduces every numeric
  `run-results.csv` summary within the declared `1e-12` tolerance.
- Independent winter-cover reconstruction confirms 37/37 strict ordering for
  Marcell deciduous < mixed < conifer, Harvard deciduous < mixed, and Hubbard
  Brook deciduous < mixed. This remains a bounded within-model response, not
  independent amplitude validation or a reason to refit the frozen timing
  ensemble.
- The 13 observation-summary groups match the admissible nonzero score groups.
  Harvard comparisons use like-scale daily depth and bulk density only.
  Excluded, unbound, and missing observations remain distinct from measured
  zero, and no unsupported snow-agreement threshold or support verdict is
  introduced.
- Predictive needle and fine-woody sources remain
  `NULL_AUTHORITY_MISSING` in all 259 forest/member runs. Residue, frost, ET,
  runoff, and erosion cells remain `NOT_EVALUATED / NOT_ADVANCED`; erosion
  output remains `NULL_NOT_EMITTED`. Runtime lineage reaches the real canopy,
  snow, interception, ET, residue/frost, runoff, and erosion-input consumers
  without downstream residual fitting.
- All six SVGs parse as accessible images and embed current SHA-256 bindings
  for every declared source table. Independent checks reproduced those
  source digests. The renderer derives null and advancement labels from the
  verdict table and exposes the required canopy, snow, litter/residue/frost,
  downstream, and complete-cell views.
- Recorded focused contracts, runner tests, Markdown lint, deterministic
  analysis/render repeats, visual inspection, and diff hygiene are
  proportionate to this documentation-and-characterization package. No
  production Rust, canonical science contract, fixture, observation, or
  public schema changed.

## Conclusion

Review B is `PASS`. CAL-06 supports only
`COMPLETE / BOUNDED GRADIENT CHARACTERIZATION / DOWNSTREAM ADVANCEMENT
WITHHELD`. It does not validate canopy-amplitude operands, rehabilitate poor
Harvard timing transferability, fill CAL-05 predictive litter-source
authority, or authorize downstream compensation.

Package closure still requires the separately assigned Review A, disposition
of terminal review completion, both independent verifications, and final
closure reconciliation.

## Plot-only presentation follow-on re-review B

Status: `PASS`

Evidence class: `Ran: read-only validator, temporary-root deterministic
regeneration, visual inspection, Markdown lint, and diff hygiene + Static:
renderer, sidecar, source-binding, accessibility, and scientific-boundary
review`

The user-directed presentation follow-on supersedes the earlier statement that
null and advancement labels are rendered inside the plots. Those ancillary
labels now belong in the paired Markdown sidecars; the machine-readable
verdict matrix remains the quantitative authority.

- The figure directory contains exactly six canonical SVGs and six
  same-basename Markdown sidecars. The README, renderer, validator, and
  fulfilled figure contract agree on the paired inventory and names.
- Visual inspection of all six canonical SVGs confirms plot-only
  presentation: plotted series, axes, panel labels, legends, categorical cell
  status, and advancement values remain, while captions, interpretation,
  source-completeness prose, residual summaries, and gated-consequence prose
  are outside the plots. The seasonal categorical panels contain no
  irrelevant calendar tick labels.
- Each SVG retains `role="img"`, an `aria-labelledby` relationship, and
  non-visible `<title>` and `<desc>` accessibility content. Stratum identity
  is carried by labels and line patterns as well as color; categorical status
  is carried by text and symbols.
- Each sidecar contains a caption, the paired SVG name, ancillary information,
  and resolvable links to the exact plotted data, relevant interpretation
  tables or lineage artifacts, and deterministic renderer. Units,
  aggregation, site/stratum scope, ensemble role, and model-response boundary
  are stated on the plot or in the adjacent sidecar.
- The sidecars preserve the complete scientific boundaries:
  `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION` for Harvard SWE,
  `NOT_EVALUATED_SCALE_MISMATCH` for Harvard vertical density profiles,
  unbound mixed/hemlock comparison, `NULL_AUTHORITY_MISSING` for predictive
  needle and fine-woody sources, `NOT_ADVANCED` for gated downstream cells,
  and `NULL_NOT_EMITTED` for erosion output. None is presented as measured
  zero.
- Embedded SVG metadata rehashes to the exact plotted source table declared by
  the validator. A clean temporary package root using the current renderer and
  copied source tables reproduced all six canonical SVGs byte-for-byte.
- The terminal validator passes with 261 runs, 24,012 period operands, 95,526
  climatology rows, 783 observation scores, 13 verdict cells, six SVGs, and
  six sidecars. Package, figure-contract, roadmap, and work-package-catalog
  Markdown lint all pass with zero errors or warnings; `git diff --check`
  passes.
- The five manifest-bound scientific result objects retain their exact
  terminal hashes. The follow-on changes presentation artifacts and their
  validation only; canopy ordering, snow scoring, source-authority
  exclusions, missing/null semantics, downstream gates, and the package
  disposition are unchanged.

Follow-on re-review B is `PASS`. No presentation, accessibility,
source-binding, semantic, determinism, validation, or write-set finding
remains.

### Downstream legend-correction re-check B

Status: `PASS`

Harvard retains exactly the open, deciduous, and mixed data strata. The
downstream renderer requests exactly those three legend entries; the canonical
SVG contains their labels and series colors and contains no conifer label or
conifer series color. The sidecar states the same availability boundary, and
the validator now rejects a downstream conifer label or a missing available
Harvard stratum.

The canopy, snow, and litter/residue/frost SVG legends still contain all four
strata present across their plotted scopes; the seasonal categorical and
verdict-matrix figures have no line-series legend to regress. Visual
inspection and clean temporary-root regeneration pass, the terminal validator
and downstream-sidecar Markdown lint pass, and all five scientific result
hashes remain unchanged. Legend-correction re-check B is `PASS`.
