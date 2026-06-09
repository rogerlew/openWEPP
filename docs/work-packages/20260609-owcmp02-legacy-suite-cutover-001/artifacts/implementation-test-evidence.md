# Implementation and Test Evidence

Status: complete
Evidence mode: Static + Ran

## Active Cutover Edits

- `README.md`
  - Repository layout now lists `tools/owcmp`.
  - Python setup now syncs `tools/owcmp/requirements.lock.txt`.
  - Dependency source list now points to `tools/owcmp`.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `TOL-SYSTEM-007` now points to
    `tools/owcmp/configs/pl14s_wat_tolerances.json`.
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
  - Static includes now bind to `tools/owcmp/semantic_wat.py`,
    `tools/owcmp/pl14s_suite.py`, and `tools/owcmp/README.md`.
  - Duplicate row-key runtime check now invokes
    `python3 tools/owcmp/owcmp wat semantic ...`.
- `tools/owcmp/README.md`
  - Now describes `owcmp` as the active PL14S comparator namespace.
  - Carries the PL14S strict-lane, source-class, row-consistency, partition, and
    year-offset guard posture previously exposed by the legacy README.
- `tools/owcmp/requirements.lock.txt`
  - Generated-command comments now reference `tools/owcmp`.
- `docs/work-packages/README.md`
  - Added OWCMP02 package discoverability entry.

## Deletion Evidence

`tools/legacy_comparison_suite` was deleted.

Command:

```bash
test ! -e tools/legacy_comparison_suite && echo legacy_suite_absent
```

Observed:

```text
legacy_suite_absent
```

## Focused Test Results

- `python3 -m py_compile tools/owcmp/semantic_wat.py tools/owcmp/pl14s_suite.py tools/owcmp/summary.py tools/owcmp/owcmp`
  - PASS.
- `cargo test --test owcmp_cli_contract`
  - PASS, 7 passed.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract`
  - PASS, 8 passed.
- `cargo fmt --check`
  - PASS.
- `git diff --check`
  - PASS.
- `find tools/owcmp tests/integration docs/work-packages/20260609-owcmp02-legacy-suite-cutover-001 -type d -name __pycache__ -print`
  - PASS after cleanup; no output.
