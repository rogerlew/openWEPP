# Gate Results

Status: accumulating; failures and retries will be preserved.

- `Static: predecessor gate` — historical PASS at `3f1cf8ee3`. Child 3 then
  exposed a canonical-JSON evidence defect; Child 1 is temporarily reopened
  for the bounded correction. Current candidate hashes are calculator
  `9278be79...`, unchanged joint core `c9555b2d...`, and vectors
  `9f171b0f...`; the corrected authority suite is 7/7 and fresh review is
  pending.
- `Static/Ran: real-owner predecessor` — PASS. Child 2 closed at
  `f3e9ed641` after both terminal verifiers and the exact-terminal comparator
  passed.
- `Ran: tools/agents/find-agents --for ...` — PASS; instruction chain recorded
  in `required-reading-map.md`.
- Runtime implementation gates have not yet run.

## Foundation implementation loop

- `Ran: cargo check -p openwepp-land-surface-energy` — initial expected FAIL
  while strict modules were incomplete; subsequent PASS after the complete DTO
  foundation was present.
- `Ran: cargo nextest run -p openwepp-land-surface-energy --profile quick` —
  FAIL, 18/19. The positive authority instance exposed a real canonical
  configuration-digest mismatch (`37eb254b...` computed versus
  `45a5d141...` retained). The failure is preserved and implementation
  correction is active; the frozen fixture will not be rewritten to match the
  Rust producer.
- `Ran: corrected strict DTO/runtime rerun` — PASS, 21/21 after generic JSON
  numeric-token exponent normalization and the corrected strict-state fixture.
  Strict crate Clippy also passed.
- `Ran: first complete covered-column solver rerun` — FAIL, 20/21. The
  multirank frozen-solution test returned typed `BacktrackingLimit` at outer
  iteration zero after all 21 authority backtracking exponents failed strict
  normalized-residual decrease. No tolerance or acceptance rule was changed;
  exact solver diagnosis is active.
- `Ran: corrected covered-column solver rerun` — PASS, 21/21. Diagnosis showed
  the test had incorrectly supplied the already accepted frozen solution as an
  initial warm start; the authority requires an accepted iterate to satisfy
  both residual and step convergence, so no strictly improving first step
  existed. The test now starts from the frozen beginning warm start. No solver
  algorithm, tolerance or acceptance predicate changed; strict crate Clippy
  remains PASS.

## Real-owner integration loop

- `Ran: first mixed real-hydrology integration target` — FAIL, 4/5. The open
  surface potential/one-authorization/final reconstruction reached independent
  boundary validation and rejected `mixed F <= A <= D`. The existing Child-2
  real-owner target remained 3/3 PASS and the LSE crate remained 21/21 PASS.
  No value was clamped, request inflated or tolerance added; exact basis and
  final-use diagnosis is active.
- `Ran: corrected mixed real-hydrology integration target` — PASS, 5/5. The
  failure was the representational roundtrip `A_stand/(f_t*dt)*(f_t*dt)` on an
  active cap. The owner protocol now uses the original typed authorization
  operand bit-for-bit when the solver identifies `AuthorizationActiveOrTie`;
  constitutive branches retain their solved final amount. Physical local rate
  and energy operands remain solver-derived. No tolerance or clamp was added.
- `Ran: Child-2 preservation and orchestrator quick suites` — PASS: existing
  real-hydrology integration 3/3 and orchestrator crate 507/507. Strict
  orchestrator Clippy then exposed only an in-progress low-crate line-count
  finding in `finalize_covered_phase`; structural split is active.
- `Ran: strict affected-crate Clippy retry` — low LSE crate PASS after its
  structural split; orchestrator FAIL solely on the newly reached
  `candidate_from_finalized_uses` line-count rule (104/100). The owner is
  splitting that function structurally; no lint suppression or constitutive
  change is used.
- `Ran: final bounded focused gate` — PASS: LSE and orchestrator checks;
  orchestrator strict all-target Clippy; LSE runtime 26/26; LSE-real-hydrology
  integration 5/5; preserved Child-2 integration 3/3; LSE authority 7/7;
  formatting; diff hygiene; and package Markdown 10/10. These gates support
  the retained bounded checkpoint, not the prohibited forest runtime endpoint.
- `Static: independent surface-liquid custody review` — PASS for HOLD
  legitimacy. No actual persistent production litter/surface store or
  condensation-credit candidate exists; residue interception,
  depression/WAT5 diagnostics and snow liquid are wrong-owner aliases.
- `Static/Ran: selector exclusion` — PASS for the bounded checkpoint. No new
  runner, production scheduler, selector, default or publication reference was
  found; the new bridge is reached only through an explicit library/test API.

## Exact HOLD checkpoint rerun

- `Ran: cargo check -p openwepp-land-surface-energy` — PASS.
- `Ran: cargo check -p openwepp-hillslope-orchestrator` — PASS.
- `Ran: cargo clippy -p openwepp-land-surface-energy --all-targets -- -D warnings`
  — PASS.
- `Ran: cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
  — PASS.
- `Ran: cargo nextest run -p openwepp-land-surface-energy --profile quick` —
  PASS, 26/26.
- `Ran: cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick`
  — PASS, 5/5.
- `Ran: cargo nextest run --test vegetation_real_hydrology_shadow_contract --profile quick`
  — PASS, 3/3.
- `Ran: cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick`
  — PASS, 7/7.
- `Ran: cargo fmt --all -- --check` — PASS.
- `Ran: git diff --check` — PASS.
- `Ran: markdown-doc lint` — PASS: Child 3 15/15, coordinator 16/16,
  `docs/ROADMAP.md` 1/1 and work-package catalog 1/1.
- `Ran: recursive runner selector scan` — PASS; no runner reference to the new
  crate or shadow module.

Broad workspace, benchmark and terminal gates were not run because the
required forest endpoint is absent. Running them would not convert this HOLD
into a complete Child-3 release.

## Dependency lift and resumption

- PASS: persistent snow-free surface-liquid hydrology custody dependency closed
  at `a7d692da4` with dual terminal verification, exact-head 2,901/2,901
  workspace evidence and byte-preserving prompt archival.
- Static: `SC-LANDSURFACEENERGY-001@3` and `SC-SURFACELIQUID-001@6` authorize
  the covered forest implementation; no successor model identity is required.
- Static: the historical HOLD and all rejected aliases remain preserved. Child
  3 resumes in this package and has not yet earned runtime, benchmark or
  terminal completion evidence.

## Covered forest real-owner increment

- FAIL then PASS: the first wrapper integration exposed a typed identity defect
  in `RootRuntimeIdentity`: root keys inherited the LSE owner. The DTO now
  carries the actual vegetation owner, root/LSE alias and mixed-owner rows are
  rejected, and the frozen `vegetation-v8` versus `land-surface-energy-v1`
  owner identities pass without rewriting keys.
- PASS: frozen two-rank forest/litter potential and fixed-cap solves feed one
  unified authorization spanning eight root rows, the litter store and the
  explicit companion open-tile row.
- PASS: covered focused tests 2/2; complete integration 71/71; LSE 31/31;
  affected checks and strict all-target Clippy; formatting and diff hygiene.
- PASS: finalized D/A/F, accepted energy/diagnostics, post-solve covered ingress,
  three physical rollback owners and byte-identical production frame are
  exercised through the public wrapper.
- Bounded limitation: the frozen covered solve is evaporative. Existing unified
  owner tests prove positive condensation credit, but a constitutive covered
  condensation fixture and the V8 vegetation/BGC five-owner envelope remain
  pending; Child 3 remains executing.
