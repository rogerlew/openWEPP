# Gate Evidence

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| prospective science review | Pass | two independent reviews; Incident 001 amendment independently passed before rerun |
| runner projection | Pass | nextest run `aad41808-006c-49fc-b2a4-2a81ac13278c`: 1 passed, 220 skipped |
| direct-runtime execution | Pass | 116,800 primary daily rows plus 36,500 analytic-ridge daily rows |
| independent reconstruction | Pass | 16/16 primary and 5/5 ridge members across complete frozen state/factor outputs; maximum difference `8.89e-16 kg m^-2` |
| local sensitivity/covariance/saturation | Pass under operator adjudication; retrospective | 8 nonzero central derivatives; ridge correlation `0.999377`; frozen-grid analysis authorized on 2026-07-28 without parameter selection |
| operator governance | Pass | explicit authorization lifts the sole Incident 002 hold; source-authority and empirical-fit limits retained |
| daily synthetic recovery | Pass | `S020-K050` is the sole zero-SSE member |
| terminal-stock ridge | Pass | 5/5 within `1.12e-15 kg m^-2` |
| typed boundary/failure | Pass | 16/16 prescribed cases match variant and exact field |
| Harvard diagnostic | Pass | 28/28 plot rows; amended arithmetic guard; no fitting |
| terminal validator | Pass | 116,800 rows, 16 reconstructions, one truth, five ridge members, 16 failure cases, 28 Harvard rows |
| Rust format | Pass | package-local executor `cargo fmt --check` after formatting |
| Rust Clippy | Pass | package-local executor all targets, warnings denied |
| Rust test harness | Pass, zero tests | package-local binary crate compiled and ran zero unit tests; behavioral evidence comes from execution and validator |
| dependency policy | Pass with configured unmatched-license/advisory warnings | advisories, bans, licenses, and sources all `ok` after versioning the path dependency |
| Python compile | Pass | `analyze.py`, `summarize.py`, and `validate.py` |
| Interpretation figures | Pass | standard-library renderer validates the frozen 16-member grid, sole zero-SSE truth, and five-member ridge before writing three accessible SVGs |
| Markdown | Pass | terminal CAL-05 Markdown set plus authority package, roadmap, and catalog; zero errors/warnings |
| diff hygiene | Pass | `git diff --check` |
| large-output attributes | Pass | both retained daily CSVs have Git LFS attributes |

No full-workspace campaign was run: this package changes no production code,
contract, fixture, or canonical test and uses focused direct gates under the
repository validation strategy.
