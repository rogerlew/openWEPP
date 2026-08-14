# Review Agent A

Status: `GO / Milestone 6 final Rust correctness review`

Evidence mode: `Static + Ran`

The original placeholder is superseded by the current bounded review below.

## 2026-08-13 Fresh E19 Rust Correctness Review

Evidence mode: `Static + Ran`

Initial disposition: `HOLD`. The first corrected allocator still rejected a
valid receipt when binary64 `internal_use + external_use` rounded one ULP above
`final_total_demand`. Neither canonical contract admits that aggregate
ordering guard; SC-VEGETATION-001@11 instead defines
`eta=min(1,Nused/Ndem_final)`.

The finding was accepted. The guard was removed and an exact adjacent-bit
regression now proves the canonical eta branch, `eta=1`, zero unsupported NSC,
and no alteration of either finalized-use operand.

Final disposition: `GO`. No remaining correctness finding. The review confirms:

- neither SC-VEGETATION-001@11 nor SC-BIOGEOCHEM-001 requires
  `Ndem_final<=Ndem_pot`;
- potential requests remain immutable and one global authorization occurs;
- `Fext` and every typed `F_N` remain within `F<=A<=D`;
- receipt-bound growth reconstructs final demand, debits internal use once,
  consumes external finalized use once, allocates six tissues with one eta,
  and retains unsupported carbon in NSC;
- no tolerance, clamp, request inflation, reauthorization, or second ordering
  guard remains;
- candidate work is clone-isolated and the public multi-owner candidate/commit
  remains fail-closed.

Ran on final reviewed bytes: vegetation 215/215, implementation contract
13/13, strict vegetation all-target Clippy, formatting, and diff hygiene all
passed. `carbon_nitrogen.rs` is 2,214 lines (WARN, below 3,000).

## 2026-08-13 V7 Increment 4A Rust Correctness Review

Evidence mode: `Static` (fresh exact-worktree review; no test command run).

Disposition: `HOLD`.

Reviewed source SHA-256 values: `vegetation_candidate.rs`
`dbb315a420800f7e63e1ef147af173315400b3fe52102f83a00ea70c0d08ca50`;
`vegetation_ledger.rs`
`682edc348b15b0f20f3e6dafec0e6c9317b4c0f1a46f3ad289c3862bfc49766e`;
`persistent_phase.rs`
`1569dd7862441eb54e62a6179187b78f0a70a8a6dce51616cad8b553ec177191`;
`transaction.rs`
`fad8e86efc45168550416f0d48f41a1a60bd8083488bfb9bef9c96fec6ebb718`;
`lib.rs`
`1cdbf8de57972764cc13983b3dd8a4ecb57f382db8e1139d4d3207861189f436`.

### Findings

#### HIGH — V7-4A-A-001: ending `XS_C` cancels out of the carbon audit

`construct_ledgers` infers reserve-funded maintenance as
`before.xs_c + reserve_recovery - after.xs_c`, while its ending inventory also
contains the same `after.xs_c`. In the closure equation, the inferred-
maintenance `+after.xs_c` cancels the inventory `-after.xs_c`. Poisoning the
ending reserve therefore changes inferred respiration by the opposite amount
and leaves the residual unchanged. This is a producer-consistent tautology,
not independent E17 reconstruction.

Paths: `crates/openwepp-vegetation/src/vegetation_candidate.rs:351-363,
395-404`; `crates/openwepp-vegetation/src/vegetation_ledger.rs:90-96`;
authority: `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:535-573,977`.

Required correction: supply or independently reconstruct the admitted total
maintenance debit and audit `XS_C'`; do not infer respiration from the ending
field that the ledger is meant to validate.

#### HIGH — V7-4A-A-002: a noncanonical nonnegative guard is applied to signed `XS_C`

Both `total_carbon` values include finite signed `xs_c`, but
`validate_nonnegative` requires the combined beginning and ending scalars to be
nonnegative. Canonical state explicitly permits finite signed `XS_C` and does
not require `sum(nonnegative pools) + XS_C >= 0`. A state accepted by
`CoupledOwnedState::validate` can thus fail candidate construction solely
because maintenance debt exceeds positive inventory.

