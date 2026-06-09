# Legacy Reference Disposition

Status: complete
Evidence mode: Static + Ran

## Command

```bash
rg -l "legacy_comparison_suite|semantic_hillslope_wat_compare|run_pl14s_legacy_suite" -g '!target/**'
```

## Result Summary

- Total files with remaining migration/historical references: `140`.
- Active non-work-package files with remaining references:
  - `tools/owcmp/specification.md`
- Work-package/catalog files intentionally documenting migration:
  - `docs/work-packages/README.md`
  - `docs/work-packages/20260609-owcmp02-legacy-suite-cutover-001/**`
  - `docs/work-packages/20260608-owcmp01-comparator-cli-implementation-001/**`
- Historical work-package artifacts/prompts with prior command evidence:
  - all other `docs/work-packages/20*/**` hits.

## Active Reference Disposition

No active blocker remains.

- `README.md` has no remaining legacy-suite hit.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` has no
  remaining legacy-suite hit and now references the `tools/owcmp` tolerance
  profile.
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
  has no remaining legacy-suite hit and runs against `tools/owcmp`.
- `tools/owcmp/README.md` and `tools/owcmp/requirements.lock.txt` have no
  remaining legacy-suite hit.
- `tools/owcmp/specification.md` keeps legacy-suite references only as migration
  history, compatibility mapping, and completed package-plan criteria.

Historical work-package artifacts are preserved as evidence of what ran at the
time and do not block deleting `tools/legacy_comparison_suite`.
