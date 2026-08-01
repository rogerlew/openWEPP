# Independent Authority Review A

Evidence mode: `Static`.

Reviewer: independent authority reviewer A

Scope: Phase A dimensional authority, result-blindness, separation of
represented-layer lifecycle semantics, and the proposed version-6 contract
amendment. This review read the package intake and frozen authority artifacts,
then only the four source files listed in `authority-input-manifest.md`. It did
not read any EB-04R path, retained factorial output, observation, residual,
score, attempt record, or terminal audit.

Disposition: `GO_WITH_AMENDMENTS`.

## Findings

### Medium — Preserve the distinct vapor-aggregation tolerance explicitly

The dimensional decision is correct, but the contract amendment must scope
`1e-6 kg m^-2` specifically to the same residual expressed in area-mass units,
including the vapor-to-sublimation transfer identity. It must not imply that
every mass-unit check uses that tolerance. The prospectively frozen EB-04E
protocol separately specifies hourly/daily vapor aggregation at
`1e-9 kg m^-2` and vapor-to-sublimation closure at `1e-6 kg m^-2`
(`prospective-qualification-protocol.md`, lines 31–33). Retain both predicates
and name their different operands/aggregation boundaries in version 6.

Required amendment: add unit-explicit contract text that distinguishes all
three quantities:

- same-residual snow-mass closure: `1e-9 m SWE`, equivalent to
  `1e-6 kg m^-2` through `rho_w = 1000 kg m^-3`;
- hourly/daily vapor-aggregation reconstruction: `1e-9 kg m^-2`;
- represented-layer lifecycle boundary: `1e-9 kg m^-2`, equivalent to
  `1e-12 m SWE`, and never a residual-acceptance or layer-deletion substitute.

This is amendment-blocking wording, not a challenge to the frozen dimensional
decision.

## Authority Assessment

### Dimensional authority — PASS

`SC-SNOWENERGY-001` version 5 declares runtime mass closure in
`m` water equivalent and separately binds density-layer lifecycle in
`kg m^-2`. The named
`snow_water_equivalent_meters_to_area_mass_kg_m2` helper multiplies by
`LIQUID_WATER_DENSITY_KG_M3 = 1000.0`. Therefore:

```text
1e-9 m * 1000 kg m^-3 = 1e-6 kg m^-2
```

The frozen receipt's `CROSS_UNIT_PROTOCOL_TRANSCRIPTION_ERROR`
classification follows directly from the declared units and named conversion.
Use canonical decimal notation `1e-6 kg m^-2` in contract prose; the JSON
binary64 rendering `1.0000000000000002e-06` does not change the authority.

### Result-blindness — PASS

The manifest fixes exactly four pre-result inputs. Their current SHA-256
identities match `authority-freeze.json`, and the frozen reconciliation contains
only authority anchors, the dimensional derivation, and the classification.
No result-bearing value is present in the reviewed Phase A record. This review
also honored the same forbidden-source boundary.

### Lifecycle separation — PASS

`SC-SNOWENERGY-001#INV-SNOWENERGY-027` explicitly defines the represented-layer
lifecycle boundary as `1e-9 kg m^-2`, equivalent to `1e-12 m SWE`, and states
that the independent `1e-9 m` aggregate residual tolerance cannot delete a
represented layer. The frozen reconciliation preserves that distinction.

### Proposed contract amendment — GO WITH AMENDMENTS

Promoting the equivalence into version 6 is scientifically and dimensionally
defensible if the amendment:

1. states the equation and both units explicitly;
2. binds `1e-6 kg m^-2` only to the same-residual area-mass or
   vapor-to-sublimation closure;
3. preserves both distinct `1e-9 kg m^-2` predicates identified above;
4. updates the tolerance notes, constants/parameters table, applicable
   invariant/guard text, and change log so package-local evidence does not
   become hidden authority; and
5. does not change runtime physics, observations, coefficients, or the frozen
   empirical rubric.

No high- or critical-severity authority defect was found. Phase B may begin
only after this wording is incorporated, the second independent authority
review is dispositioned, and the amended authority receipt is sealed as the
package requires.
