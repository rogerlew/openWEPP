# R6J No-Compatibility Proof

Evidence class: Static plus Ran.

## Proof

- Producer sources: `DirectPublicationFrameCutover` builds a typed
  `DirectRunPublicationFrame` through the cutover simulation-output adapter.
  The adapter consumes simulation-owned WB13 publication rows, climate span
  calendar state, parsed static inputs, and producer-authoritative runtime
  scalars, then projects them into typed direct publication operands before any
  public writer runs. The production direct writer does not read or rebuild
  compatibility HBP/loss/WAT/PASS artifacts as publication authority.
- In-memory direct frame / projection objects: HBP bytes, WAT rows, PASS rows,
  loss text, and manifest provenance are built from
  `DirectRunPublicationFrame` and retained in `DirectPublicationArtifacts`.
- Runner handoff: `write_hillslope_run_outputs` dispatches
  `DirectPublicationFrameCutover` directly to
  `write_hillslope_direct_publication_outputs`; that writer takes only
  `inputs`, `targets`, and `execution.direct_publication`.
- Downstream consumers: the direct writer writes `artifacts.hbp_bytes`,
  `artifacts.loss_text`, `artifacts.wat_rows`,
  `artifacts.pass_projection_rows`, and direct manifest provenance. It no
  longer builds compatibility HBP/loss/WAT/PASS inside the production cutover
  gate.
- Manifest selection: direct manifest provenance is selected only for
  `DirectPublicationFrameCutover`. `DirectPublicationFrameShadow` may build
  direct artifacts for evidence, but it keeps compatibility publication
  provenance because public outputs remain compatibility-authored.
- Forbidden source scans: static scan after correction shows
  `build_hbp_output`, `build_loss_output_json`,
  `build_hillslope_wat_rows(&execution.wb13_rows)`, and
  `execution.pass_rows` remain in the compatibility-mode output writer and test
  evidence only. The direct cutover writer and manifest direct provenance path
  do not call those sources.
- Test-backed proof:
  `r6j_cutover_candidate_writes_direct_outputs_and_manifest` asserts
  `compatibility_edge_invocations = 0`, `skeleton_runs = 0`, direct
  publication artifacts were built, all public outputs were written, and manifest
  `/direct_runtime_counters/compatibility_edge_invocations = 0`.
  The CLI contract repeats the public-output and manifest-counter assertions
  through `--direct-publication-frame-cutover`.
- Run-local counter proof:
  `r6j_manifest_direct_runtime_counters_are_run_local_after_prior_activity`
  proves manifest counters are deltas, not global snapshots. H2637 opt-in
  direct cutover also recorded all direct runtime counters at `0`, including
  `compatibility_edge_invocations = 0`, at `235961` direct rows.
- Residual compatibility references and why they are comparison-only:
  `r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss` deliberately
  builds compatibility HBP/WAT/PASS/loss artifacts as test evidence. The
  mismatch reducers are now `#[cfg(test)]`; they are not compiled into the
  production library. Runtime-surface reads in the cutover adapter are
  producer-authoritative scalar reads used to populate typed direct operands,
  not compatibility public-output artifact reads.

## Final Cutover Proof

No-compatibility authority is proven for the cutover path. Fresh H2637
same-binary release runs prove that direct cutover writes HBP, WAT parquet,
PASS parquet, loss JSON, and plot parquet byte-identically to the default path
while the direct manifest reports `direct-publication-frame`, empty replay
candidate surfaces, direct output checksums, direct row count `235961`, and
`compatibility_edge_invocations = 0`. WAT and PASS also have zero
bidirectional DuckDB `EXCEPT ALL` row differences.
