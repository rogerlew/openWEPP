# Pre-Implementation Contract Gate

Status: completed

Evidence mode: Static + Ran

Static:
- Contract-first sequencing satisfied before production edits:
  1. Canonical contract amendments completed in `SC-PERC-001` and
     `SC-WATBAL-001`.
  2. Contract-derived WB18 tests added.
  3. Pre-implementation contract gate executed and recorded here.

Ran:
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`
  failed as expected before production edits with two HPHYS0248 vectors failing.
- Failure confirms the production path does not yet consume `ui_bdrkth` for
  hourly bottom restrictive-layer seepage and does not fail closed when the
  required hourly restrictive-thickness symbol is absent.
- H39 execution after that first fix exposed an additional pinned-baseline
  bottom-layer `meblfc` branch (`fx=1` for hourly bottom seepage). The contract
  and test were amended before the second production code change; the amended
  vector failed with expected `D=0.00026396333842521876` and observed
  `D=0.00000418081710894704`.
