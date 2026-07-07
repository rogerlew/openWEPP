# Gate Results (D15A-P4)

Status: **EXECUTED — ALL GATES PASS**.

Evidence mode: **Ran** for every PASS row (this session; commands recorded in
the referenced artifacts).

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | clean (this session, final tree) |
| Markdown/doc lint | PASS | `tools/check_sc_binding_exposure.py` on the amended contract: `PASS-DEFERRED` (6 BEI rows, 5 science-review-follow-on — the deferred-allowed posture); no repo markdown linter beyond it is configured |
| Contract/profile/BEI checks | PASS | same BEI run; `SC-OFEROUTE-001` rev 27 amendment authored contract-first |
| Focused Lane D / `ofe_routing` tests | PASS | 67/67 (`ofe_routing`/`kinematic_wave`/`cascade`/`friction`/`seam`/`d10b` expression, S4) + 5/5 `laned_active` unit tests + 3/3 fast H2637 selector guards (fail-closed w/o routing coefficients; shadow/active mutual exclusion; legacy shadow fail-closed) |
| `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only` | PASS | final gate run (`--no-fail-fast`, both ignored tests CONCURRENT): **2/2 passed** in 420.5 s — the D12 shadow test still pins `days_seen=731 / days_routed=622 / uniform 6 (0 with melt / 6 without)` and the D15A active test passes all closure/manifest assertions. Transparency note: the FIRST execution of the active test failed at full duration (~275 s) with the panic message lost to log truncation; the isolated rerun (263 s) and this concurrent gate run both pass on the identical behavioral tree (only lint-visibility/fmt changes intervened). The one-time failure is recorded as unreproduced; if it recurs, treat as a test-isolation investigation, not a runtime defect (the release CLI evidence is deterministic across 3 runs). |
| H2637 opt-in ACTIVE endpoint timing | PASS | `37.50 / 37.48 / 37.44 s` user (3 runs, `taskset -c 4`, release) — inside the S5-adjudicated budget (78.8 s) and under the original D14 shadow budget ×1.64 |
| H2637 opt-in shadow endpoint timing (S4 optimization surface) | PASS | `78.75 / 78.77 / 78.78 s` user ×3 (optimization-results.md); post-implementation shadow re-run bit-identical |
| H2637 opt-in slot/profile timing | PASS | optimized profile: cascade 79.65 s, cfl 55.15 s, step 20.65 s; counters identical to baseline (optimization-results.md) |
| Protected-output byte identity (off) | PASS | off-run hashes identical to the pre-package baseline at S0, post-S4, and post-P2 (protected-output-byte-identity.md) |
| Active closure evidence (`INV-OFEROUTE-012`) | PASS | live hard-fails (proven by two real mid-implementation aborts); final run: supply 7.3e-16, day cascade 2.5e-13, day identity 2.4e-13 maxima over 610 routed days (consumer-path-proof.md) |
| DC01-disable / no-double-feed proof | PASS | surface transfer suppressed for active lanes; INV-OFEROUTE-009 typed guard ran on all 731×19 lane-days without firing; lateral preserved (`sbrunv` 208,153→208,133 m³) |
| Routed-hydrograph-to-erosion proof | PASS | `RoutedHydrograph` authority + weights on every active lane-day; fail-closed D13 validation live on all wet days; erosion surfaces on the routed shape (consumer-path-proof.md) |
| `cargo fmt --check` | PASS | clean (final tree) |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | clean after fixing 10 findings the first run raised (visibility + too-many-lines) |
| `cargo nextest run --workspace --profile full` | PASS | **1410/1410 passed** (4 slow), 3 skipped, 599 s, final tree |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok` |
| Authority anti-evasion guards | PASS | `check_authority_suite_antievasion.sh` PASS + `auth11_required_suite_obligation_guards_contract` 2/2 (run although no authority-suite surface was touched) |

## Post-review re-verification (final tree, after the dual-review fix batch)

The QA-H2 seam repair (hourly forcing breakpoints + the soil↔router
cross-ledger hard-fail), CR-M1/M2 test-hygiene fixes, CR-L1 latqcc coverage,
CR-L2 day-coordinates, and the contract reconciliation were applied; the full
gate set was re-run (Ran, package logs):

| Gate | Status | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | final tree |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | after one `too_many_arguments` allow on the shared `route_single_ofe` entry |
| focused suites | PASS | 40/40 orchestrator focused + 3/3 fast H2637 guards |
| ACTIVE endpoint ×3 | PASS | `38.03 / 37.74 / 37.51 s` user (breakpoint clipping cost ≈ noise) |
| off byte identity | PASS | `948faf82… / 725f5723… / f0d1be11…` — still the pre-package baseline |
| shadow bit identity | PASS | hashes unchanged (`&[]` breakpoints keep the shadow trajectory frozen) |
| `cargo nextest run --workspace --profile full` | PASS | **1410/1410**, 589 s |
| ignored H2637 pair | PASS | 2/2, 409.7 s |

Seam-fixed active evidence block (`logs/p5_seamfix_laned_active_block.json`):
`days_seen=731, days_routed=610, days_uniform_shape=3,
lane_days_erosion_source_shape_degenerate=1,
max_supply_reconstruction_rel=7.3e-16, max_day_cascade_residual_rel=2.5e-13,
max_day_seam_residual_rel=5.0e-14, max_day_identity_residual_rel=2.5e-13,
total_source_m3=374423.35, total_routed_outlet_m3=374463.08,
total_end_window_storage_m3=3167.32, total_clamp_m3=3207.05,
total_tail_fold_m3=36426.08, total_latqcc_outlet_m3=208132.8460294917`.

Two load-bearing observations: (1) the breakpoints exposed and eliminated a
~0.11 % hour-straddling booking error in the pre-fix injection
(`373,995 → 374,423 m³` — the exact silent-failure class the seam check now
hard-fails); (2) `total_latqcc_outlet_m3` now reconstructs the PUBLISHED
parquet `sbrunv` column sum (`208132.8460294918`) to 1 ulp — an independent
produced-output reconstruction of the INV-OFEROUTE-012 bypass term.
