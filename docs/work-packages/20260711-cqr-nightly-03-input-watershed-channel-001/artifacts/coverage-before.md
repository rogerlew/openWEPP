# Coverage Before

Ran: fresh full-workspace LCOV reported `393/520` target lines (`75.577%`),
below the ADR-0021 glue-tier `85%` cover-first floor.

Before any production extraction, a tests-only characterization run used:

`cargo llvm-cov --workspace --test infile_watershed_channel_parser_contract`

It reached `466/520` lines (`89.615%`) and `592/651` regions (`90.937%`). Raw
tests-first hashes are recorded in `coverage-closure.md`. The subsequent
implementation attempt was rolled back for the contract mismatch described in
`hold-legitimacy-audit.md`; landed source/test posture is scaffold `a7d07708`.
