# Review Agent A

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Reviewed contract snapshot: `a45968d831c040fa30714d5756e60c8e07a33aee887524ab874b24daf0f982b0`

Findings (severity-ordered):

1. `A-001`
- severity: `high`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:52`
  - `docs/specifications/science-contract-authoring-procedure.md:53`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:16`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:26`
- issue: Document-level evidence tokens use lowercase `static` instead of the normative `Static` value.
- why_it_matters: Procedure-level policy checks and review readability depend on canonical evidence-mode tokens.
- proposed_disposition: `amend`

2. `A-002`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:49`
  - `docs/specifications/science-contract-authoring-procedure.md:50`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:146`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:190`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:193`
- issue: Claim-bearing rows in `Allowed Degenerate States` and `Tolerance and Numeric Notes` are not evidence-tagged.
- why_it_matters: Missing claim-level provenance weakens auditability for behavioral and numeric assertions used in promotion gates.
- proposed_disposition: `amend`

3. `A-003`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:59`
  - `docs/specifications/science-contract-authoring-procedure.md:90`
  - `docs/specifications/science-contract-authoring-procedure.md:99`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:91`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:107`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:142`
- issue: `τfe` is an externally relevant and invariant-referenced symbol but is missing from the Symbol Alias Map.
- why_it_matters: Symbol traceability is incomplete at a critical erosion-coupling boundary surface.
- proposed_disposition: `amend`

4. `A-004`
- severity: `low`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:48`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:60`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:63`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:64`
- issue: Authority-anchor source-path style is inconsistent (`references/50201000/chap10.pdf` and bare `chap10.pdf` both appear).
- why_it_matters: Inconsistent citation-path style increases replay friction for provenance audits.
- proposed_disposition: `amend`

Recommendation:
- `HOLD`
