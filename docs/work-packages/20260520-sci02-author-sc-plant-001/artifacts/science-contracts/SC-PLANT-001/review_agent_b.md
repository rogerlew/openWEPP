# Review Agent B

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Reviewed contract snapshot: `1a1b42d0d505304e3cc5e93a2b49d41bac3afd9d67d0208e72e8f4c05702917e`

Findings (severity-ordered):

1. `B-001`
- severity: `high`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:36`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:88`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:105`
- issue: `INV-PLANT-002` was written as global hard-fail but described cropland-only heat-unit semantics while scope includes rangeland.
- why_it_matters: Could trigger false hard failures for valid rangeland states.
- proposed_disposition: `reject` (as written) and replace with scoped invariants.

2. `B-002`
- severity: `high`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:151`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:158`
- issue: Open cross-contract dependency gaps were not marked as promotable/non-promotable.
- why_it_matters: Promotion readiness remains ambiguous without explicit non-promotable labeling.
- proposed_disposition: `amend`

3. `B-003`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:109`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:118`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:134`
- issue: Normative claims were missing claim-level evidence tags in several sections.
- why_it_matters: Evidence-label correctness is incomplete without claim tags.
- proposed_disposition: `amend`

4. `B-004`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:64`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:89`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:91`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:92`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:105`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:147`
- issue: Symbols used in invariants/tolerances were absent from the variables table.
- why_it_matters: Weakens boundary clarity and completeness.
- proposed_disposition: `amend`

5. `B-005`
- severity: `low`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:16`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:26`
- issue: Evidence mode capitalization was inconsistent (`static` vs `Static`).
- why_it_matters: Inconsistency can break policy/tool parsing assumptions.
- proposed_disposition: `amend`

Recommendation:
- `HOLD`
