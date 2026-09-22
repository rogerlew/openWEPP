# Required full-validation recorder compatibility QA — 01

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static review of `artifacts/run_recorded.py`, package authority, and existing profile facts. No recorder edit or command execution occurred.

## Findings

No blocking conflict with the proposed narrow exception. The current recorder rejects any physical command above 180 seconds twice, before and after custody preflight. That is incompatible with the package’s already required Critical full-workspace command, `cargo nextest run --workspace --profile full --no-fail-fast`, whose established declared bound is 900 seconds. The package treats this as required longer validation, not as an optional retry or a reason to alter the test profile.

The exception must be a strict allowlist, not a general physical-timeout relaxation. `--required-full-validation` may bypass the 180-second physical guard only when all of these are true:

1. `--physical` is present;
2. timeout is exactly 900 seconds;
3. the selected command equals the established full Nix/environment/workspace argv, including `CARGO_TARGET_DIR`, `CARGO_BUILD_JOBS=2`, `cargo nextest run --workspace --profile full --no-fail-fast`; and
4. the support record independently has that exact argv, timeout, and `physical: true`.

Any missing flag, nonphysical declaration, changed timeout, added/removed/reordered argv token, or changed support-record field must retain the existing rejection. Apply this same predicate at both current timeout checks, so preflight and immediately-prelaunch behavior cannot diverge. Keep the current full-declared-bound-plus-1800-second reserve calculation, source/support/binary custody checks, exclusive attempt identity reservation, and zero automatic retries unchanged.

The full profile contains slow tests with a 90-second period and `terminate-after = 8` (720 seconds per slow test, with inherited/overridden cases). The 900-second full-workspace declaration is therefore an explicit longer-validation exception. It does not assert that the profile enforces a 180-second cap per child, and it does not change the 180-second cap for individual M1 physical diagnostics.

## Non-blocking follow-up

The implementation should add narrowly scoped recorder guard tests for the exact allowlisted success preflight and each rejection category above. These tests should exercise validation only and must not launch the full workspace command.

## QA disposition

**PASS — release the minimal allowlisted recorder correction and its guard tests for review.** This is recorder compatibility for the established Critical gate only; it is not run clearance, a physical-policy reclassification, a profile change, or acceptance evidence.
