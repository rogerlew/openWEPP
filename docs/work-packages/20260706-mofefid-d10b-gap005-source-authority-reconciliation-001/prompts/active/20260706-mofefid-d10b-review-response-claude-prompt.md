# D10B Review-Response Prompt for Claude

Task: revise the committed D10B execution (`main` head `1d202b10`) in response
to Codex's post-execution review, then commit the fixes and hand back for
Codex re-check.

Authoritative review artifact:

- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/artifacts/review-codex.md`

Mode:

- Execute the accepted review findings end to end.
- Do not reopen D10B's science direction unless a fix disproves it.
- Do not perform D15 activation, D14 endpoint profiling, default promotion, or
  new Lane D runtime policy work.
- Preserve clean-room boundaries and the D11/D12/D13 surfaces.

Required fixes:

1. **Fix High 1 — CFL fail-open.**
   - In `kinematic_wave.rs`, make non-finite, negative, or unsatisfiable true
     celerity fail closed instead of letting `dt <= 0` break to a successful
     partial result.
   - Add a focused regression proving corrupt/extreme celerity cannot return
     `Ok(RoutingResult)`.
   - Preserve typed error semantics (`RoutingError::CflViolation` or
     `RoutingError::NonFiniteState`, whichever the contract/code path supports
     most clearly).

2. **Fix High 2 — duplicated/divergent Iwagaki Case-4 source authority.**
   - Centralize or otherwise single-source the Iwagaki Case-4 configuration
     used by `dval::run_iwagaki_manning` and `iwagaki_oracle`.
   - Make the solver path match the oracle source duration exactly. In
     particular, remove the `t > dur` cutoff mismatch and prevent steps that
     cross `10 s` from applying lateral supply across the post-cutoff part of
     the step.
   - Add a regression proving solver-side and oracle-side lateral-source
     total/duration agree.

3. **Fix Medium 1 — terminal negative bin carry.**
   - Ensure the terminal exported bin series cannot contain a negative bin.
   - If exact non-negative redistribution is impossible, fail closed with a
     typed error rather than publishing a negative outlet discharge.
   - Add a regression for the terminal/single-OFE outlet case, not only the
     downstream cascade case.

4. **Fix Medium 2 — stale `cascade_seam_ledger` evidence labels.**
   - Update `examples/cascade_seam_ledger.rs` and related artifact wording so
     fields derived from solver-ledger outflow are not described as sampled
     quadrature evidence.
   - If sampled-quadrature evidence is still claimed, add an explicit computed
     sampled-quadrature diagnostic under the correct name.

5. **Fix Medium 3 and Medium 4 — stale GAP-005/D15 authority language.**
   - Reconcile `SC-OFEROUTE-001` producer obligations and BEI rows with rev 26:
     `GAP-OFEROUTE-005` is resolved.
   - Reconcile `docs/planning/mofe-fidelity-campaign-strategy.md`,
     `docs/ROADMAP.md`, and `docs/work-packages/README.md`: the next item is
     D14 endpoint-timing refresh, then D15 rerun. Do not leave text saying D15
     is blocked by rev-23/GAP-005.

6. **Fix Low cleanup items in the same pass.**
   - Update `oracle-reanchoring-evidence.md` from superseded monotone/strict-TVD
     language to the rev-26 bounded-wobble/bounded-transient acceptance surface,
     or explicitly mark the old text superseded.
   - Sync R-102/R-103 bibliography rights status with the rights log.
   - Reword/rename strict-TVD test comments/names to bounded TV transient.
   - Decide whether to expand the H2637-class durable regression from three to
     six sweep points; if not, record the reason.
   - Update stale top-level comments in `kinematic_wave.rs` and `ofe_routing.rs`.
   - Refresh line-count and focused-test-count artifacts.

Required evidence updates:

- Append a review-response section to D10B package artifacts, or add a
  dedicated response artifact, mapping each Codex finding to accepted/rejected
  disposition and evidence.
- Update `gate-results.md` with the post-fix commands and results.
- Update `disposition.md` so no accepted finding remains open.

Required gates after fixes:

- `git diff --check`
- Markdown lint over touched docs/package artifacts.
- Focused Rust tests for the changed D10B/ofe_routing surfaces.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

If the full suite is too slow for the immediate handback, still run the focused
tests and clippy, record the full-suite run as in progress or not run, and do
not claim final closure until it is green.

Commit:

- Commit the revision on `main`.
- Hand back the commit SHA, gate results, and a concise finding-by-finding
  disposition for Codex re-check.
