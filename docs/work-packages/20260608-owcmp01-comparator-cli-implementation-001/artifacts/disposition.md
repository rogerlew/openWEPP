# Disposition

Status: complete

## Decision

OWCMP01 is complete.

The package implemented `tools/owcmp` as the first-class comparator CLI path for
the active PL14S WAT semantic and replay-suite behavior while preserving
`tools/legacy_comparison_suite` unchanged for the follow-on cutover package.

## Acceptance Criteria Status

- `tools/owcmp/owcmp wat semantic` preserves `pl14s-semantic-wat-v2` behavior:
  met by porting the legacy semantic comparator byte-for-byte and exercising
  duplicate-key and semantic/summarize smoke tests.
- `tools/owcmp/owcmp pl14s run` preserves `pl14s-legacy-suite-v2` behavior:
  met by porting the legacy suite runner with only namespace/path changes and
  exercising a dynamic fixture-backed `pl14s run` provenance test.
- `tools/owcmp/owcmp summarize` emits compact file-backed summaries:
  met by `summary.py` and regression tests for semantic reports, policy-skipped
  commands, and failed command verdicts.
- Tolerance config identity is proven:
  met by SHA-256 and `cmp` evidence in `tolerance-identity.md`.
- Focused `owcmp` tests pass:
  met by `cargo test --test owcmp_cli_contract` with 7 passed.
- Existing legacy-suite contract tests are not weakened:
  met by unchanged `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract`
  with 8 passed.
- No long-lived silent compatibility wrapper was added:
  met; `tools/legacy_comparison_suite` was not modified and no wrapper delegates
  from the old path.
- `owcmp observe normalize` remains deferred:
  met; the command fails closed with an explicit deferred-boundary message.

## OWCMP02 Readiness

OWCMP02 can start for the planned path cutover:

- Retarget active docs/tests from `tools/legacy_comparison_suite` to
  `tools/owcmp`.
- Prove the retargeted tests pass.
- Delete `tools/legacy_comparison_suite`.

OWCMP02 must not treat OWCMP01 as completing full manifest validation. OWCMP01
`manifest run` is intentionally limited to PL14S `args` pass-through. Full
manifest schema, identity-evidence, tolerance-profile, and promotability
validation remains future work unless OWCMP02 explicitly chooses to add it.

## Residual Risks

- Dynamic parquet/partition/year-offset coverage remains narrower than the
  inherited legacy behavior. This is acceptable for OWCMP01 because the
  semantic comparator file was copied byte-for-byte and the existing legacy
  contract test still passes.
- Full manifest validation is deferred and documented.
- Historical work-package artifacts still reference `tools/legacy_comparison_suite`;
  OWCMP02 should decide whether to preserve those as historical evidence or
  exclude them from active-reference cleanup criteria.
