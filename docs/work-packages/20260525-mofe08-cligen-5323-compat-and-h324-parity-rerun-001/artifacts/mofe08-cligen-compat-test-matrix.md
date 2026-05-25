# MOFE08 CLIGEN Compatibility Test Matrix

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Matrix covers new `5.3x` acceptance/canonicalization behavior and boundary
  rejection posture.

| Surface | Scenario | Fixture/Input | Expected | Result |
| --- | --- | --- | --- | --- |
| climate parser | accept `5.323` and canonicalize to `5.3` | `tests/fixtures/infile/climate/datver_5_323.cli` | parse success; `datver == 5.3` | pass |
| climate parser | reject `5.4` boundary | strict fixture with line-1 replaced to `5.4` | typed `UnsupportedDatver` | pass |
| climate parser | regression suite | `infile_climate_parser_contract` | no regressions | pass |
| slope parser | regression suite | `infile_slope_parser_contract` | no regressions | pass |
| soil parser | regression suite | `infile_soil_parser_contract` | no regressions | pass |

Ran:
- `cargo test -p openwepp --test infile_climate_parser_contract --test infile_slope_parser_contract --test infile_soil_parser_contract`
  - climate: `16 passed`
  - slope: `18 passed`
  - soil: `11 passed`
