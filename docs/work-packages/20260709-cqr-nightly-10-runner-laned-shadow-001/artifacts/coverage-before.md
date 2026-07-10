# Coverage Before

Evidence label: Static.

Status: `SCAFFOLDED`

Source:

- `/tmp/openwepp-cqr-nightly.lcov`
- SHA-256:
  `7dd0b93fcd5e0f217d5b4e6fd0a6871a04976ac0b5c788dbb2a9fdffca37217a`

Target:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- Line coverage: `251/452` (`55.53097345132743%`)
- Function coverage: `23/39`
- Branch coverage: `0/0` (no LCOV branch rows emitted)

Existing relevant tests observed:

- Module-local tests in `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
  for dynamic canopy operands, dynamic rainfall intensity, profile-slot
  accumulation, and drain-tail routing window behavior.
- Static source guard:
  `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs::laned_shadow_consumes_live_dynamic_friction_operands`.
- Integration fixture:
  `tests/integration/laned_shadow_h2637.rs`, including fail-closed legacy
  coefficient authority, native shadow output identity, active/shadow mutual
  exclusion, and default/active selector behavior.

Initial coverage concern:

The three high-CRAP target helpers are currently zero-covered in the saved
nightly CRAP report. Phase B must add or confirm focused characterization
before decomposition.