Paths: `crates/openwepp-vegetation/src/vegetation_candidate.rs:395-404` and
`crates/openwepp-vegetation/src/vegetation_ledger.rs:74-89`; authority:
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:543-545,
1600-1608`.

Required correction: validate nonnegative physical stocks/fluxes separately
and retain `XS_C` as a finite signed operand.

#### HIGH — V7-4A-A-003: the nitrogen phase is not digest-bound to the exact water phase

`UncommittedNitrogenPhase` retains only `transaction_id`. Candidate construction
checks that ID and the separately supplied water phase's beginning digest, but
cannot prove the nitrogen result came from that exact water phase. Two water
executions from the same beginning state share transaction ID and beginning
digest; a nitrogen phase from A can be paired with water phase B. Shared C/N
state and `t10_k` then come from A while ending occupancy warm starts come from
B. The current caller pairs correctly, but the constructor seam does not
enforce its claimed immutable-snapshot lineage.

Paths: `crates/openwepp-vegetation/src/persistent_phase.rs:38-46,107-159` and
`crates/openwepp-vegetation/src/vegetation_candidate.rs:103-128`.

Required correction: bind the nitrogen result to a digest/token covering the
exact capped water phase and require it at candidate construction.

#### HIGH — V7-4A-A-004: whole-ledger stratum and digest identity is incomplete

`validate_vegetation_ledgers` checks equal lengths and row-local C/N/DM identity
only. It does not require unique stratum IDs, one common transaction/beginning/
ending digest across all rows, the exact configured stratum set, or global
proposal-ID uniqueness. `validate_sealed` also does not bind ledger identities
back to candidate identity. Duplicate copies of one closed stratum with another
omitted, mixed same-row transaction/digest identities, and cross-stratum reused
proposal IDs can therefore pass this purported independent whole-candidate
gate. The constructor currently emits rows by configuration, but the validator
does not independently prove it.

Paths: `crates/openwepp-vegetation/src/vegetation_candidate.rs:82-100,313-392`
and `crates/openwepp-vegetation/src/vegetation_ledger.rs:58-108`.

Required correction: validate against the exact candidate identity and
configured stratum set; reject duplicate strata and globally duplicate
proposal identities.

#### HIGH — V7-4A-A-005: whole C/N closure uses a looser tolerance

`require_closed` uses `1e-12 + 64*epsilon*operand_scale`. The imported
`SC-BIOGEOCHEM-001` whole-owner conservation rule is
`1e-14 kg m^-2 + 64*epsilon*operand_sum`. V7's `1e-12` applies only to its
per-tissue storage/transfer relabeling amendment and does not supersede this
whole-ledger rule, so current acceptance can admit noncanonical residuals.

Path: `crates/openwepp-vegetation/src/vegetation_ledger.rs:156-182`;
authority: `docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md:183-187`
and `SC-VEGETATION-001.md:1534-1544`.

#### MEDIUM — V7-4A-A-006: contract failures collapse into generic taxonomy

Candidate identity, capped-occupancy, unresolved-transfer, and ledger failures
use generic `Receipt`, `Domain`, or `Closure` variants. Canonical guards
distinguish `VEG-E-093`, `VEG-E-097`, and `VEG-E-100`; current tests generally
assert only `is_err`, so callers cannot classify the contract failure.

Paths: `crates/openwepp-vegetation/src/vegetation_candidate.rs:179-195,
241-264`; `crates/openwepp-vegetation/src/vegetation_ledger.rs:119-163`;
`crates/openwepp-vegetation/src/error.rs:16-48`.

#### MEDIUM — V7-4A-A-007: exact derived-area science is triplicated

The exact-order LAI/SAI/RAI equations are hand-coded in candidate construction,
state validation, and migration. They currently match, and validation catches
candidate drift, but three production copies invite migration/runtime
divergence. Centralize one exact-order derivation helper.

Paths: `crates/openwepp-vegetation/src/vegetation_candidate.rs:204-221`,
`crates/openwepp-vegetation/src/transaction.rs:2002-2017`, and
`crates/openwepp-vegetation/src/migration.rs:1297-1312`.

### Residual risk and missing tests

- Add an ending-`XS_C` poison and a valid finite signed-`XS_C` fixture whose
  signed aggregate is negative.
- Mix two same-beginning water executions and prove nitrogen from one cannot be
  combined with the other's capped state.
- Add multi-stratum missing/duplicate/mixed-transaction/mixed-digest ledger
  poisons and cross-stratum duplicate proposal-ID poison.
- Add residuals between the canonical and current thresholds and exact
  contract error-code assertions.
- No focused tests were rerun; recorded passing tests do not exercise these
  poisons.

Static no-blocker checks: ending occupancy lanes are taken from
`water_phase.final_columns()`, require `CoupledSolvePass::Capped`, and reject
missing, duplicate, wrong-tile, or stale prior lineage
(`vegetation_candidate.rs:224-267`). Area caches are recomputed from ending
displayed leaf C before canonical digest validation (`:129-147,162-221`).
Material proposals are deterministically sorted by typed stratum, donor,
receiver, and source sequence and receive increasing positive IDs (`:269-305`).
Beginning state is immutably borrowed and candidate work is cloned. The sealed
candidate is crate-private, has no commit method, and the public execution and
commit APIs remain non-mutating/fail-closed (`lib.rs:20-25`;
`transaction.rs:700-780,2025-2049`). BGC receiving state, independent energy
ownership, public multi-owner candidate, and atomic commit are truthfully
pending, not Increment 4A claims.

Line-count governance: `vegetation_candidate.rs` 518,
`vegetation_ledger.rs` 258, `persistent_phase.rs` 498, `transaction.rs` 2,049,
and `lib.rs` 32. `transaction.rs` is WARN only; the package artifact records a
decomposition rationale and terminal test-module split intent. No reviewed file
reaches 3,000 lines.

Approval statement: **HOLD**. The sealed/no-commit claim boundary is truthful,
but Increment 4A is not correctness-acceptable until signed-reserve closure,
exact phase/whole-ledger identity, and canonical tolerance are corrected and
independently poisoned. BGC/energy candidates and atomic commit remain later
scope.

## 2026-08-13 V7 Increment 4A Final Remediation Rereview

Evidence mode: `Static + Ran`.

Disposition: `HOLD`.

Reviewed exact SHA-256 values: `vegetation_candidate.rs`
`3a75f82039818ae01073cb8b770e3d8853936ab39ce2cbd944b8d6de4fd3751c`;
`vegetation_ledger.rs`
`45b5d66ca46feab1a587e05e75d8d82d3495eaafeaa2e49c3256814ac6816afe`;
`persistent_phase.rs`
`b9be5e28c81a971cd937b0657b2b3c2520534152d7d33b136bb1327afbcd16c1`;
`transaction.rs`
`5a8556ac0d53cef8ca252611e78c1e48499bc8e2266145ae9179e9a28249a6ef`;
`error.rs`
`77d6ee08b8656df447ac5ff0858550ae90987ef910865569cbb4835c689d4827`;
`migration.rs`
`ab62ef22fe438547173462c863ded3d0b88cfda3810bbcbe525aa6d15ad23d45`;
and `lib.rs`
`1cdbf8de57972764cc13983b3dd8a4ecb57f382db8e1139d4d3207861189f436`.

### Remaining findings

#### MEDIUM — V7-4A-A-006 remains: the new taxonomy does not implement canonical guard identities

Remediation adds `VegetationCandidate` and `VegetationLedger`, but their
serialized codes are newly invented `VEG-E-CANDIDATE-001/002`. The binding
contract still identifies capped-candidate failure as `VEG-E-093`, V7
allocation/closure failure as `VEG-E-097`, and V7 rollback/precommit failure as
`VEG-E-100`. Moreover, nonnegative and transfer-identity failures inside the
ledger validator still return generic `VEG-E-DOM-001`, while closure returns
generic `VEG-E-CLOSURE-001`. The correction therefore groups some failures by
module but does not make the contract taxonomy reachable or consistent.

Paths: `crates/openwepp-vegetation/src/error.rs:16-53`,
`crates/openwepp-vegetation/src/vegetation_candidate.rs:82-174`, and
`crates/openwepp-vegetation/src/vegetation_ledger.rs:61-245`; authority:
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:1564-1577,
2185-2188`.

Required correction: represent the binding guard identities in the typed
error enum, map each candidate/ledger failure to its canonical code, and assert
the exact variant and rendered code in negative tests.

#### MEDIUM — V7-4A-A-008: required identity and numerical boundary poisons remain missing

