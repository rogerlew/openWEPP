# Review Agent A Repeat Review

Status: `HOLD / material evidence and schema findings remain`

Evidence mode: `Static + Ran`

Review role: independent canopy-interception/topology/energy science rereviewer.

This is a full repeat review of the current exact worktree bytes. It reassesses
every historical Agent-A finding and the complete Stage-A authority, model
identity, oracle, Rust reconstruction, lifecycle, migration, and focused-gate
envelope. The historical `review_agent_a.md` was not modified.

## Evidence Run

- Regenerated `openwepp_c3_woody_v2_topology_vectors.json` with the independent
  Python calculator: PASS, byte-identical fixture.
- Verified the package and model-stack V2 JSON copies are byte-identical,
  recursively lexicographically sorted canonical JSON and both SHA-256
  `e62d448b045db1577fe9367b5b531fcd7b1cfc9b544800c11c4ed305d14da10a`.
- Verified historical V1 SHA-256 remains
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- Verified `SC-VEGETATIONTRANSACTION-001` SHA-256 is
  `bbe498113e3130825b03e0e0a0a6134fa708c37326a3663f994dc44e3422f725`.
- Vegetation authority suite: PASS `14/14` against the current 31-name
  inventory and committed fixture.
- Authority anti-evasion: PASS.
- AUTH11: PASS `3/3`.
- Unit compliance: PASS for `SC-VEGETATION-001` and
  `SC-VEGETATIONTRANSACTION-001`.
- Science admission: expected FAIL because both proposed contracts correctly
  remain `in_review/draft`; this is not a release gate until promotion.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- Focused authority-test Clippy with `-D warnings`: FAIL on
  `clippy::too_many_lines` at
  `tests/integration/vegetation_boundary_authority_contract.rs:723`.

## Historical Finding Reassessment

| Historical finding | Repeat-review status | Assessment |
|---|---|---|
| `A-CRITICAL-001` | authority corrected; acceptance evidence remains open | `SC-VEGETATION-001.md:563`--`580` now unambiguously binds the fully supplied potential column, one immutable same-snapshot arbitration, and a final top-to-bottom rebuild from original beginning state under fixed caps. Descendants consume newly finalized upstream releases and no outer iteration is permitted. The canonical ordering defect is closed. The advertised authorization-sensitive oracle does not actually execute authorization or a coupled solve; see `A-RR-CRITICAL-001`. |
| `A-CRITICAL-002` | corrected | The new shared `SC-VEGETATIONTRANSACTION-001.md:62`--`100` binds occupancy-preserving water identity, receiving-owner validation, independent occupancy/stand energy reconstruction, and atomic owner commit. `SC-VEGETATION-001` and the V2 definition digest-bind this contract. No material adjacent-owner authority gap remains. |
| `A-HIGH-003` | still open | The field inventory and migration rules are now substantially explicit, but two exact unit/serialization contradictions remain; see `A-RR-HIGH-002`. |
| `A-HIGH-004` | still open | The fixture now exposes exactly 31 named booleans and the Rust test verifies their inventory, but many booleans are tautologies or do not execute the named obligation; see `A-RR-CRITICAL-001`. |
| `A-HIGH-005` | corrected | The contract and registry are now consistently `in_review/draft`; the index calls V2 proposed, V1 remains the approved historical definition, and the package does not claim Stage-B authority. |

## Remaining and New Findings

### A-RR-CRITICAL-001: The 31-check fixture materially overclaims executable, independent coverage

The fixture regenerates deterministically, and the Rust test now requires all 31
names at
`tests/integration/vegetation_boundary_authority_contract.rs:723`--`773`.
That proves fixture identity and inventory, not the named scientific behavior.
Several checks in `artifacts/reference_calculator.py:187`--`227` are
self-fulfilling or check a different condition:

- `heterogeneous_upper_columns` compares column lengths at line 191; the
  required case is the **same lower stratum present beneath different upper
  columns**, but `lower` exists only in tile A at lines 102--114.
- `single_tile_reduction` now directly compares the local V2 column with the E04
  primitive at lines 140--144 and 199--202; this obligation is adequately
  exercised.
- `homogeneous_two_tile_reduction` compares two separately recomputed identical
  dictionaries at line 203; it does not reconstruct the weighted stand
  reduction.
- `rollback_exact_bytes` now constructs and mutates a separate candidate before
  injecting failure at lines 86--97 and compares serialized beginning bytes at
  line 208; this is a valid Stage-A candidate-isolation example, although full
  owner rollback remains a Stage-B execution obligation.
- `capped_upper_changes_lower` manually changes vapor from evaporation to
  condensation between lines 153--164 and compares descendant incident at line
  209. It supplies no water requests, arbitration, authorization cap, or coupled
  re-solve, so it is not evidence for the corrected potential/arbitration/final
  column algorithm.
