# SC-OFEROUTE-002 Review Agent B

Lane: Agent B, code-vs-contract fidelity and retained-vector anchoring.

Evidence:

- Static: read `SC-OFEROUTE-002` rev 1 draft, `SC-OFEROUTE-001` rev 32 pointer rows, the registry row, package reference material, and current implementations in `implicit_recession.rs`, `kinematic_wave.rs`, `cascade.rs`, runner selector/profile plumbing, and active runtime selector wiring.
- Ran: `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/index.md --path docs/work-packages/20260707-laned-router-hybrid-contract-authority-001` -> PASS, 7 files.
- Ran: `python tools/check_sc_binding_exposure.py` on `SC-OFEROUTE-002.md` and `SC-OFEROUTE-001.md` -> PASS-DEFERRED for both, as expected for review-follow-on rows.
- Ran: `bash tools/release/check_sc_unit_compliance.sh --path ...` on both contracts -> PASS for both.

Verdict: GO-WITH-AMENDMENTS for lifting `status: draft` to `approved`.

No High findings. The core algorithmic mapping is present: LOW->HIGH branch order, cold and warm seed acceptance, basin-locked Steffensen, fail-closed double-collapse, aggressive zero-source mask, hour-partition guard, cross-span carry, wrapper fail-closed posture, selector wiring, manifest flag, and the rev-31 counter names all match current implementation. The amendments below should land before approval because several draft statements are stricter or less anchored than the current implementation.

## Medium

### B-M1: Exact-total claims do not consistently carry the bounded all-dry drop exception

The implementation and retained C-L1 vector allow an unabsorbable sub-noise terminal carry to be dropped on an all-zero or insufficient-gross series: `dispose_terminal_carry` computes the floor, rejects only material deficits, walks bins backward, then returns `Ok(())` without requiring the remaining sub-noise carry to reach zero (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:387`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:391`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:396`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:402`). The retained vector pins this behavior with bins left unchanged (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1143`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1150`).

The draft records the drop in the algorithm text (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:241`), but other binding surfaces still make unqualified exact-total claims: required outputs require `sum(bins) == booked outflow` (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:104`), composed surfaces state exact equality (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:247`), `INV-OFEHYB-006` states exact sum while also mentioning the bounded drop (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:286`), and consumer/producer obligations let consumers rely on exact equality (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:310`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:318`).

Why this matters: as written, the contract is stricter than the retained implementation in the C-L1 degenerate class. Amend the exact-total statements to explicitly say "exact except for the approved C-L1 bounded all-dry/insufficient-gross attribution drop" or change the implementation to preserve exact total in that class.

### B-M2: The implicit residual hard guard is not transactionally aligned with "commit only validated states"

`OBL-OFEHYB-P-002` says the implicit stepper commits only validated `(h, q)` states (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:311`), and the algorithm says no exit path returns an unvalidated pair after the residual hard guard (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:192`).

The implementation mutates the caller-provided depth and optional discharge buffers cell-by-cell before the post-march residual guard runs (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:164`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:165`). If the residual guard fails, the function returns `ImplicitSolveNonConvergence` after those mutations (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:194`). Current production callers fail closed on the error path, so I do not see a publication leak, but the lower-level API is not transactional in the way the draft wording implies.

Why this matters: approval would bind a stronger state-commit rule than the implementation currently provides. Either stage new depths/discharges and copy them out only after the residual guard, or narrow the contract wording to say the production publication path fails closed without accepting the mutated state after an error.

### B-M3: `maturity: experimental` is outside the current contract/registry maturity vocabulary

The draft front matter uses `maturity: experimental` (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:5`), and the registry row repeats `experimental` (`docs/specifications/science-contracts/index.md:56`). The science-contract spec and registry define the maturity vocabulary as `proposed`, `draft`, `active`, or `deprecated` (`docs/specifications/science-contract-spec.md:41`, `docs/specifications/science-contracts/index.md:36`).

Why this matters: the package intent is clear, but lifting the contract to approved while leaving a non-vocabulary maturity value creates structural non-compliance unless the schema is first amended. Use an allowed lifecycle maturity plus a separate selector posture note, or amend the vocabulary before approval.

## Low

### B-L1: Several guard-map rows still use vector-family prose instead of actual retained test names

The prompt asks the B lane to verify guard-map rows anchor real tests by actual names. Some rows do this, for example `hybrid_rejects_cadence_that_does_not_partition_the_seam_hour` and `case4_hybrid_manning_ladder_meets_iwagaki_oracle` (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:299`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:302`). Other rows use family labels such as "I1 exactness/positivity vectors", "LOW-jump->HIGH-root regressions", "direct regression vector", "recorder deficit-return vector", and "all-explicit bit-identity vector" without actual function names (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:296`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:297`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:298`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:300`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:301`).

The real retained tests exist, including `implicit_step_ledger_is_exact_and_positive`, `implicit_step_books_upstream_inflow_exactly`, `steady_state_is_a_fixed_point_of_the_implicit_step`, `dust_scale_steps_do_not_accumulate_a_material_leak`, `low_jump_recovers_high_branch_root_and_never_commits_filippov`, `branch_warm_seed_preserves_solution_and_reduces_or_matches_map_work`, `branch_warm_seed_acceptance_is_basin_locked`, `hybrid_is_bit_identical_on_all_explicit_windows`, `hybrid_rejects_non_integral_windows`, the `rev30_deficit_carry_tests` functions, and `bin_recorder_returns_material_terminal_deficit_exactly` (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:466`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:503`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:554`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:581`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:900`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1016`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1073`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1388`).

Why this matters: the guard map is usable, but the approval artifact would be stronger if every test-backed row cited exact retained function names rather than review-era family labels.

### B-L2: The SC-OFEROUTE-001 registry row is stale relative to the rev-32 front matter

`SC-OFEROUTE-001` front matter now says `last_reviewed: 2026-07-07` (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:17`), but the registry row still says `2026-07-06` (`docs/specifications/science-contracts/index.md:55`). The SC-OFEROUTE-002 row itself is registered (`docs/specifications/science-contracts/index.md:56`).

Why this matters: this is not a code-vs-contract physics issue, but it is a structural cleanup needed for a clean pointer-transfer approval record.

## Positive Checks

- `implicit_recession.rs`: LOW->HIGH solve chain, no filled-jump commit on double collapse, cold seeds, branch-side finite positive warm seed acceptance, residual dust floor, and retained warm-seed/LOW-jump/dust tests match the draft.
- `kinematic_wave.rs`: Steffensen basin lock matches the draft; `run_with_options` remains the fail-closed wrapper and `run_with_options_deficit_carry` remains `pub(super)` for hybrid composition.
- `cascade.rs`: aggressive zero-source mask, upstream-fed implicit bins, hour-partition guard, non-integral-window guard, cross-span carry, material deficit fail-closed behavior, sub-noise backward absorption, and bounded all-dry drop match the current implementation.
- Runner plumbing: `OPENWEPP_LANED_ACTIVE_IMPLICIT` composes with the active selector, the active config carries `hybrid_implicit`, the manifest records `hybrid_implicit_stepping`, and the profile line/counter names match `ofe_routing::profile`: `solver_steps_implicit`, `implicit_equilibrium_map_evaluations`, and `implicit_branch_evaluations`.
- SC-OFEROUTE-001 rev-32 pointer rows point to SC-OFEROUTE-002 and no dangling old hybrid-row references were found outside historical/package artifacts.
