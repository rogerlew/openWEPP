# WSHED-W7DC01 Handoff Prompt

Scope: local repository science-contract and Rust implementation task for
openWEPP; flat-file reads/edits plus local validation commands only; no
external connectivity.

Execution mode: package-end-to-end (default).

Task: execute
`docs/work-packages/20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/package.md`
to close defect `WSHED-W7-HOLD-001`.

First actionable item: close defect `WSHED-W7-HOLD-001`; do not start with
another broad fixture search. Diagnose why production direct hillslope execution
emits zero HBP sediment for real multi-OFE source substrates where EROD14 is
enabled.

Required reading:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/artifacts/sediment-fixture-inventory.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/artifacts/disposition.md`
- relevant sediment/erosion `SC-*` contracts before production physics edits

No surrogate physics: production code must implement actual contract-backed or
baseline-authoritative sediment physics. Surrogate/provisional/proxy/heuristic
stand-ins are forbidden.

Subagent requirement: REQUIRED for review/verification/comparator work when the
live session explicitly authorizes delegation. This prompt explicitly
authorizes subagent spawning/delegation to `rust_code_reviewer`,
`rust_qa_reviewer`, `science_contract_reviewer`, and `comparator_suite_runner`.

Outputs: update package artifacts, tests, source, review, verification,
gate-results, roadmap/index state, and handoff back to WSHED-W7.
