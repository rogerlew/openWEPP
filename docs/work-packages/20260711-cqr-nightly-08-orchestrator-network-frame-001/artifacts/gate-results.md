# Gate Results

| Gate | Result |
|---|---|
| Package library | PASS: `121/121` |
| Focused network frame | PASS: `10/10` |
| Science-tier coverage | PASS: `92.431%` lines, `94.068%` regions, `66/70` functions; all eligible functions above `75%` |
| Target CRAP | PASS: zero eligible rows above `30`; selected helper `17.126` |
| `cargo fmt --check` | PASS: exit `0`, `2.08s` |
| Workspace clippy | PASS: exit `0`, `7.20s` |
| Full workspace nextest | PASS: `1717/1717`, `4` slow, `3` skipped, run `fa6912ff-84b6-4a6d-a16e-9e7ca1513172`, `613.19s` |
| `cargo deny check` | PASS: exit `0`, `0.85s` |
| `git diff --check` | PASS: exit `0`, `0.02s` |
| Package/catalog Markdown lint | PASS: zero errors/warnings |

Final delegated logs are under
`/tmp/openwepp-cqr-20260711-t08-closure-final/`; the runner edited no file.
