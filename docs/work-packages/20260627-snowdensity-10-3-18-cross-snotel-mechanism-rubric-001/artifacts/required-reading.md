# Required Reading

Static:

- `docs/planning/snow-frost-fidelity-strategy.md` §10.2/§10.3, including the
  queued 10.3.18 diagnostic and the 10.3.17 non-promotion carry-forward.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-050`, `GAP-SNOWFREEZE-002`, `REF-SNOWFREEZE-FROST-OBS`, and
  `TOL-SNOWFREEZE-011`.
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`.
- `tests/fixtures/snotel_observed/README.md`.
- `tests/fixtures/cancov_forest/README.md` and
  `tests/fixtures/cancov_forest/observations/README.md`.
- `tools/snowfreeze_observed/snotel_density_three_way.py`.
- Prior package lineage: SNOWFROST-FIDELITY-H and SNOWDENSITY-10.3.17.

Conclusion: no SC-SNOWFREEZE-001 amendment was needed. The package consumes the
existing `INV-SNOWFREEZE-050` rubric authority and adds no new gate.
