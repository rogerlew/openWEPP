# R full-release failure comparison — review B

Ran: read-only parsing of completed A/F/R full logs, SHA256 verification,
binary64 vector comparison, and six existing R child stderr hashes. Static:
source/authority inspection against the preserved R-source-04 archive. B ran
no build or Rust test and made no source edits. Root/work-package instructions
were discovered for this artifact.

## Findings first

- **No new failed test name or runtime error family was detected in R relative
  to A or F.** All 178 R failing names also fail in both comparison arms.
  This is a bounded relative result, not full correctness PASS.
- **Medium — full gate remains FAIL:** retained authority/source guards,
  runtime/custody/model failures, generated-reference compilation errors and
  release golden nonconformance are not waived by matching A.
- **Medium — complete natural-limit diagnostics remain a separate gate.**
  All three printed fifteen-entry actual vectors match literally and in every
  binary64 bit, but the golden assertion aborts before later diagnostic
  assertions. The common observer is not part of this old frozen cut.
- **Low — historical revision-31 source seam reports a different missing
  list, not a new runtime defect.** Its single-file search misses actual R
  module definitions and still requires three historical audit API names not
  introduced by the prospective implementation. Preserve the old FAIL.

## Exact executed cut and inventory

`raw/R-full-release-01.log` is 365912 bytes; SHA256
`0bfb1ba5d5f6de852530764fede9d73acaa01ac5c709a73498081088d5ba53df`,
matching its execution metadata. Exit 100; wrapper 1100.101936368 seconds.
The command is the unchanged canonical full profile:

```sh
nix develop --offline -c env CARGO_PROFILE_RELEASE_LTO=false \
  cargo nextest run --release --workspace --profile full
```

