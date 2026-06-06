# Contract implementation evidence

Status: complete

Evidence mode: static

Static:

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-038` at contract version `42`.
- Added `SC-WATBAL-001#INV-WATBAL-086` at contract version `135`.
- Both invariants require split-route HPHYS0313 evidence before producer,
  branch-predicate, melt-term, WB13, WB17, WB18, WB19, or WB12 edits.
- Drift/depth authority now explicitly cites pinned-baseline
  `/workdir/wepp-forest_260430_baseline/src/snowd.for:145-146` for cold
  no-snow `driftg` final-depth addition.
- Existing pinned-baseline source-line authority remains
  `/workdir/wepp-forest_260430_baseline` plus canonical `SC-*` contracts.

Ran:

- Not applicable; contract implementation evidence is static document review.
