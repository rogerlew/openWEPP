# Review Agent A

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Reviewed contract snapshot: `12fc4173d9f2f8a44149d4a36e7fa5dc95c804649e417393711b5eb8ed278633`

Findings (severity-ordered):

1. `A-001`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:100`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:193`
- issue: `INV-SUBHYD-001` requires residual-bounded daily continuity but the contract does not publish an explicit term-level closure identity showing how residual is assembled from Eq. [6.2.1] terms.
- why_it_matters: Without an explicit closure identity, independent implementations can satisfy the invariant with mismatched accounting boundaries.
- proposed_disposition: `amend`

2. `A-002`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:87`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:105`
- issue: Chapter-6 text requires drainage flux to be limited by hydraulic capacity (drainage coefficient), but no explicit invariant/guard captures cap enforcement on `Qdd`.
- why_it_matters: Missing cap semantics can produce non-physical drainage outputs and inconsistent replay/comparator behavior.
- proposed_disposition: `amend`

Recommendation:
- `GO-WITH-AMENDMENTS`