Overrides: `RUST_MIN_STACK=67108864`, `CARGO_NET_OFFLINE=true`; no external
timeout. Cwd is `/tmp/openwepp-controlled-mechanisms-Hb6uS2/R`, checkout A
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3` plus archived R-source-04:

- name: `R-PC1-SG1-N2-corrected`;
- source identity: `a1c128139825c70f48c1df86504f344fb9a0aff082a19645a409a5a64f3bb157`;
- patch SHA256: `57198bd0a55a179d47fc6fb4b95f96b9261926e1a040b6f2268f3a6ad4329261`.

This predates the R lint correction and new common natural-limit observer;
neither later cut inherits full-run validation automatically.

| Arm | Run | Passed | Failed | Skipped | Test seconds |
| --- | ---: | ---: | ---: | ---: | ---: |
| A full-release-01 | 4202 | 4022 (2 slow) | 180 | 75 | 513.757 |
| F full-release-01 | 4207 | 4005 (1 slow) | 202 | 75 | 149.392 |
| R full-release-01 | 4206 | 4028 (2 slow) | 178 | 77 | 549.998 |

R's log explicitly starts 4206 tests across 246 binaries, with 77 tests and
five binaries skipped, including 28 tests/five binaries via the unchanged
full default filter. Its extra skipped tests are not execution evidence.
Passing names are not all printed by this status configuration; do not infer
individual new-test PASS records solely from aggregate count arithmetic.
These suite durations are not controlled mechanism timing measurements.

Comparison log SHA256 values:

- A: `5f4c279602e193c305048e7613337c688728daf7dc87124abbe44083c892917e`;
- F: `13c1f3d6ebed0a36e38debc33cc4da2dfbb41376fcd985dc7aa7bf596bf6fb6e`.

## Exact failed-name and error-output classification

Deduplicated immediate/final failure presentations by full binary/test name.
R adds zero failing names relative to either A or F. The two A-only names are:

- `laned_shadow_h2637::h2637_default_malformed_routing_coefficients_fails_closed`:
  A stopped while flat-copying a fixture directory; clean worktree execution
  reaches the intended guard. Not an R physics improvement.
- `stage3_native_vegetation_laned_throughput_recovery::stage3_lane_d_qualification_reads_canonical_manifest_provenance`:
  the separately reviewed common integration-only source-reading correction
  follows the extracted authentic helper and preserves four required reads.

The 24 F-only failures are exactly the previously classified 23 worktree
tooling failures plus the F-specific stale resolved-provider spelling guard.
None is newly present in R. See F-full-failure-review-b for their full names;
this contrast does not attribute a tooling difference to replay.

Compared all 178 R stderr bodies, not merely names. After only worktree prefix,
thread PID and source-line relocation normalization, 170 match A exactly and
170 match F exactly. The remaining eight in each comparison are fully
accounted for:

1. Historical revision-31 structural seam, detailed below.
2. Six nested watershed failures differ in unique scratch paths only. Read
   every actual child stderr; all six are byte-identical to A/F retained
   failures. Five are 337 bytes, SHA256
   `1c56b7717a09235f0f6b6e1fcdb4f5740c778123e1526a46f0ada1496309ca1a`:
   wshedw2 relative/serial, wshedw3 jobs1, wshedw7 relative, wshedw7r p102.
   They reject forest litter entering the Stage-3 open-snow solver at the
   same 0..1800000000000 ns interval. Wshedw10 is 240 bytes, SHA256
   `46e458f1c1f3f42739fe79a608b5883912d1261624a5c5e79b1c2d71c1246335`,
   rejecting the same pending V11 parent-finalization record source. Exact R
   source paths remain in the raw log under `/tmp/openwepp-roger-R-344ae65295ff/`;
   identical bytes already have custody in raw/baseline-child-failures.
3. Versus A, `h2637_default_mixed_routing_coefficients_fails_closed` reaches
   the same error as F: eighteen lanes with coefficients and one without;
   the test expected Stage3 seed refusal first. A failed earlier at fixture
   copy. This is the already-exposed inherited guard-order expectation, not
   a new R numerical failure.
4. Versus F, generated restart reference stderr has A/R's inherited single
   `ending_snow_hint` warning rather than F's then-new two-field warning.
   Both actual compiler errors remain identical E0308 at evidence_fixture:163
   and E0599 at :404, caused by the unchanged Result-returning soil snapshot
   accessor. They are not a new R production compile failure.

Other inherited failures retain their actual support intervals, guards,
budgets, identities and error values. This includes the p102 native V3
solver-ready projection failure and the missing native-forest canopy trace;
neither is attributed to new R observation merely because its test fails.

## Historical source-seam distinction

`land_surface_energy_balance_authority_contract.rs:875` reads only
`solver_covered_solve.rs` and expects all seven old names to be unconditional
top-level items in that one string. A/F report seven missing names; R reports
six, correctly finding the actual sweep base at archived solve.rs:271.

Three other implemented items are elsewhere in archived R-source-04:

- graph.rs:11: `CoveredComponentTemperatureDependencyGraph`;
- replay.rs:143: `ValidatedCoveredComponentProbeReplay`;
- replay.rs:303: `covered_component_temperature_probe_residuals`.

The three old audit names are genuinely not implemented under those spellings;
the reviewed prospective treatment uses the common audit API. Do not claim
all six messages are false positives, fabricate aliases, or mark this old
structural test PASS. The prospective authority explicitly preserves
historical revision-31 FAIL/HOLD results and requires separate actual graph,
capability, arithmetic, first-error, lifecycle and real-consumer proof. This
is the historical test's changed observation, not a new failed name or a
runtime proof that replay is absent. No source-guard edit is proposed here.

## Same-release natural-limit vector

Extracted both actual and expected fifteen-element vectors from every arm's
completed log. A/F/R actual arrays are literal-identical and each parsed f64's
eight bytes match exactly; their expected arrays also match each other.
R still differs from the declared golden exactly as A/F do. The actual vector
is retained in F-full-failure-review-b and each raw log; no tolerance or
normalization was applied.

All three reach the same Rejected/IterationLimit metadata, 50 iterations and
340 backtracks before the failing residual-vector assertion. Subsequent
pivot/matrix/ordered-row assertions do not execute. Thus this full run proves
the printed-vector relative match, not the complete failure DTO, actual input
rollback or Newton trajectory. The newly reviewed common observer and its
three-arm execution/comparison remain separate required evidence.

## QA disposition

The bounded R-source-04 failure classification is complete: no new failing
name or runtime failure family found, with changed source-guard/environment
messages explicitly resolved. Full correctness remains **FAIL**, not waived
or promoted. New source cuts require their own applicable validation; common
diagnostic parity, lint corrections/target coverage, detailed mechanism and
scientific admission gates remain with the parent's active workflow.
