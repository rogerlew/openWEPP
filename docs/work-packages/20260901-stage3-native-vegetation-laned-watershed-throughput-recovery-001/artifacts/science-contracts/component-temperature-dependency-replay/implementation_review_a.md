# Independent implementation review A — final seventh-cut rereview

Evidence class: `Static + Ran`.

This report supersedes every earlier implementation-review-A verdict. The
reviewed authority is `SC-LANDSURFACEENERGY-001` revision 31,
`INV-LANDSURFACEENERGY-164`, and `OBL-LANDSURFACEENERGY-C-020`. The reviewed
source is the exact ordered 16-path set reconstructed from
`/tmp/component_replay_review_cut3_sha256.txt`. Its ordered manifest digest was
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`
before review and again immediately before conclusion.

## Findings

No blocking correctness, science-contract, error-taxonomy, serialization,
runtime-seam, source-quality, fallback, or duplicated-production-physics
finding remains in the exact seventh cut.

## Closure of `CTDR-IMPL-A-R8-001`

Status: **CLOSED**.

Path:

- `crates/openwepp-land-surface-energy/src/solver_component_dependency_replay_test_support.rs:1`

The ordered manifest comparison proves this is the only source change from the
approved-correctness sixth cut. The wildcard import is replaced with the exact
types, traits, statics, and functions used by the test-support module. No item,
field, expression, control flow, feature predicate, visibility, or behavior in
the module changed. The formerly failing feature-enabled all-target
warnings-denied Clippy command now passes cleanly. This closes the sole sixth-cut
source-quality blocker without changing the revision-31 evidence or runtime.

## Closure of `CTDR-IMPL-A-R7-001`

Status: **CLOSED**.

Paths:

- `crates/openwepp-land-surface-energy/Cargo.toml:10`
- `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs:1445`
- `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs:1449`
- `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs:1457`
- `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs:1462`
- `crates/openwepp-hillslope-orchestrator/Cargo.toml:42`
- `crates/openwepp-runner/Cargo.toml:63`

The compiler-only wrapper now uses the existing default-off `test-support`
feature rather than `cfg(doctest)`. With that feature enabled, both rustdoc
snippets resolve `CoveredReplayCapabilityCompileFail` and fail for their
intended operation:

- the post-conversion second-use case emits `E0382`, identifies `value` as moved
  by the first ownership-taking `consume(self)`, and points to the second call;
- the escape case emits `lifetime may not live long enough` and specifically
  states that returning `CoveredReplayCapabilityCompileFail<'static>` requires
  `'a` to outlive `'static`.

The same-module compiler ambiguity assertions continue to bind the actual
private signed-probe and converted-replay concrete types and protect against
`Clone`, `Copy`, `Serialize`, and `DeserializeOwned`. The wrapper contains the
actual converted replay type, has no public constructor or field, and exists
only to expose its ownership/lifetime behavior to the compiler-negative
fixture. The default feature list is empty; ordinary/default rustdoc builds
contain zero such tests. Runner and orchestrator enable `test-support` only as
a development dependency.

The authentic API matrix remains intact: distinct input generations produce
distinct immutable bases; each capability consumes only against its own base;
non-component conversion rejects typed; per-sign pair second use rejects; and
the exact bases remain unchanged. No reviewed test assigns a private
probe, binding, graph, identity, sign, stencil, or capability field.

## Closure of `CTDR-IMPL-A-R7-002`

Status: **CLOSED**.

Paths:

- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_integrity.rs:324`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_integrity.rs:349`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_integrity.rs:389`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_integrity.rs:401`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_integrity.rs:419`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_integrity.rs:441`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_execution.rs:348`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_execution.rs:432`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_execution.rs:543`
- `crates/openwepp-land-surface-energy/src/solver_tests/component_dependency_replay_execution.rs:669`
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md:3006`
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md:3009`
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md:3015`

The retained corrected cut replaces the coarse successful-call assertions with reviewable
pre-evaluation implications over the actual source guards:

- admitted probe temperature, beta, pressure, CO2, canopy humidity, boundary
  conductances, and Medlyn inputs are checked before the leaf call;
