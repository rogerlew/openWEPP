# Codex Execution Prompt: D16 Disturbed Route-Coefficient Source Acquisition

Execute
`docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/`.

Subagent authorization: this package explicitly authorizes
spawning/delegation to implementation, science-authority review, WEPPpy QA,
openWEPP verification, comparator/timing, and documentation subagents for
bounded work on the intended write set. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`,
`artifacts/coefficient-due-diligence.md`, and compact command/comparator
evidence. Write access is read-only unless a worker is explicitly assigned a
bounded implementation fix in either openWEPP or WEPPpy.

Primary objective: make WEPPpy Disturbed the canonical native `ow-lanuse-1`
producer for Lane-D route coefficients through the extended lookup table, then
prove openWEPP active Lane-D consumes the generated native coefficients.

Critical constraints:

- Do not infer the five route coefficients mechanically from legacy WEPP fields.
- Do not use the H2637 `500.0 0.0 0.0 0.0 0.0` timing recipe as cohort policy.
- Preserve legacy WEPP/Fortran management outputs unless native openWEPP mode
  is explicitly selected.
- Hold, do not fill gaps, if coefficient due diligence cannot defend every
  required row.
- Record Static vs Ran evidence in every audit/review/verification artifact.

Start with `package.md`, then complete the due-diligence artifact before
implementation edits.
