# Final Disposition

Status: EXECUTED-HOLD-SOLVER-CORRECTION-REQUIRED
Evidence mode: Static + Ran.

## Outcome

The package landed the rev-40 active clamp-source publication guard and fixed
the executor ordering so active route books are checked before any row consumer,
dynamic transfer publication, or frame commit can observe a pathological
active-routed day.

WA now fails closed instead of silently publishing material positivity-clamp
amplification:

- `baseline_fixed10`: day 1418, `laned_active_clamp_exceeds_source`,
  clamp/source `14.291141234409194`.
- `dx5`: day 1167, `laned_active_clamp_exceeds_source`, clamp/source
  `11335.893753002358`.

This closes the silent-publication blocker. It does not make WA active routing
physically acceptable, does not promote target-`dx`, and does not relax any
closure tolerance.

## Gates

Final gates passed:

- `git diff --check`
- Markdown/doc lint: 16 files, 0 errors, 0 warnings
- BEI checker: PASS-DEFERRED, 8 BEI rows
- SC unit compliance: PASS
- Unit registry: 21/21
- `cargo fmt --check`
- focused active guard and runner selector tests
- WA expected-fail release rerun
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`: 1418/1418
- `cargo deny check`

## Follow-On

Next package: solve the active router positivity amplification itself. The
rev-40 guard must remain in place while the solver correction is developed and
must be passed by WA fixed10/dx5 before target-`dx` promotion reopens.
