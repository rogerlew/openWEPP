# Promotion Readiness Audit

Status: EXECUTED-HOLD-FIDELITY-TOLERANCE. Evidence mode: Static + Ran.

## Scope Boundary

Promotion candidate: hybrid implicit-explicit stepping becomes the default
when `OPENWEPP_LANED_ACTIVE=1` selects the active Lane-D production owner at
the current 10-cell/OFE mesh.

Out-of-scope default activation: turning on the active Lane-D production owner
without `OPENWEPP_LANED_ACTIVE=1`.

## Current Preconditions

| Gate | Status | Evidence |
|---|---|---|
| Case-4 full-hybrid ladder | PASS | Ran `cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick`; 1 passed in `144.949 s`. |
| H2637 solve-cost blocker | PASS | D16 pre-change explicit hybrid: `33.45 s` user / `0:33.47` wall, `980804` implicit steps, `0` equilibrium map evaluations. |
| H2637 timing vs active plain | PASS | Active plain: `39.73 s` user / `0:39.75` wall. Explicit hybrid: `33.45 s` user / `0:33.47` wall. Hybrid is `6.28 s` user faster (`15.8 %`). |
| H2637 active closure under explicit hybrid | PASS | Hybrid manifest: supply residual `7.31e-16`, cascade `4.58e-13`, seam `4.08e-14`, identity `4.44e-13`. |
| H2637 fidelity/delta tolerance | BLOCKED | Active hybrid changes routed outlet by `-1646.027977 m3` (`-0.4396 %`) and pass annual sediment surfaces by `-6.474 %`; no current `SC-*` tolerance authorizes default promotion on those deltas. |
| Active day closure under promoted default | BLOCKED | No promoted-default run was made because the tolerance gate blocked before implementation. |
| Protected default/off byte identity | NOT RUN | No code/default flip landed; pre-change default/off baseline exists for a future follow-on. |
| Selector/provenance semantics | NOT RUN | No implementation landed; current semantics remain env opt-in. |

## Disposition

Promotion is held before implementation. The hold is not a timing hold and not
a Case-4 hold. It is a fidelity/tolerance authority hold: the current evidence
does not justify changing the default active path to hybrid without a ratified
production tolerance for the observed H2637 output movement.
