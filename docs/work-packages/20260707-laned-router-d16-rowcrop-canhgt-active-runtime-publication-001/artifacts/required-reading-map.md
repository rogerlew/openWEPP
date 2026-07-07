# Required Reading Map

Status: COMPLETE

Static:

- `AGENTS.md`: root package/kernel requirements; do not switch branches;
  contract authority governs kernel-affecting work.
- `docs/work-packages/AGENTS.md`: defect-closure package rules, consumer-path
  proof requirement, evidence non-deferral, and required review/verification
  posture.
- `docs/specifications/science-contracts/AGENTS.md`: contract-first sequencing
  and baseline-authoritative physics migration rule.
- `tests/AGENTS.md`: contract-derived tests and focused/full Rust gate
  expectations.
- `crates/AGENTS.md`: Rust crate edit and validation expectations.
- `SC-PLANT-001`: daily plant `Hc`/canopy-height authority, currently
  under-specified in PL16 growth state.
- `SC-OFEROUTE-001`: Lane D Rev-21 friction operand authority, currently
  pairing post-growth LAI with static typed-management `canhgt`.
- Baseline `grow.for`, `initgr.for`, and `frcfac.for`: canonical daily and
  initial canopy-height equations plus downstream live-cover consumption.

Ran:

- Read the package-local and nested agent instructions before contract/code
  edits.
- Read the D16 selected-cohort active-suite package and hold artifact to
  confirm the precise failing member, mode, lane, and guard.
- Read baseline `grow.for`, `initgr.for`, and `frcfac.for` around plant canopy
  height initialization, daily growth update, and live-cover consumption.
- Read the active/shadow Lane D operand publication surfaces and R5D growth
  projection tests before implementation.
