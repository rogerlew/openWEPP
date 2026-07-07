# Codex Review Request — LANED-T3-AGG aggressive-rule deficit-carry fix (rev 30)

Task: dual review (code-correctness lane + QA/governance lane) of the
LANED-T3-AGG change set. Diff base: `ef4172d5` on `main`. At authoring time
the change set is the UNCOMMITTED working tree on top of that base; if a
commit has landed it by the time you run, review that commit instead (same
content — verify the base).

Repo: `/home/workdir/openWEPP`. Read `AGENTS.md` +
`docs/work-packages/AGENTS.md` first. Evidence rules apply: label `Static:`
vs `Ran:`; you may execute any gate (focused `ofe_routing` suites are fast;
the full workspace suite was `1424/1424` at execution; H2637 endpoint runs
need the fixture recipe in
`docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/baseline-profile.md`
— and note the BUILD CAUTION below before timing anything).

## Package

`docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/`
(status EXECUTED-PRIZE-NOT-REALIZED) closed the rev-28 named composition
defect — short explicit spans between implicit bins stranding front-arrival
terminal-bin deficits (`NegativeOutletBin`, H2637 lane 17 day 54) — and
flipped the hybrid switching mask to the AGGRESSIVE zero-source-only rule
(`SC-OFEROUTE-001` rev 30, amendment authored before code; selector remains
EXPERIMENTAL/evidence-gathering).

Read first: `artifacts/fix-evidence.md` (design deltas, unit-vector lattice,
executed H2637 evidence incl. the instrumented carry-fire run, the honest
timing disposition), `artifacts/gate-results.md`, the rev-30
revision-history entry in
`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`, and
the parent handoff item-1 disposition in
`20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/worker-handoff.md`.

Change surface (all in `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`):

- `kinematic_wave.rs`: `run_with_options` split into a fail-closed wrapper
  over the new composition-scoped
  `run_with_options_deficit_carry(...) -> (RoutingResult, f64)`; new
  retained vector `bin_recorder_returns_material_terminal_deficit_exactly`.
