# Verification Agent A

Status: `PASS / terminal exact-byte verification`

Evidence mode: `Static + Ran`

## Scope And Exact Bytes

I independently reviewed the current dirty worktree based on commit
`cd51fef9583f77973a2f4898864b9fe12b42545a`. I wrote only this verifier report.
I did not alter implementation, package evidence owned by another role, model
authority, or the active kickoff prompt.

Static: the terminal implementation hashes independently recomputed during
this review are:

| Surface | SHA-256 |
|---|---|
| kernel resource transaction | `766a331f18aa83756ed42d2b960cc3324825da0abff41f6c096a152158db54cb` |
| vegetation error taxonomy | `00c611aa3a56d11165680ab2a715fc4f1c14904696204288b7f2110b2d666979` |
| vegetation public transaction | `ebc08804ff941ce629cfc689d9ab60f3e71119a23a60e490c39752b27adfa4b1` |
| vegetation energy proposal | `de5a973b71d49914d7cdcf12a4634ee0aba4644c2498c9a9bc6d773583e4bb9f` |
| BGC owner | `308c9f20c978deb009a1b160e0816afa81fc6e1a0012a9c33bfda6367e2ef6cc` |
| energy owner | `d2ed144b5b648b6ce3d56c3735709bcefdaa34fcd5388fc2618891f3afe3d04f` |
| default-off diagnostic | `b73b528310142fb29bd4f9083fe0aa75ff3d8f4cda1813783632a3c798e1bc51` |
| public implementation contract | `80cb4a3ec409b45e953f204ef4fec7829a75f78f2413c096baa31071a5f92bc3` |

These hashes match the terminal-diff reconciliation and the final independent
correctness/QA rereviews. No changed Rust file postdates the authoritative
short-TMPDIR heavy campaign.

## Original Review-B Findings

Static + Ran: all seven accepted findings are genuinely corrected.

- `B-CRITICAL-001`: `transaction::execute_candidate` runs the real water phase,
  one persistent mineral-N phase, and sealed vegetation-candidate construction.
  The default-off diagnostic consumes that public candidate and connects the
  retained water owner, independently constructed BGC owner, and independently
  constructed energy owner before one atomic publication. The 17-test public
  implementation target passes this path.
- `B-CRITICAL-002`: the radiation implementation uses the admitted two-stream
  matrix-exponential and analytic particular/resonance branches with distinct
  VIS/NIR and direct/diffuse identities. Source inspection found no fixed
  `kd=0.8`, 4,000-step RK4, or Beer-law substitute on the V7 path. Radiation
  oracle and authority tests pass.
- `B-CRITICAL-003`: the public physical path reaches the admitted Brent `ci`
  solve, multi-node energy equations, four-potential hydraulics, hydraulic
  vulnerability, authorization-cap complementarity, and typed numerical
  failures. Potential and fixed-cap passes are distinct, and the final pass
  rebuilds from beginning state under immutable authorization.
- `B-CRITICAL-004`: E16--E22 preserve six display/storage/transfer tissue C/N
  identities, signed maintenance reserve, phenology, retranslocation,
  allocation, turnover, mortality, leaf-C/SLA area ownership, and exact
  litter/CWD C/N/dry-material proposals. The public path constructs the ending
  persistent state rather than stopping at helper kernels.
- `B-CRITICAL-005`: typed transaction, owner, occupancy, soil-layer, amount
  basis, and `Ammonium`/`Nitrate` identity survives request, authorization,
  finalized use, owner debit, and receipt. The BGC owner calls the centralized
  deterministic proportional arbiter and debits each `(layer,species)` ending
  inventory only by finalized use.
- `B-HIGH-006`: water, component/stand energy, vegetation carbon, vegetation
  nitrogen, and dry material are reconstructed from explicit typed operands.
  BGC independently reconstructs mineral inventory and receiving material
  operands. No producer residual is accepted. Twenty-seven phase and malformed
  envelope tests compare the complete serialized four-owner beginning state;
  all reject without mutation.
- `B-HIGH-007`: the admitted stable biochemical temperature responses,
  finite/domain guards, solver convergence criteria, failure diagnostics, and
  error precedence are present and unchanged on the reviewed bytes. Strict
  four-crate Clippy passes with warnings denied.

