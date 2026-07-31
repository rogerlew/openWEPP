# Owned File Manifest

Status: `complete`

Evidence mode: `Static`

## Package-Owned Changes

| Path | Role |
| --- | --- |
| `tools/local_ci/cqr_quality_evidence.py` | Controlled synthetic self-test dependency seam |
| `tests/integration/assurance_v2_publication_contract.rs` | Preserve and independently schedule 14 negative cases |
| package directory | ExecPlan, prompt, evidence, reviews, disposition |
| `docs/planning/snow-surface-energy-balance-roadmap.md` | EB-03B and EB-04 admission state |
| `docs/ROADMAP.md` | Campaign queue state at final disposition |
| `docs/work-packages/README.md` | Current package catalog |
| EB-03A status/evidence artifacts | Hold-lift reconciliation at final PASS only |

## Inherited Changes

The working tree already contained the complete uncommitted EB-03 and EB-03A
contract, Rust, integration-test, assurance-source/render, roadmap, catalog,
and package changes. EB-03B relies on that exact terminal tree for required
validation but does not claim authorship of, revert, or semantically alter
those changes.

No `.config/nextest.toml`, production assurance, snow-runtime, canonical
science-contract, coefficient, selector, trust-root, or timeout edit belongs
to EB-03B.
