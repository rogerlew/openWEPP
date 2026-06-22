# R6J Review Disposition

Evidence class: Static plus Ran.

Two existing subagents performed read-only review after the first R6J cutover
implementation. Both reviews found blocking issues. The findings were accepted
except where the proposed remedy would have reintroduced compatibility as a
production oracle; in that case the blocker was resolved through test/evidence
gates and direct production guards.

## Review 1 - Correctness

Reviewer: Socrates.

Findings:

- Manifest `publication_area_m2` was day-multiplied because direct provenance
  summed lane-day rows.
- Manifest direct runtime counters were global snapshots, not run-local deltas.
- Missing per-family producer authority could still write defaulted HBP/PASS
  erosion fields.
- Direct manifest provenance leaked into `DirectPublicationFrameShadow`.
- Evidence artifacts were still pending.

## Review 2 - QA / Maintainability

Reviewer: Avicenna.

Findings:

- Production cutover no longer enforced byte/Arrow parity before writes.
- Evidence artifacts were still pending.
- Manifest direct runtime counters were global snapshots.
- Disk-level WAT/PASS readback and checksum proof were weak.
- Line-count governance was pending.

## Finding Disposition Table

| Finding | Disposition | Action |
| --- | --- | --- |
| Manifest area day-multiplied | accepted / fixed | Direct manifest provenance now sums one stable area per OFE and rejects inconsistent repeated row areas. `r6j_direct_manifest_provenance_accepts_multiofe_direct_rows` asserts `publication_area_m2 = 1200.0` for a two-OFE/two-day frame. |
| Global direct runtime counters | accepted / fixed | Manifest counters are computed as a run-local delta between start/end audit snapshots. `r6j_manifest_direct_runtime_counters_are_run_local_after_prior_activity` dirties global counters before cutover and verifies manifest counters stay run-local. |
| Missing direct producer authority/defaulted erosion fields | accepted / fixed for current output contract | Direct rows now carry explicit zero-authority erosion publication operands for the current engine output contract, and direct HBP/PASS consumers fail on missing/non-finite/non-negative erosion operands instead of silently defaulting absent fields. |
| Shadow manifest leaked direct provenance | accepted / fixed | `build_hillslope_publication_provenance` takes `HillslopeRuntimeSelection`; direct manifest provenance is used only for `DirectPublicationFrameCutover`. Shadow mode remains compatibility-provenanced. |
| Production parity oracle removal | accepted with architecture disposition | The production cutover writer remains direct-only and does not rebuild compatibility outputs. Parity is enforced by focused tests, CLI evidence, and H2637 scale gates. The H2637 scale blockers exposed after review were iteratively closed in R6J. |
| Disk readback/checksum proof weak | accepted / fixed | Runner tests recompute manifest checksums against written files and write/read back direct and compatibility WAT/PASS Parquet batches for schema/value equality. |
| Evidence artifacts pending | accepted / fixed | Verification, no-premature-stop, line-count, output parity, no-compatibility, and worker-handoff artifacts were populated with final evidence. |
| H2637 endpoint/RSS evidence absent | accepted / fixed | Ran release build, three H2637 default-disabled reps, one initial opt-in direct-cutover H2637 endpoint/RSS run that exposed blockers, then fresh same-binary default/direct H2637 runs proving public-output byte identity. |

## Review Verdict

The accepted implementation findings were fixed and verified. The
post-review H2637 opt-in endpoint exposed additional scale blockers, and R6J
continued instead of stopping: PASS was reduced to outlet-only direct public
projection, HBP to HBP-specific producer-authoritative erosion operands, and
PASS byte instability to nondeterministic Arrow schema metadata. Fresh
same-binary H2637 default/direct runs now prove byte identity for all public
outputs and zero compatibility-edge invocations.

Review disposition is satisfied for `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`.
