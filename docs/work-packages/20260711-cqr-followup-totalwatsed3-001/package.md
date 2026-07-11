# FQ-04 totalwatsed3 cover-then-decompose closure

Package: `20260711-cqr-followup-totalwatsed3-001`
Status: `ACTIVE`
ExecPlan: `docs/work-packages/cqr-nightly-followup-burndown-execplan.md`
Queue item: `FQ-04`
Target: `crates/openwepp-runner/src/totalwatsed3.rs`
Quality dimension: CRAP/cyclomatic complexity

## Objective

Close the originating coverage prerequisite and reduce every eligible target
function to CRAP `<=30` without changing accepted inputs, typed error order,
schema/API, floating grouping, accumulation order, or output identity.

## Write set

- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`
- focused fixture inputs needed for optional soil/element coverage
- this package, catalog, and follow-up ExecPlan

## Required execution

Cover first at science tier: at least 90% lines/regions, every named function at
least 75% regions or reviewed closed-list exclusion, and exact A-H obligations
for column/type/null/value order, optional soil/element aggregation, typed error
codes, date/OFE keys, and valid output rows. Only then decompose rows above 30.

Conservation/publication acceptance is current scope. Record operand lineage
for water/sediment fields, units, area/normalization basis, and authority.
Expected outputs must be independently reconstructed from source rows using
fixtures where wrong columns, areas, row keys, or OFE sums produce different
answers. Preserve row-read and floating accumulation order exactly.

Required gates: focused CLI, exact output identity, format, workspace clippy,
full-profile nextest, deny, Markdown/diff, line/security, dual review and
finding disposition, dual verification, and terminal commit.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to coverage/fixture implementers, independent reviewers,
verification agents, and heavy coverage/gate runners. Expected outputs are
package evidence and bounded source/test corrections; write access is limited
to an explicitly assigned part of the declared write set.
