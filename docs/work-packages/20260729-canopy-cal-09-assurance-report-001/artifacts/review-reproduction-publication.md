# Reproduction And Publication Review

Status: `SOURCE-LEVEL PASS / CANONICAL ADMISSION BLOCKED`

Evidence class: `Ran — independent internal agent review and re-review`

The reviewer independently reconstructed all material values and assessed
identities, units, deterministic builds, tables, figures, accessibility,
portable links, research-object completeness, public-boundary integrity, and
audience fit.

## Findings And Closure

1. `CRITICAL`, confirmed external package blocker: no typed operation admits a
   new V2 report. The initially proposed catalog row invalidated the catalog
   identity without admitting the report and was removed. Existing admitted
   reports validate; canopy-specific validation returns an unknown report ID.
2. `HIGH`, accepted and corrected: exact-use initially omitted most research
   objects, every reference, and one value. Exact-use now passes for 32 values,
   one table, one native figure, 12 references, and 39 research objects.
3. `HIGH`, accepted and corrected: figure inputs contained absolute paths and
   incomplete research-object coverage. All nine manifest inputs are now
   repository-relative and hash-matched; every plotted raw and derived file is
   a public-safe object.
4. `MEDIUM`, accepted and corrected: manually duplicated claim-bearing
   numbers were replaced by strict value directives.
5. `MEDIUM`, accepted and corrected: main time-series SVGs lacked adjacent
   accessible alternatives. Each selected figure now links its Markdown
   caption, data description, and limitations; the supplement inventories all
   figures, source rows, and sidecars.
6. `MEDIUM`, accepted and corrected: structured references were missing and
   the Keane citation was inconsistent. Both narrative files now render all 12
   declared references through typed directives.
7. `MEDIUM`, accepted and corrected on re-review: the figure-build record did
   not identify its non-standard-library environment and the manuscript could
   imply figures were standard-library-only. The record now identifies
   CPython 3.12.3, Matplotlib 3.10.8, NumPy 2.4.6, and Pandas 3.0.3; the
   manuscript limits the standard-library statement to strict-result
   reproduction.

## Independent Checks

- Fresh seven-input strict reconstruction byte-matched the retained result at
  SHA-256
  `515344ded0cc73b344cc40f7972439a2036adef39401443c432f79f72d605dba`.
- A separate implementation independently matched all 32 semantic values.
- All 39 object paths are confined regular non-symlinks with unique basenames
  and no absolute workspace content.
- All eight SVGs match frozen hashes and parse as XML.
- Direct report, result, and catalog schemas; Markdown; American English;
  paths; references; and protected-public hashes pass.
- `validate --all` passes for both existing admitted reports.

## Verdict

Source-level reproduction/publication preparation: `PASS`, with no open
source-level finding after correction.

Canonical canopy admission/build: `BLOCKED`. The report is correctly absent
from the catalog, the identity lock contains no canopy member, and the CLI has
no typed admission operation. This internal review is not accountable human
reproduction/publication approval.
