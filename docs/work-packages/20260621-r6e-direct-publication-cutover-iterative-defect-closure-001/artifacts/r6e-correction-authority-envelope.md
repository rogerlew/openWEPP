# R6E Correction Authority Envelope

Evidence mode: Static.

Status: updated after execution.

Accepted R6E corrections:

- line-count helper split;
- typed direct publication day inputs;
- parsed climate precipitation/effective-temperature binding into direct day
  frames;
- retained cutover publication as direct executor output instead of
  compatibility-loop row construction.

Resolved marker:

- `HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT`.

Current terminal marker:

- `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

Authority boundary:

R6E may wire parsed inputs into direct frames and may fail closed when direct
process operands do not pass public output parity. It may not copy
compatibility WB13 rows, runtime publication surfaces, writeback payloads, or
stale logical state into direct-named structures to force parity.

The remaining HBP process parity blocker requires contract-backed direct
process parity work before public output cutover can proceed.
