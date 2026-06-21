# Scope Selection

Status: executed-hold.
Evidence mode: Static + Ran.

## Selected Scope

R6 remains the direct-publication cutover package with this hard ordering:

1. Promote the PERFDEEP06 publication operand ledger into canonical authority.
2. Cut over HBP to typed direct projection only.
3. Cut over WAT to typed direct projection only.
4. Cut over PASS to typed direct projection only.
5. Cut over loss JSON to typed direct projection only.
6. Cut over run manifest publication/provenance to typed direct projection
   only.
7. Close package-wide no-compatibility, identity, metadata, reconstruction,
   timing, review, and verification gates.

## Executed Slice

This run executed the next available slice after R6A:

- add an explicit direct-publication cutover candidate mode;
- build direct publication artifacts for that mode;
- route candidate HBP/WAT/PASS/loss writes through direct artifacts only after
  parity gates;
- fail closed before writing when the first current-scope identity gate fails.

## Not Selected

- R7 hot compatibility runtime deletion.
- Output schema redesign.
- Physics or conservation equation changes.
- Default activation without successful R6 acceptance.
- Wrapping compatibility WB13 rows/runtime symbols in direct-named structures.

## Stop Conditions

- Direct publication frame absent after ledger promotion. Cleared by R6A.
- Ledger promotion cannot be made canonical without broader contract authority.
  Cleared by architecture section `5.2.1`.
- Output-family identity or metadata residual cannot be adjudicated under
  current authority. Current blocker.
- Production manifest cannot be cut over without compatibility provenance.
  Current blocker.

## Gate

BLOCKED. The selected R6 scope remains valid, but current direct operands fail
HBP byte identity and manifest cutover remains incomplete.
