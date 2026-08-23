# Direct authority decision — Stage 3 lane area basis

Static: On 2026-08-22 the user prospectively selected Option A: one persistent
Stage 3 snow state per production lane is expressed per unit OFE ground.

Binding direction:

- stored SWE, ice/liquid mass, cumulative mass, cold content, energy, flux,
  and terminal liquid use OFE-ground units;
- tile physical operands remain per tile ground and enter the lane exactly
  once as `sum_i(f_i X_i)` over the complete tile set;
- covered-subset renormalization is prohibited;
- mixed open/covered OFEs require both covered-canopy and open-snow receipts
  and fail closed while either is missing;
- one lane snow owner implies one common lane snow temperature and latent heat;
- uniform-depth terminal projection preserves `sum_i(f_i M_i)=M_lane`;
- lane/OFE identity, OFE-ground basis, ordered complete tile fractions, and
  topology identity bind restart; no provisional covered-area state migration
  exists;
- future per-tile or per-routing-cell snow ownership requires a new versioned
  topology rather than reinterpretation of this lane owner.

This artifact records the direct decision for citation. Canonical binding
remains in `SC-SNOWENERGY-001`; this artifact does not replace that contract.
