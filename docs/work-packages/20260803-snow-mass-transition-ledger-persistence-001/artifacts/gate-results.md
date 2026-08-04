# Gate Results

Status: `PASS / exact-current direct and heavy gates complete`

Evidence mode: `Ran`

Source anchor: scaffold commit `3490ca153106`; exact runtime candidate SHA-256
`4e0ebd96da7daa74c6a2c22dce200c87208997df9ac424a0e0b31de83b51da47`.

| Gate | Result |
|---|---|
| Format/diff/syntax | `cargo fmt --all -- --check`, `git diff --check`, Python compile, and assurance JSON parse: PASS |
| Full workspace Clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS after direct test-only lint remediation |
| Focused science/contract | seven binaries `37/37`; post-lint ledger+EB03 set `24/24`; ledger contract `8/8`; typed errors `2/2`; capture selector `2/2`: PASS |
| Runner | `cargo nextest run -p openwepp-runner`: `228/228` PASS |
| Layout/footprint | constructor `4112 B`, day frame `15552 B`, partition `656 B`, compact bundle `112 B`: PASS |
| Paired real CLI | `paired-suite baseline-final/candidate-final` and `compare`: PASS; trace/WAT/HBP byte identity, selectors PASS, wall ratio `0.964632`, RSS ratio `1.018441` |
| Dependency policy | `cargo deny check`: PASS; existing unmatched `MIT-0` allowance warning only |
| Markdown | `markdown-doc lint` over package, contract, catalogs, and roadmaps: `38` files, zero errors/warnings |
| Assurance | `validate --all`, snow `inspect`, `plan --all`: PASS; generation `57f3999b...`, DRAFT, zero active events/public reports; adoption check `changed: false` |
| Quick | `cargo nextest run --workspace --profile quick`: `2172/2172` PASS in `2207.342 s`; 38 skipped |
| Frost | `cargo nextest run --workspace --profile frost`: `352/352` PASS in `532.530 s`; 1912 skipped |
| Critical full | `cargo nextest run --workspace --profile full`: `2221/2221` PASS in `2204.266 s`; 31 skipped |
| Workspace doctests | `cargo test --doc --workspace`: PASS; zero failures |

The first clean quick attempt was externally interrupted after `2169/2172`
passes while three unrelated Iwagaki refinement tests were still CPU-active.
Those exact tests then passed `3/3` in `146.314 s`, and the complete clean
quick rerun passed `2172/2172`. The interruption is not counted as a source or
science failure. Exact logs are retained under
`target/snow_mass_transition_ledger_persistence/gates/` as
`ofe-timeout-recheck.log`, `nextest-quick-rerun.log`,
`nextest-frost-rerun.log`, `nextest-full-rerun.log`, and
`workspace-doctests.log`.
