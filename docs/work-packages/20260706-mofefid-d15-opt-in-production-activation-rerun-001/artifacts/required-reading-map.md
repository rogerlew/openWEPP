# Required Reading Map

Status: **EXECUTED-SCAFFOLD**.

Evidence mode: Static.

Core required-reading byte total at scaffold: **350,039 bytes** (`OK`, below
the 400,000-byte WARN threshold used by prior Lane D package maps).

## Core

Read before edits:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1 and §7
- `docs/ROADMAP.md` §M
- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/package.md`
- D14 artifacts, especially `baseline-timing.md`, `slot-timing-evidence.md`,
  `protected-output-evidence.md`, `gate-results.md`, and `worker-handoff.md`
- `docs/work-packages/20260705-mofefid-d15-opt-in-production-activation-001/package.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/package.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/artifacts/gate-results.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/artifacts/review-response-claude.md`
- D11/D12/D13 final dispositions and handoffs enough to verify active
  consumer obligations.

## Conditional

Read before editing the relevant surface:

- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `tests/fixtures/AGENTS.md` when fixture files are touched.
- `docs/standards/AGENTS.md` and the specific standard when adding prompt,
  gate-selection, or work-package governance text.
- `SC-RUNOFFPART-001`, `SC-SUBHYD-001`, or `SC-SED-001` if implementation
  changes their owned surfaces or contracts.

## On Demand

- Lane D seam design, seam implementation, and runtime shadow package artifacts
  when tracing `INV-OFEROUTE-012`.
- `docs/standards/local-ci-gate-selection.md` for gate tier selection.
- `docs/specifications/science-contract-authoring-procedure.md` and
  `kernel-process-contract-profile.md` only if an `SC-*` amendment is required.
