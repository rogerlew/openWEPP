# QA/evidence review — 2026-10-05

Reviewer: `/root/qa_final_review`, independent QA/evidence review. Scope: final
`tools/dev` lifecycle behavior, host timer activation, custody evidence, and
validation status. Source reviewed: base `2d1d6b0a4973f2fa76226dc3e3d6022ab3309c8d`
plus the current uncommitted Nix-tooling/package diff.

## Ran evidence

- `tools/dev/tests/nix-lifecycle-test` — PASS.
- `tools/dev/develop tools/dev/check-nix-tooling` — PASS.
- `.venv/bin/python artifacts/real_pin_release.py` — PASS. This is an isolated
  real-Nix profile test, not a mocked lifecycle invocation.
- `git diff --check` — PASS.

## Findings and same-reviewer fix verification

The initial real-Nix pin/release test found a release defect: Nix's owned
`<name>-1-link` generation remained a GC root after `release`, and `pin` created
an unwanted checkout `result` link. This was a blocking lifecycle/custody
finding.

The current `nix-lifecycle` repair uses `nix build --no-link`; before deleting
an owned profile or pin, it enumerates only numeric `<prefix>-<generation>-link`
siblings, requires every candidate to resolve to the profile target, and makes
no deletion if any numeric candidate is inconsistent. It preserves nonnumeric
siblings. The reviewed real-Nix test proves all of these behaviors: a tampered
candidate refuses without partial release/prune; release removes the package
pin's root; and prune removes its normal profile generation root. The final
root query contains none below the isolated package state directory. The earlier
leaked root is deliberately outside that isolated test and remains correctly
visible as prior-test residue, not evidence of a new leak.

Host activation is verified by `host-retention-activation.json`: reviewed unit
hashes match installed user units, the timer is enabled and active with the next
scheduled elapse on 2026-10-12, and an explicit service invocation succeeded
with journal evidence. The record reports `Linger=no`; scheduled execution is
therefore session-bound while that remains true. This is documented operational
scope, not a lifecycle code failure.

## QA verdict

No remaining in-scope QA/evidence or maintainability defect was found after the
generation-root fix. Focused checks, real concurrency evidence, parsed
environment equality with byte-equal lock evidence, 183-path recovery-backed
deletion evidence, five retained tool roots, and host timer activation are
acceptable for their stated scope.

The package remains **HOLD**, not complete. Its required critical full-workspace
`cargo nextest run --workspace --profile full` completed with exit 100 after
1,494.022 seconds: 4,204 tests run, 4,013 passed, 190 failed, one timed out,
and 76 were skipped. This is a required gate **FAIL**, not a deferred or passed
result. The failures are outside this package's Nix/custody scope; no science
repair, test filtering, or authority change was made here. The terminal failure
prevents package closure.