- all Arrhenius and peaked-response operands and results are finite/positive;
  `vcmax`, `jmax`, `kc`, `ko`, `gamma`, `tp`, `rd`, surface saturation, VPD,
  absorbed photon flux, and the electron result are explicitly checked;
- the electron, `ac/aj`, and `ag/ap` quadratic calls are all reconstructed. The
  carbon closure checks positive denominators, finite/nonnegative `ac`/`aj`,
  and finite `ai`, `ag`, and `an`;
- the stomatal closure checks finite positive surface CO2, potential/actual
  conductance, resistance, predicted CI, and residual. CI residuals are
  evaluated at the exact `gamma`/ambient endpoints; positive assimilation
  asserts an endpoint zero or opposite signs;
- the represented-snow respiration-dominated path asserts same-sign negative
  ambient residuals, finite positive dark conductance, bounded finite dark CI,
  and a nonnegative dark endpoint before confirming the accepted branch;
- separate pre-evaluation witnesses cover `Inactive` zero area and
  `ExactZeroPar` conductance/CO2/CI predicates. Authentic zero-area, zero-PAR,
  and low-light V11 evaluations assert the exact branch and then run every
  component/sign through replay-versus-complete parity;
- the same leaf proof runs with current beta and exact beta one, covering both
  current and maximum-leaf calls rather than inferring maximum validity from a
  generic successful evaluation.

The non-leaf implication helper now reconstructs finite/domain facts for
liquid preparation, preliminary store/capacity/wet fraction, saturation and
wet-flux finalization, derived emissive areas/temperatures, reciprocal
longwave, root series denominators, immutable caps, six hydraulic
residuals/tolerances, canonical route/finalization ledger, upper-to-lower
incident rain/stemflow, and represented-snow lower-boundary resistance. It is
exercised for wet, dry-zero, and exact-capacity routes with and without caps;
exact replay/complete evaluation projection follows for every component and
both signs.

The missing hydraulic branches are now explicit. The limiting-root vector
reconstructs a positive constitutive law for each occupancy's first root,
installs an authorization at one half of that law, observes
`AuthorizationActiveOrTie`, and proves `0 < final < law` with exact authorized
bits. The zero-scale boundary uses an authentic represented-snow inactive
canopy and asserts exact bits for all six hydraulic residuals and all six
tolerances: the two potential equalities use `1e-7`, the two beta equalities use
`1e-8`, and the last two use `water_tolerance(0)`. Both limiting and zero-scale
cases then run the complete all-component/all-sign projection oracle in both
potential and fixed-final solve classes.

The proof code intentionally reconstructs guard operands in test-only scope.
This is independent contract evidence, not a second production evaluator: it
does not publish results, select a solver, or provide a runtime path. Complete
and replay production continue to share the single canonical evaluator.

## Reconfirmed correctness surfaces

- `ValidatedCoveredComponentProbeReplay::consume` remains O(1) before the
  required evaluator call: it consumes the constructor-minted compact binding
  directly. It performs no trial/probe clone, stencil rebuild, whole-vector or
  whole-input scan, per-probe string/map lookup, hash/length/address proxy
  comparison, fallback, or alternate complete evaluation. The canonical probe
  constructor alone creates the two vectors and selects the stencil.
- The topology-generic direct graph retains every independently enumerated
  node/edge, inclusive closure, and closure-derived action. N=1/S=1 remains 170
  edges with SHA-256
  `418fe7f87bfa93bd6ae8111014e5d31a49f3f5a8074b72aab1b3b13d5b6a6674`;
  N=2/S=6 remains 326 edges with
  `0018f43e5cdd0aa3e8a08bd34f2f5d2cf01175f85f664233678dea08e6cb0207`.
  The walker executes reachable nodes in source order, copies only
  closure-proven unreachable nodes, and supplies every lower route from the
  actual retained upper stemflow prefix.
- Source-real `surface_co2` and `surface_vpd` cover all four sun/shade leaf
  positions and potential/fixed classes. Replay and forced modes return the
  exact source error, record one logical replay with zero complete fallback,
  and preserve input, trial, and byte-complete replay-base custody.
