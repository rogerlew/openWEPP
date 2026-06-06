# Contract implementation evidence

Status: complete

Evidence mode: static

Static:

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-038` at contract version `43`.
- Added `SC-WATBAL-001#INV-WATBAL-086` at contract version `136`.
- Both invariants require split-route HPHYS0313 evidence before producer,
  branch-predicate, melt-term, WB13, WB17, WB18, WB19, or WB12 edits.
- Branch/depth authority now explicitly cites pinned-baseline
  `/workdir/wepp-forest_260430_baseline/src/snowd.for:166-172` for positive
  `hrsnow` snowing-branch depth addition and keeps
  `/workdir/wepp-forest_260430_baseline/src/snowd.for:145-146` available only
  for branch-gated no-snow drift evidence.
- Existing pinned-baseline source-line authority remains
  `/workdir/wepp-forest_260430_baseline` plus canonical `SC-*` contracts.

Ran:

- Not applicable; contract implementation evidence is static document review.
