# Operand Reconstruction

Status: held.

Evidence class: Ran.

Completed checks:

- Manifest checksum maps were independently sorted with `jq -S` and compared:
  default-disabled compatibility vs rollback compatibility `cmp=0`;
  direct default vs explicit direct `cmp=0`.
- Direct runtime counters independently confirm no compatibility edges:
  `compatibility_edge_invocations=0` and `day_frame_commits=235961`.
- WAT/PASS Parquet schemas and row counts match between compatibility and
  direct captures.

Held reconstruction:

- Protected WAT/PASS/HBP operands do not reconstruct to compatibility identity.
- First reduced conservation-sensitive blocker is active frost state:
  production direct now has active-frost endpoint execution, but persistent
  fine/shadow state, no-material carry, coarse layer mutation, and publication
  operands are not owned by one stateful typed solver.
- Fine-layer carry preservation reduced the first day-5 mismatch to partition
  and front-state authority rather than total water closure, but later endpoint
  evidence remained red for both performance and protected output parity.

Disposition:

Independent reconstruction cannot close R7G until a stateful frost sub-solver
supplies the same frozen-water/frost-depth state, downstream runoff/storage
operands, and publication surfaces as compatibility without map-backed
request/surface authority.
