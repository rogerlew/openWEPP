# CRAP after

Status: PASS
Evidence mode: Ran

`cargo crap` reported no target function above 30. The maximum is
`read_wat_values` at 23.0 (cyclomatic 23, coverage 100%). The next
highest rows are `write_totalwatsed3` at 14.0 and
`WatRequiredColumns::from_batch` at 13.0. The former monolithic
`read_wat_batch` is now CRAP 4.0 with 100% coverage.

Raw `crap-after.json` SHA-256:
`df3a152353ffb4891858d4ef3f4c403df7a92d3cb7c602249689ca97f4c5a078`.
