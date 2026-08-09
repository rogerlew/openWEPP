# Review Agent B: SC-VEGETATION-001

Status: `complete / initial HOLD remediated`

Date: 2026-08-08 UTC

Evidence mode: `Ran + Static + primary-source discovery`

Canonical contract:
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`

Reviewed commit SHA:
`ea1df89d78fa7a79d7b1d0aac4f81899b90c68f0`

## Findings

### B-01 — Critical — Exact cited table and figure routes were omitted

- File refs:
  `artifacts/authority-route-attempts.md:7-22`,
  `artifacts/schema-profile-initial-state-gate.md:19-25`.
- Issue: the initial record did not follow the bibliography's exact White,
  Reich, Hwang, Ford, Coweeta, and related primary-source leads.
- Scientific/governance impact: the package could not claim an exhausted
  authority search or distinguish species observations from catchment/model
  parameters.
- Proposed disposition: `accepted`; inspect the exact routes and record both
  recovered authority and counterevidence.

### B-02 — High — The claim that observations were absent was overbroad

- File refs:
  `artifacts/authority-route-attempts.md:16-18`,
  `artifacts/schema-profile-initial-state-gate.md:61-76`.
- Issue: Ford, Day, Monk, and related Coweeta sources contain dated partial
  stand/species observations even though they do not form the selected complete
  initial state.
- Scientific/governance impact: inaccurate absence language weakens the hold
  rationale and hides the actual stand/date/topology incompatibility.
- Proposed disposition: `accepted`; record partial observations and identify
  the unselected stand/date plus missing C/N/root coverage.

### B-03 — High — Exact 71-by-2 selected-field ledger was missing

- File refs:
  `artifacts/selected-field-ledger.md:7-29`,
  `artifacts/selected-field-ledger.md:31-101`.
- Issue: the review boundary lacked a row-complete pine/oak account of raw
  values, role, units/basis, alias posture, source route, domain issue, and
  disposition.
- Scientific/governance impact: aggregate claims could not be reconciled to
  every candidate input.
- Proposed disposition: `accepted`; add exactly 71 rows for both selected
  profiles and preserve every held/rejected value.

### B-04 — High — Alias authority was assigned to implementation

- File refs:
  `artifacts/schema-profile-initial-state-gate.md:38-47`,
  `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:371-379`.
- Issue: parser/runtime tests can prove mechanics but cannot establish semantic,
  unit, cadence, or basis equivalence for a candidate alias.
- Scientific/governance impact: implementation evidence could improperly create
  science authority.
- Proposed disposition: `accepted`; require reviewed source/contract authority
  before implementation proof.

### B-05 — High — Wet-canopy `gsurf_*` fields were incorrectly excluded

- File refs:
  `artifacts/selected-field-ledger.md:89-90`,
  `artifacts/canopy-water-energy-gate.md:13-16`.
- Issue: `gsurf_intercept` and `gsurf_slope` are consumed by the wet-canopy path,
  so classifying them as non-consumed was incorrect.
- Scientific/governance impact: exclusion would evade a required conductance
  authority decision.
- Proposed disposition: `accepted`; classify both as consumed, reject the
  unauthoritative source law, and hold replacement authority.

### B-06 — High — Full `AUTH-RHEC-001` admission was overclaimed

- File refs:
  `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:355-360`,
  `artifacts/authority-gap-disposition.md:7-12`,
  `tests/integration/vegetation_boundary_authority_contract.rs:110-135`.
- Issue: schema form is admitted, but the complete selected-field declarations
  and reviewed aliases remain missing.
- Scientific/governance impact: a full-admission claim could incorrectly release
  parameter parsing or the implementation successor.
- Proposed disposition: `accepted`; narrow contract, lifecycle, and tests to
  schema-form-only partial admission.

### B-07 — Low — Canonical contract contained a sentence fragment

- File refs:
  `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:342-350`.
- Issue: the constants/parameters prohibition was grammatically incomplete.
- Scientific/governance impact: the intended no-value-admission rule was less
  precise than required.
- Proposed disposition: `accepted`; state affirmatively that no source default,
  profile value, physiological bound, or parameter set is admitted.

### N-01 — Medium — Version-3 protections retained version-2-only wording

- File refs:
  `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:115`,
  `:221-222`, `:243`, `:263`, `:409-411`, `:452`.
- Issue: the first post-fix verification found active canopy-snow,
  fixed-point/fallback, and tolerance protections phrased as applying only to
  version 2 after the contract advanced to version 3.
- Scientific/governance impact: global scope remained fail-closed, but the
  current-version guard and vector wording was ambiguous.
- Proposed disposition: `accepted`; state explicitly that version 3 preserves
  each version-2 prohibition.

### N-02 — Low — B-05 ledger line references were stale

- File refs:
  `artifacts/selected-field-ledger.md:89-90`, this review's B-05 record, and
  `disposition.md` B-05.
- Issue: the first cycle draft cited ledger lines 76-77 rather than the corrected
  wet-canopy rows 89-90.
- Scientific/governance impact: the scientific disposition was correct, but the
  evidence locator was not replayable directly.
- Proposed disposition: `accepted`; update both B-05 artifact references.

## Recommendation

Initial recommendation: `HOLD`.

After two science-remediation passes, the first formal post-fix verification
reported `FAIL` on N-01/N-02 only. Both findings were accepted and corrected;
the final closure record is `verification_agent_b.md`.
