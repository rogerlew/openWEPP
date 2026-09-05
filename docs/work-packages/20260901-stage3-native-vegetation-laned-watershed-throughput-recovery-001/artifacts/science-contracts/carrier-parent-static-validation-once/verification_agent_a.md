# Independent verification A

Evidence mode: `Static + Ran + Expected-red`

Manifest verified:
`f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`.

Verdict: `PASS`

## Findings first

No open contract-cycle finding remains. This verdict authorizes the bounded
production implementation attempt described by version 30; it does not claim
that implementation exists, that the expected-red behavior tests pass, or that
the `0.10 s` performance-retention gate has passed.

The prior `216375200839ee0526a37e43985bf8f6729a795985d2a0501c67b5eb48b7ecc3`
identity was superseded only by rustfmt normalization of the integration test.
Both independent reviewers appended formatting-only `PASS` confirmations for
the current manifest. Inspection and the focused assertion found no semantic or
authority change.

## Finding closure

| Finding | Status | Verification evidence |
|---|---|---|
| `CPSVO-A-001` | `CLOSED` | Contract version 30 distinguishes structural V8 objects from resident V3/V2 objects, retains forcing -> V8 -> ingress -> native order, and sources native omission only from the exact resident validated revision. Current source has those distinct object seams and the stated order. |
| `CPSVO-A-002` | `CLOSED` | The parity declaration makes native use conditional and requires ordinary execution to report zero native physical calls and zero resident-proof mint/consume calls. |
| `CPSVO-A-003` | `CLOSED` | Required role/path coverage is compared per regime. Independent poisons and adjacent pairs cover carrier, forcing, structural V8, ingress, resident/proof, vegetation, surface, soil/hydrology, solver/residual, and output precedence with full/admitted error and ordinal equality, no fallback/publication, and rollback. |
| `CPSVO-A-004` | `CLOSED` | The supplemental source guard names the intended owner and actual carrier, forcing/V8, ingress, resident, and native seams; it checks derive/manual `Clone` and serde restrictions for all three ephemeral types. Behavioral reuse, transfer, and restart poisons remain the primary evidence. |
| `CPSVO-A-005` | `CLOSED` | `contract_ref.md` records exact commands, result classes, ordered paths, hash recipe, and the current manifest. It truthfully states that no durable command log is claimed. |
| `CPSVO-A-006` | `CLOSED` | `assert_lse_registry_lifecycle` checks only registry identity, path, status, maturity, and review date. Detailed version-30 assertions read the canonical contract. |
| `B-01` | `CLOSED` | Same source-real resident-lineage correction as A-001; structural/native LSE and surface poison families are distinct. |
| `B-02` | `CLOSED` | Plan construction and joins are lazy at the checks replaced and cannot precede existing carrier, support, duration, transaction, joint, or forcing failures. Paired poisons bind the retained precedence. |
| `B-03` | `CLOSED` | The expected-red population includes structural/native configuration, state, and owner cases plus revision, second-use, cross-map, cross-parent, restart, dynamic, solver/residual, and output cases. |
| `B-04` | `CLOSED` | The intended owner and all material current seams are included in the supplemental source scan; executable real-call-site evidence remains mandatory. |
| `B-FINAL-01` | `CLOSED` | The matrix now has both missing adjacent pairs: dynamic vegetation -> dynamic surface and dynamic surface -> dynamic soil/hydrology, followed by solver/residual -> output. |

`disposition.md` names the canonical contract and base commit, binds the current
ordered manifest, uses the required seven-column schema, and contains one
accepted row for every finding above. No accepted, amended, rejected, deferred,
or follow-up item is absent or still open. The obsolete duplicate finding ledger
has been removed, and `contract_ref.md` plus `readiness-matrix.md` point to the
procedure-compliant disposition.

## Contract, schema, BEI, and units

Static: `SC-LANDSURFACEENERGY-001` has version 30, approved/active lifecycle
metadata, the required core kernel-contract sections, the version-30 algorithm,
state/guard/unit/calibration/test-vector profile mapping, `INV-159` guard-map
extension, `OBL-LANDSURFACEENERGY-C-019`, a single active BEI row mapped to
`INV-159/C-019`, gap posture, and change-log entries. No new invariant, equation,
solver, dimensional symbol, conversion, parameter, tolerance, output,
publication field, or wire format is admitted. `CALIBRATION_NOT_APPLICABLE` is
therefore supported by the amendment's architecture-only scope.

