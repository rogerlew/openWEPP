# Correctness review — 2026-10-05

Reviewer: `/root/correctness_review`, independent correctness reviewer. Source
reviewed: base `2d1d6b0a4973f2fa76226dc3e3d6022ab3309c8d` plus the current uncommitted
Nix-tooling, governance, and package diff.

Evidence class: **Static and Ran**. I inspected the final lifecycle code,
governance text, custody records, environment-equivalence evidence, host-service
evidence, real-Nix integration evidence, and complementary QA disposition. I
also reran the focused lifecycle test, Nix-tooling check, shellcheck, and scoped
`git diff --check` against the final cut.

## Findings

### Critical — required full-workspace correctness gate remains HOLD

The mandatory `cargo nextest run --workspace --profile full` exited 100 and
ended `FAIL`: 4,204 tests ran, 4,013 passed, 190 failed, one timed out, and 76
were skipped. Its terminal log records substantive Stage-3/V11 and
watershed-runner failures and a 720-second timeout in test 4204/4204,
`accepted_stage3_real_runner_routes_lane_d_and_publishes_summary`. The package
correctly classifies the infrastructure and custody change as Critical and
provides no waiver or retrospective deferral. These failures block package
completion even though the reviewed diff does not modify the protected
Rust/science sources and the protected-source recheck passes.

No High, Medium, or Low correctness finding remains in the affected Nix
generation-root repair.

## Same-reviewer fix verification

The initial release/prune implementation left Nix's numeric profile-generation
links as GC roots. The final implementation in `tools/dev/nix-lifecycle` fixes
that defect within the owned profile namespace:

- `pin` uses `nix build --no-link --profile`, preserving the named profile root
  without creating a checkout-level `result` link.
- `remove_generations` performs a validation pass over every numeric
  `<profile>-<generation>-link` symlink before unlinking any candidate. Every
  selected link must resolve to the already validated profile target; a
  mismatch makes `release` fail or makes `prune` preserve the profile, staged
  source, and receipt. Removal occurs only after the complete validation pass.
- `release` holds the global pin-name lock while it validates and removes the
  numeric generation roots, profile alias, and metadata. `prune` holds the
  environment's exclusive lifecycle lock while performing the equivalent
  owned-profile operation.
- Nonnumeric siblings are outside Nix generation syntax and remain untouched.
  Numeric non-symlink filesystem entries are likewise preserved and cannot act
  as Nix profile GC roots.

`artifacts/real-pin-release-final.json` is adequate real-Nix evidence for the
repair. It demonstrates root creation, pin retention during prune, refusal with
no partial mutation for a mismatched numeric pin generation, successful release
of the valid pin generation, refusal with no partial mutation for a mismatched
normal-profile generation, removal of the valid normal-profile generation, and
absence of isolated-state GC roots afterward. It also demonstrates that no
checkout `result` link is created and that a nonnumeric lookalike is preserved.
The complementary QA reviewer independently reran that harness and accepted the
fix in `artifacts/qa-final-review.md`.

The narrowed governance wording is accurate: bounded retention applies to the
wrapper-owned staged Nix environments and explicitly does not claim automatic
Cargo-cache pruning. `artifacts/host-retention-final-service.json` shows the
installed user service completed successfully and the timer remains enabled and
active. `artifacts/protected-work-check.json` confirms the pre-existing dirty
Rust files and untracked paths remained intact.

## Ran evidence

- `bash -n tools/dev/nix-lifecycle tools/dev/tests/nix-lifecycle-test && tools/dev/tests/nix-lifecycle-test` — PASS.
- `tools/dev/develop tools/dev/check-nix-tooling` — PASS.
- `tools/dev/develop shellcheck tools/dev/develop tools/dev/nix-lifecycle tools/dev/check-nix-tooling tools/dev/openwepp-env tools/dev/tests/nix-lifecycle-test` — PASS.
- Scoped `git diff --check` for `tools/dev`, the bounded-execution standard,
  and this package — PASS.

## Residual risk and missing validation

The lifecycle files live in same-user cache/state directories, so a same-user
process can race filesystem entries between validation and unlink. The tool
never follows these links for recursive deletion, locks lifecycle operations,
and removes only literal owned link paths; this is an acceptable local tooling
residual rather than a custody blocker.

The required full-workspace test has a terminal `FAIL` disposition. No further
affected lifecycle test is missing; resolving or authoritatively disposing the
full-workspace failures requires follow-on scope and cannot be inferred from
the passing infrastructure checks.

## Disposition

**APPROVE the scoped Nix lifecycle implementation, host activation, governance,
and completed recovery-backed historical cleanup.** The generation-root defect
is fixed and verified. **Overall package status remains HOLD** until the
mandatory full-workspace failures are resolved and the required regression
passes under authorized follow-on scope. The terminal failing run cannot
support package completion.
