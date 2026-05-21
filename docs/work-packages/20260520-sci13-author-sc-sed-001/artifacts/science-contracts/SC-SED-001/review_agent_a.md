# Review Agent A

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Reviewed contract snapshot: `d5b6cf8cd105b8cb613e2ebcbc2015bda0ae446ba1e94f01f08a0cc920b3c85b`

Findings (severity-ordered):

1. `A-001`
- severity: `high`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:4`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:16`
  - `docs/specifications/science-contracts/index.md:37`
  - `docs/work-packages/20260520-sci13-author-sc-sed-001/package.md:77`
  - `docs/specifications/science-contract-authoring-procedure.md:174`
- issue: Contract lifecycle metadata and registry state are inconsistent (`in_review`/`draft`/`static` in contract vs `open`/`proposed`/`none` in registry), leaving governance state ambiguous for this cycle.
- why_it_matters: Promotion/readiness gates depend on canonical lifecycle consistency; mismatch blocks deterministic gate evaluation.
- proposed_disposition: `amend`

2. `A-002`
- severity: `high`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:74`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:94`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:145`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:189`
- issue: Invariant language effectively enforces `Di > 0`, which conflicts with physically valid erosion-inactive forcing states (`Pr = 0` or `tr = 0`) where interrill delivery can be zero.
- why_it_matters: Overly strict sign constraints can produce false hard failures and reduce scientific soundness.
- proposed_disposition: `amend`

3. `A-003`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:49`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:139`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:141`
- issue: `Allowed Degenerate States` contains claim-bearing rows without explicit per-row evidence labeling.
- why_it_matters: Claim-level provenance is required for auditability and deterministic review replay.
- proposed_disposition: `amend`

4. `A-004`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:200`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:4`
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md:4`
- issue: `GAP-SED-003` says companion contracts are "not yet fully authored" even though canonical files already exist as in-review drafts.
- why_it_matters: Stale dependency wording can misstate actual promotion blockers and create governance ambiguity.
- proposed_disposition: `amend`

5. `A-005`
- severity: `low`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:53`
  - `docs/specifications/science-contract-authoring-procedure.md:54`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:16`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md:26`
- issue: Document-level evidence-mode token is lowercase `static` instead of canonical `Static`.
- why_it_matters: Canonical token consistency reduces tooling/policy ambiguity.
- proposed_disposition: `amend`

Recommendation:
- `HOLD`
