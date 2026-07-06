# Final Disposition

Status: **EXECUTED-COMPLETE** (2026-07-06).

Evidence mode: Ran (endpoint timing, profiling, identity comparisons, gates,
focused tests) + Static (scope/boundary reasoning), labeled per artifact.

## Disposition

D14 executed all six phases end-to-end. The Lane D runtime cost is now
empirical, attributed, and substantially reduced without touching numerical
authority, closure evidence, protected outputs, or activation semantics.

## Runtime-budget outcome

- Release-grade H2637 baseline re-pinned at commit `1d7dc75a`: default/off
  `2.3 s`, shadow-on `67.6 s` → **Lane D shadow overhead `+65.3 s` user
  CPU** (the campaign's `+207 s` figure was a dev-profile estimate).
- Slot + perf attribution: **~97 % solver math** (`solver_cfl` 51.4 % —
  redundant per-step alpha recomputation; `solver_step` 46.2 %; libm `pow`
  35.6 %); allocation ~2 %; per-day/OFE setup, sampling, source-rate
  construction, operand build, handoff, and runner integration all ≤ ~1 %.
- Three bit-identical optimizations landed (single alpha evaluation per
  cell per step; per-solver step scratch; hoisted skin rain term):
  shadow-on **`67.6 s → 29.9 s` wall (2.26×), overhead `+27.6 s` (−58 %)**,
  shadow multiplier ~29× → ~13×. The dev-profile H2637 evidence test
  dropped `325.24 s → 226.86 s`.
- D15 receives the explicit budget, the refresh rule, and the remaining
  activation risks in `worker-handoff.md`.

## Behavior preservation (the package's acceptance bar)

- Default/off protected outputs byte-identical to pre-optimization
  references (legacy and native fixtures; independently recomputed by
  Review B and Verifier B).
- Shadow-on protected outputs byte-identical; shadow-off manifest carries
  no shadow keys.
- The `laned_shadow` manifest diagnostic block is bit-identical at full
  float precision; trajectory counters identical (10,334,879 steps) as the
  preservation witness.
- No solver-method, limiter, CFL-target, fixed-point, tolerance,
  resolution, activation-selector, SC-* contract, or schema change
  (diff-level boundary sweep by Verifier B: zero `const` definition
  changes; `cf`/`phi` byte-identical).

## Gate summary

All required gates PASS (`gate-results.md`): package timing/identity/parity
gates; focused tests 64/64; markdown lint; `git diff --check` (one
trailing-whitespace FAIL caught by the delegated runner, fixed, re-run
clean); `cargo fmt --check`; workspace clippy `-D warnings`;
`cargo nextest run --workspace --profile full` **1387/1387** (2 skipped);
H2637 ignored evidence test PASS; `cargo deny check`. Anti-evasion guards
N/A (no governed files touched).

## Review and verification

Dual independent reviews (adversarial code review + governance/truthfulness
QA with independent executional re-verification) produced 16 findings; all
accepted, none rejected or deferred; all fixed or recorded
(`review-disposition.md`). Dual independent verification confirmed 10/10
fixes and issued a READY call (`verification-disposition.md`). Subagent
role substitutions (Claude agents in the Codex-named roles) are recorded in
`gate-results.md`.

## Explicit non-goals preserved

- No production/default Lane D activation; no D15 selector, DC01 disable,
  routed-path publication cutover, or manifest activation claim.
- No D10 `GAP-OFEROUTE-005` correction, limiter/handoff method change,
  Case-4 acceptance, or tolerance loosening.
- No D11 friction-source policy change (rev-21 operand path preserved);
  no D12 melt-limb rule change; no D13 erosion-shape semantic change.
- No D16 default-promotion policy; no watershed/channel routing work.
- No surrogate, provisional, proxy, heuristic physics, or numerical-method
  shortcut (OPT-4/OPT-7 pre-registered and rejected).

## Handoff and hygiene

- D15 handoff: `worker-handoff.md` (budget, risks, refresh rule).
- Commit hygiene: the pre-commit verifier correctly flagged that all D14 work
  was still uncommitted and that
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs` had to
  be included. The D14 closeout commit stages that file with the package
  changes. The delegated-gate logs live in the session scratchpad; the durable
  record is this artifact set.
