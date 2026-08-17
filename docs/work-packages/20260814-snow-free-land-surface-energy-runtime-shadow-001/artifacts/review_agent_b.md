# Independent Land-Surface Science Review B

Evidence class: `Static + Ran`

Reviewed exact commit: `dfc7cf971284d772246f147382f4bb8a2292ee4c`

Reviewer role: independent land-surface science, conservation, and owner-boundary
review. The review covered the released Child-1 authority, the Child-2 real-water
owner, the Child-3 package and evidence, the V8/LSE contracts, the exact covered
runtime, the physical-to-persistent projection, the receiving-owner envelope,
and the focused integration tests.

## Disposition

`HOLD / six material findings / not ready for Child-3 terminal gates`

The constitutive covered-column solver, persistent surface-liquid custody, and
typed water protocol are substantial and the focused suites pass. The exact
public endpoint is nevertheless not yet the admitted complete V8/LSE runtime.
It accepts unbound caller physics, accepts caller-created canopy ingress and
companion-tile candidates, lacks the required independent whole-column energy
reconstruction, publishes synthetic rollback hashes, and does not bind the
covered oracle vectors. These are endpoint defects inside Child 3, not reasons
to replace the package or alter authority.

## Commands run

- `cargo nextest run -p openwepp-land-surface-energy --profile quick` — PASS,
  33/33.
- `cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract
  --profile quick` — PASS, 73/73.
- `cargo nextest run --test land_surface_energy_balance_authority_contract
  --profile quick` — PASS, 7/7.
- `git diff --check` — PASS.

Passing focused tests do not disposition the findings below because the
positive endpoint fixture itself supplies the unbound or missing operands.

## Findings

### B-CRITICAL-001 — Final V8 canopy release is not the hydrology ingress

Authority: `SC-LANDSURFACEENERGY-001@3`, “Immutable-beginning water
transaction and current ingress,” requires hydrology to accept the **final**
capped throughfall, both drainage terms, and stemflow after the solve. V8
requires every release to carry its accepted wet-surface temperature and
enthalpy exactly once (`INV-VEGETATION-114`).

The public covered path instead accepts an independent caller-created
`DirectSurfaceLiquidIngressInput` at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_forest.rs:66`
and passes it to the unified owner at lines 85--147. No code in that module
compares it with
`final_tile.vegetation_operands.ground_canopy_release_kg_m2_tile_ground`,
`ground_stemflow_kg_m2_tile_ground`, or the per-occupancy final E04 ledgers.
The final physical release is retained, but it is not the ingress consumed by
hydrology.

The positive public fixture demonstrates the gap rather than closing it. Its
physical column sets `top_rain_kg_m2_tile = 0.0` at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract/covered_forest_tests.rs:175`,
while `covered_ingress(0.05)` independently creates `0.05 kg m^-2` of covered
throughfall at lines 632--675 and supplies that record to the endpoint at line
1034. Consequently arbitrary mass and enthalpy can be inserted or a real final
release can be omitted while all current validators pass.

Required correction: construct the covered ingress only from the accepted
fixed-cap E04 ledgers after the final solve. Preserve each release kind,
occupancy/tile/OFE identity, accepted wet-surface temperature, and enthalpy;
conservatively mix only where hydrology explicitly owns a merge. The finalizer
and unified-owner boundary must make a caller-supplied replacement impossible.
Add wrong-mass, wrong-temperature, missing-release, duplicate-release, and
potential-pass-release poisons.

### B-CRITICAL-002 — The public V8 identity does not bind the physical model inputs

Authority: `SC-LANDSURFACEENERGY-001@3` requires strict configuration, state,
forcing, topology, and owner identity before calculation. It also requires V8
to perform the unchanged V7 full-column E01--E03 two-stream solve using the
ground VIS/NIR albedos as the lower boundary.

