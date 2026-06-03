# Kernel-Profile Compliance Checklist

Status: completed/HOLD
Evidence mode: static + ran

Static:

- Contract-first authority: satisfied for the implemented slice through
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-015` and `SC-WATBAL-001#INV-WATBAL-055`.
- Provenance: pinned baseline source line ranges are recorded in
  `baseline-provenance-map.md`.
- Production physics: implemented slice uses baseline-derived retained-rain and
  signed-melt lineage; no empirical tuning or surrogate coefficient was added.
- Guards: non-finite/domain behavior remains routed through existing typed
  hydrology guard machinery; no silent default wrapper was added.
- Write set: production edits stayed within package write set except for
  test-only `#[allow(clippy::too_many_lines)]` annotations in the already
  touched trace test module.
- Completion posture: `HOLD`; full baseline-authoritative winter/snowpack
  migration and semantic parity are not complete.

Ran:

- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture` -> pass.
- Full H1..H39 diagnostic suite -> runtime pass, semantic pass `0/39`.
