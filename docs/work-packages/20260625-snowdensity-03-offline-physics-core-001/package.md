# SNOWDENSITY-03 Offline Physics Core

Status: complete.

Package type: offline implementation / candidate characterization.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: implement `physics_bulk` in Rust snowbench only, with a typed bulk
snowpack state, candidate fresh-snow density, dry/wet densification,
liquid-retention/release/refreeze, and cold-content accounting; add unit and
contract guard tests; and emit SNOTEL rubric output for all five SNOTEL fixtures
without per-site constants.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`crates/AGENTS.md`, `tests/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
(`INV-SNOWFREEZE-051`, `OBL-SNOWFREEZE-P-026`), ADR-0017, ADR-0026, ADR-0027,
`docs/planning/snow-frost-fidelity-strategy.md`, and SNOWDENSITY-02 handoff.

Subagent authorization: none. Execute locally and record review/disposition in
package artifacts.

## Scope

In scope:

- Offline Rust snowbench `physics_bulk` candidate model and CLI command.
- Candidate state CSV/JSON/Markdown outputs under caller-provided target dirs.
- SNOTEL profile harness that calls the Rust snowbench command and scores v74/v75
  rubric cells without using per-site constants.
- Focused Rust unit/integration tests for state bounds, closure, monotonicity,
  and production non-coupling.
- Package evidence and catalog/planning updates.

Out of scope:

- No production runtime coupling, `snow_model` parser/config activation, output
  schema change, compatibility deletion, or default behavior change.
- No changes to legacy WEPP compatibility behavior.
- No PySnobal runtime dependency or hardening.
- No SNOTEL per-site constants, SSD residual fitting, or calibration loop.
- No claim that `physics_bulk` is production-promotable; SNOWDENSITY-04 owns
  offline adjudication and iteration.

## Acceptance Criteria

- Required reading is recorded.
- Candidate equations/constants are recorded with provenance and candidate-only
  disposition.
- Rust snowbench exposes an offline `physics-bulk` command that writes
  `physics_bulk_snow.csv`, summary JSON, and summary Markdown.
- Unit tests prove SWE/depth/density bounds, fresh-snow density bounds and
  monotonicity, dry/wet compaction sanity, liquid retention/release/refreeze
  conservation, and cold-content accounting.
- A focused integration guard proves `physics_bulk` appears only in snowbench,
  tests, docs, and diagnostic tools, not production runtime activation paths.
- SNOTEL five-site profile JSON/Markdown is generated from Rust `physics_bulk`
  outputs and the v74/v75 rubric.
- Review, verification, line-count governance, and worker handoff artifacts are
  complete.
- Required package gates pass or the package closes `HOLD` with a named blocker.

## HOLD Boundaries

Close as `HOLD` only if the Rust candidate cannot produce finite bounded output
for all five SNOTEL fixtures, the SNOTEL rubric profile cannot be emitted from
offline candidate output, or the implementation would require runtime coupling
or site-specific constants to proceed.

## Execution Plan

1. Scaffold package and active kickoff prompt.
2. Read required authority and existing snowbench/SNOTEL harness.
3. Implement offline `physics_bulk` model and CLI command in snowbench.
4. Add focused Rust tests and an integration guard.
5. Add a SNOTEL profile harness for `physics_bulk`.
6. Run the five-site profile and store package evidence.
7. Run focused and package gates.
8. Record reviews, verification, line-count governance, and handoff.

## Closeout

Disposition: `COMPLETE-SNOWDENSITY-03-OFFLINE-PHYSICS-CORE`.

This package implemented the offline Rust `physics_bulk` snowbench candidate,
added bounded/conservation unit tests, added a production-confinement
integration guard, generated a five-site SNOTEL rubric profile, and left the
candidate outside runtime activation paths.

The first candidate profile is not production-promotable: the five-site SNOTEL
profile has `24` forcing-robust `fail` cells, `13` forcing-robust `marginal`
cells, `3` forcing-robust `pass` cells, `5` forcing-robust `strong` cells, and
`15` unavailable robust cells. Under ADR-0017 these remain unresolved
observation-profile findings, not openWEPP defect verdicts
(`openwepp_defective_cells = 0`). SNOWDENSITY-04 owns any in-envelope offline
iteration or a documented fail route.
