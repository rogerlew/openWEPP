# Pre-Implementation Contract Gate

Status: W-D executed-hold

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

W-C gate result:

- `SC-ROUTE-001` version `45` pins zero-sediment HBP payload semantics and
  `nchnum=0` output-disabled channel semantics.
- W-C replaced one-row/default-zero writer publication for real runs with
  WAT-backed multi-row publication.
- W-D still owns the independent totalwatsed3 closure identity.

Before W-D production edits:

- Read the totalwatsed3 wepppy producer and audit scripts as the consumer
  acceptance surface.
- Treat schema unit metadata as binding: exact fields declared as `m^3` must
  publish volumes, not depth aliases.
- Keep the M-I/M-E4 rule: closure must use independent operands, not
  `runvol == Q * Area / 1000` self-consistency.

W-D gate result:

- Output unit metadata was amended for
  `watershed_totalwatsed3.Interception`.
- W-D corrected publication defects without changing process-physics contract
  authority.
- W-D did not meet the conservation gate; W-D-REDO owns daily PASS `runvol`
  authority before any closure claim.
