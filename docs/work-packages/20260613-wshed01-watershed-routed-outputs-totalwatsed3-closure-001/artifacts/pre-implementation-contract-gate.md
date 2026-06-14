# Pre-Implementation Contract Gate

Status: W-A executed

Evidence mode: Static

W-A gate result: PASS for characterization, HOLD for package implementation.

Before W-B production edits:

- Read `docs/specifications/science-contracts/AGENTS.md` because W-B changes a
  parser/runtime boundary feeding watershed kernel execution.
- Pin no-impoundment semantics before implementation: zero is valid only as a
  declared empty set aligned with zero structural impoundments.
- Add red tests before changing parser behavior.

Before W-C production edits:

- Treat totalwatsed3 output as a conservation acceptance surface, not a file
  existence target.
- Reject publication that writes default zeros or one-row synthetic data for a
  real routed run.
- Define independent operands for the watershed water-balance identity.
