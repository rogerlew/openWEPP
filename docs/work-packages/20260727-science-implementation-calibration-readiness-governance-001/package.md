# Science Implementation and Calibration Readiness Governance

Package ID:
`20260727-science-implementation-calibration-readiness-governance-001`

Status: `COMPLETE / GOVERNANCE ADMITTED`

Date opened: `2026-07-27`

Execution mode: `package-end-to-end`

Package type: repository governance and science-contract schema amendment.

## Objective

Formalize the repository-wide rule that authoritative process science should
be implemented when available; measured-data limitations constrain empirical
calibration, identifiability, validation, and transferability claims rather
than blocking the authoritative implementation. Require data-limited
implementations to demonstrate calibration readiness without misrepresenting
assumptions or synthetic recovery as empirical calibration.

## Authority

Direct operator decision in this package's initiating instruction, extending
ADR-0011's architecture-first posture and preserving the authority ranking in
`docs/specifications/correctness-authority-model.md`.

## Included Scope

- Add and register ADR-0042.
- Amend the correctness-authority model.
- Add calibration/identifiability obligations to the science-contract schema,
  kernel profile, and authoring procedure.
- Add operational rules to work-package governance and kernel package
  preparation.
- Add a terse root routing pointer.
- Update the work-package catalog.
- Complete two independent reviews, explicit finding disposition, terminal
  validation, commit, and push.

## Excluded Scope

- No production code, test, fixture, existing `SC-*` contract, dataset, or
  calibration result changes.
- No promotion of measured data to universal blocking authority.
- No permission for surrogate/proxy physics, silent defaults, invented
  physiological bounds, or empirical claims based only on synthetic data.

## Declared Write Set

- `AGENTS.md`
- `docs/decisions/README.md`
- `docs/decisions/0042-science-implementation-and-calibration-readiness.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/science-contract-spec.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/work-packages/README.md`
- this package subtree

Existing uncommitted CAL-04/CAL-04A and Daymet paths are predecessor work and
are not modified by this package.

## Plan

1. Freeze the authority map and exact terminal intent.
2. Author ADR-0042 and register it.
3. Amend canonical authority, contract-schema, package, preparation, and root
   routing documents without duplicating the full doctrine.
4. Run two independent read-only reviews: one for scientific-authority
   coherence and one for operational/schema enforceability.
5. Disposition every finding and correct accepted findings.
6. Run package Markdown lint, repository doc-link/path checks where available,
   diff hygiene, root-size policy, prompt state, and exact write-set
   reconciliation.
7. Archive the kickoff prompt, close the package, commit stable increments, and
   push `main`.

## Acceptance

The package may close `COMPLETE / GOVERNANCE ADMITTED` only if:

- ADR-0042 is accepted and registered;
- A0/A1/A3 remain mandatory and A4 limitations are not confused with science
  invalidity;
- science contracts must classify calibration applicability, parameters,
  observation operators, identifiability, evidence gaps, and prohibited claims;
- legacy contracts have an explicit prospective migration boundary and owned
  backfill record;
- calibration and independent-validation data roles are prospectively distinct;
- readiness obligations have a required auditable matrix;
- work packages cannot hold solely for sparse or non-identifying data when an
  authoritative in-scope implementation/readiness path exists;
- synthetic recovery is explicitly readiness evidence, not empirical
  calibration or real-world validation;
- all findings are dispositioned;
- both reviewers pass the corrected state;
- selected terminal gates pass; and
- the committed and pushed diff matches the declared scope.

## Review and Delegation Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers and two independent
read-only terminal verifiers for scientific authority coherence, schema
completeness, operational enforceability, claim discipline, exact-diff
reconciliation, and closure; expected outputs are compact review/verification
artifacts and final `PASS`/`HOLD` dispositions; write access is read-only.

## Minimum Gates

```text
markdown-doc lint --path docs/work-packages/20260727-science-implementation-calibration-readiness-governance-001
markdown-doc lint --path docs/decisions
markdown-doc lint --path docs/specifications
markdown-doc lint --path docs/standards
git diff --check
test "$(wc -l < AGENTS.md)" -le 160
```
