# Compatibility Runtime Deletion

Status: EXECUTED-COMPLETE-PARTIAL-DELETION

Package id: `20260630-compatibility-runtime-deletion-001`

## Objective

Remove the legacy symbol-map hillslope runtime from production hot-loop selection now
that direct production is the validated default for snow, frost, water balance, and
publication surfaces.

## Operator Decision

The package keeps the explicit `--compatibility-runtime` replay/comparator seam and
deletes the obsolete production-transition modes:

- `DirectSkeletonNoop`
- `DirectSkeletonShadowOnly`
- `DirectPublicationFrameShadow`
- `DirectPublicationFrameCutover`

The retained compatibility seam is diagnostic-only. No default, implicit fallback, or
sidecar-discovery path may select it.

## Scope

- Remove public/API/CLI access to obsolete skeleton, shadow, and cutover runtime modes.
- Delete their runner hot-loop adapter paths and stale tests.
- Add source guards proving the obsolete modes and CLI flags do not re-enter.
- Refresh roadmap/spec text that still treats frost validation as a blocker to
  compatibility deletion.
- Record the static call-graph audit and gate evidence.

## Non-Scope

- No direct runtime physics changes.
- No RSS or working-set reduction.
- No watershed or stream-temperature work.
- No deletion of the explicit compatibility replay/comparator seam.
- No full migration of setup-time parsed input carriers if that proves to be a
  separate typed-setup package.

## Gates

- Direct output identity evidence for H2637 plus multi-OFE/Wave-2 fixtures where
  feasible.
- Zero forbidden compatibility counters on production direct fixtures.
- Static source audit proving no production selector can reach the deleted modes.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo nextest run --workspace --profile full`.
- `cargo deny check`.
- `bash tools/release/check_authority_suite_antievasion.sh`.
- `cargo test --test auth11_required_suite_obligation_guards_contract`.

## Disposition

This package deletes the obsolete production-transition runtime modes and their
runner hot-loop adapters. It intentionally preserves the explicit
`--compatibility-runtime` replay/comparator seam.

Full deletion of every symbol-map/setup carrier is deferred: setup-time parsed
input and replay support still use compatibility-shaped carrier types outside
the production direct hot loop. That is a typed-setup/full-replay-deletion
follow-on, not a reason to keep the old production transition modes.
