# Codex Review - MOFEFID-D4 Kinematic-Wave Solver

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-d4` / `.claude/worktrees/mofefid-d4`
Reviewed range: `cd4e70ca..c22a896b`

## Outcome

Hold as written. Do not merge yet.

The solver direction is sound: it is shadow-first, narrowly scoped to the
single-OFE D4 problem, the committed solver tests pass, and no phase-span
runtime wiring was introduced. The blockers are fail-closed/domain and
closure-evidence issues in the new public solver API, plus one contract/package
acceptance mismatch.

## Evidence Classes

Static:
- Reviewed `docs/work-packages/20260702-mofefid-d4-kinematic-wave-solver-001/package.md`.
- Reviewed `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.

Ran:
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` -> 13/13 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator` -> 161/161 passed.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` -> pass.
- `cargo fmt --check` -> pass.
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED` (expected prospective D4/D5 rows).
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` -> 2/2 passed.
- Shadow-first grep: `rg -n "KinematicWave|kinematic_wave|Forcing::|RoutingResult|RoutingError|ofe_route\\." crates tests -g '!crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs' -g '!target'`; the only relevant hit outside the new file was the module export at `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs:8`.
- Rust `f64::max` probe: `f64::NAN.max(0.0)` prints `0`.
- `/tmp/openwepp-d4-probe` against the checked-out crate with NaN rainfall excess, NaN upstream inflow, and NaN rainfall intensity returned `Ok inflow=0 rain=0 peak=0 samples=2`.

I did not run full workspace nextest, workspace clippy, or `cargo deny check`
because the package is held on source-level blockers.

## Findings

| Candidate | Verdict | Evidence | Disposition |
|---|---|---|---|
| CX-001: active-domain and non-finite inputs are not fail-closed. `step` reads `rainfall_intensity_m_s` without validation, then clamps upstream inflow and rainfall excess with `.max(0.0)`, and the friction path also uses `.max(0.0)` on rainfall intensity. Mesh/cell fields such as `S_o`, `k_o`, `C_d`, `D_r`, `lambda`, `LAI`, `h_c`, `Delta x`, and the time controls are only partially checked. This violates the ratified hard-fail posture for non-finite and invalid active operands. | Accepted, blocker | Static: `SC-OFEROUTE-001` requires non-finite active operands to hard-fail and roughness/solver domains to be enforced at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:133`, `:134`, `:135`, `:148`, `:149`, `:152`, `:153`, `:154`, `:164`, `:166`, `:175`, `:176`. Code masks or omits validation at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:343`, `:344`, `:351`, `:352`, `:451`, `:452`, `:453`, `:454` and `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs:49`. Ran: `/tmp/openwepp-d4-probe` returned `Ok inflow=0 rain=0 peak=0 samples=2` for NaN forcing. | Add typed invalid-input/domain errors and tests. Validate finite/non-negative forcing, finite positive mesh/time controls including `sample_dt_s`, and cell domains before equation evaluation. Only contract-authorized no-flow/component-absent states should normalize to zero. |
| CX-002: predictor/corrector positivity clamps are hidden from the mass ledger. `h_pred` and `h_corr` are clamped with `.max(0.0)` before the final negativity check and before `positivity_clamp_m2` is accumulated. The package uses `positivity_clamp_m2 == 0` as evidence that the conservation residual is discretization-only, but stage-level clamp mass can be injected without appearing in that accumulator. | Accepted, blocker | Static: package conservation claim at `docs/work-packages/20260702-mofefid-d4-kinematic-wave-solver-001/package.md:32`. Hidden clamps at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:372`, `:373`, `:382`, `:383`; only final clamp is recorded at `:405`, `:412`, `:414`, `:416`, `:436`. Contract requires per-increment conservation closure at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:138`, `:153`, `:166`, `:176`, and says the tolerance cannot close with an unnamed/noisy claim at `:235`. | Either fail closed on material negative predictor/corrector stages, or record every positivity correction in the ledger and add tests proving the validation cases have no hidden stage clamp. |
| CX-003: `sample_dt_s` can hang the solver. `run` validates mesh count, cell length, `end_time_s`, and `max_dt_s`, but not `sample_dt_s`. With `sample_dt_s <= 0`, `next_sample += sample_dt_s` never advances in the recording loop. | Accepted, blocker | Static: validation omits `sample_dt_s` at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:450`, `:451`, `:452`, `:453`, `:454`; sample state starts at `:463`, advances at `:474`, and loops/advances at `:507`, `:513`. | Add finite positive `sample_dt_s` validation and a regression test. This can share the invalid-input error from CX-001. |
| CX-004: D4 acceptance wording conflicts with the ratified contract's current test-vector obligation. The package honestly defers formal `Ef` because Cases 1-3 require infiltration coupling and Case 4 lacks clean observed data, but `SC-OFEROUTE-001` still says the single-OFE solver D4 evidence includes `Ef` evidence. Under the package gate non-deferral rule, package-local prose cannot silently relax a current contract gate. | Accepted, blocker unless contract is amended | Static: package deferral at `docs/work-packages/20260702-mofefid-d4-kinematic-wave-solver-001/package.md:47`, `:49`, `:60`; contract row assigning `Ef` evidence to D4 at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:247`; D-val acceptance invariant at `:158`; package process non-deferral rule in `docs/work-packages/AGENTS.md` applies. | Either produce the D4 `Ef` evidence or amend `SC-OFEROUTE-001` before close to move formal `Ef` to D-val/D5 while preserving the D4 solver evidence requirements. The latter looks scientifically correct from the package rationale. |
| CX-005: shadow-first/no phase wiring. | Rejected | Ran: the grep above found no production runtime call site for `kinematic_wave`; only the module export is present. Static: `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs:8` exports the module, and the new solver is otherwise self-contained. | No blocker. Default runtime byte-flat claim is credible for this branch. |
| CX-006: stale module headers. `ofe_routing.rs` and `friction.rs` still say `ADR-0033 Proposed`, and `ofe_routing.rs` says TVD routing lands in later stages / D3 friction only. | Accepted, minor | Static: `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs:1`, `:2`, `:3`, `:4`, `:5`; `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs:1`. | Non-blocking cleanup, but fix while dispositioning the blockers so the public module docs do not contradict D4. |

## Merge Decision

Not merge-ready. I did not fast-forward `main`.

The numerical tests and no-wiring check are promising, but CX-001 through
CX-004 need disposition before D4 can be treated as an end-to-end solver close.