Static guards now compare the complete source water phase, reject duplicate
strata/mixed whole-state identity, track proposal IDs globally, and use the
canonical tolerance. Tests cover signed `XS_C`, ending-`XS_C` corruption,
duplicate strata, mixed ending digest, wrong dry material, and deterministic
repeat construction from a real candidate. They do not exercise:

- a nitrogen phase paired with a different same-beginning/same-transaction
  water phase;
- duplicate proposal IDs across two distinct stratum ledgers; or
- residuals immediately below, at, and above
  `1e-14 + 64*epsilon*operand_scale`.

The new `assert_eq!(repeated_candidate, candidate)` proves deterministic
construction but is not a negative source-phase identity poison. These are
load-bearing transaction and conservation acceptance branches, so static code
inspection plus unrelated `is_err` assertions is incomplete validation
evidence for the requested real-candidate poison gate.

Paths: `crates/openwepp-vegetation/src/persistent_phase.rs:39-77,298-305`,
`crates/openwepp-vegetation/src/vegetation_candidate.rs:113-133`,
`crates/openwepp-vegetation/src/vegetation_ledger.rs:87-159,179-245`, and
`crates/openwepp-vegetation/src/transaction.rs:1897-2036`.

### Prior-finding disposition

- `V7-4A-A-001` corrected: the ledger consumes direct final maintenance from
  the admitted E17 calculation and carries beginning/ending `XS_C` as separate
  signed operands. Ending-`XS_C` corruption now changes closure and the real
  candidate-derived poison rejects
  (`persistent_phase.rs:154-200,219-230,274-289`;
  `vegetation_candidate.rs:374-386`; `vegetation_ledger.rs:121-134`;
  `transaction.rs:1993-2002`).
- `V7-4A-A-002` corrected: only physical inventory is nonnegative; both reserve
  values require finite signed domains. A focused fixture passes even when
  physical inventory plus reserve is negative
  (`vegetation_candidate.rs:418-426`; `vegetation_ledger.rs:105-134,365-374`).
- `V7-4A-A-003` statically corrected: the nitrogen phase owns an exact clone of
  its source water phase and construction requires structural equality with
  the supplied phase (`persistent_phase.rs:39-77,298-305`;
  `vegetation_candidate.rs:121-133`). Negative evidence remains under
  `V7-4A-A-008`.
- `V7-4A-A-004` statically corrected: validation requires exact configured
  strata, common candidate transaction/beginning/ending identities, unique
  strata, row-local cross-ledger equality, global proposal IDs, and elemental
  proposal aggregates (`vegetation_ledger.rs:61-159,179-224`). The duplicate-
  proposal poison remains under `V7-4A-A-008`.
- `V7-4A-A-005` corrected: whole carbon/nitrogen closure now uses
  `1e-14 + 64*epsilon*operand_scale` (`vegetation_ledger.rs:236-264`). Exact
  threshold evidence remains under `V7-4A-A-008`.
- `V7-4A-A-006` remains open as described above.
- `V7-4A-A-007` corrected: candidate update, accepted-state validation, and
  migration all call `transaction::displayed_leaf_derived_areas`
  (`vegetation_candidate.rs:231-242`; `transaction.rs:2074-2109`;
  `migration.rs:1289-1320`).

### Ran and residual risk

- Ran: `cargo nextest run -p openwepp-vegetation --profile quick` — `221/221`
  passed, including exact structural equality of two sealed candidates built
  from the same real two-ULP fixture.
- Ran: `cargo clippy -p openwepp-vegetation --all-targets -- -D warnings` —
  passed.
- No broad/heavy workspace gate was run.
- Current counts: `vegetation_candidate.rs` 540,
  `vegetation_ledger.rs` 408, `persistent_phase.rs` 508,
  `transaction.rs` 2,136, `error.rs` 53, `migration.rs` 2,873, and `lib.rs` 32.
  `transaction.rs` and `migration.rs` are documented WARNs with split intent;
  no reviewed file reaches 3,000 lines.
- Beginning state remains immutable, final occupancy state remains capped-only,
  material ordering remains deterministic, the sealed type remains private,
  and public execution/commit remain non-mutating and fail closed. BGC, energy,
  multi-owner publication, and atomic commit remain truthfully out of scope.

Approval statement: **HOLD**. The numerical/state construction defects from the
initial review are corrected, but canonical typed-error identity and the named
negative identity/tolerance poisons must close before Increment 4A receives GO.

## 2026-08-13 V7 Increment 4A Exact-Byte Terminal Rereview

Evidence mode: `Static + Ran`.

Disposition: `GO`.

Reviewed exact SHA-256 values: `vegetation_candidate.rs`
`daa882ccddfe071368279fd78cb83b1c913d806860dd4e0921b05f082f951a41`;
`vegetation_ledger.rs`
`336fcd53bdf726fedc4b5f12271d2e7cf5c87cca27af222f344a00b34098cd2e`;
`persistent_phase.rs`
`b9be5e28c81a971cd937b0657b2b3c2520534152d7d33b136bb1327afbcd16c1`;
`transaction.rs`
`f6c5035796057af12f5a1e303fc1d87de6a127055c910ae0c0271f9eefa0a24f`;
`error.rs`
`773f6d4c40f2b375efcc299d9a9c4e61c65d129a38b62e7c7f62ecefca0c5a51`;
`migration.rs`
`ab62ef22fe438547173462c863ded3d0b88cfda3810bbcbe525aa6d15ad23d45`;
and `lib.rs`
`1cdbf8de57972764cc13983b3dd8a4ecb57f382db8e1139d4d3207861189f436`.

### Findings

No remaining material correctness finding. The two earlier `HOLD` sections are
preserved as historical review evidence; this exact-byte terminal rereview
supersedes their dispositions for Increment 4A.

### Prior-finding disposition

- `V7-4A-A-001` closed: accepted final maintenance respiration is calculated
  directly from the accepted final carbon operands and retained in
  `StratumPreallocation`; the carbon ledger independently carries beginning
  physical C, beginning signed `XS_C`, direct GPP, direct maintenance, growth
  respiration, outgoing material C, ending physical C, and ending signed
  `XS_C`. Ending-reserve corruption therefore cannot algebraically cancel
  (`persistent_phase.rs:154-200,219-230,274-289`;
  `vegetation_candidate.rs:364-381`; `vegetation_ledger.rs:105-134`).
