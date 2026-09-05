# Independent contract review A

Evidence mode: `Static`

Verdict: `FAIL`

Promotion recommendation: `HOLD`

## Findings

| Finding | Severity | Summary | Required disposition |
|---|---|---|---|
| `CPSVO-A-001` | critical | The amendment's central same-map proof lineage does not match the current call graph and its stated ordering would change observable error precedence. | Rewrite the proof lineage and ordered algorithm against the real native-resident call site before production work. |
| `CPSVO-A-002` | high | The expected-red parity test requires a native-V3 consumer in the explicitly ordinary case, where the existing authentic oracle requires zero native physical executions. | Make native-consumer/proof use conditional on the native regime and positively prove nonuse in ordinary execution. |
| `CPSVO-A-003` | high | Role/path parity and first-error precedence are under-specified by the behavioral assertions. | Prove the declared role/path coverage per applicable regime and add competing-poison precedence vectors. |
| `CPSVO-A-004` | high | The anti-cache/nonwire source guard omits the new type-definition file and cannot detect derived `Clone`/serde implementations. | Include the owning module and use compile-time negative-capability tests or an equivalently non-evadable check. |
| `CPSVO-A-005` | medium | The contract-cycle artifact reports `Ran` results without exact commands or inspectable log paths and does not define how its manifest digest was constructed. | Record exact commands/results, retain their logs, and define/recompute the ordered manifest recipe. |
| `CPSVO-A-006` | medium | Contract tests pin the v30 narrative note in the lifecycle index, contrary to the repository's doc-coupled test rule. | Assert registry identity/path/lifecycle/date only; keep detailed v30 authority assertions on the canonical contract. |

### `CPSVO-A-001` — proof producer and error order are not source-true

The contract says that the V8 projection freshly validates the exact LSE V3
configuration/state and surface configuration/owner, then mints the proof
consumed by native V3
(`SC-LANDSURFACEENERGY-001.md:2723-2733`). That is not the current boundary:

- `strict_v8_endpoint.rs:615-642` passes the structural inner LSE
  configuration/state and the parent surface configuration to
  `project_v8_runtime_inputs_with_carriers`;
- `frozen_litter_v3_adoption.rs:1126-1138` separately passes the resident V3
  configuration/state and V2 surface configuration/owner to the native path;
- `v3_multitile_adoption.rs:176-180` performs the expensive resident V3 state
  validation and V2 owner canonicalization; and
- `strict_v8_endpoint.rs:645-657` runs fallible ingress-schedule derivation
  between the V8 projection and that native validation.

The proposed V8 proof therefore cannot attest to pointer-identical objects it
does not currently receive. Moving the native validation into V8 would both
repeat rather than eliminate that work and move its error ahead of the
intervening ingress-schedule errors. There is an earlier contradiction too:
`frozen_litter_v3_adoption.rs:958-987` validates the native forcing before the
V8 projection, while contract lines 2702-2706 and 2716-2722 require first-map
static validation before forcing and then claim unchanged error order.

A feasible source-aligned route appears to be the already validated immutable
`FrozenLitterV3Resident` revision at
`frozen_litter_v3_adoption.rs:30-65`: mint a borrowed/move-only capability from
that revision at the existing native-consumer position and consume it only for
the exact resident V3/V2 references. The parent-static plan is a separate
cross-map capability and should not be described as if V8 had freshly fully
validated the resident V3 owner. Whatever route is selected must preserve the
actual forcing, ingress, V8, and native-validation precedence and bind the
relevant carrier authority (`INV-LANDSURFACEENERGY-161` and its coupled-time
authority), which the amended `INV-159` authority column currently omits.

Impact: the current text could authorize either an impossible proof, duplicate
work with no measured benefit, or a real first-error reordering. Production
implementation is unsafe until this authority is corrected.

### `CPSVO-A-002` — ordinary parity demands a nonexistent native consumer

`carrier_validation_once_is_bitwise_equal_for_every_role_path_and_regime`
unconditionally asserts `used_real_native_v3_consumer` for every case
(`snow_stage3_v11_adaptive_production_tests.rs:2173-2185`), including
`Case::Ordinary`. The existing authentic regime oracle explicitly defines
ordinary as `native_physical_count == 0`
(`canonical_covered_solver_test_audit.rs:191,264-267`). A conforming ordinary
execution should therefore prove zero native-V3 proof mint/consume calls, not
one. As written, the test either rejects correct ordinary behavior or invites
a fabricated flag/irrelevant native call.

### `CPSVO-A-003` — parity and precedence can pass with missing coverage

