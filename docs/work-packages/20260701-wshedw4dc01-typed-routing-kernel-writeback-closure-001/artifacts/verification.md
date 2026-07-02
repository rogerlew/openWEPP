# Verification

Status: `EXECUTED`

Evidence class: `Static` and `Ran`.

Static:

- Public watershed CLI now calls `execute_watershed_dispatch_with_frame` and
  `publish_typed_routing_report`.
- Public watershed CLI contains no production call to
  `compatibility_writeback_surface`, `execute_watershed_dispatch_with_kernel`,
  or `harvest_compatibility_routing_report`.
- Direct kernel source contains no `WatershedWritebackSurface`,
  `BoundarySymbol`, `BoundaryValue`, `KernelWritebackPayload`, or
  `WatershedKernelRequest`.
- Direct kernel source calls the actual routing physics helpers:
  WS11 wave routing, WS12 impoundment stage/outflow routing, WS18 transport
  capacity, and the shared WS20 segment-routing core.
- Rejected shortcut markers remain absent:
  `route_typed_channel`, `route_typed_impoundment`,
  `incoming_peak + control.qinf`, and
  `incoming_peak * self.routing_globals.dtchr_seconds`.

Ran:

- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo nextest run --workspace --profile full`: passed, 1284 tests run,
  1284 passed, 1 skipped.
- `cargo deny check`: passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract --
  --nocapture`: passed, 24 tests.
- `cargo test -p openwepp-watershed-orchestrator -- --nocapture`: passed,
  45 tests.
- `cargo test --test ws11_channel_routing_physics_equivalence_contract --
  --nocapture`: passed, 44 tests.
- `cargo test --test ws10_watershed_kernel_contract -- --nocapture`: passed,
  5 tests.
- `cargo test --test ws12_impoundment_physics_equivalence_contract --
  --nocapture`: passed, 8 tests.
- `cargo test --test infile_watershed_structure_parser_contract
  carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate --
  --nocapture`: passed, 1 test.
- `git diff --check`: passed.
- `rg` scans over the public CLI and direct kernel found no old
  surface/request/writeback markers.

Committed-fixture note:

- `tests/fixtures/watershed/carnivorous-adobo/README.md` states that fixture is
  a committed parser/input substrate and is not a current
  `openwepp-cli-watershed` end-to-end execution fixture because the CLI surface
  requires TOML watershed runfiles with HBP pass bindings.
- Therefore this run records the committed carnivorous parser gate and public
  CLI generated/reuse output behavior, but does not claim carnivorous output
  identity.
