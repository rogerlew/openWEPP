# No-Compatibility Proof

Status: partial pass; held on frost stateful sub-solver architecture.

Evidence class: Ran + Static.

Manifest counters:

- Direct default candidate:
  `direct_runtime_counters.compatibility_edge_invocations = 0`.
- Explicit direct production:
  `direct_runtime_counters.compatibility_edge_invocations = 0`.
- Retained active-frost direct default:
  `direct_runtime_counters.compatibility_edge_invocations = 0`.
- Both direct manifests report `scheduler_kernel_executed = false` and
  `publication_source = direct-publication-frame`.

Static evidence:

- Production direct active-snow partition no longer calls the map-backed
  `direct_publication_snow_liquid_partition` helper.
- The snow path uses typed hourly forcing and
  `compute_direct_snow_liquid_partition_from_typed`.
- Active frost support now reaches the direct endpoint without counted
  compatibility edges, but it is not yet equivalently typed. The active-frost
  compute path reachable in direct runtime still goes through
  `DirectFrostRunoffSurface` and
  `Wb11HydrologyKernel::compute_direct_frost_liquid_partition`, which is backed
  by `BTreeMap<BoundarySymbol, BoundaryValue>` request surfaces.
- The zero-prior no-freeze fast path uses the same frost branch, surface
  temperature, heat path, and lower-front formulas as the active solver and
  only bypasses fine-layer mutation when those equations prove no freezing
  branch can start from zero prior frost.

Disposition:

- Direct endpoint counters prove no counted compatibility hot-loop edge.
- Full no-compatibility proof cannot close because active frost parity and
  performance now require a stateful typed frost sub-solver. The existing
  symbol-map frost surface remains useful diagnostic scaffolding, but it is not
  an acceptable hot-loop architecture for closure.
