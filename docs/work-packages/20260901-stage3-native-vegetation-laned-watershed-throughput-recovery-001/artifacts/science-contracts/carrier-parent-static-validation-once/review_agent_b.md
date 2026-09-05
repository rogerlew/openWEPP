# Independent contract review B

Evidence mode: `Static + Ran`

Scope: `SC-LANDSURFACEENERGY-001` version 30 and its carrier parent-static /
same-map validation-once contract-derived tests and package artifacts. This
review is independent and was not primed with another reviewer's findings.

## Findings

### B-01 — HIGH — the proposed same-map proof does not yet have a validated producer

The amendment says V8 freshly validates the exact current-map LSE
configuration/state and surface configuration/owner and then mints a proof that
allows native V3 to omit their repeated validation
(`SC-LANDSURFACEENERGY-001.md:2723-2733`). The current real call chain does not
have that identity:

- `strict_v8_endpoint.rs:615-642` passes the structural
  `self.inner.lse_configuration` / `self.inner.lse_state` and the hydrology
  frame's surface parent into `project_v8_runtime_inputs_with_carriers`;
- `strict_v8_endpoint.rs:664-673` separately passes the
  `FrozenLitterV3Resident`'s V3 LSE configuration/state and V2 surface
  configuration/owner into `project_native_frozen_litter_v3_solver_inputs`;
- `v8_input_projection.rs:1572-1587,1636-1646` validates the former objects,
  while `v3_multitile_adoption.rs:176-180` performs the measured validation and
  canonicalization on the latter objects.

Those are different owners/references, even if their selected topology or high
mirrors agree. A proof minted from the current V8 validation cannot authorize
omission of the native-resident validation. Moving the latter validation into
V8 would not itself remove its cost. The existing private validated resident
revision may provide a safe authority source, but version 30 does not name that
join or specify its relation to the structural V8 objects.

Impact: the claimed `63319 us` duplicate-validation opportunity and the central
proof-safety premise are not established. Implementing the text literally could
skip the only full validation of the native V3 LSE and V2 surface owner.

