# Verification Agent B

Status: `PASS / exact-worktree terminal verification`

Evidence mode: `Static + Ran`

Disposition: **PASS**. No material implementation, science-contract,
ownership, rollback, selector, evidence-legitimacy, or governance blocker was
found on the reviewed bytes.

## Reviewed Identity

Static: the review began from `main` at
`cd51fef9583f77973a2f4898864b9fe12b42545a` with the intended terminal
candidate present as an uncommitted worktree diff. Recomputed SHA-256 values
match `terminal-diff-reconciliation.md` exactly:

| Surface | SHA-256 |
|---|---|
| kernel resource transaction | `766a331f18aa83756ed42d2b960cc3324825da0abff41f6c096a152158db54cb` |
| vegetation error taxonomy | `00c611aa3a56d11165680ab2a715fc4f1c14904696204288b7f2110b2d666979` |
| vegetation public transaction | `ebc08804ff941ce629cfc689d9ab60f3e71119a23a60e490c39752b27adfa4b1` |
| vegetation energy proposal | `de5a973b71d49914d7cdcf12a4634ee0aba4644c2498c9a9bc6d773583e4bb9f` |
| BGC owner | `308c9f20c978deb009a1b160e0816afa81fc6e1a0012a9c33bfda6367e2ef6cc` |
| energy owner | `d2ed144b5b648b6ce3d56c3735709bcefdaa34fcd5388fc2618891f3afe3d04f` |
| default-off diagnostic | `b73b528310142fb29bd4f9083fe0aa75ff3d8f4cda1813783632a3c798e1bc51` |
| implementation contract | `80cb4a3ec409b45e953f204ef4fec7829a75f78f2413c096baa31071a5f92bc3` |

The active kickoff prompt is 5,100 bytes with SHA-256
`e532f3e5c16a5e40bb9e18b5e2d804b1ed6621ce5966fead77f0830536b8399f`.

## Review-B And Public-Path Verification

Static: every original Review-B finding is corrected on the current bytes.

- `B-CRITICAL-001`: `execute_candidate` executes the real potential water
  column, one water authorization, fixed-cap column re-solve, persistent
  nitrogen/growth phase, and sealed vegetation candidate. The default-off
  diagnostic consumes that public path and constructs all four owner
  candidates before commit.
- `B-CRITICAL-002`: radiation uses the admitted real 2x2 matrix exponential,
  analytic particular solution, resonance handling, directional/band
  identities, and ordered tile-column traversal. The former fixed-step RK4 and
  fixed-`kd` Beer shortcut are absent.
- `B-CRITICAL-003`: the public occupancy path reaches Brent `ci`, coupled
  leaf/wet/stem/canopy-air energy, four-potential hydraulics, active
  authorization caps, `beta_hyd` coupling, and typed numerical failures.
- `B-CRITICAL-004`: the persistent phase retains all six tissues with separate
  display/storage/transfer C/N, signed maintenance reserve, V7 phenology,
  retranslocation, allocation, turnover, mortality, derived LAI, and typed
  litter/CWD material proposals.
- `B-CRITICAL-005`: water keys retain occupancy and layer; mineral-N keys
  retain layer and `Ammonium`/`Nitrate`. Transaction, owner, key, basis,
  request, authorization, finalized use, debit, and receipt correspondence is
  validated without layer or species borrowing.
- `B-HIGH-006`: water, component/stand energy, vegetation carbon, vegetation
  nitrogen, and dry material are reconstructed from explicit operands outside
  their constitutive producers. BGC independently constructs mineral debits
  and material receipts. No producer-supplied residual authorizes commit.
- `B-HIGH-007`: stable admitted temperature responses, finite/domain guards,
  error precedence, cancellation-safe roots, numerical diagnostics, and typed
  convergence failures replace the scaffold guards.

The prior A0 suite from commit `06f7d8041f7d957a803a52db87fb5957461f84df`
was compared by test-function inventory. Every prior authority test remains in
`vegetation_boundary_authority_contract.rs`; the current target adds V2--V7
authority and oracle coverage rather than replacing the original suite.

Source scans found none of the rejected proxy expressions: no 4,000-step
radiation RK4, fixed `kd=0.8`, VPD-derived water proxy, PAR-derived N proxy,
`min(request, authorization)` final endpoint, or literal five-residual success
path remains in the public implementation. Literal zero branches that remain
are typed physical zero-area/empty-stand branches or test poisons and are
independently validated.

## Owner Closure, Competition, And Rollback

Static: `UncommittedCoupledTransaction::validate` binds one transaction,
beginning owner snapshots, exact vegetation/water protocol, exact nitrogen
protocol, BGC proposal/receipt correspondence, and energy proposal identity.
Every fallible construction and validation precedes the sole whole-owner
assignment:

```rust
*owned = ending;
```