- `V7-4A-A-002` closed: physical vegetation inventory retains finite,
  nonnegative validation while beginning and ending `XS_C` are independently
  finite and signed. The focused negative-total fixture passes
  (`vegetation_ledger.rs:105-125,378-387`).
- `V7-4A-A-003` closed: `UncommittedNitrogenPhase` owns an exact clone of the
  water phase used to prepare it, and vegetation candidate construction
  requires structural equality with the supplied phase. A real, complete
  second water phase from the same beginning state and transaction but changed
  forcing is rejected with `V7CandidateRollback`
  (`persistent_phase.rs:39-77,298-305`;
  `vegetation_candidate.rs:113-133`; `transaction.rs:1913-1933`).
- `V7-4A-A-004` closed: validation binds every row to the configured stratum
  set, exact transaction, exact beginning and ending state digests, cross-ledger
  row identity, and a whole-candidate proposal-ID set. It also reconstructs the
  proposal C/N aggregates. Duplicate proposal identity across two distinct
  strata is rejected with the expected typed error
  (`vegetation_ledger.rs:61-159,179-224,422-442`).
- `V7-4A-A-005` closed: C/N closure uses exactly
  `1e-14 + 64 * f64::EPSILON * operand_scale`. The new `5e-13` carbon residual
  poison distinguishes the canonical threshold from the former `1e-12`
  envelope and returns `V7Closure`
  (`vegetation_ledger.rs:236-264,365-375`).
- `V7-4A-A-006` closed: capped-candidate rollback is typed/rendered as
  `VEG-E-093`, V7 allocation/ledger/closure rejection as `VEG-E-097`, and V7
  candidate transaction/source/precommit rollback as `VEG-E-100`. Candidate
  and ledger branches now return those typed variants rather than invented or
  generic codes (`error.rs:47-54`; `vegetation_candidate.rs:82-354`;
  `vegetation_ledger.rs:61-245`).
- `V7-4A-A-007` closed: candidate area update, accepted-state validation, and
  V3 migration validation all call the single
  `transaction::displayed_leaf_derived_areas` implementation
  (`vegetation_candidate.rs:228-242`; `transaction.rs:2095-2121`;
  `migration.rs:1302-1320`). No substantial mirrored derived-area algorithm
  remains in production.
- `V7-4A-A-008` closed: the exact-source-water, cross-stratum duplicate-ID, and
  old-loose-envelope rejection poisons now exercise the load-bearing branches.
  The real two-ULP candidate is also constructed twice with structural equality
  and supplies candidate-derived ending-`XS_C`, forged ending-digest, and
  carbon-as-dry-matter poisons (`transaction.rs:1897-2057`;
  `vegetation_ledger.rs:365-375,422-442`).

### Residual risk and missing tests

- Ran: `cargo nextest run -p openwepp-vegetation --profile quick` — `223/223`
  passed on the hashes above.
- Ran: `cargo clippy -p openwepp-vegetation --all-targets -- -D warnings` —
  passed on the hashes above.
- No broad/heavy workspace, campaign, or legacy-comparator gate was run. The
  discriminating tolerance poison covers the prior loose envelope, although a
  separate just-below/at/just-above threshold triplet and explicit `Display`
  assertions for all three canonical codes would provide additional low-risk
  boundary evidence.
- Current counts are `vegetation_candidate.rs` 535,
  `vegetation_ledger.rs` 443, `persistent_phase.rs` 508, `transaction.rs`
  2,157, `error.rs` 57, `migration.rs` 2,873, and `lib.rs` 32.
  `transaction.rs` and `migration.rs` remain documented WARNs with split
  intent; no reviewed file reaches 3,000 lines.
- The candidate remains crate-private and has no commit method. Public
  multi-owner publication, BGC receiving ownership, independent energy
  ownership, and atomic commit remain explicitly out of Increment 4A and
  fail-closed/non-mutating.

Approval statement: **GO** for the bounded Increment 4A sealed vegetation-owner
candidate claim. No correctness blocker remains in the reviewed exact bytes.
## Increment 4B / Milestone 5 Final Rereview

Evidence: `Static + Ran`

The first review returned HOLD for the admitted empty-stand branch, generic
owner-envelope taxonomy, and a shadowed owner-validation failure injection. A
subsequent static pass also required an atomic `SC-BIOGEOCHEM-001` admission
binding. All findings were accepted and corrected.

Final exact-byte disposition: **GO**. The empty stand executes through the real
four-owner path with zero demand; `VEGTXN-E-007` is typed; malformed water, N,
energy, transaction, beginning-state, and material-receipt candidates reach the
actual envelope validator; the owner-validation failure is no longer shadowed;
prior energy state is revalidated; and BGC has separate process and transaction
contract bindings. No correctness blocker remains.

Ran independently on final bytes: `git diff --check` PASS. Parent focused gate
evidence is recorded in `gate-results.md`.

## 2026-08-14 Milestone 6 Final Rust Correctness Review

Evidence mode: `Static + Ran` against the exact current worktree at base HEAD
`cd51fef9583f77973a2f4898864b9fe12b42545a`.

Disposition: `HOLD`.

### Findings

#### HIGH — V7-M6-A-001: water proportional arbitration duplicates and diverges from the centralized algorithm

The default-off water owner reimplements proportional competition with an
insertion-order `+=` layer total followed by `request.amount *
(supply/total).min(1)`. The dependency-neutral allocator used by the BGC owner
validates supply at the allocation seam, sorts competitors, performs
compensated summation, and selects the full-supply branch before proportional
division. The implementations therefore differ in summation order, rounding,
supply-domain precedence, and arithmetic branch shape. Shared-layer water
authorization can silently differ at binary64 boundaries from the supposedly
central transaction rule, and no order-reversal or compensated shared-water
vector covers the diagnostic implementation.

This also contradicts the recorded disposition that
`RB-HIGH-006 / A-MEDIUM-007` was corrected by one centralized,
owner-sorted compensated allocator. The duplication is substantial and has
already drifted, so it meets the repository's high-severity duplication rule
for possible silent science/contract divergence.

Paths:
`crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:159-237`,
`crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs:249-312`,
`crates/openwepp-biogeochemistry/src/lib.rs:226-236`, and
`docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/review-finding-disposition.md:36`.

