# Review Agent A

Status: `GO / bounded E19 ordering remediation`

Evidence mode: `not-run`

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
