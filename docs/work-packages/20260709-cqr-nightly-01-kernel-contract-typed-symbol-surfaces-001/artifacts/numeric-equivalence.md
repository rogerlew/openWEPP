# Numeric Equivalence

Status: `COMPLETE`

Static: this package targets typed symbol strings, not floating-point math.

Identity claim:

- Numeric output is not in scope because the touched production path maps typed
  enum variants to `BoundarySymbol` strings.
- API/string identity is the relevant behavior identity surface.
- The expanded ARCH22 test asserts every touched static hillslope state symbol,
  every hillslope flux symbol, every dynamic irrigation field suffix, the
  climate forcing error display string, climate forcing accessor formatting,
  watershed channel and impoundment field suffixes, and watershed hillslope
  contributor payload formats.

Ran:

- `cargo nextest run --test arch22_typed_state_surface_contract` exited `0` with
  `17 tests run: 17 passed, 0 skipped`.
