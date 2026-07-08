# Codex Review

Evidence mode: Static + Ran.

Static:
- Reviewed the uncommitted diff for `SC-OFEROUTE-001`, Lane D active selector/runtime wiring, direct day-input authority, and `tests/integration/laned_shadow_h2637.rs`.
- Checked package gates and required artifacts in `package.md`.

Ran:
- `git diff --check` -> PASS.
- `cargo nextest run --test laned_shadow_h2637 h2637_default_mixed_routing_coefficients_fails_closed h2637_active_and_disable_are_mutually_exclusive h2637_active_fails_closed_without_routing_coefficients --no-capture` -> PASS, 3 tests.
- `cargo nextest run --test laned_shadow_h2637 h2637_legacy_shadow_fails_closed_without_routing_coefficients --no-capture` -> PASS, 1 test.
- `cargo nextest run --test laned_shadow_h2637 --run-ignored only h2637_native_active_owner_routes_and_closes --no-capture` -> FAIL after 304.18 s at `tests/integration/laned_shadow_h2637.rs:566`: `ReleaseMetadata { source: Io { path: ".../target/debug/deps/laned_shadow_h2637-9959da457becff80 (deleted)", ... } }`.

## Findings

### High - Default eligibility loses incomplete schedule-coefficient state and can silently fall back

`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:29` resolves default eligibility only by counting `lane.ofe_routing.is_some()` over the already-built lane authorities. That `Option` is lossy for default-activation policy: the upstream authority builder returns `Ok(None)` when any schedule crop slot lacks a full routing-coefficient block (`00_builders_and_authority.rs:837`) and also returns `Ok(None)` when two complete slot authorities differ (`00_builders_and_authority.rs:846`). The slot helper separately treats an all-field-absent block as `None` (`00_builders_and_authority.rs:904`).

That collapse is safe for explicit active/shadow because `laned_geometry_with_selector()` later fails on `None`, but it is not safe for rev 46 default/no-env resolution. A coefficient-present but schedule-incomplete or schedule-inconsistent management can become indistinguishable from a true no-coefficient legacy fixture. If every lane collapses this way, `laned_active_default_eligibility()` returns `Absent`, and the run proceeds on the legacy/off path instead of failing closed for mixed/incomplete coefficient authority.

This violates the package binding policy at `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md:15` and `package.md:19`, plus `SC-OFEROUTE-001`'s fail-closed statement at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:251`. The resolver needs a non-lossy authority state, for example `Complete / TrulyAbsent / IncompleteOrInconsistent`, before the default fallback can be considered safe.

### High - Coefficient-complete default activation is not currently proven by usable ran evidence

The contract requires a coefficient-complete no-env run to attach active Lane D and close the active evidence surfaces (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:553`). The package also requires all-extended default/no-env runtime proof, active closure evidence, DC01-disable/no-double-feed proof, and routed-consumer proof (`package.md:159`, `package.md:164`, `package.md:165`, `package.md:166`).

The only checked-in all-extended default-active execution test is ignored (`tests/integration/laned_shadow_h2637.rs:546`). When explicitly run with nextest, it failed at the default-active leg (`tests/integration/laned_shadow_h2637.rs:564`) before producing acceptance evidence. The failure is a release-metadata/test-harness error rather than a routed-water residual, but the gate is still unmet. Package-local acceptance artifacts are also absent: `default-activation-evidence.md`, `default-activation-evidence.json`, and `consumer-path-proof.md` are required at `package.md:142` but are not present.

Until the default-active evidence path runs cleanly and records the release-binary/runtime outputs, the default activation claim cannot be accepted.

### Medium - No-coefficient fallback and explicit-disable byte identity are not tested or artifacted

`SC-OFEROUTE-001` requires byte-identical protected outputs for the no-coefficient fallback and explicit-disable rollback paths (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:552`). The package requires the same fallback proof (`package.md:37`, `package.md:160`, `package.md:190`).

The non-ignored legacy fallback test only asserts absence of Lane D manifest blocks plus non-empty HBP/parquet outputs (`tests/integration/laned_shadow_h2637.rs:289`, `tests/integration/laned_shadow_h2637.rs:299`). The ignored explicit-disable leg similarly asserts no active keys and non-empty outputs (`tests/integration/laned_shadow_h2637.rs:553`, `tests/integration/laned_shadow_h2637.rs:560`) while the comment says the byte comparison lives in package evidence (`tests/integration/laned_shadow_h2637.rs:549`). That package evidence is not present under the required artifact list (`package.md:142`).

This leaves protected-output drift on the legacy/off and rollback paths undetected by the current tests.

### Medium - The selector resolver lacks the required unit/contract-derived test surface

The package explicitly requires unit/contract-derived tests for the selector resolver (`package.md:157`). The new resolver is small and policy-critical (`00_builders_and_authority.rs:29`), but the current coverage is only fixture-level integration. The passing non-ignored tests exercise mixed lane authority and selector conflicts, but they do not directly pin all resolver states or the lossy schedule-slot cases described above.

For a default activation switch, direct unit vectors should cover true all-absent, true all-complete, lane-mixed, schedule-slot missing, schedule-slot inconsistent, explicit active precedence, explicit disable precedence, and active+disable conflict.

## Residual Risk And Missing Tests

- I did not run `cargo fmt --check`, clippy, full nextest, or `cargo deny check`.
- No substantial duplicated Rust algorithm was introduced in the reviewed diff.
- I did not find arithmetic, clamp, unit-conversion, or erosion-water-magnitude changes in the reviewed diff; the main correctness risk is selector/domain classification and missing acceptance evidence.

## Verdict

BLOCKED for package completion. The mixed-authority and explicit-selector focused tests pass, but rev 46 default activation is not acceptable for closure until incomplete coefficient authority cannot be silently classified as absent, the coefficient-complete default-active runtime proof runs cleanly, and protected-output byte-identity evidence is recorded for fallback/disable paths.
