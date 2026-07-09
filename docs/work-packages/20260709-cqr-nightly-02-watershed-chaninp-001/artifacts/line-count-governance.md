# Line-Count Governance

Status: `WARN-DISPOSITIONED`

Touched Rust files:

| Path | Lines | Band | Disposition |
|---|---:|---|---|
| `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | `2396` | `WARN > 2000` | Accepted for this CQR package. |

Rationale:

- The line-count increase is dominated by module-local characterization tests
  for private projection and numerical helper surfaces.
- Keeping the tests module-local avoids adding public/test-only production API
  and directly serves ADR-0021 science-tier closure for private functions.
- No single newly extracted production helper is a large block, and the prior
  stale `clippy::too_many_lines` suppressions were removed.

Follow-up posture:

- Splitting bulky module-local test fixtures into a dedicated test support
  module is acceptable later, but it is not required for this package closure.
- This package does not add a BLOCK-level file and does not change subsystem
  ownership boundaries.
