# Review Agent A: SC-VEGETATION-001

Status: `complete / initial HOLD remediated`

Date: 2026-08-08 UTC

Evidence mode: `Ran + Static`

Canonical contract:
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`

Reviewed commit SHA:
`ea1df89d78fa7a79d7b1d0aac4f81899b90c68f0`

## Findings

### A-01 — Critical — Replacement-authority routes were not exhausted

- File refs:
  `artifacts/authority-route-attempts.md:7-22`,
  `artifacts/schema-profile-initial-state-gate.md:14-25`.
- Issue: the first Gate 1 draft rejected the selected GIS rows without following
  the exact White/Reich/Hwang/Ford and site-observation leads far enough to
  distinguish absent authority from unattempted authority.
- Scientific/governance impact: an unexhausted route cannot support a terminal
  scientific hold.
- Proposed disposition: `accepted`; inspect and record each concrete route,
  then narrow the hold to the unresolved cell mapping and compatible-state
  boundary.

### A-02 — Critical — Gates 2 and 3 stopped on missing selected values

- File refs:
  `artifacts/canopy-water-energy-gate.md:7-23`,
  `artifacts/carbon-phenology-root-gate.md:7-24`.
- Issue: the initial review did not independently adjudicate symbolic equation,
  domain, guard, ownership, and vector authority after Gate 1 failed.
- Scientific/governance impact: a value blocker cannot silently defer separable
  process-authority work required by the package.
- Proposed disposition: `accepted`; evaluate every Gate 2 and Gate 3 family and
  state its parameter-independent residual blocker.

### A-03 — High — Gate 1 hold overgeneralized distinct outcomes

- File refs:
  `artifacts/schema-profile-initial-state-gate.md:38-59`,
  `artifacts/schema-profile-initial-state-gate.md:61-82`.
- Issue: acquisition rules, schema form, selected empirical values, and dated
  initial state were presented as one indivisible failure.
- Scientific/governance impact: valid separable authority would be withheld and
  the implementation dependency would be misstated.
- Proposed disposition: `accepted`; report separate acquisition, schema-form,
  selected-value, and initial-state decisions.

### A-04 — High — Separable acquisition/schema authority was not canonical

- File refs:
  `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:271-274`,
  `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:355-391`.
- Issue: strict local identity, immutable raw/resolved separation, schema-field
  metadata, and dated-state identity were supportable but absent from the
  canonical contract.
- Scientific/governance impact: leaving them only in package narrative would
  make the admitted boundary non-binding.
- Proposed disposition: `accepted`; amend the canonical contract and add direct
  contract-derived assertions without admitting a profile value or runtime.

### A-05 — Medium — Readiness conflated typed surface and populated values

- File refs:
  `artifacts/calibration-readiness-matrix.md:11-22`.
- Issue: the initial readiness result did not credit type/schema progress while
  retaining value, execution, sensitivity, and identifiability blockers.
- Scientific/governance impact: calibration-readiness evidence would not
  distinguish architecture from empirical readiness.
- Proposed disposition: `accepted`; record partial schema/observation-operator
  progress and retain all execution/calibration holds.

### A-06 — Medium — Final wording did not state the full hold boundary

- File refs:
  `artifacts/disposition.md:7-16`,
  `artifacts/authority-gap-disposition.md:7-20`.
- Issue: lifecycle language needed to cite all three gates and describe only
  partial schema-form admission for `AUTH-RHEC-001`.
- Scientific/governance impact: stale language could be mistaken for selected
  parameter/state or successor readiness.
- Proposed disposition: `accepted`; make the terminal claim explicitly partial
  and retain all Gate 2/3 and selected-state blockers.

## Recommendation

Initial recommendation: `HOLD`.

After the accepted amendments, the reviewer reported `PASS`; the separate
post-fix closure record is `verification_agent_a.md`.