The 27-point integration matrix serializes the complete
`DiagnosticOwnedState` and proves byte identity after validation, scientific,
owner-construction, owner-validation, malformed-envelope, and pre-commit
failures. There is no vegetation-only commit method on `CoupledCandidate`.

The corrected release benchmark's competition row invokes
`v7_real_diagnostic_activates_shared_water_and_species_n_competition` through
the real default-off diagnostic. It uses two strata/occupancies sharing one
tile and layer with scarce water, NH4, and NO3. It proves exactly two positive
partial water authorizations, four positive partial species-preserving N
authorizations, positive bounded finalized use, exact owner lineage/debits,
and byte-identical `BeforeCommit` rollback.

## Selector And Exclusion Verification

Static: the exact diff changes no path under `crates/openwepp-runner`, hillslope
`direct_runtime.rs`, or hillslope `runtime_inputs.rs`. Recursive search found
no `run_default_off_diagnostic`, `OPENWEPP_C3_WOODY_V7`, or vegetation
`execute_candidate` reference in those production selector surfaces. Legacy
PMET and GSI-final-canopy behavior therefore remains unchanged; V7 is reachable
only through the explicitly invoked diagnostic API.

The final artifacts consistently retain these exclusions:

- no runtime activation or real production-consumer cutover;
- no empirical calibration, validation, or transferability claim;
- `calibration_evidence_status=NOT_CALIBRATION_READY`;
- `identifiability_status=NOT_ASSESSED`;
- no canopy snow, calm/nonneutral fallback, or soil BGC transformations.

## Reviews, Evidence, And Governance

Static: the superseding final sections of `review_agent_a.md` and
`review_agent_b.md` are respectively **GO** and **PASS** against the exact
hashes above. `review-finding-disposition.md` marks every original B finding
and every accepted remediation finding corrected with final GO/PASS. Earlier
HOLD, FAIL, pending-review, and retry rows remain explicitly historical and
are not erased or relabeled.

The authoritative benchmark matrix is
`m6-benchmark-final-20260814-20260814004247`. Its five release commands each
select exactly one intended test for one warm run and five retained samples;
the zero-test guard passes. The worst maximum-to-first ratio is
`1.0217391304347825`, below the frozen `2x` budget. The earlier source-only,
abundant-competition, and zero-filter attempts remain rejected historical
evidence.

The authoritative heavy campaign is
`m6-heavy-short-final-20260814005156`, with absolute short TMPDIR
`/tmp/owm6f-lQkG1z`. Its raw timing stream and summary agree on workspace
strict Clippy PASS, full nextest 2,671/2,671 PASS with 34 slow and 33 skipped,
workspace doctest invocation PASS, dependency policy PASS, formatting PASS,
and diff hygiene PASS. The preceding current-byte long-TMPDIR Unix-socket
`SUN_LEN` failure is preserved and is not counted as a passing run.

Ran independently on the reviewed bytes:

- `cargo nextest run --test c3_vegetation_implementation_contract --profile quick`
  -- PASS, 17/17, run ID `f95d8801-3b5d-4897-a9b2-a6e39d609db0`.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`
  -- PASS, 25/25, run ID `aea03f38-2e2e-4359-b226-b464d3944d88`.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
  -- PASS, 3/3, run ID `288e921f-808e-43ce-b3be-e7fc8f143178`.
- `bash tools/release/check_science_contract_admission.sh --base-ref
  cd51fef9583f77973a2f4898864b9fe12b42545a --worktree` -- PASS,
  `A0_ADMITTED contracts=45 science_surfaces=21`, authority SHA-256
  `cf46825756ad8d17ff03b34316379a430199444287cda9363f3590cbf508c68a`.
- `bash tools/release/check_authority_suite_antievasion.sh` -- PASS.
- Both affected `check_sc_unit_compliance.sh --path ...` commands -- PASS.

Exact line counts are migration 2,873, constitutive 2,790, carbon/nitrogen
2,214, transaction 2,082, energy owner 1,232, implementation contract 1,216,
BGC 827, and diagnostic 503. No Rust file reaches the mandatory 3,000-line
threshold. The four WARN modules retain explicit accepted decomposition debt
without changing numerical order, public API, or canonical serialization.

## Terminal Authorization

**PASS.** Verifier B authorizes only the post-verification lifecycle changes
already frozen by the package:

1. move the kickoff prompt byte-for-byte from `prompts/active/` to
   `prompts/archived/`, preserving SHA-256
   `e532f3e5c16a5e40bb9e18b5e2d804b1ed6621ce5966fead77f0830536b8399f`;
2. update truthful terminal evidence metadata, prompt indexes, package status,
   and final disposition to record both verifier results and the path-only
   archive.

This authorization does not permit implementation, contract, model,
configuration, fixture, test, benchmark, selector, activation, cutover, or
scientific-claim changes after verification. Any such byte change invalidates
this PASS and requires repeat review and terminal verification.
