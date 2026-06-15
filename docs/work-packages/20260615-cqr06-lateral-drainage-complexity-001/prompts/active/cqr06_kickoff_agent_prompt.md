# CQR06 Kickoff Agent Prompt

You are working in the local openWEPP repository only. Execute
`docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001/package.md`
end-to-end.

## Scope

Target production file:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`

Documentation write set:

- `docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001/**`
- `docs/work-packages/README.md`

## Non-Goals

- Do not change WB19 formulas, constants, thresholds, lane predicates, guard
  families, symbol names, unit conversions, writeback order, or public APIs.
- Do not edit runtime projection, scheduler ordering, WB17/WB18/WB12/WB13, or
  watershed routing code.
- Do not split the target file into new modules in this package.

## Required Execution

1. Read required instructions and contracts listed in `package.md`.
2. Capture before focused tests, coverage/LCOV, CRAP, function lengths, line
   count, and public surface.
3. Decompose eligible target functions into private helpers, preserving exact
   arithmetic expression grouping and guard order where externally observable.
4. Re-run focused WB19 tests, after coverage/LCOV, and after CRAP.
5. Run final closure gates:
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cargo deny check`
6. Complete required artifacts, dual reviews, dual verification, disposition,
   and worker handoff.

Stop only at a hard blocker with command evidence and a first actionable
follow-up.
