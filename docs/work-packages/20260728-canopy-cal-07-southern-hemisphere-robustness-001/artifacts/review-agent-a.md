# CAL-07 Terminal Scientific Review A

Evidence class: `Ran + Static`

Recommendation: `PASS FOR THE HOLD DISPOSITION ONLY`

This is not a pass of Southern Hemisphere robustness, a completed CAL-07
scientific evaluation, or roadmap Order 7. It is a pass of the exact
fail-closed disposition:
`HOLD / FORCING AUTHORITY INCOMPATIBLE / NO CANOPY RESULT`.

## Evidence inspected

- Exact current package, frozen intent, source authority, execution incident,
  science summary, final disposition, roadmap/catalog edits, normalized
  inputs, custody tables, source and diagnostic manifests, executor and
  diagnostic tools, and all four SVG/Markdown figure pairs.
- `SC-PLANT-001` INV-PLANT-031 and OBL-PLANT-P-013, which require negative
  VPD to hard-fail and explicitly authorize neither a zero clamp nor bounded
  negative normalization.
- `tools/validate_hold.py`: passed with 3,332 forcing rows, exactly three
  negative-VPD days, exact source/custody checks, all required figure
  structures, and no prohibited result table.
- An independent temporary-path invocation of the package Rust executor:
  exited nonzero at `SH-EN-ALERCE 2022-07-22` and created no output file.
- An independent reconstruction from all retained raw forcing rows:
  reproduced only `2022-07-22`, `2022-09-15`, and `2025-09-09` as negative,
  at approximately -58.86, -70.49, and -1.00 Pa.
- Independent SHA-256 and byte-size checks of every source-manifest and
  diagnostic-manifest row: exact.
- All four SVGs parsed and rendered successfully. Each SVG has title,
  description, accessible role binding, and embedded source hashes; every
  same-basename sidecar contains the required caption, reading guide,
  plain-language takeaway, source binding, limitations, and accessibility
  sections.
- `markdown-doc lint` over the package, roadmap, and work-package catalog:
  18 Markdown files validated with zero errors and zero warnings.

## Findings

### CAL07-TRA-001 — No partial result or unauthorized forcing repair

No `daily-kernel-output.csv`, gate receipt, shape score, transition residual,
ensemble daily table, or verdict matrix exists. The executor evaluates the
contract equation from retained `Tmax`, `Tmin`, and `Tdew`, rejects the first
negative value before writing, and does not clip, adjust dew point, delete a
day, interpolate, omit a member/site, or publish a partial trajectory.

Disposition: `PASS`.

### CAL07-TRA-002 — Source authority and proxy claims are calibrated

The package correctly describes Beza and Alerce as geographically and
climatically independent site assignments, not independent measurement
methods. It preserves provisional PhenoCam status, distinguishes V3 as
method lineage from the later provisional files, retains the live-license
versus embedded-stale-license conflict, and limits raw GCC90 to a relative
camera-greenness proxy. It does not promote GCC90 to LAI, foliar mass, canopy
cover, or quantitative evergreen floor. NASA POWER is consistently described
as gridded/reanalysis forcing, not site meteorology; the material Alerce
elevation mismatch is visible.

Disposition: `PASS`.

### CAL07-TRA-003 — The execution incident supports an input hold, not a model verdict

The three negative reconstructions are an input-domain incompatibility under
the current contract. One occurs in the planned 2025 scoring window, so the
incident cannot be narrowed to disposable warm-up. Timing, relative shape,
transition chronology, mass closure, and consumer chronology are therefore
correctly `NOT_EVALUATED`; the record does not call the phenology model
supported or contradicted. The two focused tests correctly did not run after
the preceding executor gate failed, and no test receipt is implied.

Disposition: `PASS`.

### CAL07-TRA-004 — Figures preserve the evidence boundary

The observational-lane figure shows only admitted raw GCC90 and visually
breaks longer missing intervals. The two forcing figures bind the complete
diagnostic and exact failing operands. The status figure makes the dependency
chain explicit: admitted sources remain retained, forcing failed, execution
was blocked, result cells are unevaluated, and Order 7 is withheld. The
figures and sidecars do not contain a canopy result or conceal the three
negative days.

Disposition: `PASS`.

### CAL07-TRA-005 — Roadmap and catalog remain truthful

The roadmap and work-package catalog describe CAL-07 as executed only through
the fail-closed input boundary, retain `NO CANOPY RESULT`, keep Order 7 open,
and name the two legitimate resume routes: contract-admissible continuous
forcing or prior explicit contract authority for bounded canonicalization.
They also preserve the still-open amplitude/evergreen-floor and
phase-transformed-real-consumer gaps.

Disposition: `PASS`.

### CAL07-TRA-006 — Artifact index wording is stale

`artifacts/README.md` says the directory retains daily kernel outputs,
result summaries, and verdicts. Those artifacts intentionally do not exist
after the fail-closed incident. The final disposition is unambiguous, so this
does not weaken the scientific hold, but the index should be changed to
describe the retained diagnostic-only artifact set and explicitly say that no
daily/model-observation result was published.

Disposition: `MINOR DOCUMENTATION CORRECTION REQUESTED`.

## Terminal assessment

The exact retained evidence supports only a scientifically conservative hold.
Subject to disposition of `CAL07-TRA-006`, the package is suitable to retain
and publish as a blocked CAL-07 attempt. It must not be labeled complete, used
to advance Order 7, or resumed by locally repairing the three VPD values.

## Final correction re-review

Evidence class: `Ran + Static`

Superseding recommendation: `PASS FOR THE HOLD DISPOSITION ONLY`

`CAL07-TRA-006` is accepted and corrected. The artifact index now names the
retained diagnostic-only evidence and explicitly states that no daily kernel
output, model-observation score, or scientific verdict matrix was published
after the forcing gate failed. `artifacts/finding-disposition.md` records the
correction without changing any scientific result or claim.

The six prohibited result artifacts remain absent, and both corrected
Markdown files pass `markdown-doc lint` with zero errors and zero warnings.
No Terminal Review A finding remains open.

The final ceiling is unchanged:
`HOLD / FORCING AUTHORITY INCOMPATIBLE / NO CANOPY RESULT`. This review does
not pass Southern Hemisphere robustness, complete CAL-07, or advance roadmap
Order 7.
