# Rust correctness remediation review A

Disposition: **NOT APPROVED — blocking correctness and science-contract findings remain.**

Evidence class: **Static** review of the exact current worktree, including the root, crate, test, science-contract, and work-package instruction chains; the implementation package and remediation records; `SC-VEGETATION-001`; `SC-BIOGEOCHEM-001`; the model-stack equation, solver, parameter, ownership, and vector authority artifacts; historical `review_agent_b.md`; production Rust; fixtures; and focused tests. **Ran** evidence is listed after the findings. No implementation file was edited.

## Findings

### A-CRITICAL-001 — The public commit seam still accepts forged candidates and arbitrary replacement state

Paths: `crates/openwepp-vegetation/src/transaction.rs:243-256`, `crates/openwepp-vegetation/src/transaction.rs:1554-1607`, `crates/openwepp-vegetation/src/transaction.rs:1610-1677`, `crates/openwepp-vegetation/src/lib.rs:18-21`.

`CoupledCandidate` and every one of its fields are public and are reexported. `validate_and_commit_with_failure` authenticates only the caller-provided `beginning_state_sha256` string, resource-vector shape/protocol, selected candidate-to-ledger totals, and self-closing ledgers before replacing `*beginning`. It does not prove that the candidate came from `execute_candidate`, validate `candidate.state`, bind the candidate's model/configuration/topology to the beginning state, require `diagnostics.transaction_id == beginning.last_transaction_id + 1`, require every stratum transaction ID to match, bind beginning ledger operands to the actual beginning state, or bind dry-material ending inventory to the candidate state.

Consequently, safe external Rust can construct empty request/receipt vectors, an arbitrary or empty `CoupledOwnedState`, and internally self-closing zero/rewritten ledgers, copy the opaque beginning digest string, and have commit recompute a digest and install that state. This defeats the single-mutator and owner-validation boundary and can silently delete or rewrite all vegetation inventory. Candidate construction is isolated, but isolation is not a security/correctness boundary while arbitrary candidates remain committable. Make candidate construction sealed/private or make commit independently authenticate the complete transition against the actual beginning state, configuration/model, transaction identity, owner candidates, and topology.

### A-CRITICAL-002 — Radiation conflates plant area with photosynthetic leaf area and ignores digest-bound optical parameters

Paths: `crates/openwepp-vegetation/src/transaction.rs:700-750`, `crates/openwepp-vegetation/src/radiation.rs:225-275`, `crates/openwepp-vegetation/src/transaction.rs:1094-1125`, `crates/openwepp-vegetation/src/transaction.rs:944-959`, `crates/openwepp-vegetation/src/config.rs:47-55`.

`radiation_by_stratum` passes `leaf_area + stem_area` to `two_stream` but supplies only leaf VIS/NIR reflectance and transmittance. `two_stream` then reports sunlit/shaded area from that total plant area. Those values are consumed as leaf-class LAI by energy, FvCB/GPP, and hydraulics. At the same time, `stem_rho_*`, `stem_tau_*`, and `clumping_index` are validated configuration fields but have no production consumer; dry-stem shortwave is hard-coded to zero while all absorbed shortwave is assigned to wet or dry leaf classes.

This violates E01/E02/E05 and the contract's exact consumed-field list: woody area can become photosynthetic leaf area, stem-optics/clumping poison changes cannot affect results, and a zero-leaf/nonzero-stem stand does not take the required zero-leaf-class branch. The error is systematic in every woody state with nonzero stem area and affects radiation closure ownership, leaf temperature, transpiration, GPP, and future LAI.

### A-CRITICAL-003 — Multi-stratum canopy water and energy ledgers silently double-count column inputs

Paths: `crates/openwepp-vegetation/src/transaction.rs:795-816`, `crates/openwepp-vegetation/src/transaction.rs:863-889`, `crates/openwepp-vegetation/src/transaction.rs:561-609`, `crates/openwepp-vegetation/src/transaction.rs:1437-1451`, `crates/openwepp-vegetation/src/transaction.rs:1475-1496`.

Every stratum receives the full stand `forcing.rain_kg_m2` in both preliminary and finalized interception, without topology-tile weighting or a top-to-bottom incident-water handoff. `build_ledgers` then defines precipitation as `rain * number_of_strata`, so the producer-generated water ledger closes around the duplicated source. Likewise, `EnergyAccumulator::add` reconstructs and sums a local incident-shortwave boundary for every stratum; vertically transmitted energy is therefore counted again as a new incident input at the next rank rather than the stand ledger using the top boundary once and terminal ground flux once.

For overlapping ranks, and also for disjoint fractional tiles, these ledgers can report zero residual while the stand has created extra precipitation and double-counted radiative throughput. This is exactly the producer-self-consistency failure prohibited by the package's anti-tautology acceptance rule and by `INV-VEGETATION-020`. The focused public transaction test uses one stratum and `rain_kg_m2 = 0`, so it cannot detect this defect.

### A-HIGH-004 — “Strict” parsing does not enforce digest or topology identity

Paths: `crates/openwepp-vegetation/src/config.rs:145-178`, `crates/openwepp-vegetation/src/config.rs:181-203`, `crates/openwepp-vegetation/src/config.rs:205-245`, `crates/openwepp-vegetation/src/transaction.rs:81-95`, `crates/openwepp-vegetation/src/transaction.rs:612-641`, `tests/fixtures/c3_woody_v1_diagnostic_configuration.json`, `tests/fixtures/c3_woody_v1_diagnostic_state.json`.