Required correction: centralize the common validation, deterministic
competition grouping, compensated total, and proportional authorization
arithmetic behind a typed group-key adapter, or provide an explicit
contract-backed justification for retaining separate arithmetic plus
independent parity and order/rounding boundary vectors.

#### MEDIUM — V7-M6-A-002: canonical water/N transaction failure identities are unreachable

`SC-VEGETATIONTRANSACTION-001` binds stale/duplicate identity to
`VEGTXN-E-001`, nonfinite/negative/wrong basis to `VEGTXN-E-002`, and
authorization/final-use bound failure to `VEGTXN-E-003`. The water and
nitrogen boundaries have typed local enums, but their variants render
uncoded prose and are collapsed into `VegetationError::Receipt`
(`VEG-E-TRANSACTION-001`) or BGC `InvalidRequest`. The shared
`ResourceProtocolViolation` is also an uncoded enum. Current Rust contains no
reachable `VEGTXN-E-001` or `VEGTXN-E-003`; `VEGTXN-E-002` exists only for
energy operands, not water/N protocol failures. Downstream callers therefore
cannot classify the canonical transaction cause even though the operation
fails closed.

Paths:
`docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md:81-91`,
`crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs:204-215`,
`crates/openwepp-vegetation/src/occupancy_solver/resources.rs:23-59`,
`crates/openwepp-vegetation/src/nitrogen_protocol.rs:21-55`,
`crates/openwepp-vegetation/src/occupancy_solver/request_pass.rs:240-242`,
`crates/openwepp-vegetation/src/occupancy_solver/capped_pass.rs:624-625`,
`crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:163-173`,
and `crates/openwepp-biogeochemistry/src/lib.rs:226-236`.

Required correction: preserve a typed canonical transaction error through the
resource boundary and diagnostic envelope, with exact variant/rendered-code
assertions for identity, basis/domain, `A>D`, and `F>A` poisons.

### No-blocker checks

- E01--E22 remains connected through `execute_candidate`: the potential and
  fixed-cap passes precede one E19 arbitration, sealed ending-state/ledger
  construction, BGC and independent energy candidates, exact cross-owner
  validation, and one non-fallible whole-state replacement.
- Candidate isolation remains structural. The public vegetation candidate has
  private fields and no commit method; the coupled transaction fields and
  validation/ending methods are private; the only reviewed owner-state write is
  `*owned = ending` after every fallible operation.
- Typed transaction, owner, occupancy, layer/species, basis, interval, model,
  configuration, beginning-state, and ending-state identities remain carried
  and checked. One-time `f_t` water conversion and tile-to-stand energy
  weighting remain correctly ordered. No new clamp, default, or unchecked
  transaction increment was found.
- Twenty-seven phase/owner/malformed-envelope failures compare all beginning
  owner bytes exactly. The valid empty stand takes the real zero-demand water,
  N, BGC, energy, and atomic-commit path; corrupt accepted energy history
  rejects without mutation.
- The Milestone 6 negative test scans runner and direct-runtime Rust sources.
  Static repository search confirms V7 model/candidate/diagnostic references
  occur only in the new vegetation crates and diagnostic modules; no runner or
  direct-runtime source is changed by the package diff.
- Independent equation evidence remains split across released V1/V3/V5/V7
  oracle fixtures, capped failure vectors, independent owner reconstructions,
  and alias poisons. The public success test itself primarily proves lineage
  and publication, rather than duplicating producer arithmetic as its expected
  result.

### Ran evidence

- `cargo nextest run --test c3_vegetation_implementation_contract --profile quick`
  — `16/16` passed, including the production-nonactivation test, real
  four-owner publication, complete rollback matrix, and empty stand.
- `cargo clippy -p openwepp-kernel-contract -p openwepp-vegetation
  -p openwepp-biogeochemistry -p openwepp-hillslope-orchestrator
  --all-targets -- -D warnings` — passed.
- The package records a final production-equivalent workspace campaign of
  `2,664/2,664`, doctest, deny, exact-head workspace Clippy, formatting, and
  diff checks. This review did not rerun that delegated full campaign.

### Residual risk and line-count governance

- Benchmarks, final A1/A3/Critical closure, dual terminal verification, and
  prompt archival remain post-review Milestone 6/terminal gates; passing
  focused behavior does not replace them.
- Exact relevant counts are: `migration.rs` 2,873,
  `occupancy_solver/constitutive.rs` 2,790, `carbon_nitrogen.rs` 2,214,
  `transaction.rs` 2,082, `column.rs` 1,669, `water_phase.rs` 1,142,
  `vegetation_energy_owner.rs` 1,232, `vegetation_diagnostic.rs` 503, and BGC
  `lib.rs` 699. No reviewed Rust file reaches the mandatory 3,000-line
  blocker. The documented transaction/carbon test extraction and the two
  near-threshold WARN modules remain terminal decomposition debt.
- The selector proof is source-level and exact for the current tree; terminal
  verification should retain exact diff/route evidence so a future alias or
  re-export cannot weaken that boundary.

Approval statement: **HOLD**. The numerical kernels, owner isolation, atomic
commit, rollback, empty-stand behavior, and selector nonactivation have no new
material defect, but duplicated/drifted water arbitration and unreachable
canonical water/N transaction errors prevent Milestone 6 science-
implementation closure.

## 2026-08-14 Milestone 6 Remediation Rereview

Evidence mode: `Static + Ran` against the exact remediation worktree at base
HEAD `cd51fef9583f77973a2f4898864b9fe12b42545a`.

Disposition: `GO`. No material finding remains.

### Finding closure

- `V7-M6-A-001` is closed. Diagnostic same-layer water arbitration delegates
  validation, supply grouping, compensated totals, full-supply branching, and
  proportional arithmetic to the dependency-neutral
  `authorize_proportionally_by` implementation. The allocator preserves each
  complete occupancy key and canonically orders competitors by owner,
  transaction, full request key, and basis before summation. A four-request
  binary64 vector whose compensated total is reversal-sensitive proves that
  reversed caller order returns the same exact per-key authorization bits.
- `V7-M6-A-002` is closed. One shared
  `From<ResourceProtocolViolation> for VegetationError` maps identity,
  operand/basis, and authorization/final-use bounds to the canonical
  `VEGTXN-E-001`, `VEGTXN-E-002`, and `VEGTXN-E-003` variants. Diagnostic
  water, diagnostic nitrogen, and water-owner validation all use that shared
  conversion; the prior generic BGC/receipt collapse and duplicate match tables
  are absent. Focused water and N tests now construct failures through real
  duplicate/wrong-owner, nonfinite, `A>D`, and `F>A` boundaries and assert the
  exact returned category.