`artifacts/review-finding-disposition.md` maps every original B finding and
every accepted remediation-review finding to its correction and final
GO/PASS. Historical FAIL/HOLD and pending rows remain as chronology and are
explicitly superseded; I found no currently undispositioned material finding.

## Public Transaction, Ownership, And Rollback

Static: the execution chain is:

```text
execute_candidate
  -> potential occupancy-column solve and typed water requests
  -> centralized water authorization
  -> authorization-capped occupancy-column re-solve
  -> finalized water use and water-owner candidate
  -> E16--E22 carbon, respiration, phenology, turnover, N demand/final use
  -> sealed vegetation candidate and material proposals

run_default_off_diagnostic_at_phase
  -> BGC constructs its own mineral/material candidate and receipts
  -> energy owner reconstructs component, occupancy, and stand balances
  -> UncommittedCoupledTransaction validates all four owners
  -> one non-fallible `*owned = ending` replacement
```

The sole mutation is below the explicit no-fallible-operation boundary.
Vegetation exposes no independent commit method. The diagnostic reuses the
water arbiter's candidate rather than constructing a second debit. Material
proposal/receipt identity and nitrogen protocol identity are cross-checked
between independent owners.

Ran: `cargo nextest run --test c3_vegetation_implementation_contract --profile
quick` passed 17/17, run ID
`e2417b69-ee62-42f1-b7a3-5591d85b4b81`. This includes the public sealed
candidate, default-off diagnostic, five-ledger poisons, scarce shared-resource
competition, selector exclusion, and complete rollback matrix.

The corrected scarce fixture executes two strata/occupancies sharing one tile
and soil layer. It proves exactly two positive partial water authorizations and
four positive partial layer/species mineral-N authorizations, positive bounded
final use, exact owner debit/lineage, and byte-identical `BeforeCommit`
rollback. This is the real public path, not a controlled solver callback.

## Restored Authority And No Proxy Physics

Static: every test function from commit
`06f7d8041f7d957a803a52db87fb5957461f84df` remains present in
`vegetation_boundary_authority_contract.rs`; the current file expands rather
than replaces the prior suite. The seven scaffold implementation tests live in
the separate implementation-contract target.

Ran: `cargo nextest run --test vegetation_boundary_authority_contract
--profile quick` passed 25/25, run ID
`ce075953-9297-4ad2-a949-7b4c05b41ca4`. Ran:
`cargo nextest run --test auth11_required_suite_obligation_guards_contract`
passed 3/3, run ID `032f17f8-bfef-43c1-8304-198339afb378`.

Static: source scans and public-path inspection found no VPD proxy water
demand, PAR proxy mineral-N demand, fixed-`kd` shortcut, 4,000-step RK4 route,
`min(request, authorization)` vegetation endpoint, fallback candidate after
solver failure, or producer-declared five-residual closure. Literal zero
residual values found under the vegetation candidate module occur only in a
`#[cfg(test)]` deliberately incomplete potential-lane poison fixture; they are
not a public candidate or closure input.

## Selector And Exclusion Boundary

Static: `git diff --name-only
cd51fef9583f77973a2f4898864b9fe12b42545a -- crates/openwepp-runner
crates/openwepp-hillslope-orchestrator/src/direct_runtime
crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` returned no path.
Recursive source checks in the passing implementation contract likewise find
no diagnostic selector, V7 candidate call, or V7 model selector in production
runner/direct-runtime trees. The default-off diagnostic is an explicit caller
only; legacy PMET/GSI-final-canopy behavior is unchanged.

The terminal artifacts consistently retain:

- `calibration_evidence_status=NOT_CALIBRATION_READY`;
- `identifiability_status=NOT_ASSESSED`;
- no runtime activation or real production-consumer cutover;
- no empirical calibration, validation, transferability, or parameter claim;
- no canopy snow, calm/nonneutral fallback, or soil biogeochemical
  transformation claim.

## Benchmark And Heavy Evidence

