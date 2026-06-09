# Worker Handoff

Status: complete

## Current State

OWCMP01 implemented the parallel `tools/owcmp` comparator path and validated it
without deleting or retargeting `tools/legacy_comparison_suite`.

Primary new commands:

```bash
tools/owcmp/owcmp wat semantic ...
tools/owcmp/owcmp pl14s run ...
tools/owcmp/owcmp summarize --input <report.json> --output-root <dir>
tools/owcmp/owcmp manifest run --manifest <manifest.json>
```

`owcmp observe normalize` is intentionally deferred and fails closed.

## OWCMP02 First Actions

1. Retarget active references from `tools/legacy_comparison_suite` to
   `tools/owcmp`, starting with:
   - `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
   - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
   - root README Python comparator setup references, if still active
   - active package prompts/templates that name the legacy suite for future work
2. Run the retargeted PL14S contract test.
3. Delete `tools/legacy_comparison_suite`.
4. Run `rg legacy_comparison_suite` and disposition remaining hits as either
   historical artifacts or active references requiring cleanup.

## Do Not Overclaim

- Full `owcmp manifest run` validation is not implemented in OWCMP01. The
  current manifest command accepts a PL14S lane and an explicit `args` list, then
  dispatches to `pl14s run`.
- OWCMP02 can proceed with legacy-suite path cutover without relying on manifest
  mode. If OWCMP02 wants manifest-driven cutover, add full manifest schema and
  identity validation in that package.
- `owcmp observe normalize` remains future observability work, not part of
  OWCMP02 unless separately authorized.

## Validation Snapshot

- `python3 -m py_compile tools/owcmp/semantic_wat.py tools/owcmp/pl14s_suite.py tools/owcmp/summary.py tools/owcmp/owcmp` - PASS.
- `cargo test --test owcmp_cli_contract` - PASS, 7 tests.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract` -
  PASS, 8 tests.
- `cargo fmt --check` - PASS.
- `git diff --check` - PASS.

## Residual Follow-Ups

- Add dynamic parquet/partition/year-offset coverage if those paths become
  cutover risk.
- Add expected-common-row-count failure coverage through `owcmp` if OWCMP02
  retargeting touches baseline-year policy assertions.
- Add a dedicated manifest package for schema, identity-evidence, tolerance, and
  promotability validation.
