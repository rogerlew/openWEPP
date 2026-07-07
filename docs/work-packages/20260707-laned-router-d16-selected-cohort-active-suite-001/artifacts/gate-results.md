# Gate Results

Status: EXECUTED-HOLD-ACTIVE-RUN. Evidence mode: Ran unless marked Static.

| Gate | Status | Evidence |
|---|---:|---|
| `git diff --check` | PASS | Ran after artifact finalization; no output. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path <package> --format plain`: `18 files validated, 0 errors, 0 warnings`; README lint: `1 files validated, 0 errors, 0 warnings`. |
| `tools/owcmp/owcmp env --manifest` for Minnesota corn | PASS | `artifacts/owcmp-env-minnesota-corn.json` has `status = PASS`. |
| `tools/owcmp/owcmp env --manifest` for N Idaho | PASS | `artifacts/owcmp-env-n-idaho.json` has `status = PASS`. |
| `tools/owcmp/owcmp env --manifest` for WA Cascades | PASS | `artifacts/owcmp-env-wa-cascades.json` has `status = PASS`. |
| Package-local native management generation | PASS | `artifacts/materialize_selected_cohort.py` generated four selected run dirs. |
| Active plain run for every selected member | FAIL | `h2637` PASS; `mn_corn_h4` active plain FAIL on Rev-21 `canhgt` guard. |
| Explicit active hybrid run for every selected member | BLOCKED | `h2637` PASS with `hybrid_implicit_stepping = true`; remaining members blocked by `mn_corn_h4` active plain failure. |
| Closure evidence from each run manifest | PARTIAL | H2637 plain/hybrid closure recorded; external manifests absent due stop-on-failure. |
| Plain-vs-hybrid output hash and delta evidence | PARTIAL | H2637 only; selected-cohort suite incomplete. |
| H2637-class timing run with active routed path enabled | PASS | Plain `39.64 s` user; hybrid `33.33 s` user. |
| Contract/profile/BEI checks | NOT RUN | No `SC-*` contract changed. |
| Authority anti-evasion guard | NOT RUN | No required-case binding, durable cohort fixture, or external-authority suite posture changed. |
| `cargo fmt --check` | PASS | Ran successfully; no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Finished clean. |
| `cargo nextest run --workspace --profile full` | PASS | `1439 tests run: 1439 passed (5 slow), 4 skipped`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |

Final gate status will remain held even if repository-wide hygiene gates pass,
because the selected active suite itself failed.