- the positive `stemflow_bypass` now reconstructs the lower incident equality
  without stemflow at lines 193--196. However, `wrong_tile_drainage_poison`,
  `stemflow_through_foliage_poison`, and `wrong_authorization_poison` remain
  scalar inequalities at lines 213--214 and 220--221; no validator is given a
  swapped candidate and required to reject it.
- `average_wet_fraction_poison` and `aggregate_par_poison` use arbitrary square
  and square-root functions at lines 218--219, not the admitted wet-energy or
  FvCB equations. They cannot prove nonlinear physiology consumes local inputs.
- `shared_cn_once` reuses the same weighted formula that produced its expected
  values at line 224. `mineral_n_after_aggregation` now preserves three distinct
  keys and distinguishes a collapsed map at lines 176--183 and 225--226, but it
  still does not execute the receiving-owner species/layer swap rejection.
- `distinct_beginning_store` compares ending stores rather than exposed
  beginning stores at line 190, and `local_and_stand_closure` checks only the two
  Python column residuals at line 207.

The Rust-side reconstruction at
`tests/integration/vegetation_boundary_authority_contract.rs:793`--`842`
independently recomputes only weighted ending store, weighted ground liquid, and
three area conversions. It does not reconstruct occupancy/column/stand closure
from beginning stores and flux operands, descendant routing, second drainage,
migration, capped-column behavior, rollback, shared C/N, mineral-N identity, or
any poison result. Trusting Python-produced `true` values is not independent
reconstruction.

Scientific impact: implementations that omit the same-lower/different-upper
case, never rerun a capped column, broadcast rollback state, preaggregate
wetness/PAR, duplicate shared C/N, or swap N identity can satisfy the committed
fixture and current Rust authority test. The Stage-A directive explicitly makes
these poison and independent-oracle obligations release gates.

Disposition recommendation: `accepted`. Replace boolean demonstrations with
case records containing all distinct authoritative operands, expected outputs
or typed failures, and rejected-candidate outputs. Execute the actual V1/V2
reduction, potential-request/arbitration/final-column schedule, failure mutation
attempt, admitted wet-energy/FvCB response, shared C/N transition, and N
layer/species swap. Extend Rust to reconstruct every local/column/stand closure
and each cross-owner conversion from fixture operands without consuming Python
verdict booleans.

### A-RR-HIGH-002: The frozen occupancy schema has conflicting potential units and serialization order

The canonical variable table declares hydraulic node potentials in `MPa` at
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:180`.
The newly explicit occupancy schema declares `stem_potential_mm`,
`sun_leaf_potential_mm`, `shade_leaf_potential_mm`, and
`root_potential_mm_by_layer` at `SC-VEGETATION-001.md:601`--`609` and in the
single-line V2 definition under `occupancy_state_schema`. The inherited E14
ledger separately states potentials are `mm H2O`. No named MPa-to-mm-water
conversion, direction, or density/gravity convention resolves these canonical
surfaces.

The contract also says the displayed field order enters canonical state
serialization at `SC-VEGETATION-001.md:610`--`612`, whereas
`artifacts/state-schema-amendment.md:23`--`24` requires lexicographically sorted
keys. The displayed order is not lexicographic, so two conforming readers can
produce different state bytes and digests.

Scientific impact: a unit choice changes vulnerability and hydraulic flux by
orders of magnitude, while an ordering choice breaks the promised exact state
digest and byte-identical rollback/migration identity.

Disposition recommendation: `accepted`. Select one canonical potential unit
consistent with E14/E15, correct every symbol and JSON key or bind a named exact
conversion, then recompute section/model digests and add a wrong-unit poison.
Specify one state-key order (prefer the already selected recursive lexical
order), provide a canonical serialized state fixture, and test its exact digest.

### A-RR-HIGH-003: The current focused Clippy gate is failing

`cargo clippy --test vegetation_boundary_authority_contract -- -D warnings`
fails because
`v2_committed_topology_vectors_are_independent_and_non_tautological` exceeds the
repository's 100-line function threshold at
`tests/integration/vegetation_boundary_authority_contract.rs:723`. The package
gate log claims focused authority-test Clippy PASS, but the later 31-name
strengthening invalidated that evidence.

Disposition recommendation: `accepted`. Decompose the test into focused helper
reconstructions without suppressing the lint, rerun Clippy and the authority
suite, and append the failed and successful results to `gate-results.md`.

## Final Recommendation

`HOLD`

The tile-local scientific selection, exact potential/final column ordering,
shared adjacent-owner authority, migration posture, V1 immutability, canonical
V2 JSON identity, and review-stage lifecycle posture are now sound. Stage A
still cannot release implementation authority because its advertised oracle
and independent reconstruction do not execute much of the binding acceptance
envelope, and the frozen warm-start schema remains contradictory on hydraulic
units and byte order. The current focused Clippy gate also fails. These are all
in-scope corrections, not Stage-B deferrals.
