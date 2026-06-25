# SNOWDENSITY-02 Contract + ADR

Status: complete.

Package type: contract / ADR governance.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: amend `SC-SNOWFREEZE-001` with the opt-in `physics_bulk`
snow-density remediation envelope, no-site-tuning rule, candidate state
variables, conservation obligations, and activation constraints; draft and
ratify the deliberate-legacy-divergence ADR for
`snow_model = legacy_wepp | physics_bulk`; and add contract-derived guard tests.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`docs/planning/snow-frost-fidelity-strategy.md`, ADR-0017, ADR-0026,
SNOWFROST-FIDELITY-D/E/F/H/I0, and SNOWDENSITY-01.

Subagent authorization: none. Execute locally and record review/disposition in
package artifacts.

## Scope

In scope:

- `SC-SNOWFREEZE-001` v75 contract amendment for the `physics_bulk` candidate
  envelope.
- ADR-0027 for deliberate legacy divergence under
  `snow_model = legacy_wepp | physics_bulk`.
- Contract-derived Rust guard test that preserves opt-in, no-site-tuning,
  candidate-status, and handoff language.
- Work-package, planning, ADR index, and science-contract registry updates.

Out of scope:

- No production runtime snow physics, constants, model options, parser surfaces, output schemas, or default behavior changes.
- No PySnobal hardening, rerun, or runtime dependency.
- No frost physics or frozen hydraulic conductivity changes.
- No `ssd` tuning, SNOTEL per-site fitting, or legacy bit-parity remediation.
- No deletion of legacy WEPP snow runtime or compatibility rollback.

## Acceptance Criteria

- Required reading is recorded.
- `SC-SNOWFREEZE-001` contains v75 `INV-SNOWFREEZE-051`, candidate state
  variables, candidate authority anchors, `OBL-SNOWFREEZE-P-026`, and the
  Snow-Density Physics-Bulk Candidate Envelope Addendum.
- ADR-0027 is present in `docs/decisions/` and indexed in
  `docs/decisions/README.md`.
- `docs/specifications/science-contracts/index.md` records the current
  `SC-SNOWFREEZE-001` review date.
- `docs/planning/snow-frost-fidelity-strategy.md` records SNOWDENSITY-02 as
  complete and routes next work to SNOWDENSITY-03.
- Contract guard test passes and would fail if the opt-in/no-site-tuning language is removed.
- Review, verification, line-count governance, and worker handoff artifacts are
  complete.
- `cargo fmt --check`, the focused guard test, and `git diff --check` pass.

## HOLD Boundaries

Close as `HOLD` only if the contract cannot cleanly express an opt-in candidate
without authorizing production physics, ADR-0027 conflicts with ADR-0017 or
ADR-0026, or the guard test cannot be made to fail on removal of current-scope
governance language.

## Execution Plan

1. Scaffold this package and active kickoff prompt.
2. Read required authority and SNOWDENSITY-01 handoff.
3. Amend `SC-SNOWFREEZE-001` with v75 `physics_bulk` envelope authority.
4. Add ADR-0027 and update ADR/science-contract indexes.
5. Add focused contract guard tests.
6. Update planning and work-package catalogs.
7. Run focused verification.
8. Record review, verification, line-count governance, and handoff.
