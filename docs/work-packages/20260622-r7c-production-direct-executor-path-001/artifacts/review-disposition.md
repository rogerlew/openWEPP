# Review Disposition

Status: complete.

Evidence class: Static + Ran.

## Review A

Scope: production direct executor routing and manifest evidence.

Findings:

- A1: Direct production must not accidentally run the direct skeleton first.
  Accepted and fixed. `select_direct_runtime_skeleton_once` now returns for
  `DirectProductionExecutor`, and the R7C fixture manifest verifies
  `skeleton_runs=0`.
- A2: Direct production must not start compatibility diagnostic adapters as a
  common pre-branch side effect. Accepted and fixed. Symbol-registry audit and
  indexed-shadow setup are skipped when `DirectProductionExecutor` is selected.
- A3: Direct production counters must be manifest-visible and run-local.
  Accepted and fixed. `direct_runtime_counters_for_manifest` includes direct
  production, and both focused fixture and H2637 manifests record nonzero
  direct counters with `compatibility_edge_invocations=0`.

## Review B

Scope: package boundary, validation, and residual-risk disposition.

Findings:

- B1: R7C must not claim output parity or default activation. Accepted. The
  package, timing artifact, architecture spec, and worker handoff now state
  that HBP/PASS/WAT checksums differ and R7D/R7E-R7H remain open.
- B2: The direct production path is slower and uses much more RSS than default
  compatibility on H2637. Accepted as follow-up. Same-binary evidence records
  `753.76 s / 625132 KB` direct production versus `642.77 s / 228804 KB`
  default compatibility. R7G must profile and remediate this before any
  activation package.
- B3: Static no-compatibility proof must inspect the direct production branch,
  not only rely on manifest counters. Accepted and covered.
  `r7c_direct_production_source_excludes_compatibility_entrypoints` inspects
  the direct production function body for forbidden compatibility entrypoints.
- B4: Line-count governance must remain below the `2000` line warning
  threshold after adding R7C tests and execution code. Accepted and verified.
  Current counts are `03_tests.rs=1970`,
  `04_direct_publication.rs=1932`,
  `05_runner_execution_and_outputs.rs=1525`, and
  `00_core_frames.rs=1586`.

## Finding Disposition

Accepted/fixed in R7C:

- A1 direct skeleton exclusion.
- A2 diagnostic adapter exclusion for direct production selection.
- A3 manifest-visible direct production counters.
- B3 direct-production source scan.
- B4 line-count governance.

Accepted/follow-up:

- B1 output parity and default activation are deliberately outside R7C and must
  be handled by R7D/R7E-R7H.
- B2 direct production performance and RSS are not acceptable for release;
  R7G owns profiling and remediation before activation.

Rejected:

- None.

Deferred:

- None beyond the named accepted/follow-up items above.
