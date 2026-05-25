# replay-implementation-wp-queue

Status: phase-d-complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Queue sequencing is derived from SIMIMPL13 residual, span, comparability,
  tooling, and test blind-spot artifacts.
- Every code-authoring package below must execute contract-first internally:
  1. contract amendments,
  2. contract-derived tests,
  3. pre-implementation gate,
  4. production edits.

## Ran
- Inputs used to construct this queue:
  - `simimpl13-replay-parity-residual-consolidation.md`
  - `simimpl13-pipeline-timeseries-span-audit.md`
  - `simimpl13-candidate-surface-comparability-gap-register.md`
  - `simimpl13-comparator-tooling-gap-audit.md`
  - `simimpl13-contract-test-blind-spot-assessment.md`
  - `simimpl13-replay-parity-full-closure-criteria.md`
  - `simimpl13-continuous-simulation-run-gap-assessment.md`

## Proposed queue
| order | wp_id | primary gaps | objective | depends_on | exit signal |
|---|---|---|---|---|---|
| 1 | `20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001` | `SIMIMPL13-SPAN-001`, `SIMIMPL13-SPAN-002`, `SIMIMPL13-SPAN-003`, `SIMIMPL13-COMP-001`, `SIMIMPL13-COMP-002`, `SIMIMPL13-CONT-001`, `SIMIMPL13-CONT-002`, `SIMIMPL13-CONT-003`, `SIMIMPL13-CONT-004`, `SIMIMPL13-CONT-005` | Implement continuous day-indexed runner execution (forcing progression + carried writeback state), publish replay-length WB13 candidate trajectories, and align candidate key semantics with baseline comparator expectations. | SIMIMPL13 | Candidate dat/parquet surfaces reflect executed multi-day span with monotonic day indexing and promotable key overlap evidence (`common_row_count>0`). |
| 2 | `20260525-simimpl15-replay-comparator-tooling-alignment-001` | `SIMIMPL13-TOOL-001`, `SIMIMPL13-TOOL-002`, `SIMIMPL13-TOOL-003`, `SIMIMPL13-TOOL-004`, `SIMIMPL13-COMP-003`, `SIMIMPL13-COMP-005` | Close parquet alias drift, strict/semantic lane policy asymmetry, and provenance classification gaps in legacy comparison suite. | SIMIMPL14 (or parallel for mapping-only work) | Semantic parquet lane uses canonical columns with no false missing-field drift; strict lane policy is explicit and test-enforced. |
| 3 | `20260525-simimpl16-replay-contract-derived-test-coverage-closure-001` | `SIMIMPL13-TEST-001`..`SIMIMPL13-TEST-005` | Add contract-derived tests for span overlap, key-domain semantics, parquet alias continuity, and conversion-vs-native candidate provenance guards. | SIMIMPL14, SIMIMPL15 | New tests fail on current residual modes and pass after closure implementation. |
| 4 | `20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001` | All remaining | Re-run strict + semantic replay with updated runner/tooling and produce hold-lift disposition against SIMIMPL13 closure criteria. | SIMIMPL14, SIMIMPL15, SIMIMPL16 | `CRIT-001`..`CRIT-007` pass and disposition can move from `HOLD` to `GO`; otherwise keep explicit blocker ownership. |

## Continuous-run addendum note
- `SIMIMPL13-CONT-006` and `SIMIMPL13-CONT-007` should be closed as part of
  SIMIMPL14/SIMIMPL16 acceptance criteria:
  - run-span truthful auxiliary outputs,
  - continuity assertions in manifests/tests.

## Parallelization posture
- `SIMIMPL15` mapping/test scaffolding work can begin in parallel with
  `SIMIMPL14` if production-surface assumptions are clearly version-pinned.
- `SIMIMPL16` should start after initial `SIMIMPL14/15` deltas land to avoid
  stale test expectations.

## Queue conclusion
- SIMIMPL13 remains an assessment package with disposition `HOLD`.
- Queue above defines the smallest actionable path to promotable replay/parity
  closure.