The parity test aggregates all `(case, role, path)` records, then checks only
that each role and path appears somewhere
(`snow_stage3_v11_adaptive_production_tests.rs:2218-2230`). It does not require
each applicable native/multilane regime to cover its promised role/path set, so
ordinary records can mask missing native Half1/Half2 or history/final evidence.
Likewise, the poison test uses one poison at a time and compares two reported
errors/ordinals (`:2241-2268`); it never presents competing invalid conditions,
so it cannot establish the contract's declared first-error priority. Add an
explicit applicability matrix and paired poisons spanning parent-static,
support/duration/transaction/joint, forcing, V8, native V3, dynamic, and output
boundaries.

### `CPSVO-A-004` — privacy guard misses the owning source and derived traits

The source guard scans four execution files
(`snow_stage3_v11_adaptive_production_tests.rs:2286-2293`) but omits the newly
authorized owning module
`land_surface_energy_shadow/covered_parent_structural_admission.rs`. Its
forbidden strings for `Clone`, `Serialize`, and `Deserialize` (`:2306-2319`)
match only manual `... for Type` implementations; `#[derive(Clone, Serialize,
Deserialize)]` would pass. Since non-Clone/nonwire behavior is the mechanism
that prevents cross-map/restart proof transfer, this is a material authority
loophole. The test should prove the negative capability at compile time (for
example compile-fail use attempts) and separately inspect all owning modules.

### `CPSVO-A-005` — claimed run evidence is not reproducible

`contract_ref.md:56-62` reports lint, diff, passing contract-test, and expected-
red compile results, but supplies neither exact commands nor evidence-log
paths. Lines 64-66 publish a manifest SHA-256 without saying whether it hashes
concatenated file bytes, ordered `sha256sum` rows, or another representation.
The authoring procedure requires recorded pre-implementation contract-gate
evidence; these claims are not independently auditable in their current form.

### `CPSVO-A-006` — index narrative is treated as authority

The new v30 test and several reconciled older tests require the exact v30 note
text from `science-contracts/index.md`
(`land_surface_energy_balance_authority_contract.rs:267-270`, with repeats at
approximately 296-299, 529-532, 578-581, and 642-645). `tests/AGENTS.md`
requires registry tests to check lifecycle structure and directs detailed
authority assertions to the owning `SC-*` contract. Retain the compact note,
but do not make its prose a contract-test dependency.

## Positive observations

- Reusing existing `INV-LANDSURFACEENERGY-159` rather than creating a solver or
  physics invariant is directionally sound for private validation custody.
- The amendment clearly forbids dynamic/result caches, shared owning handles,
  wire/restart restoration, digest-only admission, cross-map proof transfer,
  and silent fallback.
- The `1/52/52` authentic workload, bitwise full-validation oracle, poison
  matrix, rollback, and real-call-site requirements are the right evidence
  classes once their lineage and assertions are corrected.
- `CALIBRATION_NOT_APPLICABLE` is appropriate for this architecture-only
  amendment; no equation, parameter, unit, tolerance, output, or constitutive
  suite is changed.

Production implementation may proceed only after both critical/high findings
are amended, independently dispositioned, and successfully reverified.

## Corrected-manifest re-review A — 2026-09-04

Evidence mode: `Static + Ran`

Verdict: `FAIL`

Recommendation: retain `HOLD` before independent verification.

### Remaining finding

| Finding | Severity | Status | Evidence and required action |
|---|---|---|---|
| `CPSVO-A-003` | high | `STILL-OPEN` | Contract lines 2763-2765 and 2803-2808 require paired first-error evidence across the ordered dynamic/solver/output boundaries. The declared poison enum at `snow_stage3_v11_adaptive_production_tests.rs:2273-2297` has no dynamic-process, solver, or output poison, and the pair matrix at lines 2325-2336 stops at native-resident/proof custody. Add authentic full/admitted dynamic/solver/output poisons and adjacent precedence pairs, or narrow the canonical claim before re-review. |

### Finding closure audit

| Finding | Status | Re-review evidence |
|---|---|---|
| `CPSVO-A-001` | `CLOSED` | Amended `INV-LANDSURFACEENERGY-159` and the v30 algorithm explicitly separate structural V8 objects from resident V3/V2 objects. Forcing proof consumption remains at V8's later pointer-identical check; fallible ingress remains before the resident join; the native proof is sourced from the exact resident's validated revision at the original native-validation position. Carrier/coupled-time authority is now cited. |
| `CPSVO-A-002` | `CLOSED` | The parity declaration requires ordinary zero native execution and zero resident-proof mint/consume, with native use required only for native regimes (`snow_stage3_v11_adaptive_production_tests.rs:2189-2200`). |
| `CPSVO-A-003` | `STILL-OPEN` | Per-regime full/admitted applicability is now compared exactly and the new paired-poison framework covers early through resident/proof validation, but it does not cover the contract-promised later dynamic/solver/output boundaries. |
| `CPSVO-A-004` | `CLOSED` | The supplemental guard now includes the intended owning module plus carrier, V8, strict-endpoint, native-projector, and resident seams. It checks ordinary forbidden caches, derives, and direct Clone/serde implementations for all three ephemeral types; executable second-use, cross-map, cross-parent, and restart poisons remain required. The persistent revision's inseparable whole-resident clone is explicitly distinguished. |
| `CPSVO-A-005` | `CLOSED` | `contract_ref.md` now records exact commands, result classes, lack of claimed durable logs, ordered paths, and the manifest recipe. Independent recomputation produced the recorded `08ebf9ed86c6b9f91114c871b988983de05434d7f3d64200ffdd0f58d3c793f6`. |
| `CPSVO-A-006` | `CLOSED` | `assert_lse_registry_lifecycle` checks identity, path, status, maturity, and date only; detailed v30 assertions read the canonical contract and no test pins the v30 index note. |

