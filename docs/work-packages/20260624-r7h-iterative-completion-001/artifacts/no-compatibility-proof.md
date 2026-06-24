# No-Compatibility Proof

Evidence class: Static/Ran.

Runtime counters:

- Direct default-candidate after fix:
  `compatibility_edge_invocations=0`, `scheduler_kernel_executed=false`,
  `publication_source=direct-publication-frame`, `day_frame_commits=235961`.
- Explicit direct after fix:
  `compatibility_edge_invocations=0`, `scheduler_kernel_executed=false`,
  `publication_source=direct-publication-frame`, `day_frame_commits=235961`.

Source scans:

- Ran `rg -n "DirectFrostRunoffSurface|BoundarySymbol|BoundaryValue|HillslopeWritebackSurface|HillslopeKernelRequest|compatibility_edge_invocations|record_compatibility" crates/openwepp-hillslope-orchestrator/src/direct_runtime crates/openwepp-runner/src/hillslope/direct_publication`.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime` hits are the
  direct audit counter definitions and increment function only. Current direct
  H2637 manifests prove the production direct path did not invoke that edge.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/**`
  still contains `HillslopeWritebackSurface`, `BoundarySymbol`, and
  `BoundaryValue` because the direct publication day-input builder overlays
  typed lane state into a seed/runtime-surface adapter. This is outside the
  direct-runtime phase hot path but remains technical debt for post-R7 direct
  publication input authority.
- No `DirectFrostRunoffSurface` or `HillslopeKernelRequest` production hit was
  found in the direct-runtime winter hot path by this scan.

Forbidden authority in production direct winter hot path:

- `DirectFrostRunoffSurface`
- `BoundarySymbol`
- `BoundaryValue`
- `HillslopeWritebackSurface`
- `HillslopeKernelRequest`
- compatibility WB13 rows
- scheduler runtime surfaces as direct authority

Disposition:

- Runtime no-compatibility counter gate is green for direct modes.
- Source-scan no-compatibility gate is green for
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime`.
- Direct-publication seed-surface helpers remain a bounded adapter dependency,
  not the terminal R7H blocker found here. The terminal blocker is typed frost
  freeze parity.
