# Independent Verification A

Static: independently inspected both review artifacts, `disposition.md`, the
final Rust/contract/catalog diff, the strengthened regression, the recorder and
typed terminal guard, consumer-path evidence, endpoint/cohort/byte-identity
artifacts, gate chronology, package status, protected boundaries, and
line-count disposition. I did not read or rely on `verification_agent_b.md`.

Ran: focused nextest run `bf460275-79eb-4c50-a830-3862a69b4fc2` passed the
strengthened dry-front regression and defensive recorder test `2/2`;
`cargo fmt --check` and `git diff --check` passed; SC unit-compliance lint
passed; the binding checker returned its defined successful `PASS-DEFERRED`
posture (`10` rows, `9` preexisting follow-ons); package Markdown lint scanned
`22` then-current files with no errors or warnings. I also rechecked both
retained endpoint manifests, all five disabled-path `cmp` pairs, all three
selected-cohort manifests, release-binary SHA-256, the final source line count,
and the live production lower-bound/terminal-guard source lines. I did not
duplicate the final full workspace suite.

## Finding Disposition Verification

| Review finding | Recorded disposition | Fix verification | Result |
| --- | --- | --- | --- |
| Review A: stage faces were not directly observed | `accepted` | The vector calls `run_with_options_and_step_trace(..., true)`, asserts exact `+0.0` for the first predictor face, and requires every retained predictor/corrector face finite and nonnegative (`kinematic_wave.rs:2177-2197`). | pass |
| Review A: exact consumer/call-site map and old-path check absent | `accepted` | `fidelity-and-byte-identity.md:9-56` now names runner selection, `DirectRunFrame`, `DirectDayFrame`, `LanedActiveLaneSource`, active executor, single-OFE route, solver/result/handoff, routed erosion consumer, publication row, five output surfaces, DC01 guard, and manifest negative proof. | pass |
| `RB-M1`: exact-dry fixture aliased the committed outlet discharge | `accepted` | Penultimate/outlet depths are `1e-4/1e-6 m`; both local discharges are positive, the raw extrapolation is negative, and the accepted traced face is exact zero and bit-distinct from positive outlet `q` (`kinematic_wave.rs:2141-2191`). | pass |
| `RB-M2`: consumer evidence source/call-site incomplete | `accepted` | The same complete lineage above is supported by the final source: runner installs `DirectRunFrame.laned_active`; executor calls `laned_active_route_lane`; that function calls `route_single_ofe_with_step_trace`, publishes `UpstreamHandoff`/`DirectLanedActiveDayRouting`, selects `RoutedHydrograph`, and the erosion consumer reads it before row publication/commit. | pass |
| `RB-L1`: recorder comment described borrowing as valid production behavior | `accepted` | `BinRecorder::finish` now states valid production bins are nonnegative by construction and retains forward carry only as defensive handling for invalid/independently injected samples (`kinematic_wave.rs:865-876`). | pass |
| `RB-L2`: catalog status was stale `SCAFFOLDED` | `accepted` | `docs/work-packages/README.md:32-36` now reports `IN EXECUTION`, the truthful state while dual verification/finalization is underway. Final closure can advance that same row to `EXECUTED-COMPLETE`. | pass |

All six findings are explicitly dispositioned `accepted`; none is rejected,
deferred, follow-up, or left undispositioned. Each response is present in the
final working tree.

## Closure Checks

- The production correction remains present exactly once at
  `kinematic_wave.rs:1159`:
  `self.scratch.face_flux[n] = raw_predictor_outlet_m2_s.max(0.0);`.
  The raw face still fails `NonFiniteState` before the lower bound, and the
  bounded face still flows through the existing available-water limiter,
  predictor update, and booked predictor/corrector mean. Review response edits
  did not replace, bypass, or move this line.
- The positive-outlet vector separates all plausible aliases: raw extrapolation
  is negative, committed outlet `q` is positive, and scheme-actual predictor
  face is exact positive zero. Every traced stage face, booked outflow, bin, and
  hydrograph rate is nonnegative. Outlet-bin sum equals ledger outflow bitwise.
- Closure is not self-restating: final storage change is reconstructed from the
  solver's committed depths and independently captured initial storage, not
  `MassBalance.storage_change_m2`; the resulting identity is bounded at
  `1e-15 m2` while clamp mass remains at dust scale.
- The defensive recorder vector independently injects `-1e-4 m2`, retains that
  exact terminal deficit, and publishes no negative bin. The public run branch
  still maps any material `terminal_deficit_m2 < 0` to unchanged typed
  `RoutingError::NegativeOutletBin` (`kinematic_wave.rs:1675-1680`).
- The consumer proof is executional as well as static. Both retained 34-year
  manifests select `direct-production-executor`, report effective mode pairs
  `0/0` and `1/1`, carry identical active closure operands, one publication
  capture, zero skeleton runs, and zero compatibility-edge invocations. The
  release SHA remains
  `a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d`.
  The old DC01 surface feed is fail-closed and the erosion authority dispatches
  to `RoutedHydrograph`, not `Dc01SourceShape`.
- All five protected disabled-path outputs remain byte-identical under fresh
  `cmp` checks. All three retained selected-cohort manifests name the same
  release binary, direct production executor, target-`dx` policy at `5.0 m`,
  and numerical-scale maximum identity residuals.
- `gate-results.md:14-18` records the final post-review closure loop as green:
  format, clippy, full nextest (`1694/1694`, run
  `e6e84783-62a8-4b91-9f5f-2a8b6a0cf222`, `593.690 s`), deny, and diff check.
  The source/test response predates that final full-suite record. Contract/unit
  gates, Case-4/19-OFE, selected cohort, both full endpoints, and protected
  byte identity are also directly evidenced; no current-scope gate is deferred
  to another increment. The binding check's named `PASS-DEFERRED` is the tool's
  successful posture for preexisting BEI follow-ons, not an unrun package gate.
- The only Rust diff is `kinematic_wave.rs`. No snow/winter, seam, daily/off,
  watershed, hybrid, tolerance, selector, or publication-mask code changed.
  The declared protected boundaries remain intact and do not shield an
  authority-backed in-scope correction. No `HOLD` is claimed or needed.
- The final file has `2570` lines: warning-level (`>=2000`) and below the
  `3000+` blocker. `line-count-governance.md` retains an adequate rationale and
  owned follow-on decomposition intent.

## Verdict

`PASS` for package closure.

The implementation, review responses, evidence, and required current-scope
gates have no remaining verification blocker. After the second independent
verification lands, final artifact lint/status updates, worker handoff, and
final disposition are normal closure-order bookkeeping rather than deferred
science, implementation, or acceptance work.
