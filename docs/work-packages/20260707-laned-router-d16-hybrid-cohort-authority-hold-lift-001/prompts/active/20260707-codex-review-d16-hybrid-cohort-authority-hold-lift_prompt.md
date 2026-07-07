Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity required.

Execution mode: package-end-to-end review.

Package:
`docs/work-packages/20260707-laned-router-d16-hybrid-cohort-authority-hold-lift-001/`

Subagent requirement: review only. This prompt explicitly authorizes subagent
spawning/delegation to `rust_code_reviewer`, `rust_qa_reviewer`,
`verification_runner`, `comparator_suite_runner`, and `explorer` roles for
route-coefficient authority review, owcmp/cohort verification, package gate
review, and bounded codebase questions. Outputs: compact findings plus artifact
paths. Write access: read-only unless explicitly reassigned by the parent
agent.

Required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- Package `package.md` and `artifacts/*.md`

Task:

Review whether the package legitimately closes as
`EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY`.

Questions:

1. Does current authority really forbid constructing the external cohort by
   inferring route coefficients from legacy cropland fields?
2. Does the evidence prove selected cohort roots lack native `ow-lanuse-1`
   `routing_coefficients` and executable active run inputs?
3. Does the package avoid overclaiming H2637-only evidence?
4. Are gate statuses and not-run rationales truthful for a docs/evidence-only
   hold?
5. Is the worker handoff actionable enough to lift the hold in the next
   package?

Output protocol:

- Findings first, severity ordered, with file:line references.
- Then residual risk and missing gates.
- Verdict: `GO`, `GO-WITH-AMENDMENTS`, or `NO-GO`.
- Write the final review to a package-local `artifacts/review-*.md` file if
  assigned write access; otherwise return compact text for the parent to file.
