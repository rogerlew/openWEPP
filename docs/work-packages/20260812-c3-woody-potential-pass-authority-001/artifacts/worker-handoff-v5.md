# V5 Implementation Handoff

Status: `released / authority only`

Evidence mode: `Static + Ran`

Resume the existing coupled vegetation implementation package against:

- `SC-VEGETATION-001@9` (`approved/active`);
- `OPENWEPP_C3_WOODY_V5` definition SHA-256
  `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`;
- independent vectors SHA-256
  `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d`;
- independent generator SHA-256
  `4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775`.

Preserve the implementation package's V2/V3/V4 HOLD history. Implement the V5
fixed-authorization pass inside the same six-unknown coupled system using exact
stand/tile/rate conversion, independent `q_law`, equality-active cap selection,
frozen generalized-Jacobian branches, configured cap order, complete typed
failure diagnostics, finalized-use owner debit, and atomic rollback. Ordinary
Rust tests must consume the committed fixture without Python.

Keep the public execution path fail-closed until the capped second pass passes
the exact independent vectors and whole-owner closure gates. This handoff does
not authorize runtime activation, selector changes, production cutover,
calibration claims, canopy snow, or soil biogeochemical transformations.