Static: the lifecycle index agrees on contract identity, title, status,
maturity, owner, path, and review date. Its short version-30 note is not used as
detailed authority by the contract-derived test.

Ran:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`; 14 BEI rows fully consolidated.

Ran:

```text
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`; no findings.

## Source-real feasibility and error order

Static: the proposed route is implementable without changing physical equations
or selecting a fallback:

- `v11_covered/carrier_phase.rs:1387-1410` performs child/joint, positive-support,
  and exact-duration guards before downstream projection work. Version 30 keeps
  parent-plan joins at the later checks they replace.
- `v9_real_consumer_shadow/frozen_litter_v4_adoption.rs:942-964` constructs the
  normalized per-map forcing used by the native path.
  `frozen_litter_v3_adoption.rs:958-987` performs the existing first forcing
  identity/domain validation before V8. A proof can be bound to that live
  normalized forcing allocation at that same position; no digest-only or
  different-allocation authority is needed.
- `land_surface_energy_shadow/strict_v8_endpoint.rs:615-644` runs structural V8
  projection, lines 647-657 run the fallible ingress schedule, and lines 664-675
  enter native projection. This matches the amended order.
- `land_surface_energy_shadow/v3_multitile_adoption.rs:176-180` contains the two
  repeated resident V3-LSE/V2-surface validations that may be omitted only after
  the resident-revision join at that exact native position.
- `v9_real_consumer_shadow/frozen_litter_v3_adoption.rs:33-62,136-193` provides
  the private resident and validated-revision custody; construction and accepted
  successor paths fully validate the LSE and surface owners before installing a
  matching revision (`:196-289,578-628`). A borrowed lifetime-bound map proof can
  therefore be minted from this resident authority after ingress without asking
  V8 to attest to objects it does not receive.

Inference: the production change must thread a resident-backed borrowed
capability into the post-ingress native position and must validate the exact
normalized forcing allocation before minting its map proof. Minting either proof
earlier, treating the structural V8 objects as resident authority, or caching a
dynamic projection would violate the verified contract even if the behavioral
oracle were otherwise made green.

The corrected package write-set description and selected pre-implementation
intent now state this same resident-revision route; neither retains the rejected
V8-to-resident attestation wording.

## Contract-derived and expected-red evidence

Ran:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract version_thirty_binds_parent_static_and_same_map_validation_once_to_existing_invariant
```

Result: `PASS`, 1/1.

Expected-red:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo test -p openwepp-hillslope-orchestrator --lib carrier_parent_static_and_same_map_validation_once_has_authentic_1_52_52_counts --no-run
```

Result: `EXPECTED_RED`. Compilation fails on exactly the absent intended owner
file and three absent implementation API groups: authentic audit/count records,
full-versus-admitted role/path parity, and poison/competing-poison execution.
The red is specific to not-yet-implemented production seams; it is not counted
as runtime or passing evidence.

The declared tests require authentic `1/52/52` counts, per-applicable-regime
bitwise physical and final-owner parity, ordinary zero-native use, exact call
order, the complete independent/paired poison matrix, real call-site markers,
zero fallback/publication, and byte-identical rollback. Contract text expressly
forbids fabricated counters or source scanning alone from satisfying C-019.

## Source quality and identity

Ran:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo fmt --all -- --check
```

Result: `PASS`.

Ran: scoped `git diff --check` over the four manifest paths and the complete
contract-cycle artifact directory.

Result: `PASS`.

Ran:

```text
sha256sum docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md docs/specifications/science-contracts/index.md tests/integration/land_surface_energy_balance_authority_contract.rs crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs | sha256sum
```

Result:
`f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`,
matching `contract_ref.md`, both final review confirmations, and
`disposition.md`.

## Final authorization verdict

`PASS`. Contract-first authority, expected-red specificity, dual-review finding
closure, source-real feasibility, current error ordering, BEI/schema/unit
posture, and exact manifest identity are sufficient to begin the bounded
production implementation. Production must still make all expected-red tests
green through real seams, pass independent implementation review/verification,
and satisfy or revert under the separately declared performance-retention rule.