### Commands run in re-review

Ran:

```text
sha256sum <the four ordered canonical paths> | sha256sum
```

Result: `PASS`; exact match to the recorded manifest.

Ran:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`, 14 fully consolidated rows.

Ran:

```text
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`, no findings.

Ran: scoped `git diff --check` over the canonical v30 paths and contract-cycle
artifacts.

Result: `PASS`.

The source-real authority correction is otherwise suitable. Independent
verification should not begin until the remaining A-003 obligation/test mismatch
is corrected and this re-review returns `PASS`.

## Final manifest re-review A — 2026-09-04

Evidence mode: `Static + Ran`

Verdict: `PASS`

Recommendation: `GO` to independent contract verification; production remains
gated on both required verification verdicts.

### Findings-first closure

No open findings remain. `CPSVO-A-003` is now `CLOSED`: the independent poison
population includes `DynamicVegetationState`, `DynamicSurfaceState`,
`DynamicSoilHydrologyState`, `NativeSolverResidual`, and `OutputValidation`.
The competing-poison matrix carries precedence from native resident custody to
dynamic vegetation, dynamic soil/hydrology, native solver/residual, and output
validation. Each pair requires the full-validation and admitted paths to return
the same first typed error and ordinal, proves only the earlier poison fired,
forbids fallback/publication, and checks byte-exact rollback
(`snow_stage3_v11_adaptive_production_tests.rs:2273-2374`). This now matches the
all-ordered-boundary obligation in contract lines 2799-2810.

The correction does not regress the other closed findings:

- `CPSVO-A-001`: structural V8, forcing, ingress, and resident V3/V2 lineage and
  source order remain distinct and source-real;
- `CPSVO-A-002`: ordinary execution still requires zero native physical and
  resident-proof use;
- `CPSVO-A-004`: intended owner and actual consumer seams plus derive/manual
  trait guards remain in the supplemental anti-cache check;
- `CPSVO-A-005`: exact commands and manifest recipe remain recorded; and
- `CPSVO-A-006`: registry tests remain lifecycle-only.

### Commands run

Ran: ordered four-file manifest reconstruction.

Result: `PASS`, exact
`c8f4fbb00b17fdd1746ad2fc768c4906325820b81efe87983a02e9e949172b74`.

Ran:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`; 14 BEI rows fully consolidated and no unit-compliance findings.

Ran: scoped `git diff --check` over the canonical v30 paths and contract-cycle
artifacts.

Result: `PASS`.

Ran:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract version_thirty_binds_parent_static_and_same_map_validation_once_to_existing_invariant
```

Result: `PASS`, 1/1.

## Focused final manifest consistency re-review A — 2026-09-04

Evidence mode: `Static + Ran`

Verdict: `PASS`

No findings. The competing-poison matrix now includes both missing adjacent
links, `DynamicVegetationState -> DynamicSurfaceState` and
`DynamicSurfaceState -> DynamicSoilHydrologyState`, followed by the already
reviewed soil/hydrology -> solver/residual -> output chain. The source-real
lineage/order, ordinary zero-native assertion, per-regime applicability,
privacy/derive guard, reproducible evidence, and lifecycle-only index checks
remain intact.

Ran: ordered four-file manifest reconstruction and scoped `git diff --check`.

Result: `PASS`; manifest exactly
`216375200839ee0526a37e43985bf8f6729a795985d2a0501c67b5eb48b7ecc3`.

Recommendation: `GO` to independent verification.

## Formatting-only final identity confirmation A — 2026-09-04

Evidence mode: `Static + Ran`

Verdict: `PASS`

No finding or authority regression. The rustfmt-normalized integration test
retains the complete v30 invariant, source-order, ordinary-zero-native,
poison/precedence, BEI, and lifecycle assertions. `cargo fmt --all -- --check`
and scoped `git diff --check` passed; the focused v30 contract test passed 1/1.
The independently reconstructed ordered four-file manifest is exactly
`f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`.
