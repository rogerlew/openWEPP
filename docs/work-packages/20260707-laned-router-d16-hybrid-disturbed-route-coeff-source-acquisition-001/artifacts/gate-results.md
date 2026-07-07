# Gate Results

Status: EXECUTED-HOLD-D16-SUITE. Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| WEPPpy repo status | PASS | `master...origin/master [ahead 1]`; working-tree changes are scoped to management/Disturbed/docs/tests. |
| openWEPP repo status | PASS | `git status --short --branch` inspected during execution; changes are scoped to package docs, LANUSE authority doc, fixture, and focused projection test. |
| Coefficient due diligence | PASS | 26 base classes / 21 static extended classes covered, no `unsupported` rows. |
| WEPPpy implementation tests | PASS | Focused bundle: `114 passed, 2 warnings`; route tests: `6 passed, 2 warnings`. |
| WEPPpy docs/ADR lint | PASS | README, ENDUSER, and ADR scoped `wctl doc-lint`: 0 errors/warnings. |
| openWEPP parse/projection tests | PASS | Parser native cropland test and Disturbed projection test passed. |
| openWEPP active missing-coeff guard | PASS | `h2637_active_fails_closed_without_routing_coefficients` passed. |
| openWEPP active Disturbed cohort | HOLD | Full selected D16 active plain-vs-hybrid cohort run not executed. |
| D16 active preflight | HOLD | Source-authority blocker is lifted; D16 suite remains follow-on. |
| `git diff --check` | PASS | `git diff --check && git -C /home/workdir/wepppy diff --check`, exit `0`. |
| Markdown/doc lint | PASS | Package path, work-package README, and LANUSE authority doc: 0 errors/warnings. |
| `cargo fmt --check` | PASS | exit `0`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | exit `0`. |
| `cargo nextest run --workspace --profile full` | PASS | `1439` passed, `4` skipped. |
| `cargo deny check` | PASS | advisories/bans/licenses/sources all ok. |
| Anti-evasion guards | NOT REQUIRED | No required-case binding, external authority suite posture, or cohort fixture binding changed. |
