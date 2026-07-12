# DC CQR HB-02 — Texture Mass-Fraction Domain

Status: `TERMINAL-PASS`

## Objective

Close `DC-CQR-HB02-001`: `erosion_particle_composition` accepts
`silt = 1.1` even though `ErosionTextureInputs` defines sand, clay, silt, and
organic matter as surface-layer mass fractions.

## Correction Authority Envelope

- Canonical authority: `SC-SED-001#INV-SED-006/009/017`, its physical-bound
  hard-fail posture, the typed `ErosionTextureInputs` mass-fraction contract,
  and pinned `prtcmp.for` composition lineage.
- Production write set:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_operands.rs`,
  limited to texture-domain validation and mechanical HB-02 decomposition.
- Test write set:
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_erosion_operands.rs`.
- Allowed correction: require every individual sand/clay/silt/orgmat mass
  fraction to be finite and within `[0,1]`, preserving the existing typed error
  surface and validation-before-composition order.
- Excluded: a new sand+clay+silt sum tolerance (no canonical threshold is
  declared in current authority), formulas, class boundaries, legacy re-entry,
  constants, normalization, class order, schemas, and other process families.
- Acceptance: NaN/infinities and individual out-of-range fractions fail closed;
  nominal and all legacy clay-band/re-entry vectors retain exact outputs;
  class fractions close and a real Wave-1 seed/transport consumer passes;
  focused coverage/CRAP and gates pass.
- Security impact: none; the correction strengthens typed numeric admission.

Conversion rule: the reproduced mechanism is in-envelope, authority-backed,
safe, and directly testable, so the package must land the correction.

## Progress

- [x] Reproduce out-of-range silt acceptance.
- [x] Confirm individual mass-fraction authority and sum-threshold boundary.
- [x] Land the bounded correction and contract-derived regressions.
- [x] Resume HB-02 cover-first decomposition and measurement.
- [x] Complete dual review/verification and terminal disposition.

## Review And Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to one bounded implementer and two read-only review/verification agents for the
declared source, tests, and evidence. Expected outputs are the correction,
focused metrics, review dispositions, verification, and terminal record.

## Outcomes

All four individual texture mass fractions now fail closed outside finite
`[0,1]`; no sum rule or particle formula changed. The five-class producer is
mechanically decomposed, its real Wave-1/Yalin consumers and independent
lineage reconstructions pass, slice coverage is 98.020% lines / 97.318%
regions, and maximum CRAP is 14.042. Dual review/verification pass with no
unresolved finding. Disposition: `TERMINAL-PASS`; `DC-CQR-HB02-001` is closed.
