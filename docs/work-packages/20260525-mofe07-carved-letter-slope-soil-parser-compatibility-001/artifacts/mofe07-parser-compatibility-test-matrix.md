# MOFE07 Parser Compatibility Test Matrix

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Matrix covers scoped compatibility forms and strict-mode regression posture.

| Surface | Scenario | Fixture | Mode | Expected | Result |
| --- | --- | --- | --- | --- | --- |
| slope | shared-geometry MOFE form | `tests/fixtures/infile/slope/compat_shared_geom_multi_ofe.slp` | strict | reject | pass |
| slope | shared-geometry MOFE form | `tests/fixtures/infile/slope/compat_shared_geom_multi_ofe.slp` | compatibility | accept | pass |
| soil | quoted `7778` header, omitted `avke` | `tests/fixtures/infile/soil/compat_quoted_header_7778.sol` | strict | reject | pass |
| soil | quoted `7778` header, omitted `avke` | `tests/fixtures/infile/soil/compat_quoted_header_7778.sol` | compatibility | accept (`avke=0.0`) | pass |
| soil | quoted `7778` + per-OFE restrictive rows | `tests/fixtures/infile/soil/compat_quoted_header_7778_per_ofe_restrictive.sol` | strict | reject | pass |
| soil | quoted `7778` + per-OFE restrictive rows | `tests/fixtures/infile/soil/compat_quoted_header_7778_per_ofe_restrictive.sol` | compatibility | accept, normalize restrictive row | pass |

Ran:
- `cargo test -p openwepp --test infile_slope_parser_contract --test infile_soil_parser_contract`
  - slope: `18 passed`
  - soil: `11 passed`
