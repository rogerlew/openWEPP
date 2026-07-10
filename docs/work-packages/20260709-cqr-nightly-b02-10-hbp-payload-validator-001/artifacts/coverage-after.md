# Coverage After

Ran: full workspace LCOV emitted; target LCOV record present.

Command:

`cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-b02-t10-fullcov.lcov`

Output:

- LCOV: `/tmp/openwepp-cqr-b02-t10-fullcov.lcov`
- LCOV SHA-256:
  `ca7128c073a6f04d52dba69a286f711683785116f58628eacb447b868f8852d4`
- log: `/tmp/openwepp-cqr-b02-t10-fullcov.log`
- log SHA-256:
  `21fda25d3f538f68eac2e0c546317b7ddd5ee14018745511c67feb0ffda8215f`
- exit file: `/tmp/openwepp-cqr-b02-t10-fullcov.exit`
- exit content: `EXIT=0`

Coverage-lane caveat: the command used `--ignore-run-fail`; the log records
unrelated failures in `-p openwepp --test laned_shadow_h2637` and
`-p openwepp-hillslope-orchestrator --lib`. The emitted LCOV is used only for
target-module coverage and CRAP-after measurement. Dedicated full-nextest is
recorded separately in `gate-results.md`.

Target module:
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`

| Metric | Covered | Total | Percent |
|---|---:|---:|---:|
| Functions | 47 | 64 | 73.438% |
| Lines | 531 | 621 | 85.507% |
| LCOV branches | 0 | 0 | N/A |

Region coverage: unavailable from the final fullcov JSON export. The attempted
JSON export was written to
`/tmp/openwepp-cqr-b02-t10-fullcov-llvm-export.json` with SHA-256
`446061cdb37bf75d0f2a5eab2776dd9a935f7a009aa7f91b4788141a90003539`, but it
did not include this target file. No region-coverage claim is made.

Focused target-supporting run:

- LCOV: `/tmp/openwepp-cqr-b02-t10-focused.lcov`
- LCOV SHA-256:
  `1b445a42af2df7e2d4be0ea79ee99e00e7d91d0838eeb95cfb7183787de199e1`
- CRAP JSON: `/tmp/openwepp-cqr-b02-t10-focused-crap.json`
- CRAP SHA-256:
  `1d9f0e5994b33253776a216154e13073a6bad7cc894d41f25fe85e6693e0bd14`
- Use: proves the new non-runoff characterization covers
  `parse_non_runoff_event_payload`; not used as the final module CRAP basis
  because it undercovers existing runoff behavior that the full workspace LCOV
  covers.