- Rollback projection includes exact input/evaluation/trial bits, caps, frozen
  branches, independent map/solve/iteration/sweep identities, all retained
  canonical nodes, every graph node, every ordered direct edge, every inclusive
  closure member, and every action bit. No hash-only graph proxy remains.
- Complete and replay use one canonical occupancy/column implementation. No
  mirrored production physics/tolerance/residual algorithm, analytic or
  automatic derivative, coloring, simultaneous perturbation, sparse solve,
  pivot-order change, recovery fallback, cache, hardcoded N=2/S=6 runtime path,
  or mutation/fault-injection hook was found.
- Exact potential and fixed-final evaluations, dense Jacobians, pivot/matrix
  norms, iteration/line-search trajectories, typed errors, transaction
  potential owners, and accepted final owners remain bit-identical. Potential
  and fixed solves share the authentic transaction map identity but retain
  distinct actual solve identities.
- Backtracking retains every full prospective, first-domain-valid halved, and
  ordinary residual-decrease attempt in source order with phase,
  exponent/factor bits, complete trial, optional evaluation/norm/step fields,
  and exact decision. The seventh-cut import-only change touches no
  owner/backtracking code.
- Audit lifecycle and aggregation retain authentic completed/failed sweeps,
  solve class, stencils, attempted counts, and disjoint
  anchor/replay/complete buckets. The real centered N=2/S=6 record remains
  `58 = 14 + 16 + 28`; lifecycle identities originate at distinct callers.
  The runner still invokes the real executor in replay and forced modes and
  compares internal records plus HBP and complete WAT/PASS Parquet bytes.
- All 16 reviewed paths remain in the authorized write set. No ordinary/default
  dependency enables the private audit/projection/compiler-witness seam.

## Ran evidence

- Start and end ordered-manifest reconstruction: **PASS 16/16**, exact digest
  `edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`.
- `RUST_MIN_STACK=67108864 nix develop -c cargo test -p
  openwepp-land-surface-energy --features test-support --doc -- --show-output`:
  **PASS 2/2**, with intended `E0382` and genuine lifetime-escape diagnostics.
- The same rustdoc command without `--features test-support`: **PASS 0/0**,
  confirming the compiler-only surface is default-off.
- `nix develop -c cargo nextest run -p openwepp-land-surface-energy --lib
  component_dependency_replay_execution --no-fail-fast`: **PASS 7/7**.
- `nix develop -c cargo nextest run -p openwepp-land-surface-energy --lib
  component_dependency_replay_integrity --no-fail-fast`: **PASS 5/5**.
- Complete component-dependency-replay focused selection: **PASS 14/14**.
- `nix develop -c cargo clippy -p openwepp-land-surface-energy --features
  test-support --all-targets -- -D warnings`: **PASS**.
- Targeted tracked/untracked `git diff --check` and the `<3000`-line ceiling:
  **PASS**.
- Heavy three-run release timing/RSS gate: **NOT RUN**, as directed. This review
  makes no performance-retention claim beyond static hot-path inspection.

## Residual risk and missing gates

No residual code-correctness or source-quality blocker was found. The guard
proof is deliberately test-side arithmetic, so future production guard changes
must update the named implication inventory and should continue to fail the
exact differential corpus if dependency replay diverges. The required heavy
three-run release timing/RSS gate remains a separate release qualification and
must pass before delivery; this review did not execute or infer it.

Every reviewed Rust file is below 3,000 lines, but maintenance headroom remains
thin: `transaction.rs` is 2,987 lines, `stage3_runner_qualification.rs` 2,941,
`solver_covered_solve.rs` 2,941, and `solver_covered_evaluation.rs` 2,922.
Further growth should extract cohesive production/owner groups rather than
consume those margins.

**Disposition: APPROVE.** The exact seventh cut closes `R8-001`, retains the
approved `R7-001`/`R7-002` corrections, and satisfies the reviewed revision-31
correctness, evidence, and source-quality obligations. Release remains
contingent on the separately mandated heavy three-run timing/RSS gate.