Proposed disposition: `accepted`. Amend the algorithm and state-surface mapping
to distinguish structural V8 objects from native-resident objects. Require the
same-map proof to be minted only from authority that has already validated the
exact native configuration/state/surface references (for example, a pointer-
and-generation join to the resident's existing validated-revision custody), or
prove and bind a real V8-input substitution that validates those exact native
objects. Add independent structural-versus-native configuration, state, and
surface poisons, then re-characterize the safely removable time.

### B-02 — HIGH — the specified plan-join order changes current first-error precedence

Version 30 orders the structural-plan join before map support, duration,
transaction, and joint validation (`SC-LANDSURFACEENERGY-001.md:2714-2718` and
`:2744-2755`). In the real carrier, child/joint and positive-support checks occur
before the downstream V8 configuration validation
(`carrier_phase.rs:1387-1410`; V8 validation starts later through the evidence
builder). A stale/replaced plan combined with an invalid support would therefore
return the new plan identity error before the support error, unlike the forced-
full path. That contradicts the amendment's unchanged-call-order and exact
first-error promises (`SC-LANDSURFACEENERGY-001.md:2750-2755,2773-2778`).

Impact: error semantics and the full-versus-admitted oracle cannot both satisfy
the current text.

Proposed disposition: `accepted`. Place the reusable-plan join at the exact
location of the structural validation it replaces, after all guards that
currently precede that validation. Add combined-poison precedence vectors (at
minimum stale generation/configuration plus invalid support/transaction) rather
than comparing only one poison at a time.

### B-03 — MEDIUM — the expected-red poison population does not cover the canonical obligation

`OBL-LANDSURFACEENERGY-C-019` explicitly requires rejection of a surface-
configuration poison (`SC-LANDSURFACEENERGY-001.md:376-388`), but the behavioral
poison list has no `SurfaceConfiguration` case
(`snow_stage3_v11_adaptive_production_tests.rs:2241-2256`). One generic
`LseState` case also cannot distinguish the structural and native state surfaces
exposed by B-01. The amendment additionally prohibits proof reuse, cross-map
transfer, and cross-parent transfer (`SC-LANDSURFACEENERGY-001.md:2763-2766`),
but no behavioral expected-red vector exercises those single-use/custody
boundaries.

Impact: production could violate named C-019 and move-only/same-map authority
while the current expected-red population turns green.

Proposed disposition: `accepted`. Add the missing surface-configuration,
structural/native state, second-consumption, cross-map, and cross-parent cases;
assert the real producer/consumer call site, unchanged first error, zero
fallback/publication, and byte-exact rollback for each.

### B-04 — MEDIUM — the anti-cache test omits the file expected to own the new authority

The source guard scans only terminal execution, adaptive execution,
`v11_covered/execution.rs`, and `v11_covered/carrier_phase.rs`
(`snow_stage3_v11_adaptive_production_tests.rs:2286-2293`). The package write set
explicitly authorizes
`land_surface_energy_shadow/covered_parent_structural_admission.rs`, and the
actual V8/native seams live in `strict_v8_endpoint.rs`,
`v8_input_projection.rs`, and `v3_multitile_adoption.rs`. Forbidden dynamic
cache state or `Clone`/serde implementations can therefore be placed in the
most likely implementation files without this test observing them. Exact
substring searches such as `"Clone for ..."` also do not detect derive-based
implementations.

Impact: this supplemental guard does not substantiate its advertised private,
non-Clone, non-wire, no-dynamic-cache claim.

Proposed disposition: `accepted`. Scan the complete owned implementation
surface, including the new module and the three real seams, and bind the
single-use/private properties with executable behavior or compile-time API
checks where possible. Keep source inspection supplemental, as the contract
already requires.

## Positive observations

- Mapping this validation-custody architecture to existing
  `INV-LANDSURFACEENERGY-159` is reasonable once B-01 is made exact; it does not
  require a new process-physics invariant or solver version.
- The amendment correctly forbids dynamic/result caches, digest-only authority,
  persistence, restart restoration, fallback, and physics/tolerance/output/wire
  changes.
- The `1/52/52` authentic carrier-count test uses the real 52-map workload and
  requires real provider, V8, and native-V3 call counts.
- BEI, registry, calibration-not-applicable, and no-new-unit posture are
  internally consistent.

## Commands run

- `Ran:` `.venv/bin/python tools/check_sc_binding_exposure.py
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
  — PASS, 14 fully consolidated rows.
- `Ran:` `bash tools/release/check_sc_unit_compliance.sh --path
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
  — PASS, no findings.
- `Ran:` scoped `git diff --check` over the six review paths — PASS.
- `Ran:` `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test
  land_surface_energy_balance_authority_contract
  version_thirty_binds_parent_static_and_same_map_validation_once_to_existing_invariant`
  — PASS, 1/1.
- `Ran:` ordered four-file manifest hash —
  `f372da11b4392f099f7f4a5934aa5aa038b4d1db3de8aafd03d42dc01b184b66`,
  matching `contract_ref.md`.

## Final recommendation

`FAIL / HOLD`. Production implementation must not proceed until B-01 and B-02
are corrected in canonical authority and B-03/B-04 are represented in the
expected-red population. After amendment, the contract requires disposition
and independent verification before implementation.

## Corrected-manifest independent re-review B

Evidence mode: `Static + Ran + Expected-red`

This re-review considered the corrected contract, registry, contract-derived
tests, `contract_ref.md`, `readiness-matrix.md`, B-finding disposition rows, and
the real structural V8 / resident native-V3 source seams. It did not consult or
rely on review A.

### Re-review findings

No new closure-blocking finding was identified.

#### B-01 — CLOSED

The corrected invariant and algorithm now state that V8 validates structural
objects and does **not** attest to the distinct native resident V3/V2 objects
(`SC-LANDSURFACEENERGY-001.md:2698-2705,2732-2737`). Native omission is instead
sourced at the original native-validation position from the exact
`FrozenLitterV3Resident` and its private
`ValidatedFrozenLitterV3ResidentRevisionV1`, after fallible ingress derivation
(`:2738-2749`). This matches the source separation observed at
`strict_v8_endpoint.rs:615-642,664-673`, the resident's private revision/state
surface in `frozen_litter_v3_adoption.rs:33-65,136-193`, and the repeated native
validation calls in `v3_multitile_adoption.rs:176-180`.

The contract also binds successor validation before revision advance and fresh
restart reconstruction (`SC-LANDSURFACEENERGY-001.md:2777-2786`), so the
persistent cloneable revision is not confused with the new borrowed,
non-Clone, one-map proof. The corrected route is feasible authority for an
implementation attempt. The `63319 us` remains a measured target, not a passed
performance claim.

#### B-02 — CLOSED

Plan creation and joins are now fixed at the exact structural checks they
replace and expressly cannot precede existing support, duration, transaction,
joint, or forcing failures (`SC-LANDSURFACEENERGY-001.md:2707-2719,2721-2726`).
That is consistent with the current carrier's pre-V8 child/support/duration
guards (`carrier_phase.rs:1387-1410`). The contract gives explicit competing-
poison precedence rules (`SC-LANDSURFACEENERGY-001.md:2759-2769`), and the
expected-red population crosses early carrier guards, forcing, structural V8,
ingress, native resident, and proof consumption
(`snow_stage3_v11_adaptive_production_tests.rs:2325-2356`).

#### B-03 — CLOSED

The expected-red poison enum use now distinguishes structural and native LSE
configuration/state plus structural and native surface configuration/owner. It
also binds resident revision, second consumption, cross-map transfer,
cross-parent transfer, ingress schedule, and restart restoration
(`snow_stage3_v11_adaptive_production_tests.rs:2273-2323`). Each case requires
real call sites, forced-full error/ordinal equality, zero fallback/publication,
and byte-identical rollback. Paired poisons separately prove first-error
precedence (`:2325-2356`). This closes the missing surface-configuration and
proof-custody vectors.

#### B-04 — CLOSED

The supplemental source guard now scans the intended new owner, strict endpoint,
V8 projector, native projector, resident, and carrier/execution files
(`snow_stage3_v11_adaptive_production_tests.rs:2359-2372`). It checks the three
ephemeral type declarations for derive/manual `Clone` and serde implementations
(`:2404-2441`) and retains behavioral second-use/transfer/restart tests as the
primary evidence. Given the canonical prohibition on public/unchecked
construction and the later implementation review gate, this is adequate
pre-implementation binding rather than a source-only closure claim.

### Re-review commands run

- `Ran:` strict Binding Exposure Index lint — PASS, 14 fully consolidated rows.
- `Ran:` scoped science-contract unit lint — PASS, no findings.
- `Ran:` scoped `git diff --check` over the corrected cycle and this review —
  PASS.
- `Ran:` focused v30 contract-derived Nextest assertion — PASS, 1/1.
- `Ran:` ordered four-file manifest SHA-256 —
  `08ebf9ed86c6b9f91114c871b988983de05434d7f3d64200ffdd0f58d3c793f6`,
  matching corrected `contract_ref.md`.
- `Expected-red:` the production behavioral population remains deliberately
  uncompilable until the intended owning module and real typestate/audit/oracle
  APIs exist; it is not counted as passing runtime evidence.

### Corrected-manifest recommendation

`PASS`. B-01 through B-04 are closed in canonical authority and expected-red
bindings. The corrected contract cycle may proceed to independent verification;
this verdict does not authorize skipping that gate and makes no production or
performance-retention claim.

## Final-manifest independent re-review B

Evidence mode: `Static + Ran + Expected-red`

Manifest under review:
`c8f4fbb00b17fdd1746ad2fc768c4906325820b81efe87983a02e9e949172b74`.
This review remained independent and did not consult review A.

### Finding B-FINAL-01 — MEDIUM — dynamic-surface precedence is not paired

The final manifest adds independent poisons for
`DynamicVegetationState`, `DynamicSurfaceState`,
`DynamicSoilHydrologyState`, `NativeSolverResidual`, and `OutputValidation`
(`snow_stage3_v11_adaptive_production_tests.rs:2297-2301`). Its competing-
poison matrix then pairs resident-surface -> dynamic vegetation, dynamic
vegetation -> dynamic soil/hydrology, dynamic soil/hydrology -> solver/residual,
and solver/residual -> output (`:2342-2354`). `DynamicSurfaceState` appears in
no competing pair. The matrix therefore jumps over the named dynamic-surface
boundary and does not support the contract's assertion that competing vectors
cross each ordered boundary through dynamic validation
(`SC-LANDSURFACEENERGY-001.md:2803-2807`).

Impact: the independent surface poison proves rejection in isolation, but not
that an earlier/later dynamic failure retains source-real first-error precedence
when a surface-state failure competes with it. The artifact claim of adjacent
dynamic precedence pairs is currently too strong.

Required disposition: add source-order-correct pairs covering both sides of the
dynamic surface boundary—normally dynamic vegetation -> dynamic surface and
dynamic surface -> dynamic soil/hydrology, unless inspection of the implemented
real call sites establishes another order. Retain full/admitted error and
ordinal equality, first-only triggering, zero fallback/publication, and exact
rollback assertions. Update the manifest and matching artifacts.

### Prior-finding regression check

- `B-01`: remains closed. V8 structural objects and resident V3/V2 objects are
  still explicitly distinct; resident omission remains sourced only from the
  exact resident validated revision at the native position.
- `B-02`: remains closed in authority. Plan joins remain at replaced checks and
  cannot move before existing early guards. B-FINAL-01 concerns incomplete test
  coverage of later dynamic precedence, not a regression in the algorithm text.
- `B-03`: partially regressed as an evidence-coverage matter only: all required
  independent identity, transfer, restart, and dynamic poisons are present, but
  the new surface dynamic boundary lacks the promised competing vector.
- `B-04`: remains closed. The intended owner and actual carrier/V8/native/
  resident seams remain in the supplemental anti-cache scan with derive/manual
  trait checks.

### Evidence

- `Ran:` ordered four-file manifest recipe produced
  `c8f4fbb00b17fdd1746ad2fc768c4906325820b81efe87983a02e9e949172b74`,
  matching the supplied final manifest.
- `Static:` contract, assertion, disposition, and readiness text consistently
  name the added dynamic poison families, but none compensates for the missing
  `DynamicSurfaceState` competing pair in the executable expected-red test.

### Final-manifest verdict

`FAIL / HOLD FOR AMENDMENT`. The contract cycle is not yet ready for independent
verification because the final manifest's claimed adjacent dynamic precedence
coverage omits the dynamic-surface boundary. This does not reopen B-01, B-02, or
B-04 and requires only a focused expected-red/test-artifact correction.

## Focused final closure re-review B

Evidence mode: `Static + Ran + Expected-red`

Manifest under review:
`216375200839ee0526a37e43985bf8f6729a795985d2a0501c67b5eb48b7ecc3`.
This review remained independent and did not consult review A.

### Closure findings

No open finding remains.

- `B-FINAL-01`: **CLOSED**. The competing-poison matrix now includes the two
  missing adjacent pairs, `DynamicVegetationState -> DynamicSurfaceState` and
  `DynamicSurfaceState -> DynamicSoilHydrologyState`
  (`snow_stage3_v11_adaptive_production_tests.rs:2342-2350`). It then continues
  through soil/hydrology -> native solver/residual -> output (`:2351-2355`).
  The isolated `DynamicSurfaceState` rejection remains present in the complete
  poison population (`:2297-2301`).
- `B-03`: **CLOSED**. The expected-red test now covers each named dynamic
  surface independently and crosses both sides of the dynamic-surface boundary
  with full/admitted error and ordinal equality, first-only triggering, zero
  fallback/publication, and byte-exact rollback (`:2356-2374`). The updated
  disposition and readiness artifacts accurately describe vegetation ->
  surface -> soil/hydrology precedence.
- `B-01`, `B-02`, and `B-04`: **NO REGRESSION**. The focused amendment does not
  change distinct structural/resident custody, resident-revision authority,
  source-position plan joins, or complete source-guard coverage.

### Evidence

- `Ran:` the ordered four-file manifest recipe produced
  `216375200839ee0526a37e43985bf8f6729a795985d2a0501c67b5eb48b7ecc3`,
  matching the supplied manifest and `contract_ref.md`.
- `Ran:` scoped `git diff --check` over the corrected contract/test/artifact
  paths — PASS.
- `Ran:` focused v30 contract-derived Nextest assertion — PASS, 1/1.
- `Expected-red:` production behavioral tests remain correctly gated on absent
  real implementation APIs and are not represented as passing runtime evidence.

### Focused final verdict

`PASS`. B-FINAL-01 and B-03 are closed with no regression to B-01, B-02, or
B-04. The contract cycle is ready to proceed to independent verification; this
verdict makes no production or performance-retention claim.

## Formatting-only final identity confirmation B

Evidence mode: `Static + Ran`

The rustfmt result preserves the complete v30 assertion set: distinct structural
V8 versus resident V3/V2 custody, resident-revision authority, source-position
joins, dynamic vegetation/surface/soil-hydrology and solver/output precedence,
BEI mapping to existing `INV-159/C-019`, and calibration/no-physics boundaries
remain asserted. Inspection found formatting changes only and no semantic or
authority regression.

- `Ran:` `cargo fmt --all -- --check` — PASS.
- `Ran:` focused v30 contract-derived Nextest assertion — PASS, 1/1.
- `Ran:` scoped test-file `git diff --check` — PASS.
- `Ran:` ordered four-file manifest —
  `f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`,
  matching the supplied final identity.

Final identity verdict: `PASS`. The formatting-only change does not reopen any
review finding, and the contract cycle remains ready for independent
verification.