- `cascade.rs`: `absorb_deficit` + `dispose_terminal_carry` helpers; the
  cross-span carry threaded through `route_single_ofe_hybrid` (implicit-bin
  booking, explicit-span bins, span-terminal deficit joins the carry after
  the span's bins); mask predicate now `seam_rate_at(source, t0) == 0.0`
  only; 4 new retained vectors + a `profile::test_flag_guard()` fix on the
  new profile-counter test.

## Adversarial questions (code lane, priority order)

1. **Exact-total invariant through the carry chain.** `absorb_deficit`
   claims `booked + new_carry == mass + old_carry` exactly (pure fp
   addition, no rounding beyond the single `+`). Trace the composed
   identity: does `Σ published bins == Σ booked outflow` survive (a) a
   deficit absorbed across MULTIPLE subsequent bins, (b) deficits from
   MULTIPLE explicit spans accumulating in one carry, (c) the backward
   absorption in `dispose_terminal_carry`? Anchor any counterexample as a
   concrete bin/step sequence.
2. **The over-counting escape surface.** The deficit-carry variant returns
   a `RoutingResult` whose `outlet_bin_outflow_m2` (and derived hydrograph)
   over-counts by `|deficit|`. Audit ALL callers: is
   `route_single_ofe_hybrid` truly the only consumer, and does it discard
   the over-counting hydrograph and use only the (carry-corrected) bins?
   Could a future caller plausibly reach the variant without the carry
   obligation (visibility is `pub(super)` — is that tight enough)?
3. **Sub-noise remainder disposition.** `dispose_terminal_carry` fails
   closed above `1e-9 × gross` and absorbs backward below it. Two edge
   classes to check: (a) an all-zero (or near-zero-gross) series where the
   sub-noise carry cannot be fully absorbed — the remainder is silently
   dropped (`Σ bins` then differs from booked outflow by ≤ the floor); is
   that acceptable and is it honestly documented? (b) the recorder's own
   noise rule folds into the last COVERED bin and can go noise-negative —
   the composed rule instead guarantees non-negativity; confirm no consumer
   depends on the recorder's exact fold semantics.
4. **Attribution vs mass.** The carry moves bin ATTRIBUTION forward in
   time (never mass — ledgers book actual fluxes). Downstream consumers of
   the bin series: the inter-lane `UpstreamHandoff.bins_m2` injection and
   the D13 erosion hourly-weight mapping (`laned_active.rs`). Does shifting
   a deficit's absorption into later bins (possibly a later HOUR) create
   any contract violation on those surfaces (INV-OFEROUTE-008 shape
   authority, non-negative weights, unit-sum closure)? The magnitudes
   observed are 3e-9..1.2e-5 m² — assess whether a larger deficit could
   materially distort the erosion shape while still passing the
   end-of-window rule.
5. **Peak/hydrograph consistency.** Implicit-bin peak tracking uses the
   PRE-carry physical `outcome.outflow_m2 / dt` while the exported bins are
   POST-carry. Is that the right semantic (peak = physical diagnostic,
   bins = conservative attribution), and is it documented enough to survive
   the next reader?
6. **Aggressive mask exactness.** The predicate samples the source at bin
   START only. The seam series is piecewise-constant per hour and bins are
   900 s — verify no path can put a source transition INSIDE a bin (window
   construction, breakpoint set, non-hour-aligned windows rejected), i.e.
   the point sample is provably bin-constant.
7. **Wrapper equivalence / no-perturbation.** `run_with_options` must be
   behavior-identical to pre-rev-30 (same checks, same error, same result).
   Ran evidence: plain-active H2637 parquet `21c54bf2…` reproduced. Confirm
   statically that no non-hybrid path behavior changed (plain cascade,
   shadow, default).
8. **The missing synthetic deficit vector.** No unit vector forces the
   SOLVER to produce a terminal deficit (scans over rain-grown and
   prescribed-flux fronts found no negative dip; the real class fired only
   on H2637 — 6 instrumented events). Assess adequacy of the two-sided
   pinning (recorder-return identity + composition vectors) — and if you
   can CONSTRUCT a deterministic solver-level deficit vector (the
   front-arrival `2q_{n-1}−q_{n-2}` dip or the near-dry boundary ripple cut
   mid-cycle at a window end), specify it as a finding.

## QA/governance lane

1. **Honesty of the prize disposition.** The package claims the explicit
   work cut matched I0's 55.5 % coverage but implicit cell-solve cost
   consumed it (38.0-38.3 s vs 37.9 s plain), and reprices Tier-3
   accordingly (backlog note update). Check the numbers against
   `artifacts/logs/` (timing, profile lines) and flag any residual
   overclaim, including in the rev-30 changelog entry.
2. **Status naming**: EXECUTED-PRIZE-NOT-REALIZED — is the package status,
   README index entry, and parent-handoff disposition consistent and
   non-overclaiming (fix executed ≠ prize realized)?
3. **Posture**: the selector must remain EXPERIMENTAL/UNRATIFIED
   everywhere; the aggressive rule SUPERSEDES strict in the contract — flag
   any text that implies ratified fidelity or a settled rev 28/30.
4. **Gate completeness** per `artifacts/gate-results.md` (fmt/clippy/deny/
   full nextest/focused/plain-invariance/closure/determinism/carry
   exercise) — anything material NOT RUN and not declared?
5. **Build-provenance caution**: the stale-binary near-miss is recorded
   (workspace `cargo build --release` does not relink `openwepp-cli-hill`);
   assess whether the mitigation guidance is sufficient or should be
   promoted (e.g. into `AGENTS.md`).

## Output protocol

Findings severity-ordered (High/Medium/Low) with `file:line` anchors and
concrete failure scenarios; explicit verdict
(GO / GO-WITH-AMENDMENTS / NO-GO) per lane. Write results to:
- `20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/review-codex.md`
  (code lane)
- `20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/review-qa.md`
  (QA/governance lane; if you fold it into one artifact, say so explicitly)
Do not modify production code; findings only. Gate rerun evidence welcome.

BUILD CAUTION for any executed evidence: build the CLI with
`cargo build --release -p openwepp-runner --bins` and verify the binary
mtime/hash — the workspace-level release build silently leaves a stale
`openwepp-cli-hill` (this bit the execution session; books bit-identical to
a prior record are the staleness tell). The H2637 fixture also hardcodes
its output paths in `p2637.run.toml` (`output/H2637.*`) — `--output-dir`
only relocates the manifest, so sequence plain/hybrid runs and hash
`output/` between them.
