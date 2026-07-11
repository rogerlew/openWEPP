# Final Disposition

Status: `EXECUTED-COMPLETE`

Evidence mode: `Static + Ran`

## Outcome

`LANED-NOB-001` is closed. The deterministic lane-8/day-2621
`NegativeOutletBin` arose when the raw downstream predictor extrapolation
`2 q[n-1] - q[n-2]` became negative during a source-quiet dry-front recession.
The negative stage face was booked as reverse outlet flow and increased
storage; the unchanged bin recorder correctly failed the terminal deficit.

`SC-OFEROUTE-001` rev 51 now binds a finite, one-way downstream stage face.
`KinematicWaveSolver::step` applies the exact-zero lower bound before the
existing available-water upper cap and uses that same face in the state update
and ledger. No tolerance, state clamp, damping, mass adjustment, publication
mask, compatibility wrapper, or hybrid path was introduced.

## Acceptance Evidence

- The strengthened positive-outlet contract vector fails with
  `NegativeOutletBin` when only the rev-51 lower-bound line is absent and
  passes with exact-zero/nonnegative traced stage faces, independent committed-
  depth closure, and exact bin/ledger equality when restored.
- The canonical 1987--2020 H2637 endpoint exits `0` in effective
  `wepp_ui` modes `0/0` and `1/1`. Both runs route `10744` of `12419` days,
  produce identical active closure operands, and remain at numerical-scale
  residuals.
- The real direct producer/frame/executor/solver/handoff/erosion/publication
  consumer chain is proven; manifests report one publication capture, no
  skeleton execution, no compatibility-edge invocation, and no projection
  fallback.
- All five protected daily/off outputs are byte-identical pre/post.
- The three-member production `dx=5 m` active cohort passes mesh and closure
  assertions under release binary SHA-256
  `a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d`.
- Case-4 oracle and 19-OFE conservation gates pass.
- Final repository closure loop passes: format; clippy; full nextest
  `1694/1694` (run `e6e84783-62a8-4b91-9f5f-2a8b6a0cf222`); and deny.
- Contract binding/unit governance, unit registry, documentation, spelling,
  and diff-integrity gates pass.

## Review and Verification

Both independent reviews returned `PASS-WITH-FINDINGS`. Every finding was
accepted and fixed: direct stage-face observation, positive-outlet anti-alias
coverage, exact consumer lineage/old-path proof, corrected recorder narrative,
and catalog status. Independent verification A and B both returned `PASS`;
no finding is rejected, deferred, or left for follow-on.

The correction remains inside the declared authority envelope. Snow physics,
seam booking, selectors, watershed behavior, daily/off routing, and hybrid
code are untouched. No `HOLD` is claimed or legitimate. The `2570`-line source
file is warning-level and below the blocker; its nonblocking split intent is
recorded separately.

Security impact: `NONE`.
