# V49 multi-child prepared-install authority QA review

Status: `APPROVE`

Evidence mode: `Static + Ran`

Reviewer role: independent secondary Rust QA re-review

## Findings

No remaining V49 closure-blocking finding.

- `HIGH`, resolved during re-review —
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_v49_tests.rs:154`,
  `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs:1720`:
  the literal retained-r124 source 42 / resident 43 / predecessor 43 / target
  44 vector on support `1920..2040 s` now directly executes
  `install_v2_soil_from_authenticated_prepared_beginning_v1`, the production
  helper used by both ordinary fixed-point finalizer branches. The exact
  accepted no-op runs through that helper twice. Tests verify the installed
  resident, unchanged outer source owners, full no-op stability, and unchanged
  publication history. The source-bound call-site test remains complementary
  proof that both real finalizer branches delegate to the helper.
- `HIGH`, resolved during re-review —
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_v49_tests.rs:244`:
  authoritative-resident transaction, support, receipt, resealed state,
  resealed layer, latest-accepted custody, and latest seal are now poisoned
  individually. Prepared, accepted, opaque-authority, foreign resident,
  separately divergent source-owner, and jointly rebased source-owner cases
  remain covered. Every refusal asserts full
  `DirectV10RealConsumerShadow` equality as well as independent canonical soil
  bytes, all outer owners, and publication history.
- `MEDIUM`, resolved during re-review —
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_v49_tests.rs:11`,
  `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs:111`:
  the fixture and binding assertion now use `r124`, matching the exact capture
  provenance rather than the preceding failing r123 run.

## QA assessment

The correction remains cohesive: V48 and V49 share one validated accepted-
resident construction path, while the V49 opaque authority additionally binds
the authenticated authoritative source and reconstructs the complete resident,
prepared target, and explicit source/target authority before atomic install.
The generic path and V48 source-equals-predecessor posture remain strict. Tests
cover repeated same-parent target 44 then target 45 successors, individual
custody substitutions, missing generic authority, exact rollback, accepted
no-op, and no private publication without introducing diagnostic persistence.

Line-count governance is current: production is 2,645 lines, retained V10
tests 2,962, the V49 sibling 536, and finalization 2,936. All remain below the
3,000-line hard boundary; the existing exact-move plans remain binding before
further growth.

## Ran evidence

- Independent re-review: `nix develop -c cargo nextest run -p
  openwepp-hillslope-orchestrator -E 'test(/v49_/)'` — Nextest
  `e98b3616-e941-45e0-ad20-52a13f4258b2`, `5/5 PASS`.
- Independent re-review: `nix develop -c cargo nextest run --test
  snow_terminal_enthalpy_event_numerics_contract -E 'test(/v49_/)'` — Nextest
  `51ec200e-c28d-4bfa-9587-b49d0de34e15`, `2/2 PASS`.
- Implementation-agent retained evidence: V39/V46/V47/V48/V49 runtime
  `d1cf7bb1`, `41/41 PASS`; complete snow source-contract target `208ec63c`,
  `42/42 PASS`; persisted restart `a90a80f2`, `40/40 PASS`; all-target/all-
  feature check, authority anti-evasion, required-suite guard, and diagnostic
  scan `PASS`.
- Independent re-review: `nix develop -c cargo fmt --all -- --check` — `PASS`.
- Independent re-review: `git diff --check` — `PASS`.

## Non-blocking debt and follow-ups

- Whole-crate/workspace warnings-denied Clippy remains blocked by extensive
  shared-head debt outside the V49 increment. This focused approval does not
  mark `WGHL-CLIPPY-001` passed or waive the package's mandatory terminal
  warnings-denied gate.
- `cargo deny check` is not selected for V49 because the increment changes no
  manifest, lockfile, dependency, license, source-policy, or workspace
  resolution surface.
- The retained V10 test file has only 38 lines of headroom and finalization has
  64. Their recorded exact-move plans must execute before either reaches 3,000
  lines; no exception is approved.

## QA disposition

`APPROVE`. The prior real-finalizer and resident-custody coverage gaps are
closed, focused and retained gates are green, rollback/no-publication evidence
is exact, and no V49 QA blocker remains before parent-owned r125.
