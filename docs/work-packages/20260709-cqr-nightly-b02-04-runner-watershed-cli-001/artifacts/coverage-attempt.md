# Coverage Attempt

Ran: delegated isolated target measurement against attempted source SHA-256
`1a54cd889139ab8fbc0c65c458538f031094a7a972cc0392fa85d6f4d1650c24`.
The instrumented `watershed_cli_behavior_contract` suite passed `29/29` tests
in 127.11 seconds.

The target has no `#[cfg(test)]` block. Direct source coverage is `1524/2049`
lines (74.378%) and `1859/5100` regions (36.451%). Even the glue-tier 85%
line/region threshold fails, as does the science-tier threshold. Thirty-three
production functions are below the 75% coverage floor, including CLI dispatch,
runfile parsing, source-runfile area, sidecar/groundwater authority, manifest,
MOFE carry, and topology helper paths.

LCOV SHA-256: `a7ef7512c8326874dfa0af63dea3b7343d497c96783648abfdd970d0e1a15443`.
LLVM JSON SHA-256:
`1a7688d232cede6daba9f90e0129519172c19bade305090c224bcdc597ad42fd`.
