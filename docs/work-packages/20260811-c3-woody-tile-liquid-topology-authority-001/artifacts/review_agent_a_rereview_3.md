# Review Agent A Third Repeat Review

Status: `PASS / no material finding remains`

Evidence mode: `Static + Ran`

Review role: independent canopy-interception/topology/energy science reviewer.

This review assessed the complete current Stage-A authority and acceptance
envelope, not only the latest edits. It reassessed every historical Agent-A
finding and all findings from the first and second repeat reviews. Historical
review artifacts were not modified.

## Evidence Run

- Regenerated the V2 topology fixture with the independent Python calculator:
  PASS and byte-identical to the committed fixture, SHA-256
  `c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`.
- Verified both V2 definition copies are byte-identical recursively
  lexicographically sorted canonical JSON, SHA-256
  `b2b01f965f83a52f4c800c489079c88d97179ed6a8191734b541115308b97a5c`.
- Verified historical V1 definition remains unchanged, SHA-256
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- Verified shared transaction contract SHA-256
  `bbe498113e3130825b03e0e0a0a6134fa708c37326a3663f994dc44e3422f725`.
- Vegetation authority suite: PASS `14/14`.
- Focused authority-test Clippy with `-D warnings`: PASS.
- Authority-suite anti-evasion: PASS.
- AUTH11 obligation guards: PASS `3/3`.
- Unit compliance: PASS for `SC-VEGETATION-001` and
  `SC-VEGETATIONTRANSACTION-001`.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.

Science admission was not treated as a passing review-stage gate: both proposed
contracts correctly remain `in_review/draft` until disposition, heavy gates,
and terminal verification finish.

## Complete Reassessment

| Finding family | Status | Exact-byte assessment |
|---|---|---|
| `A-CRITICAL-001` | closed | `SC-VEGETATION-001.md:563`--`580` exactly defines the fully supplied potential column, one immutable same-snapshot arbitration, and final top-to-bottom reconstruction from beginning state under fixed caps. Final descendants consume final upstream release; no outer iteration or stale potential release is allowed. |
| `A-CRITICAL-002` | closed | Digest-bound `SC-VEGETATIONTRANSACTION-001` defines occupancy-preserving water keys, receiving-owner proportional arbitration and finalized-use debit, independent occupancy/stand energy reconstruction, typed swap rejection, and all-owner atomicity. |
| `A-HIGH-003` / `A-RR-HIGH-002` | closed | The V2 schema exhaustively names occupancy warm starts, uses `mm H2O` consistently with E14/E15, rejects MPa, defines layer cardinality and transaction identity, and selects recursive lexical serialization. The fixture and Rust test independently bind exact state bytes and SHA-256. Migration requires complete caller-supplied V2 numerical lanes and permits only the unique zero/single-tile liquid mappings. |
| `A-HIGH-004` / `A-RR-CRITICAL-001` | closed for Stage A | The fixture now contains the same lower stratum beneath distinct upper columns, exact local operands, complete same-tile routing, V1/single and homogeneous reductions, permutation, area conversions, four-owner rollback bytes, typed water/N swaps, independent layer/species arbitration, state digest, and shared-C/N duplicate poison. Rust reconstructs local/stand closure, water boundaries, key-matched N use, rollback/state digests, and nonlinear alternatives without treating producer residuals as acceptance evidence. |
| `A-HIGH-005` | closed | The canonical contract and registry remain consistently `in_review/draft`; V2 is proposed and V1 remains approved historical authority until promotion. |
| `A-RR-HIGH-003` | closed | The authority test was decomposed and focused Clippy passes with warnings denied. |
| `A-RR2-CRITICAL-001` | closed | The uncited cap-to-vapor equation and simplified nonlinear helpers are gone. The two vapor values at `artifacts/reference_calculator.py:198`--`210` are now explicitly controlled exogenous operands proving only column-routing causality. Fixture and Rust bind that narrow claim and name `STAGE_B_E11_E15_EXACT_ORACLE` as the required complete coupled acceptance gate. Wet-energy locality calls the existing V1 `wet_canopy_temperature` solver at `reference_calculator.py:296`--`317`; PAR locality calls the existing V1 `fvbc` implementation with both co-limitation quadratics at `reference_calculator.py:318`--`324`. No substitute constitutive response is represented as V2 science. |

## Scientific and Governance Assessment

The released selection is internally coherent:

- state is persistent only per valid `(stratum,tile)` occupancy, while shared
  C/N advances once from stand-weighted accepted local fluxes;
- conditional plant area uses `LAI_s/C_s` and `WAI_s/C_s` on tile-ground basis;
- complete E04 ordering preserves initial and second drainage, same-tile
  descendant routing, and ground-bypassing stemflow;
- nonlinear radiation, wet energy, FvCB--Medlyn, canopy-air, and hydraulic work
  remains occupancy-local before `f_t` aggregation;
- water identity retains transaction, stratum, occupancy, layer, resource, and
  amount basis through both owner reconstructions;
- local, column, and weighted stand closure use exposed operands rather than a
  producer-provided zero residual;
- V1 bytes are immutable, V2 bytes and section hashes are exact, and migration
  fails closed wherever a unique state mapping does not exist; and
- no runtime activation, production consumer cutover, calibration, canopy snow,
  or soil-transformation claim is introduced.

The named `STAGE_B_E11_E15_EXACT_ORACLE` obligation is a legitimate
implementation acceptance gate, not missing Stage-A authority. Stage A defines
the exact coupled algorithm and the topology around it; its controlled-vapor
case now claims only the routing property it executes.

## Findings

No material finding remains.

## Final Recommendation

`GO`

The current exact bytes are suitable to proceed through Stage-A disposition,
heavy gates, and terminal verification. Promotion and Stage-B implementation
must still preserve the named exact E11--E15 oracle gate and all other worker
handoff obligations.
