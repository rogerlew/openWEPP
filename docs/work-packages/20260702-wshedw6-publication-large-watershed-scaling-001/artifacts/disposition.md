# Disposition

Status: `EXECUTED-COMPLETE`

Evidence mode: `Ran:`

WSHED-W6 is complete.

Closure summary:

- public watershed publication now consumes `WatershedPublicationFrame`
  directly through `write_typed_publication_parquet_outputs`;
- no public CLI compatibility row-seed publication path remains;
- unavailable typed publication operands emit nulls instead of surrogate
  physics values;
- area publication uses committed source slope geometry in generated mode and
  validated manifest `publication_area_m2` in existing-pass manifest mode;
- full committed `onshore-xenophobia` (`1305` hillslopes) and
  `carnivorous-adobo` (`32` hillslopes) fixtures ran without subsetting;
- `--jobs 1` and parallel outputs matched by schema and row content for all
  `14` required watershed parquet outputs;
- pinned legacy same-fixture full runs completed for both fixtures;
- final gates passed as recorded in `gate-results.md`.

No hold boundary remains in W6.