Relevant paths:
`crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs:249-333,497-550`,
`crates/openwepp-vegetation/src/error.rs:47-85`,
`crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:158-267`,
`crates/openwepp-vegetation/src/water_phase.rs:623-670`,
`crates/openwepp-vegetation/src/occupancy_solver/resources.rs:755-813`, and
`crates/openwepp-vegetation/src/nitrogen_protocol.rs:937-987`.

### Ran evidence

- `cargo nextest run -p openwepp-kernel-contract --profile quick` — `50/50`
  passed.
- `cargo nextest run -p openwepp-vegetation --profile quick` — `225/225`
  passed.
- `cargo nextest run --test c3_vegetation_implementation_contract --profile
  quick` — `16/16` passed.
- `cargo clippy -p openwepp-kernel-contract -p openwepp-vegetation
  -p openwepp-biogeochemistry -p openwepp-hillslope-orchestrator --all-targets
  -- -D warnings` — passed.
- The operator reports hillslope `494/494`, formatting, final A0, and authority
  guards passing; this reviewer did not rerun those gates on the final bytes.

Reviewed SHA-256: kernel resource transaction
`990e5f87986baa82e6cc6e4058c3e78949fc4837d7f642517906a7288f50e835`;
diagnostic
`721d0598ff917a51ba72fb0d0b4c4f00eba95791978831719195e7e320045bcc`;
vegetation error
`1d9b1661f90aa19c69f191194c2184d2a81a6d9bf6826e312784380572329b3e`;
water phase
`51c1cf076448a68726d0650be5e8f22c5442800e81245edc3eb56b322901feb5`;
water boundary
`9a9b598a427aa50747e8adbf635132f11567ab8d57915bc0b5d3c7fbcafad207`;
nitrogen boundary
`ad309c0835aba6b106951a5b7e83815d6ed5353cb0daf22c92f24b0cc73e2105`.

### Residual risk and approval

The preceding Milestone 6 no-blocker conclusions for E01--E22 connectivity,
candidate isolation, four-owner atomic replacement, rollback/empty-stand
behavior, units, and production-selector nonactivation remain unchanged.
Benchmark, A1/A3/Critical terminal evidence, dual verification, and the existing
WARN-level line-count/decomposition debt remain package closure gates or
follow-up work; they are not defects in these remediation bytes.

Approval statement: **GO** for Milestone 6 Rust correctness. Both prior
findings are closed, and no remaining numerical, science-contract, typed-error,
serialization, ownership, rollback, activation, or duplication blocker was
found in the reviewed scope.

## 2026-08-14 Final BGC Taxonomy Delta Rereview

Evidence mode: `Static + Ran` against the final exact bytes.

Disposition: `GO`. No material finding remains.

The final taxonomy precedence satisfies both admitted authorities without an
alias or duplicate full match:

- The kernel owns the exhaustive `ResourceProtocolViolation` classification
  into identity, operand, and bound categories.
- The outer coupled diagnostic maps those categories through the vegetation
  adapter to `VEGTXN-E-001`, `VEGTXN-E-002`, and `VEGTXN-E-003`, as required by
  `SC-VEGETATIONTRANSACTION-001`.
- The BGC owner retains its independent `SC-BIOGEOCHEM-001` boundary:
  mismatched/nonfinite request operands render `BGC-E-001`, while `A>D`, `F>A`,
  aggregate mineral overdraw, and material closure render `BGC-E-010`.
  Diagnostic nitrogen arbitration calls the kernel allocator directly, so no
  BGC error is relabeled as an outer coupled-transaction error.

The focused BGC taxonomy test drives wrong basis, `A>D`, `F>A`, aggregate
inventory overdraw, and material closure through their real boundaries and
asserts the required rendered family. The wrong-species correspondence poison
remains a BGC `BGC-E-001` mismatch and preserves both mineral stores.

Relevant paths:
`crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs:205-238`,
`crates/openwepp-vegetation/src/error.rs:47-74`,
`crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:158-267`,
and `crates/openwepp-biogeochemistry/src/lib.rs:15-39,156-196,241-316,712-826`.

### Ran evidence

- `cargo nextest run -p openwepp-kernel-contract --profile quick` — `50/50`
  passed.
- `cargo nextest run -p openwepp-biogeochemistry --profile quick` — `6/6`
  passed.
- `cargo clippy -p openwepp-kernel-contract -p openwepp-vegetation
  -p openwepp-biogeochemistry -p openwepp-hillslope-orchestrator --all-targets
  -- -D warnings` — passed.
- The operator reports exact-byte vegetation `225/225`, implementation
  contract `16/16`, formatting, and diff hygiene passing; this reviewer did not
  rerun those unaffected suites after the BGC-only delta.

Reviewed SHA-256: kernel resource transaction
`766a331f18aa83756ed42d2b960cc3324825da0abff41f6c096a152158db54cb`;
vegetation error
`00c611aa3a56d11165680ab2a715fc4f1c14904696204288b7f2110b2d666979`;
BGC owner
`308c9f20c978deb009a1b160e0816afa81fc6e1a0012a9c33bfda6367e2ef6cc`;
diagnostic
`721d0598ff917a51ba72fb0d0b4c4f00eba95791978831719195e7e320045bcc`.

Approval statement: **GO**. The BGC-only delta changes no arithmetic or state,
preserves the correct owner-versus-envelope taxonomy boundary, centralizes the
shared category logic, and has no remaining Rust correctness blocker.

## 2026-08-14 Final Package-Closure Readiness Rereview

Evidence mode: `Static + Ran` against the current terminal-candidate bytes.

Disposition: **HOLD** for advancement to dual terminal verification. The Rust
science implementation remains GO; one required benchmark-evidence finding is
material and unresolved.

### Findings

#### HIGH — The benchmark matrix does not execute two frozen performance surfaces

The package freezes benchmarks for configuration parsing and active water/N
competition (`package.md:292-295`), but the accepted matrix labels tests that
do not execute those behaviors:

- The `strict V7 parse/hash` row invokes
  `v7_configuration_state_and_migration_inputs_have_no_default_path`. That test
  only reads four Rust source files and rejects strings naming `Default`
  implementations
  (`tests/integration/c3_vegetation_implementation_contract.rs:569-584`). It
  never calls `VegetationConfiguration::parse_strict`,
  `CoupledOwnedState::parse_strict`, or either canonical SHA-256 path.
- The `active water/N plus all-owner rollback` row invokes the real rollback
  test, but its accepted fixture has one stratum, one occupancy, one water
  request against `20.0 kg m-2`, and `1.0 kg N m-2` each of NH4 and NO3
  (`tests/integration/c3_vegetation_implementation_contract.rs:349-393`). A
  fresh debug execution of that exact success path returned only
  `0.13813893146449976 kg m-2` water use and
  `2.7555468386935401e-5 kg N m-2` total use. Water is fully supplied, while
  NH4 and NO3 are separate abundant supply keys; no shared-supply or cap-active
  competition branch executes. The same test legitimately benchmarks the
  27-point rollback matrix, but that cannot also establish the missing
  competition surface.

The accepted logs are real one-test release runs: all 30 warm/sample logs
select one named test and report one pass. The command provenance is
nevertheless incomplete. For `radiation_unit` and `upper_cap`,
`m6-benchmark-20260813234912/command-log.json` records the original
zero-selecting `--exact` command, not the corrected command that produced the
accepted logs. `performance-budget.md` and `gate-results.md` also say the raw
zero-filter failures are preserved, but the run directory contains no failed
zero-test log or timing record.

Required disposition: run release samples on actual parse/hash and
competition fixtures on the unchanged terminal candidate, then reconcile the
command metadata and preservation claims. Existing tests suitable for the
parse/hash surface are
`config::tests::identity_rebound_v7_configuration_parses_strictly` and
`transaction::milestone_one_tests::complete_two_tile_two_stratum_state_is_exact`;
select both so configuration and state parsing/digests are exercised. No
existing test drives cap-active water and N competition through
`run_default_off_diagnostic_at_phase`. The minimal real-path fixture must have
at least two occupancies sharing a water layer, water supply below their summed
positive demand, and NH4 and NO3 stores below the corresponding positive
multi-stratum requests. It must assert nonzero `F <= A < D`, the water
`CompetingDemand` facts, exact layer/species owner debits and receipt lineage,
and rollback on that same scarce snapshot.

### Original finding and implementation disposition

No current Rust/science blocker was found. All original Review-B findings are
closed on the exact implementation bytes:

- `B-CRITICAL-001` through `B-CRITICAL-004`: the public diagnostic reaches the
  sealed E01--E22 potential/capped, persistent, receiving-owner, and whole-
  envelope path. Exact two-stream radiation, gas exchange, energy, four-
  potential hydraulics, persistent six-tissue C/N state, phenology, turnover,
  and material transfer are connected; the former proxy equations and
  producer-authored zero residuals are absent.
- `B-CRITICAL-005`: typed `(layer, NH4|NO3)` identity survives request,
  authorization, finalized use, mineral debit, and BGC reconstruction.
- `B-HIGH-006`: water, energy, vegetation C/N/dry material, mineral N, and BGC
  receipts are independently reconstructed. Every fallible operation precedes
  the sole `*owned = ending` assignment, and the 27 failure/poison phases prove
  byte-identical rollback for vegetation, water, BGC, and energy.
- `B-HIGH-007`: admitted stable temperature responses, finite/domain guards,
  solver diagnostics, and typed error precedence replace the rejected
  scaffold behavior.

The two Milestone 6 review findings also remain closed. Water competition uses
the single kernel `authorize_proportionally_by` implementation with canonical
ordering and compensated totals; no second diagnostic allocation algorithm
exists. Kernel `ResourceProtocolCategory` owns the exhaustive identity,
operand, and bound classification. Vegetation maps it once to
`VEGTXN-E-001/002/003`, while BGC deliberately preserves its independent
`BGC-E-001/010` authority. The corresponding variants are reachable through
real water, nitrogen, BGC bound, inventory, and material-closure tests.

Candidate isolation, owner identity, unit/area/interval conversion order, and
selector nonactivation remain intact. `CoupledCandidate` has no commit method;
the diagnostic constructs and validates all four candidates before its one
non-fallible replacement. The protected diff contains no runner,
`direct_runtime`, or `runtime_inputs.rs` path, and static search finds no V7
diagnostic/candidate selector in those production trees.

### Heavy evidence, exact bytes, and line-count governance

The heavy campaign evidence is legitimate for the implementation bytes. The
raw nextest timing reports `2670 tests run: 2670 passed`, and the command log
records single-attempt passing workspace Clippy, full nextest, doctest, deny,
format, and diff-hygiene commands. No changed Rust file has an mtime after the
heavy campaign began. Current SHA-256 values for kernel arbitration,
vegetation error/transaction/energy proposal, BGC, energy owner, diagnostic,
and the public implementation contract exactly match the eight identities in
`terminal-diff-reconciliation.md`. A fresh implementation-contract run also
passed 16/16.

Exact line counts remain `migration.rs` 2,873,
`occupancy_solver/constitutive.rs` 2,790, `carbon_nitrogen.rs` 2,214,
`transaction.rs` 2,082, energy owner 1,232, diagnostic 484, BGC 827, and the
integration contract 1,106. No file reaches the mandatory 3,000-line blocker.
The line-count artifact records cohesive/versioned responsibility rationales
and follow-on extraction plans for all four WARN files, so this is visible
decomposition debt rather than an undispositioned duplication or closure
violation.

### Residual risk and next lifecycle gate

Dual verifier artifacts are intentionally still `queued / not-run`; they are
the next lifecycle gate after final reviewers freeze acceptable inputs, not a
current defect. They must remain pending while the benchmark finding changes
the terminal evidence inputs. Calibration, runtime activation, real production
consumer cutover, canopy snow, nonneutral/calm aerodynamics, and soil
transformations remain explicitly unclaimed.

Approval statement: **HOLD** solely for the benchmark-legitimacy correction
above. There is no unresolved numerical, science-contract, typed-error,
serialization, ownership, rollback, selector, heavy-gate, line-count, or
duplicated-production-logic finding in the current Rust implementation.

## 2026-08-14 Superseding Final Terminal-Readiness Rereview

Evidence mode: `Static + Ran` against the corrected terminal-candidate bytes.

