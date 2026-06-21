# Worker Handoff

Status: executed-held.
Evidence mode: Static + Ran.

Target: close `R6E-DIRECT-PUBLICATION-CUTOVER-BLOCKER`.

Starting blocker for the next worker:

- `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

R6E resolved:

- production direct publication input binding from parsed climate;
- retained direct publication execution through direct capture;
- removal of hand-authored retained rows from the compatibility loop;
- focused no-output tests that prove direct capture counters run and
  compatibility-edge counters stay zero.

First actionable item:

Implement contract-backed direct process parity for HBP publication operands.
The direct frame reaches byte comparison but direct process operands are not yet
parity-grade. Do not address this by copying compatibility
`SimulationOwnedWb13Row`, `HillslopeWritebackSurface`, `KernelWritebackPayload`,
runtime/writeback publication symbols, stale logical state, or zero skeleton
capture into direct-named structures.

Minimum first acceptance target:

- `DirectPublicationFrameCutover` still builds retained publication through
  direct capture with typed inputs;
- HBP direct operands are sourced from direct process state/projections;
- HBP byte identity passes for the fixture;
- no compatibility authority source is used as direct input;
- no public outputs are written until all required parity gates pass.

Closure still requires:

- HBP byte identity;
- WAT/PASS Arrow row/schema/metadata parity;
- loss JSON identity;
- manifest provenance/checksum parity;
- anti-alias fixtures;
- independent reconstruction;
- no compatibility authority;
- successful direct public output writes on `DirectPublicationFrameCutover`;
- default-disabled compatibility isolation.
