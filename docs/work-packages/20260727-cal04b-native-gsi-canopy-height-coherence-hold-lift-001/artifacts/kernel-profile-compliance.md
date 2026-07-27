# Kernel-Profile Compliance

Status: `PASS`

Evidence class: `Static + Ran`

- Canonical authority: SC-PLANT-001 revision 24 admits checked
  `Bt=Bs+Bf` and `Hc=(1-exp(-bbb*Bt))*hmax` for the native GSI projection.
- Provenance: the height form and total above-ground biomass basis map to the
  pinned legacy baseline; `Bf` remains the contract's foliar/interception
  biomass.
- No surrogate physics: production implements the admitted equation directly;
  it adds no proxy, fit, clamp, static fallback, or compatibility path.
- Typed numerics: parameter domains, checked sum/product, finite result, and
  positive-biomass/positive-height implications fail through typed errors.
- Transactionality: the candidate native state commits only after a successful
  checked height projection; the structural-only underflow regression proves
  rollback exactly.
- Real consumers: exact post-growth height reaches ET, active erosion, frost,
  and active Lane D; the real shadow operand-builder seam plus static
  operand-to-cell proof covers shadow Lane D. Snow, WB15, and residue/litter
  retain their same post-growth state lineage.
- Legacy isolation: the P61 sediment regression and zero-height parity tests
  prove the native correction does not replace legacy PMET height semantics.
- Contract gates: SC binding exposure/unit checks and contract-derived
  transition, invalid-state, real-consumer, and source-negative tests pass.
- Security gates: the authority-suite anti-evasion script and AUTH11 required
  obligation guard suite pass.
- Broad correctness: the non-assurance full profile passes 2,180/2,180. After
  the separately owned assurance/TESTGATE prerequisite corrections, the
  unfiltered profile passes 2,301/2,301.

Coverage/CRAP is `DEFERRED_TO_QUALITY_CI` under ADR-0041. The package is
kernel-profile compliant and terminally closed.
