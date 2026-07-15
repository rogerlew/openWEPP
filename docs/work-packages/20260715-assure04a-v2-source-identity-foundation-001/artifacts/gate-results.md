# ASSURE-04A Gate Results

Status: PASS — amended source heavy gates complete

Evidence class: Static + Ran

## Focused Contract Gates

- `cargo fmt --check`: PASS.
- `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`: PASS.
- focused quick-profile Nextest: PASS, 25/25 tests, run
  `3971cb34-0b18-451b-b52e-2db7c483888c` after B-T01 remediation.
- real `validate --all`, named-report validation, zero-public `check --all`,
  protected-byte assertions, and `git diff --check`: PASS.

## Pre-B-T01 Intermediate Workspace Gates

The delegated runner restarted the complete sequence after the first fresh
CRAP HOLD. These results are retained chronology and were not reused for final
closure after B-T01 changed production code.

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | exit 0 |
| Workspace Clippy, all targets, warnings denied | PASS | exit 0 |
| Full-profile workspace Nextest | PASS | 1,985/1,985 selected passed; 3 skipped; run `363c4169-11bb-43bc-ae1b-acbe3bb07ad1` |
| `cargo deny check` | PASS | advisories, bans, licenses, and sources OK |
| Fresh adjudicated CRAP, threshold 30 | PASS | raw 2; adjudicated 2; actionable 0; actionable touched rows 0 |

That intermediate JUnit SHA-256 is
`a80b01bb1efcaeadc63c00185b5be0047ed0a4ef8de0bad1779b126160527826`.
The full chronology and checksum-bound evidence are in
[`heavy-gate-runner.md`](heavy-gate-runner.md).

The intermediate results apply to the pre-presence-remediation `v2.rs`
identity
`422b62a30e4863122c51898914202d85b6214ab051188829991a787a1d635345`.
Verification B subsequently found that schema-required nullable fields could be
omitted. The amended source passes strict focused Clippy and 25/25 focused
tests, but none of those prior terminal PASS results close the amended tree.

The complete amended-source sequence subsequently passed:

| Gate | Amended-source result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | exit 0 |
| Workspace Clippy, all targets, warnings denied | PASS | exit 0 |
| Full-profile workspace Nextest | PASS | 1,986/1,986 selected passed; 3 skipped; run `8d011a3f-91ca-4814-b310-6b0fc65e6c7a` |
| `cargo deny check` | PASS | advisories, bans, licenses, and sources OK |
| Fresh adjudicated CRAP, threshold 30 | PASS | raw 2; adjudicated 2; actionable 0; actionable touched rows 0 |

The amended JUnit SHA-256 is
`86e802b20b7b23b217e532103526e588c4933cc73ea659bea3b924570acd1faa`.
The fresh production manifest before/after/final is
`9db9abcf7cb4bbd5ef7387bcada9831528d0f5f529ca7656584669739139831a`.
The source freeze records `v2.rs` at
`886a5693d67ab88b0b0a6901260017eeca636aa7ccad1ad0faed7ccf24104b58`.

The amended CRAP acquisition's coverage instrumentation uses its canonical
`--ignore-run-fail` collection posture and recorded unrelated libtest
concurrency/failure diagnostics. That collection is measurement input, not the
workflow gate. The actual required amended full Nextest workflow independently
passed 1,986/1,986 selected tests. The fresh measurement closed with zero
actionable rows and stable before/after/final production-source manifests.

## Historical HOLD And Remediation

The first fresh CRAP acquisition reported four actionable touched functions in
`v2.rs` (CRAP 32.94, 81.42, 33.29, and 60.65). The package did not adjudicate
or waive them. Semantic decomposition was implemented, focused gates reran,
and the full five-gate sequence restarted. The failed report is retained under
[`adjudicated-crap-hold-20260715T160553Z/`](validation-evidence/adjudicated-crap-hold-20260715T160553Z/);
the intermediate passing report is retained under
[`adjudicated-crap-pass-pre-presence-remediation-20260715T165752Z/`](validation-evidence/adjudicated-crap-pass-pre-presence-remediation-20260715T165752Z/).
The current amended passing report is under
[`adjudicated-crap/`](validation-evidence/adjudicated-crap/).

Verification B then issued a separate terminal HOLD for required-nullable
schema/admission parity. The package added a three-state typed representation
that distinguishes missing, explicit null, and present values and added
omission vectors across authorship, dependency, research-object, review, and
publication families. The fresh five-gate sequence passed as recorded above.

## Source And Publication Boundaries

- Amended terminal production manifest SHA-256 before/after/final:
  `9db9abcf7cb4bbd5ef7387bcada9831528d0f5f529ca7656584669739139831a`.
- Protected public catalog, template, export, `usersum` catalog, and aggregate
  `usersum` hashes equal the frozen ASSURE-03 values.
- The v2 catalog still admits exactly one internal `DRAFT`, `fixture_only`
  report and zero public reports.
- No ASSURE-04B planner, ASSURE-04C renderer, ASSURE-04D promotion path,
  release snapshot, or WEPPcloud vendor path was enabled.

## Documentation And Size Gates

Canonical Markdown lint/validation, local links, and final whitespace checks
passed. UK-to-US preview found no conversion in the new package, assurance, or
roadmap text; the full historical work-package catalog still reports unrelated
preexisting conversions outside this write scope. `v2.rs` is 2,042 lines; its
required 2,000-line warning is explicitly dispositioned in
[`line-count-governance.md`](line-count-governance.md).

The local `wctl doc-lint` wrapper could not initialize because its Python
environment lacks `typer`. This was a wrapper-environment failure, not a
Markdown result. Direct canonical `markdown-doc lint` and `markdown-doc
validate` runs passed for the package, roadmap, implementation roadmap, and
work-package catalog.
