# Disposition

Status: `EXECUTED-COMPLETE`

Evidence class: `Static` and `Ran`.

Result:

- `WSHED-W4-HOLD-001` is closed for the production public watershed CLI path.
- The rejected non-authoritative shortcut remains removed.
- Public CLI routing now consumes `WatershedNetworkFrame` through
  `execute_watershed_dispatch_with_frame`.
- Typed publication now consumes routed frame state through
  `publish_typed_routing_report`.

Implementation:

- Added direct typed channel/impoundment execution over frame records.
- Channel routing reuses the existing WS11 branch/wave helpers and WS18/WS20
  sediment helpers rather than shortcut arithmetic.
- Impoundment routing reuses the existing WS12 coefficient projection,
  adaptive stage integration, outflow, and continuity helpers.
- WS20 segment routing was factored into a shared core so direct typed routing
  and legacy compatibility tests execute the same case 1/2/3/4 equations.
- Downstream routed channel state now carries wave and sediment class payloads
  needed by later typed channel nodes.

Validation:

- Full workspace final gates passed:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full`, and `cargo deny check`.
- Public watershed CLI behavior contract passed all 24 tests.
- Focused WS10, WS11/WS20, and WS12 physics contracts passed.
- Source guards prove the public CLI and direct kernel do not use old
  surface/request/writeback markers.

Boundary note:

- `tests/fixtures/watershed/carnivorous-adobo/README.md` states that fixture is
  a committed input/parser substrate, not a current CLI end-to-end fixture with
  TOML/HBP pass bindings. This package therefore records the committed parser
  gate and public CLI output gates, but no carnivorous output identity claim.
