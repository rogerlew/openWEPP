# Independent Science Review A

Evidence class: `Static exact-diff review`

Verdict: `PASS`

Review scope: the current `SC-VEGETATION-001` version 4 amendment, its focused
contract test, package authority artifacts, Stevens Canyon diagnostic role,
primary-source role declarations, and the coupled-vegetation successor
amendments. Reviewer B output was not consulted.

## Findings

### `REVIEW-A-LOW-001` — stale version reference in iteration authority

- Evidence: `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:238-244`
- Finding: version 4 text says “Version 3 preserves the version-2 prohibition.”
  The prohibition remains clear and scientifically conservative, but the active
  contract version should identify version 4 as preserving it.
- Impact: editorial traceability only; it does not admit iteration, fallback
  flux, or an incomplete Penman-Monteith family.
- Proposed disposition: `accepted`; replace “Version 3” with “Version 4.”

### `REVIEW-A-LOW-002` — stale canopy-snow vector version range

- Evidence: `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:507`
- Finding: the canopy-snow test-vector row says execution is rejected under
  versions 2-3, while the variables, guard, invariant, and constants sections
  correctly preserve the prohibition through version 4 at lines 127, 267, 287,
  and 384-386.
- Impact: editorial consistency only; the binding guard remains fail-closed.
- Proposed disposition: `accepted`; change the vector text to “versions 2-4.”

## Authority Assessment

- **A0/caller/demonstration/empirical roles:** Correctly separated. Canonical
  authority owns schema meaning, units, basis, required presence, mathematical
  domain, process role, guards, ownership, conservation, and admitted equations
  (`SC-VEGETATION-001.md:297-303,382-402,425-435`). Caller-supplied
  `external_configuration` and `initial_state` values are accepted only against
  that contract and do not become defaults or evidence of site suitability.
  `ASSUMED_FOR_EXECUTION` fixtures are explicitly barred from calibration,
  validation, ecosystem-applicability, or transferability claims. Fixed science
  constants and constitutive equations remain A0/A3 authority obligations rather
  than caller choices.
- **Stevens Canyon evidence:** Correctly classified as static diagnostic
  mechanism evidence, not calibration or independent validation
  (`SC-VEGETATION-001.md:97`; `artifacts/stevens-canyon-invariant-map.md:3-10`).
  The categorical prohibition is narrowly directed at the algebraic
  agricultural complementary `Kcb`/LAI redistribution. The investigation does
  not select replacement equations or values, and the legacy-ET ablation is
  correctly interpreted as evidence that disabling PMET alone is insufficient.
- **Penman-Monteith nuance:** Correct. The contract neither mandates nor broadly
  prohibits Penman-Monteith; a component may select it only after admitting its
  complete equation, constants, units, resistance scale, domains, guards, and
  limiting vectors (`SC-VEGETATION-001.md:242-244,553`). The audited defective
  psychrometric-constant expression remains rejected.
- **Native-forest component invariants:** Scientifically legitimate as
  architectural and conservation obligations without pretending to admit
  constitutive physics. Canopy transpiration, wet-canopy evaporation, and each
  forest-floor recipient retain distinct state, operands, resistance/energy
  lineage, and ledgers; the unchanged-floor-operand poison vector isolates and
  forbids automatic donation of lost canopy demand (`SC-VEGETATION-001.md:205-211,
  299-303,511-520`). Layer-resolved root requests remain hydrology-arbitrated,
  bounded by same-snapshot availability, and exact with Stage C transpiration.
- **Primary sources:** Gash, Shuttleworth-Wallace, Verstraete, Javaux/Cai,
  Medlyn/Bernacchi, and Samanta are truthfully labeled process precedents or
  leads. The contract repeatedly states that no complete constitutive family is
  admitted from citation alone (`SC-VEGETATION-001.md:98-102`; package
  `artifacts/primary-source-ledger.md:3-15`). Licensed RHESSys/GIS sources remain
  implementation/format provenance rather than scientific authority.
- **Successor amendments:** Correctly remove universal pine/oak value selection
  and a co-observed mixed-state surface as implementation release gates while
  retaining strict schema, caller ingestion, complete constitutive authority,
  contract-first tests, and `AUTH-RHEC-016` implementation as blockers
  (`docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/package.md:85-119,
  133-139,407-410`). The successor remains held and no production or cutover
  claim is made.

No material source/claim mismatch, surrogate-physics admission, empirical
overclaim, component-conservation defect, or premature successor release was
found. The two low findings do not change the `PASS` science verdict but should
be accepted and corrected before terminal verification.