Static: the authoritative corrected benchmark matrix is
`artifacts/m6-benchmark-final-20260814-20260814004247/`. Its command metadata
names the exact command for each surface. All 30 warm/sample logs select one
test and pass; no zero-test run is counted. The five commands exercise strict
V7 configuration parse/hash, strict complete-state parse and identity,
two-rank radiation, the public sealed candidate with energy owner, and the real
scarce two-stratum water/N competition plus rollback. The worst
maximum-to-first-sample ratio is `1.0217391304347825`, within the frozen `2x`
budget.

The rejected initial matrix remains in
`artifacts/m6-benchmark-20260813234912/` and is explicitly excluded from
closure evidence. Its inaccurate/zero-filter and noncompetitive-surface issues
were not erased.

Static: the authoritative Critical campaign is
`artifacts/m6-heavy-short-final-20260814005156/`, using absolute short TMPDIR
`/tmp/owm6f-lQkG1z`. Raw commands, logs, exit codes, timings, environment, and
summary agree:

- workspace strict Clippy: PASS;
- full workspace nextest: 2,671/2,671 PASS, 34 slow, 33 skipped, run ID
  `471dafdc-4948-436f-8201-63fd4ad7326f`;
- workspace doctest invocation: PASS, zero doctests discovered;
- dependency policy: PASS with the retained nonfatal unmatched-license
  warning;
- formatting and diff hygiene: PASS.

The earlier 2,670-test campaign predates the added real-competition test. The
first current-byte heavy attempt in `artifacts/m6-heavy-final-20260814004723/`
retains the Unix-socket `SUN_LEN` failure caused by an overlong generated
TMPDIR. Neither historical record is substituted for the corrected 2,671-test
campaign.

## Fresh Verifier Commands

Ran on the reviewed bytes:

```text
cargo nextest run --test vegetation_boundary_authority_contract --profile quick
cargo nextest run --test c3_vegetation_implementation_contract --profile quick
cargo nextest run --test auth11_required_suite_obligation_guards_contract
cargo clippy -p openwepp-kernel-contract -p openwepp-vegetation \
  -p openwepp-biogeochemistry -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
bash tools/release/check_science_contract_admission.sh \
  --base-ref cd51fef9583f77973a2f4898864b9fe12b42545a --worktree
bash tools/release/check_authority_suite_antievasion.sh
bash tools/release/check_sc_unit_compliance.sh \
  --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
bash tools/release/check_sc_unit_compliance.sh \
  --path docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md
cargo fmt --all -- --check
git diff --check
```

All passed. Admission reported `A0_ADMITTED contracts=45
science_surfaces=21` with authority SHA-256
`cf46825756ad8d17ff03b34316379a430199444287cda9363f3590cbf508c68a`.

## Reviews And Line-Count Governance

Static: the final exact-byte Rust correctness rereview is `GO` for dual
terminal verification, and the final independent science/closure QA rereview
is `PASS`. Both explicitly supersede their benchmark-evidence HOLDs and retain
the historical findings. Their reviewed implementation hashes match those
recomputed above.

Ran: exact current line counts are migration 2,873; constitutive 2,790;
carbon/nitrogen 2,214; transaction 2,082; energy owner 1,232; implementation
contract 1,216; BGC 827; and diagnostic 503. No Rust file reaches the mandatory
3,000-line split threshold. The four 2,000-line WARN modules remain explicit,
accepted decomposition debt and are not hidden as PASS-sized modules.

## Disposition And Prompt Authorization

Disposition: **PASS**. I find no unresolved science, numerical, ownership,
closure, rollback, selector, evidence-legitimacy, benchmark, heavy-gate,
finding-disposition, or line-count blocker on these exact bytes.

The kickoff prompt remains active with SHA-256
`e532f3e5c16a5e40bb9e18b5e2d804b1ed6621ce5966fead77f0830536b8399f`;
no archived copy exists during this verification.

On the condition that independent terminal verifier B also returns PASS on
the same implementation bytes, I authorize **only** the byte-preserving path
move of that kickoff prompt from `prompts/active/` to `prompts/archived/`,
followed by truthful terminal lifecycle, prompt-index, gate-history, and
terminal-diff metadata updates. This authorization does not permit any source,
test, authority, fixture, benchmark, solver, selector, or scientific-content
change. Any such change invalidates this PASS and requires fresh terminal
verification.
