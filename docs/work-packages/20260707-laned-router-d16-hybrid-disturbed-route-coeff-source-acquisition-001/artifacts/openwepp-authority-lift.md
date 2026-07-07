# openWEPP Authority Lift

Status: EXECUTED-SOURCE-AUTHORITY / D16-SUITE-HOLD. Evidence mode: Static + Ran.

Authority doc update:

- `docs/contracts/openwepp-management-lanuse-authority-contract.md` rev 2 names
  WEPPpy Disturbed extended lookup as an authorized explicit producer of native
  `routing_coefficients`.
- The no-legacy-field-bridge rule remains binding.
- No `SC-*` contract text was changed.

Generated native proof:

- Disturbed-generated fixture:
  `tests/fixtures/disturbed_native_route_coefficients/p1.man`.
- Datver: `ow-lanuse-1`.
- Native cropland sentinel: `landuse=4`.
- Route coefficients: `490.00000 0.40000 0.01600 0.05000 0.20000`.

openWEPP consumer proof:

```text
cd /home/workdir/openWEPP
cargo test -p openwepp-hillslope-orchestrator disturbed_native_route_coefficients -- --nocapture
1 passed
```

The test parses the generated fixture through
`openwepp_input_contract::parsers::management::parse_management_from_str(...)`
and projects the five route coefficients to real PL schedule symbols via
`build_hillslope_pl_runtime_surfaces_from_management(...)`.

Remaining hold:

- The selected D16 executable active plain-vs-hybrid cohort preflight was not
  run in this package. Source authority is lifted, but D16 suite execution
  remains follow-on work.
