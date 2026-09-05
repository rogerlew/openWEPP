# Independent verification B

Evidence mode: `Static + Ran + Expected-red`

Manifest verified:
`f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`.

Verdict: `PASS-WITH-NOTES` for production implementation authority, contingent
only on the separately required verification-A verdict. No contract-cycle
finding remains open.

## Findings first

No closure-blocking scientific, ownership, trust-boundary, error-order,
anti-cache, test-binding, or disposition finding remains.

Static + Ran: both independent review artifacts carry final identity
confirmations with `PASS` against this same `f6bd360c...` manifest; the
formatting-only identity change reopened no review finding.

The initially observed disposition defect is closed. The procedure-compliant
`disposition.md` now names the canonical contract and base commit, uses all
seven required columns, and dispositions every distinct review finding,
including `B-FINAL-01`. `contract_ref.md` and `readiness-matrix.md` now point to
that ledger; the superseded non-procedure filename has been removed.

Static: the authorized-write-set description in `package.md` and the selected
increment in `pre-implementation-gates.md` now use the same source-real
resident-revision route as v30: V8 supplies no resident V3/V2 authority, the
fallible ingress step remains first, and the borrowed native proof is minted at
the existing native-validation position. Their 52-map forcing count also keeps
forcing proof authority per-map rather than in the cross-map static payload.

Two implementation notes do not weaken this verdict:

1. Static: the present covered forcing validator operates on an inspection-only
   normalized clone before passing the original allocation into V8
   (`frozen_litter_v3_adoption.rs:974-987`). The implementation must make the
   new first-validation proof attest to that exact live original allocation and
   its validated normalized values; proof authority cannot attach only to the
   temporary clone. V8 consumption must still require pointer identity with the
   original passed at `strict_v8_endpoint.rs:629-632`.
2. Static: the resident revision already stores publication-chain and tail
   identity fields, but its current `validate_same_revision` comparison stops
   after prefix head/tail (`frozen_litter_v3_adoption.rs:165-192`). Before that
   revision can mint the new map proof, the source-real join must cover the
   contract's complete revision fields, including chain, transaction,
   predecessor, and support. This is expected implementation work, not an
   existing production claim; the red gate correctly prevents treating it as
   implemented.

After both verification verdicts land, the readiness row that presently marks
independent verification `BLOCKED` must be reconciled before implementation.

## Review-finding closure

| Finding | Status | Verification |
|---|---|---|
| `CPSVO-A-001` / `B-01` | `CLOSED` | V30 distinguishes V8 structural objects from resident V3/V2 objects. V8 receives the structural LSE/surface inputs at `strict_v8_endpoint.rs:615-642`; ingress remains fallible at `:645-657`; the resident inputs reach native projection only at `:664-673`. The native projector's repeated V3 validation/canonicalization is the exact removable work at `v3_multitile_adoption.rs:176-180`. |
| `CPSVO-A-002` | `CLOSED` | The parity declaration conditions resident-proof and native-physical use on native regimes and requires zero use for `Ordinary`. |
| `CPSVO-A-003` / `B-03` / `B-FINAL-01` | `CLOSED` | Required role/path sets are compared exactly. Independent poisons distinguish structural/native configuration, state, and owner surfaces plus custody transfers and later dynamic failures. Adjacent competing pairs now cross vegetation -> surface -> soil/hydrology -> solver/residual -> output, with full/admitted first-error and ordinal equality, first-only firing, no fallback/publication, and rollback. |
| `CPSVO-A-004` / `B-04` | `CLOSED` | The supplemental scan names the intended owner plus carrier, V8, strict endpoint, native projector, and resident seams and checks derive/manual Clone/serde surfaces. Executable second-use, cross-map, cross-parent, and restart poisons remain the primary binding. |
| `CPSVO-A-005` | `CLOSED` | `contract_ref.md` records reproducible commands, evidence classifications, the ordered four-file recipe, and exact digest without claiming an unretained log. |
| `CPSVO-A-006` | `CLOSED` | Integration assertions read detailed authority from the canonical contract and restrict the index assertion to lifecycle identity/path/status/maturity/date. |
| `B-02` | `CLOSED` | V30 requires lazy plan joins only at the checks they replace and behind pre-existing carrier guards. The real carrier performs child/joint, support, duration, and forcing-duration guards before the V8 evidence path (`carrier_phase.rs:1387-1410`). Paired poisons bind the preserved precedence. |

There are no rejected findings whose rationale requires separate validation.

## Independent ownership and trust-boundary reconstruction

Static:

- Parent-static authority is limited to immutable configuration/topology/index
  facts, is lazy and generation-bound, and cannot hold V8 projections, solver
  tiles, hydrology snapshots, dynamic owners, physical results, or shared
  `DirectV10` ownership.
- Forcing authority is one-map, move-only, allocation-bound, and flows only
  from the first exact normalization/validation to V8's later validation of the
  same object. Digest equality alone cannot substitute for allocation identity.
- `FrozenLitterV3Resident` owns the distinct V3 LSE and V2 surface objects and
  a private validated revision (`frozen_litter_v3_adoption.rs:29-62`). Its
  constructor fully validates LSE and surface state (`:196-205`), and accepted
  successors are validated before revision installation. A new borrowed proof
  is therefore feasible only from the exact resident/revision join and must be
  consumed at the current native-validation position after ingress.
- Every map retains fresh V8 dynamic state validation, native topology/rebind,
  solver/residual/output validation, and physical evaluation. Ordinary maps
  neither mint nor consume resident proof authority.
- Restart/external/durable/untrusted inputs cannot restore any ephemeral plan
  or proof and retain canonical full validation. Second use, cross-map,
  cross-parent, generation/revision replacement, and equal-digest allocation
  substitution reject without fallback.
- Role and regime remain orthogonal: Initial/history/final and
  direct/Half1/Half2 applicability are enumerated per regime, with direct before
  composed and Half1 before Half2. The plan/proofs do not carry disposition or
  solver-selection authority.

These constraints extend existing `INV-LANDSURFACEENERGY-159`; they introduce
no process equation, tolerance, physical regime, solver, output, wire field,
unit, parameter, or calibration claim. The BEI mapping to existing INV-159 and
new C-019 is internally consistent.

## Commands run

Ran: ordered manifest reconstruction:

```text
sha256sum <SC-LSE, index, integration assertion, expected-red test in recorded order> | sha256sum
```

Result: `PASS`; exact
`f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`.

Ran:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`; 14 binding-exposure rows fully consolidated.

Ran:

```text
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`; no findings.

Ran:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract version_thirty_binds_parent_static_and_same_map_validation_once_to_existing_invariant
```

Result: `PASS`, 1/1.

Ran: scoped `git diff --check` over the canonical four-file manifest and this
contract-cycle directory.

Result: `PASS`.

Expected-red:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo test -p openwepp-hillslope-orchestrator --lib carrier_parent_static_and_same_map_validation_once_has_authentic_1_52_52_counts --no-run
```

Result: `EXPECTED_RED`. Compilation failed only because
`covered_parent_structural_admission.rs` and the intended validation-once
audit/parity/poison APIs do not yet exist. No unrelated compiler failure was
reported.

## Final gate statement

The corrected v30 contract, existing-invariant mapping, complete disposition,
source-real proof lineage, expected-red population, and trust-boundary
prohibitions are sufficient to authorize the bounded production experiment
once verification A also returns `PASS` or `PASS-WITH-NOTES`. This verdict does
not claim implementation, runtime parity, or the `0.10 s` performance-retention
gate; all remain post-implementation requirements.
