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
  `7a95025d00a8dd655f1a858c6990573c85ca202a11228520ed2370ba2d09cea3`.

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

Corrected terminal subject
`117e43ab1803cbe5d9e3bac8afdd254a7349a044`:

- combined advisory-authority, quality-observatory, and AUTH11 contract run:
  PASS, 14 / 14;
- science-contract admission from the package base: `A0_ADMITTED`,
  39 contracts and zero science surfaces;
- authority anti-evasion: PASS;
- formatting and diff hygiene: PASS; and
- retained evidence manifest: PASS, 9 / 9.
