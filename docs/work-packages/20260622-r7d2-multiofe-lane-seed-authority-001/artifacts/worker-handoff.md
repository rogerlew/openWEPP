# Worker Handoff

Status: executed-held.

## Handoff

- R7D2 corrected the first R7D hold:
  `HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT`.
- Do not reopen aggregate seed/profile aliasing unless a new fixture proves
  regression. The current H2637 residual persists after lane-indexed seeds and
  profiles.
- Continue with:
  `HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT`.
- First code action: implement a direct WB14/R4K producer inside direct runtime
  rather than filling `DirectInfiltrationDepressionInputs` from compatibility
  runtime surfaces.
- Required producer outputs:
  - same-pass cumulative infiltration;
  - depression-storage delta;
  - reconciled runoff inputs for R4A;
  - same-pass infiltration into WB18 percolation inputs;
  - same-pass infiltration into ET inputs;
  - shadow projection and downstream operands with nonzero direct counters.
- Required contracts:
  `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-PERC-001`, and any local
  `AGENTS.md` files under touched crates/docs.
- Required first evidence:
  a direct runtime fixture where positive precipitation and infiltration
  capacity produce nonzero infiltration and runoff less than precipitation,
  plus an H2637 rerun showing `Q` no longer equals raw liquid input across
  multi-OFE lanes.
- Continue iterating inside the next package until H2637 HBP/WAT/PASS/loss/
  manifest parity closes or the next out-of-envelope process blocker is named
  with field-level residuals and first implementation action.
