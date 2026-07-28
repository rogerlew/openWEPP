# Focused Validation

Evidence class: Ran.

Working directory: `/home/workdir/openWEPP`.

Initial focused execution:

- `cargo nextest run --test advisory_linter_authority_contract --test
  quality_observatory_workflow_contract`
- Result: FAIL, 10 passed / 1 failed.
- Finding: the live direct-authority impact map still contained the previous
  SHA-256 for the amended canonical testing strategy.

Disposition:

- Added the exact impact-map file to the declared terminal write set.
- Refreshed only its `policy_sha256` to
  `af484a7cd399b0574a257912415b749805e822b12885840afea0ccb0c94b0edd`.

Corrected focused execution:

- `cargo nextest run --test advisory_linter_authority_contract --test
  quality_observatory_workflow_contract`
  - PASS, 11 / 11.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS.
- `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract`
  - PASS, 3 / 3.
- `cargo fmt --all -- --check`
  - PASS.
- `python -m json.tool
  tools/release/authority-policy/impact-map.json`
  - PASS.
- `markdown-doc lint` for the package, roadmap, testing strategy, ADR-0043,
  root roadmap, and work-package catalog
  - PASS, 18 files total, zero errors or warnings.
- deletion assertions for the command, source, focused tests, and tool README
  - PASS.
- `git diff --check`
  - PASS.

No full-workspace, modeling, CAL, synthetic, population, freeze/open, or
Harvard command was run.
