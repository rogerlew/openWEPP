# Independent Verification B

Status: `PASS`

Static: independently inspected the final working-tree diff, both review
reports, the accepted finding dispositions, the rev-51 contract, strengthened
regression, solver face/update/ledger path, defensive recorder guard, exact
consumer lineage, endpoint/off-path manifests, gate record, and line-count
disposition. I did not read or rely on `verification_agent_a.md`.

Ran: the strengthened rev-51 regression and defensive recorder test passed
`2/2` in nextest run `fc163a4b-7d3d-456e-bf48-644a9a8a5fc2`;
`cargo fmt --check` and `git diff --check` passed; the contract
binding-exposure checker returned its defined successful `PASS-DEFERRED`
posture, SC unit-compliance lint passed, and `markdown-doc lint` passed the
package tree with no errors or warnings after this report was written. I also
re-inspected both accepted endpoint manifests, verified the release binary
hash, reran the five disabled-path `cmp` checks, and inspected the
selected-cohort summary. I did not rerun the already-recorded full workspace
suite.

## Finding Verification

| Review finding | Verification | Evidence |
| --- | --- | --- |
| Review A Medium: stage-face invariant was not directly observed | `PASS` | The vector now uses `run_with_options_and_step_trace(..., true)`, asserts a nonempty trace, pins the first raw-negative predictor face to exact `+0.0`, and requires every retained predictor/corrector outlet face to be finite and nonnegative. |
| Review A / `RB-M2`: exact consumer-path and old-path proof missing | `PASS` | `fidelity-and-byte-identity.md` now names runner selection, `DirectRunFrame`, `DirectDayFrame`, `LanedActiveLaneSource`, active executor, `laned_active_route_lane`, `route_single_ofe_with_step_trace`, `KinematicWaveSolver`, `RoutingResult`, `UpstreamHandoff`, `DirectLanedActiveDayRouting`, routed erosion authority/consumer, publication row, commit, and five output surfaces. It records the DC01 feed guard, routed-vs-DC01 authority dispatch, and manifest negative proof. |
| `RB-M1`: exact-dry vector aliased committed outlet discharge | `PASS` | The outlet state is now finite and positive (`depth = 1e-6 m`, locally consistent positive `q`) while `2 q[n-1] - q[n-2] < 0`. The trace asserts the accepted predictor face is exact zero and bit-distinct from committed outlet `q`; independent committed-depth closure and bin/ledger equality remain. |
| `RB-L1`: recorder narrative treated borrowing as normal production | `PASS` | `BinRecorder::finish` now states that valid production bins are nonnegative by construction and labels forward redistribution as defensive for invalid or independently injected samples. The material terminal-deficit path remains live. |
| `RB-L2`: catalog remained scaffolded | `PASS` | The package and catalog both report `IN EXECUTION` during verification; finalization can advance both to the completed disposition together. |

All review findings are accepted and fixed. None is rejected, deferred, or
routed to follow-on work.

## Contract, Face, Ledger, and Guard Checks

- The raw downstream predictor expression is checked for finiteness before
  applying the exact physical lower bound. The bounded face then passes
  through the existing available-water cap and is the same `pred_out_face`
  used by the predictor depth update and the booked
  `0.5 * (pred + corr) * dt` outflow.
- Predictor face zero is distinguished from both the negative raw expression
  and positive committed outlet discharge. Upstream faces are validated
  nonnegative inputs; interior predictor faces are committed nonnegative
  discharges; corrector faces are nonnegative `alpha * h_pred^1.5` values.
  The retained trace directly verifies finite/nonnegative predictor and
  corrector outlet faces over every step in the contract vector.
- `RoutingError::NegativeOutletBin`, the recorder noise floor, dry-depth
  tolerance, closure tolerances, and publication guards are unchanged. The
  separate recorder test still injects a material negative sample and proves
  the terminal deficit is retained while published bins remain nonnegative.
- The correction is a face-domain restriction before state formation, not a
  post-update storage clamp, damping term, unbooked mass adjustment, or
  publication-only mask. Clamp mass remains independently surfaced and is
  constrained at numerical scale by the vector and endpoint evidence.

## Consumer and Protected-Path Checks

- Both accepted H2637 manifests still identify release binary SHA-256
  `a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d`,
  select `direct-production-executor`, carry active summaries, report one
  publication capture and zero skeleton/compatibility-edge calls, and prove
  requested/effective modes `0/0` and `1/1` without divergence.
- The disabled-path post-run manifest identifies that same post-fix binary and
  no active summary. Fresh `cmp` checks passed for HBP, loss JSON, pass
  parquet, plot parquet, and water parquet against the preserved pre-fix
  outputs.
- The selected-cohort summary remains `PASS`: all three real `dx=5 m` members
  exited zero, and every mesh-policy and closure assertion passed under the
  same release hash.
- Static diff inspection finds no snow/winter-physics, seam-booking,
  selector, watershed, daily/off, or hybrid implementation edit. Historical
  hybrid changelog provenance remains documentation only; no removed selector
  or runtime path is revived.

## Gate Legitimacy and Non-Deferral

- The strengthened positive-outlet vector failed with `NegativeOutletBin`
  when only the rev-51 lower-bound line was temporarily removed (run
  `22a7683c-1528-444b-9bb6-c7f630bc96f4`) and passed after that exact line was
  restored (run `287ebe1a-0f18-4a1f-bdc2-86c352289576`). The result is not a
  vacuous absence-of-error gate.
- The final post-review record is current to the strengthened test and comment
  disposition: formatting, clippy, full workspace nextest (`1694/1694`, run
  `e6e84783-62a8-4b91-9f5f-2a8b6a0cf222`), and deny all pass. Case-4,
  19-OFE conservation, selected cohort, both effective H2637 modes, and
  disabled-path byte identity also have direct current evidence.
- Review response changed only tests, comments, evidence, and catalog status;
  the restored production correction and release binary are unchanged, so
  the retained endpoint/cohort/byte evidence remains provenance-valid.
- No required current-scope gate is reclassified as later work. Final package
  status, artifact catalog, and handoff updates are closure bookkeeping after
  dual verification, not deferred implementation or validation evidence.

## Governance and Closure Verdict

`kinematic_wave.rs` is `2570` lines: warning level, below the `3000`-line
blocker. The package records a proportionate rationale, openWEPP maintainer
ownership, and a concrete follow-on intent to separate tests/recorder and then
solver state from stepping without hiding this semantic correction.

No `HOLD` is claimed, and none is legitimate: the mechanism, authority,
production correction, regression, endpoint, and protected-path evidence all
close inside the declared envelope.

Closure verdict: `PASS`. I found no unresolved review finding, gate deferral,
protected-boundary violation, guard weakening, tolerance widening, hybrid
revival, anti-alias defect, or line-count blocker. The package may proceed to
the normal final disposition and handoff updates after the other independent
verification report lands.
