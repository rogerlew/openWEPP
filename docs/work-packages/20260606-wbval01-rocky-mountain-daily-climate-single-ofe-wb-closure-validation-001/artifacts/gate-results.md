# Gate Results

Status: complete

Evidence mode: Ran

Ran:

| Gate | Result | Evidence |
|---|---|---|
| Release hillslope binary build | pass | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` |
| Direct legacy `.run` front door | fail closed | `CLIHILL-E-010` invalid TOML for `/wc1/.../p1.run` |
| Generated TOML wrapper batch | mixed, expected characterization | `12/22` WAT emitted; `10/22` fail-closed domain blockers |
| Required WAT term population | pass for emitted WAT | `0` missing required residual terms across `12` emitted ledgers |
| Annual closure classification | conservation-break | `12/12` emitted hillslopes break `1.0 mm/year` in years `2..6` |
| Watershed routing preview | not run | Optional observe-only scope skipped because single-OFE ledger is incomplete |
| Package Markdown lint | pass | `markdown-doc lint --path docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001 --no-ignore` validated `24` files |
| Roadmap Markdown lint | pass | `wctl doc-lint --path docs/work-packages/README.md` validated `1` file |

Also ran `git diff --check`: pass.