Configuration validation checks only that configuration/initial-state identities look like hexadecimal strings; execution never compares `configuration_sha256` to canonical configuration bytes, `initial_state_sha256` to the supplied initial state, or `state_sha256` to a recomputation. The accepted diagnostic fixtures demonstrate the gap with placeholder identities of 64 repeated `1` and `2` characters. State validation checks digest length only. Thus stale/tampered caller bytes can retain a convenient identity and pass the purported immutable snapshot guard.

Topology validation also checks only the aggregate tile sum and duplicate tile IDs. It does not require each tile fraction to be finite and strictly positive, reject empty IDs, reject two strata occupying the same tile/rank, enforce rank/height consistency, or require the configuration and state stratum sets to be identical. A negative tile fraction can cancel a greater-than-one fraction; a `NaN` aggregate supplied through the public Rust API bypasses the comparison. Domain enforcement also accepts `clumping_index > 1`, zero emissivity, and zero `drymatter_carbon_fraction`, contrary to the admitted `(0,1]` domains. These conditions are required to fail before Stage A, not later or incidentally.

### A-HIGH-005 — Conservation tolerances are looser than the admitted BGC contract and are not scale-aware

Paths: `crates/openwepp-vegetation/src/ledger.rs:95-173`, especially `:157-166`; authority: `docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md:183-187`.

All non-energy ledgers use a fixed `1e-10` tolerance. The authoritative C/N/material tolerance is `1e-14 kg m^-2 + 64*epsilon*operand_sum`. The implementation can therefore accept materially larger unexplained C/N/dry-material residuals, and it has no operand-scale term. Energy similarly uses `1e-6 * interval_s` without an identified authority operand scale. Because closure is the final atomic-commit gate, this is silent conservation-contract drift rather than a diagnostics-only difference.

### A-HIGH-006 — Numerical error taxonomy and convergence evidence do not match the model-definition contract

Paths: `crates/openwepp-vegetation/src/error.rs:3-24`, `crates/openwepp-vegetation/src/numerics.rs:147-255`, `crates/openwepp-vegetation/src/numerics.rs:265-292`, `crates/openwepp-vegetation/src/energy.rs:170-199`, `crates/openwepp-vegetation/src/transaction.rs:1208-1262`; authority: `docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/numerical-solver-and-convergence-contract.md:10-22`.

The shared Newton/LU helper always emits the `Hydraulic` variant and hence `VEG-E-NUM-004`, even when called by the energy solve, whose required code is `VEG-E-NUM-003`. The outer coupled limit also emits `VEG-E-NUM-004` instead of `VEG-E-NUM-005`, and radiation quadrature has no `VEG-E-NUM-006` variant. Trial residual errors are discarded wholesale during backtracking, so the eventual generic error can erase the original typed cause. The normalized norm uses the current residual itself as the relative scale, and the coupled equality accepts a fixed `1e-10` mismatch rather than the admitted hydraulic `1e-12 + 1e-9*scale` criterion. Required failure residual/iteration/pivot diagnostics are unavailable on `Err`; `pivot_failure` is only ever constructed as `false`.

Downstream callers therefore cannot distinguish the failed component or reliably audit whether the admitted stopping rule was met. This is contract-visible error/diagnostic drift at the runtime seam.

### A-MEDIUM-007 — Proportional resource arbitration is duplicated and has already drifted

Paths: `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:95-129`, `crates/openwepp-biogeochemistry/src/lib.rs:87-131`.

The water diagnostic and BGC owner implement mirrored proportional-allocation algorithms instead of one typed generic resource-owner helper. The copies already differ: BGC rejects nonfinite/negative requests before accumulation, while the water version does not. This duplication can silently change authorization behavior by resource type and meets the repository's medium-severity duplication criterion. Centralize the shared arithmetic/protocol checks and retain only typed key/basis adapters at each owner.

## Ran evidence

- `cargo nextest run -p openwepp-vegetation --profile quick` — PASS, 2 tests.
- `cargo nextest run --test c3_vegetation_implementation_contract --profile quick` — PASS, 10 tests.
- `markdown-doc lint --path docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/review_agent_a_remediation.md` — PASS, 1 file, 0 errors and 0 warnings.
- `git diff --check -- <review artifact>` — PASS.
- Static line-count audit — PASS against the package warning threshold: largest touched Rust module is `crates/openwepp-vegetation/src/transaction.rs` at 1,677 lines; no reviewed Rust file reaches 2,000 lines.

These focused passes establish compilation and the current single-stratum/unit vectors only; they do not exercise or negate the findings above.

## Residual risk and missing tests

- No adversarial test constructs a public `CoupledCandidate` with forged state, transaction identity, beginning operands, dry-material inventory, or ledger identity and proves commit rejection.
- No exact digest test rejects mutated configuration/state bytes or the placeholder digest fixtures.
- No optics poison family independently varies leaf area, stem area, leaf optics, stem optics, and clumping while reconstructing leaf/stem shortwave and zero-leaf behavior.
- No wet, multi-rank or fractional-tile vector independently reconstructs one stand precipitation input, ordered interception handoff, top radiation boundary, and terminal ground flux.
- No near-tolerance C/N/material closure vector checks the authoritative scale-aware bound.
- No forced energy, hydraulic, coupled, or radiation nonconvergence vector asserts the distinct `VEG-E-NUM-003/004/005/006` codes and failure diagnostics.
- The current independent fixture covers selected component values, but the public end-to-end transaction test asserts successful commit/rollback and transaction IDs rather than an independent E01--E22 ending-state and five-ledger oracle.

Approval statement: **blocked**. The focused tests pass and line-count governance has no blocker, but the exact current worktree is not acceptable for science-implementation closure until the critical commit, optical-area, and column-ledger defects and the high-severity identity/tolerance/taxonomy drift are remediated and independently tested.
