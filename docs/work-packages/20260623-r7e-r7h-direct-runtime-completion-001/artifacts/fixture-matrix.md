# Fixture Matrix

Status: blocked-by-R7F.

## Required Coverage

- Snow/frost active days: not run; blocked behind R7F.
- Breakpoint climate: not run; blocked behind R7F.
- PMET branches: not run; blocked behind R7F.
- Irrigation when enabled: not run; blocked behind R7F.
- Multi-OFE transfer ratios: not run; blocked behind R7F.
- Nonzero erosion: not run; blocked behind R7F.
- Sidecar absence/presence: not run; blocked behind R7F.
- Management transitions: not run; blocked behind R7F.

## Independent Operand Reconstruction

- Not run. R7G fixture hardening and operand reconstruction remain invalid
  while production direct still has a counted hot compatibility edge in the
  day-input builder.

## Disposition

Fixture hardening must resume after
`HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE` is closed.
