# Coverage Before

Baseline LCOV:

- `/tmp/openwepp-cqr-nightly-new-isolated.lcov`
- SHA-256:
  `87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`

Baseline CRAP rows show two slope parser functions above the nightly threshold:
`SlopeParserError::fmt` at 0% coverage and `parse_slope_str` at 71.7647%
coverage. The package must add characterization for display/error paths and the
top-level parser orchestration before production decomposition can be
dispositioned complete.
