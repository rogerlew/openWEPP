# Review Disposition (LANED-T3, Codex dual review 2026-07-07)

Status: **EXECUTED**. Every finding accepted and fixed; re-verification
evidence below. The code lane's NO-GO applied to SETTLING rev 28 — that
posture is preserved (package status EXECUTED-HOLD-REV28-RATIFICATION); the
experimental opt-in itself continues under the fixed tree.

| Finding | Disposition | Resolution |
|---|---|---|
| T3-H1 (Filippov can mask a real branch-solve failure) | **accepted** | The Filippov COMMIT is REMOVED entirely — the rev-29 monotonicity argument proves both-branches-jump is unreachable for genuine physics (one upward jump per branch rating; strictly decreasing cell line), so a double collapse can only be a solve failure and now FAILS CLOSED (`solve_cell`, typed `ImplicitSolveNonConvergence`). Stronger than the requested hull validation: no filled-jump commit path exists to mask anything. |
| T3-H2 (Steffensen basin determinism seed-biased) | **accepted** | Acceleration is BASIN-LOCKED: accepted only when the whole plain triple `(q_est, q1, q2)` sits in one basin AND the accelerated point stays on that side — the accelerated sequence is side-locked to the plain iteration each cycle and converges to the plain-iteration limit from the given seed (which is the rating's defined value). Documented at the guard. |
| T3-M1 (package text overclaims closure) | **accepted** | `package.md` status re-written to EXECUTED-HOLD-REV28-RATIFICATION with the open I2 closure acceptance named; `worker-handoff.md` "all closure gates green" corrected. |
| T3-M2 / T3-QA-M2 (missing direct Filippov-chain vector) | **accepted** | Retained test `low_jump_recovers_high_branch_root_and_never_commits_filippov`: a constructed jump-crossing cell (bare 0.05/500, `Δt/Δx = 450`, `rhs = h_b + 450·1.5e-3`) — asserts the LOW branch reports `Jump`, the chain recovers the genuine HIGH-branch root (verified against the converged high equilibrium), and the cell mass identity holds. |
| T3-L1 (dust floor under-pinned) | **accepted** | Retained test `dust_scale_steps_do_not_accumulate_a_material_leak`: 10,000 dust-scale steps on a short mesh accumulate < 1 % of one dry-threshold depth over the mesh. |
| T3-QA-M1 (gate artifact absent; "gates green" overclaim) | **accepted** | `artifacts/gate-results.md` created with the honest table — run-level gates PASS, the two I2 CLOSURE gates (Case-4 hybrid ladder; fidelity ratification) marked OPEN and holding the package. |
| T3-QA-M3 (hybrid selector not neutralized in harness) | **accepted** | All three H2637 helpers neutralize `OPENWEPP_LANED_ACTIVE_IMPLICIT` at entry; harness contract comment updated. |
| D15A re-check Low (stale `package.md` status) | **accepted** | D15A `package.md` status updated QUEUED → EXECUTED-COMPLETE with pointers. |

## Post-fix re-verification (Ran, 2026-07-07, rev-29 tree)

- `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D
  warnings` PASS; `cargo deny check` PASS; contract/BEI lint PASS-DEFERRED
  (posture unchanged).
- Focused suites 45/45 (incl. the two new review vectors:
  `low_jump_recovers_high_branch_root_and_never_commits_filippov`,
  `dust_scale_steps_do_not_accumulate_a_material_leak`) + 3/3 fast H2637
  selector guards.
- Full workspace suite: **1419/1419 passed** (586 s).
- H2637 hybrid ×3 on the fixed tree: `36.96 / 36.99 / 37.14 s` user; closure
  maxima and the routed books are BIT-IDENTICAL to the pre-fix evidence
  (outlet `373581.05999359465`; seam `1.7e-14` / cascade `6.4e-14` /
  identity `2.1e-13`; `logs/rev29_hybrid_block.json`) — empirical
  confirmation that the removed Filippov commit had never fired on this
  fixture and that basin-locking altered no converged equilibrium, exactly
  as the rev-29 determinism argument predicts.
- H2637 plain-active: parquet hash unchanged (`21c54bf2…`) — rev-27
  no-perturbation holds through the review fixes.
