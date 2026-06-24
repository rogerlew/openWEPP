# Worker Handoff

Status: COMPLETE.

Final disposition: COMPLETE.

Completed:

- Extracted production direct active-frost computation behind typed frost input
  structs and the typed `Wb11HydrologyKernel` entry.
- Cut production direct frost day context away from
  `DirectFrostRunoffSurface` and `HillslopeKernelRequest`.
- Preserved the remaining surface bridge only as a comparator seam.
- Added focused typed-vs-adapter parity tests for active no-freeze and inactive
  no-material fixtures.
- Added runner source-scan tests proving production direct frost does not call
  the compatibility surface/request path.
- Fixed the snow-only authority regression exposed during full gate reruns by
  making frost layer reads lazy on frost projection presence.

Not completed by this package:

- Deletion of `DirectFrostRunoffSurface`, `DirectFrostLiquidPartition`,
  direct-runtime `frost_liquid_partition`, or temporary runtime carry mirrors.
  Those remain the later consumer-cutover/deletion step.
- HBP/WAT/PASS parity closure, default activation, or performance closure.

Recommended next package:

- Execute the consumer deletion/cutover step for frost publication once
  downstream consumers are ready to read the typed frost lane state/projection
  directly and no longer need the temporary `DirectFrostLiquidPartition` bridge.
