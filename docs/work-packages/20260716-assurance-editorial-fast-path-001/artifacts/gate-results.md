# Focused Gate Results

Evidence class: Ran

Candidate base: `25bcb17f4a62924976a19381e974a36612ed4845`

The final pre-review candidate passed:

- `cargo fmt --all` and `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`;
- normalization and deterministic transaction unit selection: 10/10 tests;
- `assurance_v2_normalization_contract`: 5/5 tests in 9.15 seconds;
- `cargo nextest run --workspace --profile assurance-editorial`: 65/65 tests
  in 9.44 seconds;
- the real production `normalize --language en-US --check`, with no changes and
  equal old/new report roots;
- selected-report validation, with lifecycle `DRAFT` and source root
  `08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`;
- `markdown-doc lint` over 11 changed Markdown files, with zero errors and zero
  warnings;
- American-English preview over the same documentation set, with no drift; and
- `git diff --check`.

The first canonical heavy closure preserved a CRAP failure at 4 raw / 2
adjudicated / 2 actionable. After decomposition, the terminal heavy candidate
passed formatting, workspace warnings-as-errors Clippy, dependency policy, and
2,063/2,063 full-workspace tests. Its fresh census passed at 2 raw / 2
adjudicated / 0 actionable; maximum touched CRAP is exactly 30.0 and maximum
normalization CRAP is `15.101256515775034`. The real normalization check,
selected validation, and diff checks also pass. Exact timings, identities, and
both census generations are retained in `heavy-gate-runner.md` and
`validation-evidence/`.

The editorial profile intentionally excludes publication integration because
normalization is restricted to a pre-review DRAFT. The package-level full
workspace, dependency, and adjudicated-CRAP results were recorded separately by
the delegated heavy-gate runner.
