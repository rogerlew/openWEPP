# verification_agent_a

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Verification scope: replay execution reproducibility and criteria evidence
  integrity for parquet + dat lanes.

## Ran
- Verified persisted rerun bundle exists and contains candidate, comparator,
  provenance, and hash manifests:
- `artifacts/replay-run-20260525T072842Z/`
- Confirmed parquet lane provenance schema and semantic summary values:
- `strict-equivalent-required`, `strict_equivalent_ready=true`,
  `semantic_pass=false`, `common_row_count=365`, `only_candidate_count=730`.
