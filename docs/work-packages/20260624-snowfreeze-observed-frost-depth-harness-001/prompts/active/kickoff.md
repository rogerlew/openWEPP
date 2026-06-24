# Kickoff Prompt

Execute `docs/work-packages/20260624-snowfreeze-observed-frost-depth-harness-001/package.md`.

The objective is to acquire the frost-depth observation datasets, normalize
them with provenance locks, and build a local openWEPP comparison harness
against `tests/fixtures/snowfreeze_observed/`.

Do not resume R7H compatibility frost parity. Do not change frost physics in
this package unless the package is amended with contract authority first.
Direct remains opt-in.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only data-provenance, harness-design,
science-contract, and verification subagents. Record findings in
`artifacts/review-disposition.md` and `artifacts/verification.md`.

Required first actions:

1. Read root, work-package, science-contract, and tests instructions.
2. Fill the dataset inventory and observation schema artifacts.
3. Implement acquisition/normalization and harness work only after the data
   governance policy is explicit.
4. Run required focused, workspace, deny, and anti-evasion gates before closure.