`execute_covered_v8_transaction()` accepts an already assembled
`CoveredColumnInputs` at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_transaction.rs:153`.
The function validates the separate V8 configuration/state and a small
persistent-forcing receipt at lines 170--211, but it never derives or verifies
the column's LAI/SAI, optical absorption, terminal band/direction shortwave,
biochemical capacities, hydraulic parameters, root fractions/soil state,
aerodynamic geometry, atmospheric forcing, ground thermal properties, or LSE
beginning enthalpy against those identities. `construct_v8_beginning_trial()`
only joins numerical warm starts and canopy liquid (lines 35--142).

`RuntimeTileIdentity` carries claimed LSE digests, but `CoveredColumnInputs`
contains no digest and no canonical projection is checked. The public fixture
manually constructs the physical column at
`covered_forest_tests.rs:146--225` and independently constructs/mutates a V8
configuration at lines 318--514. Matching selected values in that one fixture
does not prevent a caller from changing absorbed PAR, LAI, `Vcmax25`, a root
conductance, reference wind, or ground terminal radiation while retaining the
same accepted V8/LSE identity.

Most importantly, the endpoint consumes precomputed leaf/stem absorption and
ground terminal radiation; it never invokes or validates the admitted E01--E03
whole-column radiation solution. Thus an arbitrary or stale shortwave handoff
can drive E04/E11--E22 under a valid V8 digest.

Required correction: add one strict, digest-bound projection from admitted
V8/LSE configuration, state, forcing, topology, radiation, hydrology, and soil
thermal snapshots into the joint column problem. Compute E01--E03 in that
path, including the ground-albedo lower boundary, and reject any caller or
receipt whose complete primitive operands do not match the bound identities.
Add one-bit poisons for every presently independent physical family.

### B-CRITICAL-003 — The mixed open/covered owner set contains caller placeholders

The endpoint accepts these companion surfaces directly at
`covered_v8_transaction.rs:162--165`:

- potential requests;
- finalized uses;
- ending LSE tile states; and
- soil-thermal candidates.

`execute_covered_forest_shadow()` appends those arrays without executing their
potential or final surface solve (`covered_forest.rs:70--73`, 77--83,
111--130). The integration configuration assigns the open tile fraction
`1 - 0.38 = 0.62` (`covered_forest_tests.rs:371--379`), yet the fixture supplies
an exact-zero open request/use and manually created ending LSE/thermal state at
lines 967--1020. No open-tile radiation, surface temperature, evaporation,
ground heat, or energy closure participates in this purported complete owner
set.

This violates the Child-3 tile-local runtime and weighted-OFE requirements and
allows arbitrary companion candidates to borrow the valid unified-owner
envelope. It also means the present 73/73 fixture does not prove the required
one-open/one-covered authority vector through one authorization.

Required correction: execute every configured tile from one immutable
beginning snapshot, sort by typed tile identity, issue all potential requests,
authorize exactly once, and rebuild every tile under fixed caps. Remove public
caller injection of requests, final uses, and ending owner states. Independently
close each local tile before weighting the complete OFE exactly once.

### B-HIGH-004 — The energy owner validates ground only, not the coupled column

The retained `TileEnergyOperandSet` contains only ground-surface energy,
ground latent identity, and a ground-heat join
(`crates/openwepp-land-surface-energy/src/transaction.rs:613--633`).
`build_covered_energy_operands()` reconstructs only those same ground fields
at lines 1460--1512.

Canopy sun leaf, shade leaf, wet surface, dry stem, shared canopy-air heat and
vapor, reciprocal longwave, and full directional/band shortwave closure remain
inside producer solver residuals in
`crates/openwepp-land-surface-energy/src/solver.rs:1475--1550` and
1881--1985. The endpoint test merely calls `energy_operands.validate()` at
`covered_forest_tests.rs:1525--1531`, so it proves the ground subset, not the
authority-required independent local and weighted whole-column closure. The
caller-placeholder open tile has no energy operands at all.

Required correction: expose primitive, identity-bearing component operands and
have a validator outside the producing residual module reconstruct every
canopy component, the shared heat/vapor node, reciprocal longwave, directional
VIS/NIR shortwave, ground, soil storage/G, latent mass-energy joins, and the
weighted OFE ledger. It must consume finalized water, never authorization or a
producer residual.

### B-HIGH-005 — Rollback hashes are declarations, not hashes of owner bytes

`rollback_hashes()` in
`crates/openwepp-land-surface-energy/src/transaction.rs:715--750` sets every
`after_sha256` equal to the supplied `before_sha256` by construction. It does
not serialize or hash any after-failure owner state. Worse, the vegetation and
BGC rows use generic owner strings and reuse the LSE beginning-state digest at
lines 733--741 rather than the actual V8 vegetation and BGC state identities.

The public test at `covered_forest_tests.rs:1542--1547` then asserts only that
these producer-created strings are equal. Although the borrowed production
frame is also compared after a successful uncommitted call, there is no
phase-specific failure matrix over V8 vegetation, surface/soil hydrology, LSE,
soil thermal, BGC, request/authorization/use records, ingress, and diagnostics.

Required correction: compute rollback evidence from deterministic serialization
of each real beginning owner and the actual post-failure owner/envelope bytes.
Use the real owner IDs and distinct digests. Inject failures after each
Child-3 construction/validation boundary and prove exact byte identity; do not
populate both sides from one claimed digest.

### B-HIGH-006 — Covered potential/final oracle vectors are not bound to Rust

The checksum-bound authority fixture includes `covered_single_rank` and
`covered_multirank` potential, fixed-cap, alternate-warm-start, and failure
vectors. The Rust covered test copies many input and expected-solution literals
into `solver.rs:2530--2997`, but it does not load the fixture and does not
compare the accepted multirank potential or fixed-cap solution with the frozen
expected arrays. Its terminal assertions are acceptance, cardinality, branch,
and `F<=A` checks. Repository search finds the committed fixture consumed for
the open fixed-cap solution at `transaction.rs:2190`, but no corresponding
covered-vector consumption.

This leaves a required Child-3 exit gate unproven: the same tests can pass after
a coherent drift in the covered constitutive result. The authority suite's
7/7 PASS proves fixture and contract integrity, not Rust reproduction.

Required correction: consume the committed digest-bound covered fixtures in
ordinary Rust tests and compare all accepted state, component flux, D/A/F,
energy, diagnostics, alternate-warm-start, singular, iteration-limit, and
backtracking outputs under the authority's exact comparison rules. Regenerate
the fixtures only in the separate independent-oracle gate.

## Non-material lifecycle observation

`artifacts/final-disposition.md` still presents the historical surface-liquid
custody HOLD as the current package disposition. Preserving that HOLD as
historical evidence is correct, but the terminal artifact will need an
in-progress placeholder during remediation and a new exact-byte disposition
only after these findings, both reviews, benchmarks, heavy gates, and terminal
verification close.

## Exit-readiness decision

Child 3 must not start benchmark/heavy/terminal closure gates or mark its final
forest endpoint complete at `dfc7cf971`. Continue in the existing package.
The released authority is sufficient for the correction routes above; no new
contract-first package or model identity is indicated by this review.
