# MOFEFID-B02FIX — cli03 QOFE Convention Regression

Status: **EXECUTED — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) — B02
follow-up. Owner: Claude Code. Scope: **test-only** (migrate a stale
contract-derived integration test to the ratified B02 convention).

## Defect

`tests/integration/cli03_runner_contract_derived_tests.rs::cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`
is **red on main** — a B02 regression. Its helpers asserted the pre-B02
M-F-REDO2 convention where the published `QOFE` carried the OFE-local-length
depth (`QOFE != Q`, ratios `[1.0, 2.5, 6.0]`), and reconstructed the runon
handoff as `UpStrmQ = QOFE_upstream x area_ratio`. B02 (`INV-RUNOFFPART-032`)
adopted the post-`wepp_260516` convention `QOFE == Q` on all rows and
superseded the M-F-REDO2 `QOFE != Q` anti-clone proxy (see the reconciled
`INV-WATBAL-098` / `INV-SYSTEM-031`).

**Root cause of the slip:** B02 migrated the orchestrator (`r7d4`) and runner
crate (`03_tests`) QOFE fixtures, but this **workspace integration test** was
not in either crate-scoped suite run at B02 merge time; only the full-
workspace nextest exercises it, and that was not run. No sibling old-
convention QOFE test exists (grep-verified) — this file was the only miss.

## Fix (test-only)

- `assert_b02_qofe_equals_q_all_rows` (was `assert_mfredo2_qofe_local_depth_geometry`):
  now asserts `QOFE == Q` on every row (`INV-RUNOFFPART-032`); the pre-B02
  ratio/anti-alias assertions removed (superseded; genuineness proven by the
  handoff + not-cloned checks).
- `assert_mf_multiofe_publication_surface_handoff`: the runon handoff physics
  is unchanged, but the published `QOFE == Q` no longer carries the local
  depth, so `UpStrmQ` is reconstructed from published `Q x local-length ratio
  x area ratio` (verified: `593.75 = Q 118.75 x 2.5 x 2.0`) — the B02 consumer
  consequence `INV-RUNOFFPART-032` documents (recover from `Q`, not `QOFE`).

## Validation

- `cli03_runner_contract_derived_tests` **22/22** (was failing at the handoff
  assertion). Full-workspace scan: no other old-convention QOFE test.
- Remaining full-workspace failure `auth04_...release-gates.yml` is a
  setup-dependent harness test (reads a staged `/tmp/openwepp-origin-main-check`
  checkout absent here) — not code, not in scope. fmt clean.

No production code changed; the fix aligns a stale test with the already-
ratified `INV-RUNOFFPART-032`.
