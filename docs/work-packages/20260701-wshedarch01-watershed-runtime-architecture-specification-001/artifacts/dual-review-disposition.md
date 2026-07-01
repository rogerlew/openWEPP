# Dual Review Disposition

Status: `UPDATED`

Scope: read-only dual review of
`docs/architecture/watershed-runtime-architecture-specification.md`.

Reviewers:

- Primary correctness review: `rust_code_reviewer`
  `019f1eb6-4ee3-7373-adb6-53df86882190` (`Epicurus`) - `COMPLETE`.
- Secondary QA/test/governance review: `rust_qa_reviewer`
  `019f1eb6-7f4f-7613-9984-24c2e9e2c6ea` (`Nietzsche`) - `COMPLETE`.

## Review Verdicts

Primary correctness review verdict:

- Not ready before fixes. Architecture direction plausible, but required
  correction on payload validation, benchmark truthfulness/comparability,
  `--jobs` default authority, and test-deletion governance.

Secondary QA/governance review verdict:

- QA hold before fixes. Direction broadly right, but not closure/ratification
  ready until stale evidence, consumer-path gates, legacy-comparison wording,
  and queued dual-review disposition were fixed.

## Findings Disposition

| Reviewer | Severity | Finding | Disposition | Evidence |
| --- | --- | --- | --- | --- |
| Primary | High | `PassInventory` did not require fail-closed validation for missing latest-event routing payloads; current CLI silently defaults absent latest-event values to zero. | `accepted-fixed` | Revision 2 adds typed latest-event state (`EventPayload`/`NoEvent`) and forbids synthesized zeros from absent payloads in `docs/architecture/watershed-runtime-architecture-specification.md`. |
| Primary + Secondary | Medium | Baseline evidence was stale: the spec still said one validated full-chain run while WSHEDPERF01 now records three stability repeats plus one profile run. | `accepted-fixed` | Revision 2 updates the WSHEDPERF01 timing table and narrative to three stability repeats plus one profile run. |
| Primary + Secondary | Medium/High | Legacy comparison wording implied a direct speedup/parity gate even though WSHEDPERF01 records non-equivalent legacy/openWEPP scopes. | `accepted-fixed` | Revision 2 labels pinned-legacy comparisons as cross-scope engineering-budget evidence unless a legacy-equivalent openWEPP scope is introduced. |
| Primary | Medium | `--jobs` default was both specified and unresolved. | `accepted-fixed` | Revision 2 removes the unratified default from draft authority; performance runs require explicit `--jobs`, `--jobs 1` remains deterministic baseline, and default policy is ADR-owned. |
| Primary | Medium | Test deletion guidance cited hillslope deletion without carrying the open coverage caveat. | `accepted-fixed` | Revision 2 requires pre-deletion classification, protected assertion inventory, net protected coverage restoration, or `EXECUTED-HOLD` with a restoration package. |
| Secondary | Blocker | Package closure was premature while dual review remained queued/running. | `accepted-fixed` | This artifact now records both reviews complete; `gate-results.md` marks dual review `PASS`; package/disposition status is `EXECUTED-COMPLETE-DRAFT-SPEC-REV2-DUAL-REVIEW-DISPOSITIONED`. |
| Secondary | High | W2/W3 could close without proving the real `openwepp` consumer reads the new runtime path or proving the old path is unused. | `accepted-fixed` | Revision 2 adds Consumer-Path Proof requirements and specific W2-W5 acceptance bullets for runner/downstream/negative old-path proof. |
| Secondary | Medium | W2-W5 acceptance lists omitted required Rust closure gates for implementation/deletion packages. | `accepted-fixed` | Revision 2 adds implementation closure gates: `cargo fmt --check`, clippy, full nextest, `cargo deny`, and package-specific identity/consumer/deletion gates. |
| Secondary | Medium | W1 repeat benchmark wording was ambiguous after WSHEDPERF01 repeat evidence landed. | `accepted-fixed` | Revision 2 re-scopes W1 as baseline refresh/independent confirmation, with current WSHEDPERF01 repeat evidence accepted for draft orientation. |

## Residual Open Questions

- Whether a pass with no latest-event payload can ever represent a valid
  watershed `NoEvent` state is left to implementation/contract work; Revision 2
  requires that state to be explicit and validated, never silently defaulted.
- The production default for `--jobs` remains ADR-owned.
- Cross-scope comparison against pinned legacy remains an operator budget until
  a legacy-equivalent openWEPP scope is defined.
