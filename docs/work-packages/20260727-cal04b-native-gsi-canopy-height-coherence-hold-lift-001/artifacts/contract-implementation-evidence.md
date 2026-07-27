# Contract Implementation Evidence

Status: `PASS`

Evidence class: `Static + Ran`

Canonical amendment: `SC-PLANT-001` revision 24.

Admitted authority:

- `Bt=Bs+Bf` is a checked internal native height-projection operand.
- `Bf` remains the separate foliar/interception biomass handoff.
- `Hc=(1-exp(-bbb*Bt))*hmax` uses Chapter 8 Equation 8.2.8 and the
  bit-identical pinned `grow.for` expression whose source comment historically
  labels it Equation 8.2.6.
- The native branch validates finite positive `bbb/hmax`, checked sum/product
  arithmetic, finite bounded height, and `Bt>0 => Hc>0`.
- Static, pre-GSI, compatibility, and fallback height are forbidden.

Contract surfaces updated:

- variables/units and primary authority anchors;
- native branch/guard table;
- `INV-PLANT-033` and new `INV-PLANT-038`;
- `OBL-PLANT-P-015`;
- guard, alias, and unit-governance maps;
- CP-GSI02 typed operands, algorithm, and six height test vectors;
- Binding Exposure Index and registry lifecycle metadata.

Independent review:

- Reviewer A: initial `BLOCK`; six findings plus two recheck findings accepted
  and corrected; final `PASS`.
- Reviewer B: initial consumer-scope finding and later checker
  self-authentication finding accepted and corrected; final `PASS`.

Ran:

- strict SC-PLANT Binding Exposure Index checker: `PASS`, two consolidated rows;
- SC-ROUTE checker non-regression: `PASS`, eight consolidated rows;
- checker regression vectors: `3 passed`;
- SC unit-governance lint: `PASS`;
- Markdown lint for contract, index, and package: zero findings;
- `git diff --check`: `PASS`.
