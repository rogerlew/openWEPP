# Gate Results

Status: `EXECUTED-COMPLETE-METRIC-AUTHORITY`
Evidence mode: Ran.

## Required Gates

| Gate | Result | Evidence |
|---|---|---|
| Analyzer py-compile | PASS | `PYTHONPYCACHEPREFIX=/tmp/openwepp-annual-sediment-pycache .venv/bin/python -m py_compile docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/analyze_annual_sediment_metric.py` -> no output |
| Analyzer replay | PASS | `PYTHONPYCACHEPREFIX=/tmp/openwepp-annual-sediment-pycache .venv/bin/python docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/artifacts/analyze_annual_sediment_metric.py` -> regenerated `annual-sediment-metric-replay.json` |
| Replay counts | PASS | `jq` summary: `21` comparisons, `1` pre-rev44 strict blocker, `0` rev-44 blockers |
| `git diff --check` | PASS | no output |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001` -> `18 files validated, 0 errors, 0 warnings` |
| Contract/profile compliance | PASS | `artifacts/kernel-profile-compliance.md` |
| BEI check | PASS-DEFERRED | `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED`, `8` rows, `7` science-review-follow-on rows |
| SC unit compliance | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS: SC unit compliance lint found no findings` |
| Unit registry | PASS | `bash tools/release/check_unit_registry.sh` -> `21 passed` |
| Independent review | PASS-WITH-DISPOSITION | `artifacts/review-codex.md`; one P1 closure-hygiene finding accepted and fixed by this gate-results artifact plus final-disposition update |
| Independent verification | PASS-WITH-DISPOSITION | `artifacts/verification-codex.md`; replay coverage and blocker counts independently reproduced. Closure-hygiene findings accepted and fixed by this gate-results artifact, recorded Markdown lint evidence, and final-disposition update. |

## Conditional Gates

| Gate | Result | Evidence |
|---|---|---|
| Focused Lane D / `ofe_routing` tests | NOT RUN | No Rust code changed and no runtime binding changed. |
| `cargo fmt --check` | NOT RUN | No Rust code changed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | No Rust code changed. |
| `cargo nextest run --workspace --profile full` | NOT RUN | No Rust code changed. |
| `cargo deny check` | NOT RUN | No Rust code or dependency changes. |
| Authority anti-evasion guard | NOT RUN | This package did not touch required-case bindings, cohort fixture posture, or external-authority suite posture. |

## Release-Binary Provenance

The replay uses the prior coupled space-time package's selected-cohort run
outputs. The replay JSON carries the recorded source binary metadata:

- command: `cargo build --release -p openwepp-runner --bins`
- path: `target/release/openwepp-cli-hill`
- SHA256: `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`
- git head at source run: `8faa56b43ed42f54fd40e64a94a002ad372240cc`

Bulk run trees remain in the predecessor package's ignored artifact area; this
package commits compact replay summaries and reproducible analyzer code.
