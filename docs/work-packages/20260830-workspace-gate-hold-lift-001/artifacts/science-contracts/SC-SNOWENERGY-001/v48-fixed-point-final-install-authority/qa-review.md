# V48 fixed-point final-install authority QA review

Status: `APPROVE`

Evidence mode: `Static + Ran`

Reviewer role: independent secondary Rust QA re-review

## Findings

No remaining V48 closure-blocking finding.

- `HIGH`, resolved during re-review —
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_tests.rs:558`,
  `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs:1717`:
  the positive V48 vector now constructs the literal retained-r122 custody
  chain, source 42 / target 43 / predecessor 42 on support `1800..1980 s`, and
  directly executes the same production finalizer install helper used by both
  non-continuation branches. It verifies the installed resident, unchanged
  outer source owners, unchanged authoritative beginning bytes, and unchanged
  publication history. The retained source-bound call-site test separately
  proves both ordinary finalizer branches call this helper and never the
  generic split-refusing installer.
- `HIGH`, resolved during re-review —
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_tests.rs:599`:
  the V48 suite now independently exercises generic missing/erased authority,
  foreign prepared custody, swapped and foreign explicit authority, prepared
  support and receipt-chain substitutions, accepted target, predecessor,
  support, receipt, state, layer seal, and orchestrator-seal substitutions.
  Every refusal compares soil owner bytes, all three source owners, and
  publication history. A separate positive vector executes same-ID install
  followed by exact accepted no-op through the production helper and verifies
  no publication.
- `MEDIUM`, resolved during re-review —
  `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs:1716`:
  the new helper's `clippy::result_large_err` finding is explicitly scoped to
  that function using the repository's existing error-posture convention.
  Whole-crate warnings-denied Clippy remains unavailable because of broad
  pre-existing/concurrent shared-head debt; the implementation artifact now
  records that limitation without representing `cargo check` as Clippy.
- `MEDIUM`, resolved during re-review —
  `docs/work-packages/20260830-workspace-gate-hold-lift-001/artifacts/line-count-governance.md`:
  current counts and decomposition intent are recorded. The V10 source/test
  exact-move plan remains binding, and `owner_finalization.rs` has a concrete
  exact-move plan for its leading test module before reaching 3,000 lines.

## QA assessment

Static review confirms that the generic/public installer still passes no split
authority and remains exact same-ID-only. The specialized path validates the
authoritative native-V2 resident and prepared beginning, derives the mutually
equal outer source transaction, reconstructs and then re-authenticates the
typed source/soil-target authority, validates the accepted result and seals,
and calls the unchanged V47 atomic clone/install only after all joins pass. It
contains no transaction arithmetic, adjacency inference, identity rewrite, or
diagnostic seam.

The behavior coverage is now substantive rather than name-only. Direct helper
execution proves the exact r122 split can traverse production authority code;
the source-bound call-site guard proves the real finalizer delegates both
non-continuation branches to that helper. Individually separated poisons prove
prepared and accepted custody cannot be substituted, while byte/source/history
comparisons prove rollback and no private publication. Retained V39/V46/V47
coverage continues to exercise the predecessor transaction model, budget
preflight, continuation custody, exact no-op, and atomic install posture.

## Ran evidence

- Independent re-review:
  `nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E
  'test(/v48_/)'` — Nextest
  `158e6614-7763-47f5-b3d0-9f0c7e15edf7`, `7/7 PASS`.
- Independent re-review:
  `nix develop -c cargo nextest run --test
  snow_terminal_enthalpy_event_numerics_contract -E 'test(/v48_/)'` — Nextest
  `8c10d27e-0e1e-46c2-b969-8f457d04cc37`, `2/2 PASS`.
- Reused current implementation-agent evidence: retained V39/V46/V47/V48
  behavior Nextest `13d19bf0-7cee-4fed-ac75-c2be27d49ccf`, `36/36 PASS`;
  complete source-contract target `c3c678fa-9754-44af-9d3d-547f3b1ca12a`,
  `40/40 PASS`; persisted restart `63b17fa0-d750-4cec-ae55-73539f8a1dfb`,
  `40/40 PASS`; required-suite guard
  `c6cfd6b7-49e3-43fe-b62d-27e7ac7afb57`, `3/3 PASS`; all-target/all-feature
  check, anti-evasion, formatting, diff hygiene, and diagnostic scan `PASS`.
- `wc -l` terminal counts: `2,468` for `v10_soil_thermal_v2.rs`, `2,956` for
  its included tests, `2,933` for `owner_finalization.rs`, and `1,516` for the
  source-contract integration test.

## Non-blocking debt and follow-ups

- The whole-crate strict Clippy command remains blocked by extensive
  shared-head warning debt outside V48. This focused approval does not mark the
  wider `WGHL-CLIPPY-001` package gate passed or waive it at package closure.
- The two files at 2,956 and 2,933 lines have little remaining headroom. Their
  recorded exact-move splits are binding before either reaches 3,000 lines; no
  exception is approved.
- `cargo deny check` is not selected for V48 because the increment changes no
  manifest, lockfile, dependency, license, source-policy, or workspace
  resolution surface.

## QA disposition

`APPROVE` for V48 implementation and parent-owned r123 qualification. The
authenticated prepared-beginning path is readable, exact, fail-closed, and now
supported by real-helper, call-site, positive, poison, rollback, no-op,
no-publication, retained-regression, and source-authority evidence. This
focused approval does not close the wider WGHL package or waive canonical
r123, wider strict-Clippy debt, dual verification, or final disposition.
