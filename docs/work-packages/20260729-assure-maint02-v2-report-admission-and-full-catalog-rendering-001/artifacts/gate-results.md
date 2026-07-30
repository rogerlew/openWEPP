# Gate Results

Status: complete / pass

Evidence class: Ran

All commands ran from repository root on the terminal implementation state.

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo nextest run -p openwepp-assurance` | PASS, 32/32 |
| Five affected assurance integration targets | PASS, 62 passed and 2 skipped |
| `cargo nextest run --workspace --all-features` | PASS, 2,161 passed and 5 skipped in 2,802.891 seconds; complete log in `artifacts/gates/assure-maint02-full-workspace-nextest.log` |
| `cargo deny check` | PASS; advisories, bans, licenses, and sources OK; preexisting unmatched MIT-0 allowance warning only |
| `markdown-doc lint` on 27 changed assurance/package documents | PASS, 0 errors and 0 warnings |
| `git diff --check` | PASS |

Focused assurance evidence included:

- admission check/apply/stale/repeat-no-op, malformed/non-draft source, exact
  path spelling, symlink rejection, rollback, and schema-v2 receipt tests;
- archived schema-v1 decoding plus forged/root-mismatch receipt rejection;
- four retained-SVG sanitizer/parser tests, including adversarial CSS,
  namespace, external-resource, active-content, and document-shape cases;
- CAL-09 named-versus-all completeness and byte equivalence;
- the existing generated bar figure and zero-public-report boundaries; and
- three-report source validation and planning.

Direct workflow gates:

- `verify-generation --base-ref
  15763d7f6d5d4125333d9b7583424c714f5f5ea4`: PASS, 22 anchored transitions,
  current generation
  `30db4a7e6a691601426428b7772e28143ff9fa1bf10dd9d1ae80062d7f0002a2`.
- `validate --all`: PASS, three DRAFT reports and zero public reports.
- `plan --all --format json`: PASS, all three reports current.
- `build --all` and `check --all`: PASS in two unrelated roots and the stable
  preview.
- Complete unrelated-root inventories: 91/91 files, byte-identical.
- `cmark-gfm`: PASS for all six top-level report and supplement documents.

The source-level authority-suite anti-evasion gates were not applicable: this
package changes neither external-authority cohort fixtures nor required-case
bindings. The full workspace correctness run is the canonical Critical
campaign gate; no frozen TESTGATE or gate-planner tooling was invoked.
