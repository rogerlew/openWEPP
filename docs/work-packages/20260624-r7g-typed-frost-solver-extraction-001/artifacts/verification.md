# Verification

Status: COMPLETE.

Static Verification:

- Verified package exit criteria against final code:
  - Typed production direct frost day context exists and consumes direct
    lane/day/winter-column inputs.
  - Active-frost typed entry exists and does not construct
    `HillslopeKernelRequest`.
  - R4A mutates `DirectWinterColumnState.frost` from the precomputed typed
    partition result.
  - Production direct day input no longer assigns a `frost_runoff_surface`.
  - Comparator surface remains only as named adapter seam.
  - Focused active and inactive/no-material typed-vs-adapter parity tests exist.
  - Source scans prove the production hot path avoids compatibility request and
    surface symbols.

Ran Verification:

- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed with `advisories ok, bans ok, licenses ok, sources ok`.
- Focused reruns:
  - `cargo test -p openwepp-hillslope-orchestrator r7g_typed_active_no_freeze_partition_matches_surface_adapter -- --nocapture`
  - `cargo test -p openwepp-hillslope-orchestrator r7g_typed_inactive_frost_partition_matches_surface_adapter_without_material -- --nocapture`
  - `cargo test -p openwepp-runner r7g_direct_production -- --nocapture`
  - `cargo test -p openwepp-runner --lib`
