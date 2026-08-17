# Rust Correctness Review — Agent A

Evidence class: `Static + Ran`

Reviewed commit: `dfc7cf971284d772246f147382f4bb8a2292ee4c`

Verdict: `HOLD`

The review used a clean detached worktree at the exact reviewed commit so that
concurrent remediation in the shared primary worktree did not alter the
evidence below.

## Findings

### HIGH — The public covered-V8 endpoint does not consume or validate strict LSE configuration, state, or forcing

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_transaction.rs:148-168`
accepts a caller-constructed `RuntimeTileIdentity` and raw
`CoveredColumnInputs`, but no `LandSurfaceEnergyConfiguration`,
`LandSurfaceEnergyState`, or strict LSE forcing DTO. Its preflight at
`:169-207` validates only V8 vegetation state/configuration and selected
persistent forcing. `crates/openwepp-land-surface-energy/src/transaction.rs:33-53`
therefore carries `configuration_sha256` and `beginning_lse_state_sha256` as
unproven caller assertions; `RuntimeTileIdentity::validate` at `:55-90` never
binds them to actual strict objects. The physical seam at `:447-466` checks
only tile fraction/interval before solving the raw column.

This bypasses the available strict authorities in
`crates/openwepp-land-surface-energy/src/config.rs:394-490` and
`crates/openwepp-land-surface-energy/src/state.rs:24-219`. It permits raw
physical parameters, topology, forcing, and beginning state to be solved and
sealed beneath unrelated valid-looking digests. The public integration fixture
demonstrates the gap by using fabricated digests at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract/covered_forest_tests.rs:263-283`
rather than digests reconstructed from strict LSE configuration/state. This is
silent science/provenance divergence from `SC-LANDSURFACEENERGY-001@3`, and it
also prevents the endpoint from proving the contract's serialization ->
identity -> topology -> operand error precedence. Closure requires a single
strict runtime DTO/seam that validates configuration, state, lineage, forcing,
and their correspondence to the derived `CoveredColumnInputs` before any
numerical bound or solve.

### HIGH — Component/occupancy binding omits the authoritative vertical rank and can silently swap strata

`V8ComponentOccupancyBinding` contains only component and occupancy IDs at
`crates/openwepp-vegetation/src/v8_candidate.rs:53-57`. The beginning-trial
builder checks only a bijection at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_transaction.rs:44-85`,
the projection repeats set/bijection checks at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_projection.rs:174-256`,
and final validation repeats them again at
`crates/openwepp-vegetation/src/v8_candidate.rs:135-153`. None binds the
positionally ordered LSE component to its configured stratum
`vertical_rank`. The solver then routes rain and solves directly in caller
vector order (`crates/openwepp-land-surface-energy/src/solver.rs:1689-1742`),
while the contract requires top-to-bottom rank then typed occupancy ID
(`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md:761-770`).

A caller can therefore associate the upper component's water, carbon, and
thermal result with the lower persistent occupancy, or vice versa, while all
current validators pass. Centralize the three duplicated binding algorithms
behind one typed, configuration-derived mapping authority that proves rank and
canonical order. Because the duplication currently permits silent physical
and persistence divergence, it is closure-blocking under the repository's
duplication policy. Add a deliberately heterogeneous two-rank swapped-binding
poison at both the component seam and public endpoint.

### HIGH — The covered Newton solver accepts a non-decreasing step contrary to the canonical strict-decrease algorithm

The canonical contract requires every accepted factor to produce a strict
decrease and classifies failure through `b=20` as a backtracking limit
(`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md:777-797`;
the authority numerical summary repeats strict infinity-norm decrease at
`docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/numerical-solver-contract.md:26-39`).
However, when the current residual norm is already at most one,
`crates/openwepp-land-surface-energy/src/solver.rs:2240-2267` takes a full
Newton step based only on trial-domain and dimensional-step acceptance. Unlike
the ordinary backtracking branch at `:2269-2300`, it does not require
`prospective_norm < norm`. The next loop can accept that new iterate when its
residual remains at most one (`:2149-2174`), including an equal or increased
norm.

The package reference calculator contains the same exceptional branch, so
this is also an authority-artifact/canonical-contract conflict; matching that
calculator does not supersede the canonical `SC-*` text. Adjudicate the
authority conflict, then either restore strict decrease in both implementation
and oracle or amend/release the canonical contract before accepting this
behavior. A poison must demonstrate disposition at floating-point stagnation
and at a step whose residual increases while remaining below one.

### HIGH — Runtime numerical failures discard their typed kind and required diagnostic/rollback evidence

The solver returns a detailed `NumericalFailure`, but the covered potential and
final runtime paths discard it and return the unrelated generic
`NumericalAcceptedResidual` variant at
`crates/openwepp-land-surface-energy/src/transaction.rs:463-467` and
`:1877-1883` (the open path does the same at `:996-1001`). The detailed
constructor at `:814-870` is consequently not used by these failure paths.
Moreover, `NumericalSingular`, `NumericalBacktrackingLimit`, and
`NumericalIterationLimit` are defined at
`crates/openwepp-land-surface-energy/src/error.rs:61-68` but have no production
construction sites.

