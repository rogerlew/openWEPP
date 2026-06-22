# R6J No-Premature-Stop Audit

Evidence class: Static plus Ran.

## Audit

- Fail-closed marker reduced beyond symptom: yes. The inherited
  `manifest direct projection is not wired` marker was reproduced, then traced
  to production manifest provenance still using compatibility WB13 publication
  provenance. R6J wired direct manifest provenance from retained
  `DirectRunPublicationFrame`.
- Parity mismatch reduced beyond symptom: yes. Enabling PASS parquet exposed
  `year,peakro`; R6J reduced that to simulation-year numbering and direct
  erosion peak-runoff source selection, then fixed both.
- Producer / consumer / authority traced: yes. The blocker ledger records
  direct operands, producers, consumers, and authority for manifest, PASS, and
  production gate compatibility-oracle removal.
- Manifest writer wiring status: closed. Direct manifest provenance reports
  `direct-publication-frame`, empty replay candidates, direct row keys/counts,
  direct runtime counters, and output checksums for all public outputs.
- PASS/loss/WAT/HBP fixture status: closed for current R6J scope. Focused
  parity evidence covers HBP bytes, WAT rows, PASS rows, and loss JSON. CLI
  cutover writes HBP/loss/PASS/WAT/plot/manifest. A targeted two-OFE direct
  frame test covers multi-OFE direct row/provenance shape.
- Comparison and reconstruction helper status: closed. Compatibility
  comparison helpers now live in test-only evidence; production cutover gates
  validate direct artifacts only.
- Direct producer status: closed. H2637 direct cutover produced `235961`
  direct rows, all public outputs, and zero compatibility-edge invocations.
- New blockers iterated instead of deferred: yes. R6J iterated through manifest
  wiring, PASS `year/peakro`, production compatibility-oracle removal, and
  direct manifest counter wiring.
- Output-family parity / anti-alias / reconstruction status: closed. Focused
  current-fixture tests cover direct reconstruction and alias reductions.
  Fresh H2637 same-binary release runs prove HBP/WAT/PASS/loss/plot byte
  identity; WAT and PASS have zero bidirectional DuckDB row differences.
- Full R6 closure evidence: complete. `DirectPublicationFrameCutover` is
  direct-only, default-disabled H2637 performance passes, H2637 opt-in cutover
  passes public-output byte/Arrow identity, and root Rust gates passed.
- HOLD legitimacy, if applicable: not applicable. No remaining in-envelope R6J
  blocker is deferred.
