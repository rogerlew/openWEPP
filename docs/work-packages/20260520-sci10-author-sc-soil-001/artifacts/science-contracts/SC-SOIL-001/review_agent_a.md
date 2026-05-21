# Review Agent A

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Reviewed contract snapshot: `887f02651b28840130e7ce8ba79266ea64ea0ca7a5ee41907090e419b7318530`

Findings (severity-ordered):

1. `A-001`
- severity: `high`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:53`
  - `docs/specifications/science-contract-authoring-procedure.md:54`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:16`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:26`
- issue: Document-level evidence mode is written as lowercase `static` in metadata/body instead of canonical `Static`/`Ran` tokens.
- why_it_matters: Policy checks and promotion audits rely on canonical evidence-mode tokens; lowercase variants create normalization ambiguity.
- proposed_disposition: `amend`

2. `A-002`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:99`
  - `docs/specifications/science-contract-authoring-procedure.md:100`
  - `docs/specifications/science-contract-authoring-procedure.md:191`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:78`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:100`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:133`
- issue: `τcadj` is listed as an externally relevant symbol and required by erosion coupling invariants, but the Symbol Alias Map only includes `τcb` and omits `τcadj`.
- why_it_matters: Missing alias coverage breaks symbol-continuity guarantees for a threshold variable consumed by Chapter-11 coupling equations.
- proposed_disposition: `amend`

3. `A-003`
- severity: `medium`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:49`
  - `docs/specifications/science-contract-authoring-procedure.md:50`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:140`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:142`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:146`
- issue: `Allowed Degenerate States` includes non-trivial behavioral claims without per-row claim evidence tags.
- why_it_matters: Claim-level provenance is required for deterministic review replay and auditability.
- proposed_disposition: `amend`

4. `A-004`
- severity: `low`
- file refs:
  - `docs/specifications/science-contract-authoring-procedure.md:43`
  - `docs/specifications/science-contract-authoring-procedure.md:48`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:62`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:92`
- issue: Freeze-thaw anchor `REF-SOIL-CH3-FT` mixes Chapter-3 intro context with Chapter-7 equation sections and a non-rooted `chap7.pdf` path, reducing citation specificity.
- why_it_matters: Ambiguous anchor composition weakens top-down provenance for freeze-thaw adjustment invariants.
- proposed_disposition: `amend`

Recommendation:
- `HOLD`