This collapses singular, backtracking, and iteration-limit taxonomy into an
accepted-step/residual error and loses ordered residuals, iteration/backtrack
counts, pivot/matrix evidence, active caps, and rollback hashes required by
`SC-LANDSURFACEENERGY-001@3` at
`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md:803-814`.
Propagate a typed rejection carrying `NumericalDiagnostics` through the public
transaction error. Add public-endpoint poisons for all three solver rejection
kinds and assert code, kind, evidence, and all-owner rollback lineage.

### MEDIUM — Valid mineral-N authorization reordering is accepted by vegetation but rejected by the BGC join

`NitrogenArbiter::authorize` promises no response ordering
(`crates/openwepp-vegetation/src/transaction.rs:596-602`), and
`ValidatedMineralNitrogenAuthorizations::try_new` correctly validates an
unordered authorization vector by keyed map
(`crates/openwepp-vegetation/src/nitrogen_protocol.rs:243-276`). The persistent
phase retains the arbiter's original global order at
`crates/openwepp-vegetation/src/persistent_phase.rs:309-321,378-383`; its new
ordering helper canonicalizes only finalized uses at `:394-447`.
`construct_biogeochemistry_candidate` then positionally zips requests,
authorizations, and uses at
`crates/openwepp-biogeochemistry/src/lib.rs:253-280`. Thus a complete,
key-correct authorization batch in a different order passes vegetation
validation but fails the BGC owner join. The regression at
`crates/openwepp-vegetation/src/persistent_phase.rs:665-745` reverses only uses
and leaves authorizations request-ordered, so it does not cover the seam.

Canonicalize requests, authorizations, and finalized uses once by the complete
typed protocol identity before publication, or make the BGC join key-based.
Add a valid arbiter that deliberately returns reversed authorizations in the
public two-stratum transaction.

### MEDIUM — Terminal write-set and line-count governance evidence is stale and incomplete

The declared write set at
`docs/work-packages/20260814-snow-free-land-surface-energy-runtime-shadow-001/package.md:17-28`
does not authorize `crates/openwepp-vegetation/**`, yet the retained Child-3
delta `a7d692da4..dfc7cf971` changes twelve paths in that crate, including its
manifest and model registry. The owned-file manifest still reports
`solver.rs=2802`, LSE `transaction.rs=1674`, covered tests `=677`, and only two
2,000+ WARN files at
`docs/work-packages/20260814-snow-free-land-surface-energy-runtime-shadow-001/artifacts/owned-file-manifest.md:18-26`.
At the reviewed commit the corresponding material counts include
`solver.rs=2998`, LSE `transaction.rs=2557`, orchestrator shadow `mod.rs=2954`,
vegetation `transaction.rs=2082`, integration root `=2757`, and covered tests
`=1555`. The checkpoint reconciliation still says persistent V8/BGC is later
work at
`docs/work-packages/20260814-snow-free-land-surface-energy-runtime-shadow-001/artifacts/checkpoint-diff-reconciliation.md:21-34`,
although this commit implements it.

No affected file crosses the 3,000-line hard stop, but the exact terminal write
set, all 2,000+ WARN files, decomposition rationale/follow-up, and retained
scope must be reconciled before package closure.

## Residual Risk and Missing Tests

- Static inspection found no runner/scheduler call site, activation/default
  change, public commit method, or production-state mutation. The endpoint is a
  library export only, and its candidates remain uncommitted.
- The positive public test proves one nitrogen call and immutable hydrology
  frame, but there is no public end-to-end negative matrix proving byte-identical
  rollback for vegetation, hydrology, LSE, soil thermal, BGC, and envelope after
  failures injected at potential solve, authorization, final solve, V8
  projection/persistence, and BGC construction.
- The two-rank public fixture does not independently reconstruct full C/N
  inventory closure or poison rank association; its nitrogen arbiter preserves
  request order and its LSE lineage hashes are fabricated.
- No terminal benchmark or exact-head campaign-strength correctness regression
  was present or run as part of this review. Package progress itself leaves
  review, benchmark, and dual verification open at `package.md:54-55`.

## Ran

- `cargo nextest run -p openwepp-vegetation -p openwepp-land-surface-energy --profile quick`
  at the detached exact commit: `283 passed`.
- `cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick`
  at the detached exact commit: `73 passed`.
- `cargo clippy -p openwepp-vegetation -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`:
  `PASS`.
- `cargo fmt --all -- --check`: `PASS`.
- `git show --check --oneline dfc7cf971284d772246f147382f4bb8a2292ee4c`:
  `PASS`.
- Static runner/activation/commit scans: no production consumer, activation, or
  commit surface found.

The passing focused tests and static isolation checks do not cover the
closure-blocking strict-state, rank-binding, solver-contract, and failure-
diagnostic defects above. Rust correctness approval is withheld: `HOLD`.
