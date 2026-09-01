# V41 one-way canonical enthalpy-boundary implementation and validation

Status: `IMPLEMENTED_FOCUSED_GREEN_CANONICAL_PENDING`

Implemented:

- Four consecutive exact-static rolling active-set windows retain the V40
  support, source, event, topology, custody, receipt, density/carry, opposite
  vapor-side, promoted-root chain, raw-owner parity, cadence, shared-budget,
  rollback, and no-publication guards.
- Eligibility reconstructs every recorded canonical phase predicate from the
  exact five-point lane `W/H` sequence. Water must remain bit-identical;
  enthalpy must be finite and strictly one-way with one direction shared by all
  lanes; the observed lane set must contain exactly one adjacent canonical
  phase-boundary crossing.
- Missing, reversed, stagnant, direct cold-to-liquid, or multiple crossings,
  water drift, nonfinite values, changed joins/sides/chain/cadence, insufficient
  solver reserve, and publication posture fail typed.
- The bracket is only early eligibility evidence. It supplies the current
  authentic/interface seed to the unchanged physical solver and cannot admit
  a root, interpolate, project, repair, converge, finalize, replay, accept, or
  publish a private image.

Evidence:

- Retained r109 `/tmp/wghl_001d_v40_64m_r109.log`, SHA-256
  `da9ebf633cb9194a91c55dc7679ac3ecd34da3a3bb3750c1dc0c77a538cb3770`,
  recorded exact constant water `0.3168113... kg m^-2` and the monotone
  enthalpy sweep `-3327 -> -2445 -> -957 -> +1454 -> +2782 J m^-2` while all
  cadence, chain, static-join, side, finiteness, and parity predicates passed.
- Contract-first source gate run `7e9f4512-6a1d-4b52-bbfa-cd2769c90519`
  was expected red only for the absent V41 production seams and five required
  behavior names.
- Focused V41 behavior/poison run
  `41e80694-2b41-4179-b5fb-8828b9e13382`: 5 passed.
- Source-bound V41 contract/production run
  `f30647de-91d7-47a2-bf83-5902269c8291`: 2 passed.
- Retained V31 plus V33--V41 solver regression run
  `5705226d-c6e4-4578-bb5e-84a1e37e0039`: 52 passed.
- `cargo check -p openwepp-hillslope-orchestrator --all-targets`: passed.
- The bounded production/test scan contains no `DFF_R107`, `DFF_R108`,
  `DFF_R109`, `DFF_V*`, `eprintln!`, or `dbg!` diagnostic seam.

Canonical r110 proved V41 dispatches the unchanged solver early, then returned
a generic safeguarded-solve refusal before V42 corrected the governed
cold-content-export-complete `H` coordinate. Its log SHA-256 is
`9d049fc583f9d50bd055e7758b6d3c35f10f7e9c1ce09289bf312c4fd4c09c53`.
V42 subsequently closed the two independently exposed snow-reappearance
support-coordinate failures and the full persisted-restart suite is 71/71.
Root-owned canonical r111 remains required before further solver diagnosis.
