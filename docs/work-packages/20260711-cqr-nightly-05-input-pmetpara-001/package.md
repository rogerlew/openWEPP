# CQR Nightly Batch 01, Target 05 — PMETPARA Parser

Package: `20260711-cqr-nightly-05-input-pmetpara-001`
Status: `QUEUED`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `01`
Target module: `crates/openwepp-input-contract/src/parsers/pmetpara.rs`
Target rank: `5` of `8`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective And Scope

Disposition the sole eligible row above `30`, or reduce it behavior-preservingly
if review proves decomposition/testing is appropriate. Preserve PMETPARA grammar,
normalization, warnings, typed error IDs/messages/sources, and public API.

Core reading: root/crate/test/work-package governance; nightly ExecPlan;
mechanical/CQR guides; ADR-0021; prompt guide; target; and
`tests/integration/infile_pmetpara_parser_contract.rs`. Conditional science
authority triggers only on semantic contact, which is out of scope.

Write set: target, focused test, package directory, and catalog. No parser,
threshold, normalization, warning, error-contract, API, or accepted-input change.

## Subagent Authorization And Protocol

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner roles
for CQR disposition, metrics, behavior, and gates. Outputs are compact verdicts,
logs, metrics, and artifacts; write access is read-only unless a bounded fix is
explicitly assigned.

Subagent requirement: REQUIRED for full-workspace coverage/CRAP, clippy,
full-nextest, deny, comparator, or fixture batches. Commit scaffold before any
implementation/test edit; commit reviewed completion or local hold before target
06. Formatting/error-display rows must follow the nightly ExecPlan's explicit
ADR-disposition route rather than forced tests/decomposition when review agrees.
