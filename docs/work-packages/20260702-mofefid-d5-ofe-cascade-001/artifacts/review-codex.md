# Codex Review - MOFEFID-D5 OFE Cascade

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-d5` / `.claude/worktrees/mofefid-d5`
Reviewed range: `origin/main@c3de21df..78f7dfd4`, plus Codex merge-prep
cleanup before merge.

## Outcome

Accepted for merge.

D5 correctly lands an opt-in, shadow-first OFE-by-OFE routing cascade under
`SC-OFEROUTE-001`: each OFE is routed with the D4 single-OFE solver, the upstream
outlet hydrograph becomes the downstream upstream-boundary profile, and the
handoff is width-scaled so total discharge/volume is continuous across width
changes. The implementation is not wired into any production phase span.

I made review-time cleanup before merge:

- Corrected package truthfulness: D5 has 5 cascade tests, not 6, and the package
  now cites `SC-OFEROUTE-001` rev 5.
- Updated `SC-OFEROUTE-001` stale prospective wording so `GAP-OFEROUTE-003` is
  design-resolved while runtime infiltration composition and DC01-disable guards
  remain integration scope.
- Made the SC revision-history numbering unique by keeping D4 at rev 3, Codex D4
  cleanup at rev 4, and D5 at rev 5.
- Updated the module header to mention the D5 cascade and removed an unused local
  `peak_total` block from `cascade.rs`.

No production phase wiring or behavior was added by the cleanup.

## Evidence Classes

Static:
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs`.
- Reviewed `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`.
- Reviewed D4 solver integration points in
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.
- Reviewed `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- Reviewed `docs/work-packages/20260702-mofefid-d5-ofe-cascade-001/package.md`.
- Grep-reviewed for production wiring/call sites outside `cascade.rs`.

Ran:
- `cargo fmt --check` -> pass.
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` -> 23/23 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator` -> 171/171 passed.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED`.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` -> 2/2 passed.
- `cargo deny check` -> pass.
- `cargo nextest run --workspace --profile full` -> 1228/1228 passed, 1 skipped.
- No-wiring grep:
  `rg -n "run_cascade|CascadeSegment|CascadeForcing|CascadeResult|ofe_routing::cascade|cascade::" crates tests -g '!crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs' -g '!target'` -> no hits.

## Findings

| Candidate | Verdict | Evidence | Disposition |
|---|---|---|---|
| Cascade handoff and width scaling. | Accepted | Static: `run_cascade()` feeds the previous OFE outlet hydrograph into the next OFE as `q_up * (prev_width / current_width)`, then records received runon from the downstream solver's own inflow ledger multiplied by downstream width. Ran: handoff and width-change tests pass in the 23/23 OFE-routing suite. | Closed. |
| Cascade mass balance. | Accepted | Static: interior handoffs cancel in the cascade residual; D5 sums rainfall excess, storage change, and positivity-clamp terms by width, and only the terminal outlet contributes to exported volume. Ran: cascade conservation test passes and full orchestrator/workspace suites are green. | Closed. |
| Default-path/shadow-first claim. | Accepted | Static: the only non-doc call-site grep hits are inside `cascade.rs`; no phase-span or runtime publication path calls `run_cascade`. Ran: no-wiring grep returned no hits outside `cascade.rs`. | Closed. |
| `GAP-OFEROUTE-003` disposition. | Accepted with integration caveat | Static: contract rev 5 resolves the design as supersede-then-compose: routed hourly runon supersedes DC01 daily-lump admission when active, then composes with downstream hourly infiltration. The actual runtime infiltration composition and DC01-disable guard are explicitly integration scope, not claimed as D5 production behavior. | Deferred to integration/D-val for runtime composition evidence. |
| Package/contract truthfulness drift. | Accepted, fixed by Codex | Static: package claimed 6 tests but `cascade.rs` has 5 D5 tests; package/SC revision text had stale rev/prospective wording. | Updated package and SC text; no behavior change. |

## Merge Decision

Merge-ready. D5 is a shadow-first cascade mechanism, not an activation or D-val
package. The cascade logic, contract wording, no-wiring claim, focused tests,
workspace clippy, authority guards, `cargo deny`, and full workspace nextest are
green.
