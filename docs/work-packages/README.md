# Work Packages

Initiative tracking convention inherited from wepp-palimpsest. Each work package lives in a dated directory under this tree.

## Directory naming
`YYYYMMDD-<short-slug>/`

## Required files
- `package.md` — scope, deliverables, dependencies, exit criteria
- `prompts/` — agent prompts (active and archived)
- `artifacts/` — produced docs, contracts, evidence

## Phase shape (inherited from wepp-palimpsest)
- **Phase 0**: docs-only audit / inventory
- **Phase 1**: architecture decision with operator-signed acceptance
- **Phase 2**: single-mechanism implementation, replay-and-checkpoint between mechanisms
- **Phase 3**: closeout disposition

## Conventions
- Dates are UTC.
- Evidence classification per claim: `[DIRECT]` (read source / contract / output) vs `[INFERENCE]` (reasoned from evidence).
- Evidence mode per assessment: **Static** (read and reasoned) vs **Ran** (commands actually invoked).
- Single-mechanism rule: one landed change per replay checkpoint.

## Queued packages

Authorized packages:

- `20260511-openwepp-runner-bootstrap/`
  - Purpose: establish runner boundary, release-sidecar contract, and release
    lint gates before kernel implementation.
- `20260520-arch01-subsystem-map-and-contract-spine/`
  - Purpose: architecture discovery for subsystem boundaries, state-surface
    ownership, top-down invariant cataloging, legacy `.run`/sidecar
    compatibility bridge definition, and comparator confidence-tier policy.
- `20260520-sci01-50201000-process-contract-mapping/`
  - Purpose: map `references/50201000` chapters to process-based science
    contract domains and seed invariant families for top-down contract
    authoring.
