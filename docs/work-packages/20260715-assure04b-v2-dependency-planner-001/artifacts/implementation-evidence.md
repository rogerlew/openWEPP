# ASSURE-04B Implementation Evidence

Status: implementation complete; review and independent heavy closure PASS

Evidence classes: Static and Ran

## Consumer Path

Static: `openwepp-assurance plan` first validates the protected ASSURE-03
zero-public source, opens `V2Repository`, and calls `plan_report` or `plan_all`.
Both repository methods call the same `planner::plan_sources` and per-report
graph builder. Human and JSON output are renderings of the returned `V2Plan`;
neither CLI format has a separate traversal.

Static: report-specific `build` and `check` still reject the selection and name
ASSURE-04C. No production function writes from a v2 plan. The only registered
new integration consumer is `assurance_v2_planner_contract`.

## State And Graph Behavior

Static: local manifest, manuscript, supplement, result, public research-object,
and local dependency nodes compare observed bytes with declared SHA-256.
Immutable external and restricted dependencies retain their declared identity.
Embedded records receive a deterministic identity bound to the observed
manifest and a direct manifest edge.

Static: dependency-first topological sorting uses lexical IDs among ready
nodes. Intrinsic stale/blocked states are retained; a blocked prerequisite
blocks its consumer; a stale or selected prerequisite selects its consumer.
Cycles, missing destinations, duplicates, and nodes unreachable from the report
target return typed errors.

Static: a parseable stale manifest remains plannable after structural authority
checks. An unreadable/unparseable manifest produces a bounded blocked report
plan. Unsafe paths, malformed digests, invalid logical relations, unused
records, and invalid current result JSON remain errors rather than statuses.

## Ran Evidence

- Pre-implementation `cargo nextest run --test
  assurance_v2_planner_contract` failed at compile time only for the absent
  `V2Plan*` types and `V2Repository::{plan_report,plan_all}` methods.
- `cargo nextest run -p openwepp-assurance --no-fail-fast`: 6/6 pass, including
  graph-core cycle/missing/unused and propagation tests plus descriptor-
  replacement confinement tests.
- `cargo nextest run --test assurance_v2_planner_contract --test
  assurance_v2_source_contract --test assurance_dossier_build_contract
  --no-fail-fast`: 35/35 pass after review remediation.
- `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`: pass.
- Post-remediation `cargo nextest run --workspace --profile quick`: 1,916/1,916
  pass, 34 profile skips; the same three routing-oracle tests exceeded 120
  seconds and passed.
- Real human and JSON CLI plans both report one current internal report, zero
  public reports, and source root
  `2917f1226fe586cb2c0a1498c627e6049494b104560e6fc74aec893b74cb6b60`.
- Status-before/status-after comparison around both real plan formats is empty:
  ordinary planning writes no tracked or untracked repository file.
- Terminal independent heavy closure passed all five required gates: full
  Nextest 2,001/2,001 with three skipped; fresh CRAP raw/adjudicated/actionable
  2/2/0; every touched-production-file maximum at or below 26.

## Scope Boundary

Static: no v2 source/schema, public assurance source, `usersum`, export,
snapshot, kernel, science-contract, or vendoring path changed. No plan state is
presented as scientific evidence or report approval.
