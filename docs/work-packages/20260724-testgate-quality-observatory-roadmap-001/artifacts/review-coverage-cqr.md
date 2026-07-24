# Merged Coverage And CQR Review

Evidence class: `Ran` (reviewer-reported command evidence)

Reviewer role: independent merged-coverage/CQR subagent

## Findings

1. `HIGH`: at head `4b3e5435b1831c2a8a7d021c2dae879c18a6cd17`,
   `cargo nextest list --workspace --profile <profile> --message-format json`
   selected 2,263 nonignored `full` tests
   and 36 `science-manual` tests, with disjoint intersection and a 2,299-test
   union at the reviewed head.
2. `HIGH`: collection must clean once, source one LLVM environment, execute
   ordered profiles into one instrumented root, and emit LCOV once after both;
   concatenating post-hoc LCOV is not equivalent.
3. `HIGH`: current evidence does not bind ordered profiles, inventory/JUnit
   identities, union identity, or the complete report lineage.
4. `HIGH`: CQR Nightly and its templates mandate redundant recollection.
5. `MEDIUM`: compact publication must use an allowlist; target/profraw/raw
   reconstruction trees must stay local and CQR can select from compact
   actionable CRAP data.

Initial recommendation: derive `quality_evidence_id` from a canonical manifest,
require exact-head currency for CQR intake, and recollect only after a typed
stale/invalid disposition.
