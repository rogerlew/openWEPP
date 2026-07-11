# DC FDIR finite-value guard closure

Status: EXECUTED-COMPLETE
Evidence mode: Static and Ran as labeled in artifacts
Queue item: `FQ-01`
Defect: `FDIR-FINITE-VALUE-GUARD-001`

## Objective

Close the fixed-date irrigation parser defect end-to-end: make every parsed
real field reject non-finite values with the canonical typed `FDIR-E-005`
surface, prove no non-finite value can enter typed parser output, and close the
module's ADR-0021 science-tier coverage and eligible CRAP obligations.

This is parser-boundary closure only. No production runtime currently consumes
`FixedDateIrrigationFile`, so this package makes no runtime-readiness claim.

## Correction Authority Envelope

Included authority and write set:

- `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md`
- `docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md`
- `crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs`
- `tests/integration/infile_irrigation_fixeddate_parser_contract.rs`
- `tests/fixtures/infile/irrigation_fixeddate/`
- this package, `docs/work-packages/README.md`, and
  `docs/work-packages/cqr-nightly-followup-burndown-execplan.md`

Pinned legacy provenance is `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, specifically `infile.for`,
`irinpt.for`, `irrig.for`, `inidat.for`, `cdat.inc`, and `cirfixd.inc`.

Allowed edits are canonical contract/spec clarification, contract-derived
tests/fixtures, the minimal typed parser correction, and behavior-preserving
decomposition needed for eligible CRAP `<=30`. Public output meaning,
serialization, runtime scheduling, and unrelated parser behavior are protected.

## Conversion and HOLD rules

The seven-gate bar is already met: the observed valid-token class (`NaN` and
infinities accepted by Rust `f64` parsing) violates canonical finite-field
requirements, the parser and typed error surface are in-envelope, and a direct
fail-closed correction is testable. Execute a contract-first correction.

`HOLD` is exceptional. Diagnostic uncertainty, implementation effort, edit
size, or incomplete coverage do not justify stopping while in-envelope source
reading, implementation, or validation remains possible. A hold requires
`artifacts/hold-legitimacy-audit.md` naming and proving a declared boundary,
the correction route considered, and why it cannot close here.

## Required sequence

1. Amend the canonical contract and spec, including pinned anchors, finite
   constraints, A-H obligations, and guard mapping.
2. Add contract-derived failing tests before production edits.
3. Record and pass the pre-implementation contract gate.
4. Correct production parsing and add characterization coverage.
5. Reach science-tier `>=90%` line and region coverage, every logical function
   `>=75%` region coverage or an ADR-0021-authorized exclusion, complete A-H
   obligation binding, and eligible CRAP `<=30`.
6. Run focused and full closure gates, dual review, disposition, dual
   verification, and terminal queue/README updates.

## Deliverables and exit criteria

- Non-finite `datver`, `irint`, `irdept`, `nozzle`, `qspply`, `tstart`, `tend`,
  and `tdepl` fail with typed `FDIR-E-005`; no typed parse output contains them.
- Existing finite strict/compatibility behavior remains contract-consistent.
- Contract/spec use pinned baseline anchors for touched legacy claims.
- A-H obligation-to-test map is complete.
- Target coverage and CRAP gates above pass with before/after evidence.
- `cargo fmt --check`, workspace clippy with warnings denied, focused nextest,
  full-profile workspace nextest, and `cargo deny check` pass.
- Line-count governance, security impact (`no impact` unless evidence changes),
  two independent reviews with every finding dispositioned, and two independent
  verifications pass.
- Every gate is evidenced in this package; no current-scope gate is deferred.

## Subagent authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent contract/technical reviewers, verification agents, and heavy
gate/coverage runners. Expected outputs are package review, verification, and
gate artifacts. Review/verification access is read-only; heavy runners may
write only their named package artifact and ordinary untracked build outputs.

## Dependencies and evidence

Use `artifacts/required-reading-map.md` for the tiered reading set and budget.
Truthfulness labels are mandatory: `Static:` for inspection and `Ran:` for
executed commands. Required artifacts are pre-created as queued placeholders.
