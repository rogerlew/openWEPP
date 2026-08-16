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

## V8 state and accepted-operands increments

- PASS at `1202fb76f`: separate strict `OPENWEPP_C3_WOODY_V8` vegetation state,
  exact registry identity, deterministic state digest, restart parsing and
  V7-to-V8 migration. Occupancy lanes remove exactly the two canopy-air fields;
  each covered vegetation tile owns exactly one temperature/humidity lane.
  Ambiguous V7 tile lanes report both unresolved fields and never average,
  select, reset or synthesize values. The V7 executable identity remains
  unchanged.
- PASS at `b127cbfdf`: the joint LSE solver retains accepted class-resolved
  `Ag`, `An` and `Rd` at the solved `ci` and temperature. A sealed potential
  payload and a distinct sealed fixed-authorization-final payload preserve V8
  model, LSE configuration/beginning-state, owner, OFE, tile, pass and typed
  root D/A/F identity. The cap-active regression proves potential and final
  carbon are not aliases.
- `Ran: cargo nextest run -p openwepp-vegetation -p openwepp-land-surface-energy --profile quick`
  — PASS, 273/273.
- `Ran: cargo check -p openwepp-hillslope-orchestrator` — PASS.
- `Ran: cargo clippy -p openwepp-vegetation -p openwepp-land-surface-energy --all-targets -- -D warnings`
  — PASS.
- `Ran: cargo fmt --all -- --check` and `git diff --check` — PASS.
- Bounded limitation: the accepted payload intentionally does not claim a
  complete E04 ending canopy store or release ledger. The current covered
  solver has not yet integrated rainfall, interception, throughfall, stemflow
  and both drainage terms. The later V8 vegetation owner remains fail-closed
  until those accepted operands and exact vegetation beginning-state lineage
  are present.

## V8 persistent composition and E04 increments

- PASS at `fac5b4fd7`: the dependency-neutral V8 persistent phase consumes the
  sealed potential and fixed-authorization carbon receipts without rerunning
  V7 water or E01--E15 physics. Potential carbon issues the immutable mineral-N
  requests; capped carbon determines final demand and receipt-bound growth.
  One global nitrogen authorization is retained and `T10` is advanced from the
  immutable beginning state and forcing. The phase remains uncommitted.
- PASS on the current E04 bytes: rainfall, interception, throughfall,
  stemflow, initial drainage, signed wet evaporation/condensation and second
  drainage are evaluated inside every covered-column residual. Descendants
  receive the accepted upper release top-to-bottom; stemflow bypasses lower
  foliage. Potential and fixed-authorization-final passes rebuild separately
  from the immutable beginning stores.
- PASS: sealed vegetation-facing potential and final payloads now retain the
  complete pass-tagged occupancy liquid operands, ground canopy release and
  ground stemflow. Independent validation reconstructs occupancy closure,
  top-to-bottom routing, final signed wet phase change, exact root `D/A/F` and
  the canonical 4,218 J kg^-1 K^-1 liquid enthalpy.
- `Ran: cargo nextest run -p openwepp-land-surface-energy --profile quick` —
  PASS, 33/33.
- `Ran: cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick`
  — PASS, 71/71.
- `Ran: cargo clippy -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
  — PASS.
- `Ran: cargo fmt --all -- --check` and `git diff --check` — PASS.
- `Static: line-count governance` — PASS: `solver.rs` is 2,998 lines after
  extracting the E04 ledger and covered output DTOs; no lint suppression was
  added.
- Remaining boundary: the public covered positive-condensation custody vector
  and the complete V8 vegetation/BGC/heterogeneous owner envelope are still
  pending. No public V8 commit claim is made.

## Public covered condensation custody vector

- PASS: the actual covered forest wrapper solves a positive-condensation case
  from humid air and cold ground/soil starts while the hydrology-owned litter
  store begins at its exact 6 kg m^-2 tile capacity.
- PASS: the accepted ground branch is typed `Condensation`; ground request,
  authorization and finalized withdrawal are exact zero. The signed credit
  retains transaction, hydrology owner, OFE, tile, surface, stand-ground basis,
  temperature and canonical liquid enthalpy.
- PASS: the full store remains at capacity and the entire condensation excess
  becomes typed `CondensationOverflow` ingress. WB14 receipt mass and enthalpy
  reconstruct under the canonical closure envelope after the required
  stand/tile conversion roundtrip.
- PASS: unified candidate validation, unchanged production-frame bytes and all
  rollback hashes (`before_sha256 == after_sha256`).
- `Ran: focused covered condensation case` — PASS, 1/1.
- `Ran: cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick`
  — PASS, 72/72.
- `Ran: cargo fmt --all -- --check` and file-scoped `git diff --check` — PASS.
- Remaining boundary: persistent V8 vegetation/BGC and the complete
  heterogeneous owner envelope remain pending.

## Uncommitted V8 vegetation owner candidate

- PASS: an explicit typed LSE-component-to-vegetation-occupancy bijection
  binds the dependency-neutral final receipt without parsing or concatenating
  identity strings. Missing, duplicate and non-bijective mappings fail closed.
- PASS: candidate construction requires the exact potential carbon pass,
  fixed-final carbon pass, persistent phase, V8 configuration, beginning state,
  transaction, interval, tile fraction and one consistent LSE lineage.
- PASS: ending occupancy liquid and numerical lanes and the shared tile
  canopy-air lane are sourced only from the fixed-authorization-final receipt.
  Shared strata come only from the already finalized E16--E22 preallocations.
- PASS: the V8 state digest is recomputed and validated; material proposals are
  bound deterministically; vegetation C, N and dry-material ledgers are
  independently reconstructed. The type exposes no commit method, water
  arbiter or V7 E01--E15 entry.
- `Ran: cargo nextest run -p openwepp-vegetation --profile quick` — PASS,
  249/249.
- `Ran: cargo check -p openwepp-vegetation` — PASS.
- `Ran: cargo clippy -p openwepp-vegetation --all-targets -- -D warnings` —
  PASS.
- `Ran: cargo fmt --all -- --check` and `git diff --check` — PASS.
- Remaining boundary: construct the independent BGC receiving candidate and
  bind vegetation, real hydrology, LSE, BGC and soil thermal into one
  heterogeneous atomic shadow transaction.