Disposition: **GO** for dual terminal verification. The benchmark-legitimacy
HOLD immediately above is superseded; no material finding remains.

### Findings

No blocking, high, medium, or other material finding.

- The former benchmark finding is closed. The authoritative matrix at
  `artifacts/m6-benchmark-final-20260814-20260814004247/` records the exact
  command actually run for every surface. All 30 warm/sample logs select one
  intended test and pass; no zero-test run is accepted. The two parse rows call
  the real strict V7 configuration and complete-state parsers and validate
  their canonical configuration/state identities. The radiation and sealed
  public-candidate rows retain their genuine production tests.
- The competition row now executes
  `v7_real_diagnostic_activates_shared_water_and_species_n_competition` through
  `run_default_off_diagnostic_at_phase`. Its validated V7 fixture has two
  strata/occupancies on one tile and root layer, scarce water, and scarce NH4
  and NO3. The diagnostic reports exactly two positive partial water
  authorizations and four positive partial mineral-N authorizations. Finalized
  water and N remain positive and bounded; the exact water debit, vegetation/
  water/BGC transaction lineage, four-owner success, and same-snapshot
  `BeforeCommit` byte-identical rollback pass. The production protocol and BGC
  candidate validators independently retain per-request `F <= A < D`, exact
  layer/species correspondence, and species-specific mineral debits.
- The added diagnostic counters are read-only output diagnostics over already
  validated finite D/A pairs. They add no clamp, tolerance, default, second
  arbitration, owner mutation, or altered E01--E22 arithmetic. Water reason
  validation still binds every partial nonzero authorization on available
  shared storage to `CompetingDemand`; nitrogen retains exact owner/layer/
  species keys.
- The corrected matrix has five release commands covering the six frozen
  surfaces: strict configuration/state parse and hashes, two-rank overlapping
  radiation, the one-stratum interval candidate, scarce water/N competition,
  and rollback. Each command has one warm run and five samples. The largest
  maximum-to-first-clean ratio is `1.02174`, within the frozen `2x` budget.
- The finding-disposition table now marks every original B and remediation
  finding corrected with final GO/PASS. Historical checkpoint statements are
  retained as chronology and explicitly superseded by terminal V7 sections.
  The diagnostics, contract-implementation, and forcing/provider artifacts now
  record the completed public candidate, independent BGC/energy owners,
  four-owner commit, scarce competition, and 17-test implementation contract.

### Rust and science-contract correctness

All prior implementation conclusions remain valid on the corrected bytes.

- `B-CRITICAL-001` through `B-CRITICAL-004` remain closed: E01--E22 execute
  through exact radiation, gas exchange, multi-node energy, four-potential
  hydraulics, cap complementarity, persistent six-tissue C/N state,
  phenology/turnover/mortality, and material-transfer construction in the real
  sealed public path.
- `B-CRITICAL-005` remains closed: transaction, owner, occupancy, layer,
  NH4/NO3 species, and amount-basis identity survive D/A/F, owner debit, and
  receipt. The new scarce fixture exercises rather than aliases those keys.
- `B-HIGH-006` remains closed: independent water, energy, vegetation C/N/dry-
  material, mineral-N, and BGC receipt reconstruction precede the sole
  non-fallible `*owned = ending` assignment. The existing 27-point matrix and
  new scarce failure retain the entire serialized beginning owner state.
- `B-HIGH-007` remains closed: stable admitted numerical responses, finite and
  domain guards, solver diagnostics, typed failures, and error precedence are
  unchanged.
- `V7-M6-A-001` and `V7-M6-A-002` remain closed. Water arbitration still has
  one kernel implementation with canonical ordering and compensated grouped
  totals. Shared protocol classification remains centralized in the kernel;
  vegetation exposes `VEGTXN-E-001/002/003`, while BGC correctly retains
  `BGC-E-001/010`. No substantial duplicated Rust arbitration or taxonomy
  logic was reintroduced.

Candidate isolation, exact owner validation, one-time area/interval
conversions, and selector nonactivation are unchanged. No runner,
`direct_runtime`, `runtime_inputs.rs`, model definition, or canonical science
contract changed. The diagnostic remains explicit and default-off; legacy
PMET/GSI-final-canopy production behavior cannot select V7.

### Exact-head evidence and governance

The authoritative Critical campaign is
`artifacts/m6-heavy-short-final-20260814005156/`. Its exact commands, exit
codes, environment, raw logs, timings, and summary agree: workspace strict
Clippy passed; full nextest passed 2,671/2,671 with 34 slow and 33 skipped;
workspace doctest invocation, dependency policy, formatting, and diff hygiene
passed. The prior long-TMPDIR run truthfully retains its Unix-socket `SUN_LEN`
failure and is not counted as passing evidence. The short absolute TMPDIR
removes that environmental cause without changing code or test selection.

No changed Rust file postdates the authoritative campaign start. Current
terminal hashes match the reconciliation table, including diagnostic
`b73b528310142fb29bd4f9083fe0aa75ff3d8f4cda1813783632a3c798e1bc51`
and implementation contract
`80cb4a3ec409b45e953f204ef4fec7829a75f78f2413c096baa31071a5f92bc3`.
A fresh exact implementation run passed 17/17, and strict target Clippy passed.

Exact line counts are migration 2,873, constitutive 2,790, carbon/nitrogen
2,214, transaction 2,082, energy owner 1,232, implementation contract 1,216,
BGC 827, and diagnostic 503. No Rust file reaches the mandatory 3,000-line
threshold. The four WARN modules retain explicit decomposition rationale and
follow-on split intent; none is new duplication or silent science drift.

### Residual risk and next lifecycle gate

Dual terminal verifiers remain intentionally queued until these reviewed bytes
are accepted; their absence is the next lifecycle gate, not a defect. The four
WARN modules remain visible debt. Calibration remains
`NOT_CALIBRATION_READY`, identifiability remains `NOT_ASSESSED`, and runtime
activation, real-consumer cutover, empirical validation, transferability,
canopy snow, calm/nonneutral aerodynamics, and soil transformations remain
explicitly excluded.

Approval statement: **GO** for dual exact-byte terminal verification. No
unresolved numerical, science-contract, typed-error, serialization, ownership,
rollback, selector, benchmark, heavy-gate, line-count, evidence-legitimacy, or
duplicated-production-logic blocker remains in this review scope.
