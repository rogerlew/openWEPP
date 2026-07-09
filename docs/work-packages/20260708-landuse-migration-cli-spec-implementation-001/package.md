# Landuse Migration CLI Specification And Implementation

Status: `EXECUTED-COMPLETE`
Package ID: `20260708-landuse-migration-cli-spec-implementation-001`
Owner: Codex
Scaffold date: `2026-07-08`
Amended: `2026-07-08` for canonical YAML output
Execution date: `2026-07-09 UTC`
Evidence mode: `Static + Ran`

## Objective

Implement a Rust landuse migration library and CLI for crates.io distribution,
and finalize the public specification for that CLI.

The CLI converts frozen legacy WEPP management flat files and flat
`ow-lanuse-1` source files into canonical typed openWEPP management YAML. The
first target is YAML schema version 1 with `datver: ow-lanuse-1`.

## Rationale

M-T2Q makes `ow-lanuse-1` or later native management semantics the canonical
production input surface for new openWEPP physics. The legacy flat `.man`
format is difficult to author safely and openWEPP does not currently have a
Rust management flat-file writer. A crates.io-distributed openWEPP toolchain
therefore should migrate from frozen flat sources into typed canonical YAML
rather than investing in new native flat-file output.

The migration tool must not reintroduce the ambiguity that M-T2P and M-T2Q
removed. For pre-native legacy cropland, migration to `ow-lanuse-1` YAML
requires disturbed-class authority sufficient to embed explicit Lane D
`routing_coefficients`. There is no compatibility-only migration mode for
pre-native datvers.

## Dependencies

Closed dependency:

- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/`
  authorized the canonical YAML schema, typed Rust schema surface, and real
  openWEPP runtime consumer path before this implementation closed.

## Scope

In scope:

- Refine
  `docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md`.
- Add an implementation package plan for:
  - new Rust crate `crates/openwepp-landuse-migrate`;
  - library API for migration planning, validation, and execution;
  - binary `openwepp-landuse-migrate`;
  - embedded/versioned Disturbed route-coefficient table;
  - CLI `--args-for-migration-to` discovery;
  - CLI `--validate`;
  - migration from legacy cropland datvers to coefficient-complete
    `ow-lanuse-1` YAML;
  - default output naming that appends `.yaml` to the source filename, yielding
    `.man.yaml` for flat `.man` inputs;
  - flat `ow-lanuse-1` to canonical YAML bridge;
  - native YAML `ow-lanuse-N` to `latest` migrator scaffolding.
- Update package catalog and roadmap pointers.
- Record review, verification, gates, and implementation handoff.

Out of scope for implementation:

- No crates.io publish.
- No runtime hillslope-driver behavior change.
- No hidden coefficient inference from legacy fields.
- No sidecar-based coefficient authority.

Out of scope for this implementation unless the package is amended:

- No compatibility-only output mode for pre-native datvers.
- No native flat `.man` writer.
- No producer output extensions other than lowercase `.yaml`.
- No migration of unsupported legacy rangeland/forest/roads to new physics
  without explicit authority.
- No network or WEPPpy runtime dependency.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md`
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md`
- `docs/work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/package.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/artifacts/ow-lanuse-canonical-consensus-addendum.md`
- this package's `artifacts/required-reading-map.md`

Conditional before Rust implementation:

- `docs/specifications/science-contracts/AGENTS.md` if any contract is amended.
- `tests/AGENTS.md` before adding integration tests.
- `crates/openwepp-input-contract/src/parsers/management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py`
- `/home/workdir/wepppy/docs/adrs/ADR-0014-disturbed-openwepp-route-coefficients.md`

## Intended Write Set

Spec/docs:

- `docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md`
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/specifications/wepp-input-files/specs/README.md`
- `docs/specifications/wepp-input-files/README.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/**`

Implementation:

- `Cargo.toml`
- `crates/openwepp-landuse-migrate/**`
- `crates/openwepp-input-contract/Cargo.toml` for publishable parser-crate
  metadata required by the migration crate's crates.io path
- `crates/openwepp-management-schema/**` or the shared YAML schema owner
  ratified by the YAML authorization package
- focused integration tests under `tests/integration/**`.

Protected:

- Do not change hillslope runtime default behavior in this package.
- Do not alter Lane D routing numerics or groundwater/baseflow implementation.
- Do not add coefficient sidecars.
- Do not add a native flat management writer.

## Phase Plan

### Phase A - Scaffold And Spec Draft

1. Amend this package and spec for YAML-only output.
2. Ensure the YAML authorization package exists and is referenced.
3. Update roadmap/catalog/spec index pointers.
4. Record local scaffold verification.

### Phase B - Contract And Design Confirmation

1. Confirm the YAML authorization package has closed or record a HOLD.
2. Finalize class-map, `--args-file`, validation-report, and migration-report
   schemas.
3. Define Disturbed coefficient table version/checksum strategy.
4. Record crate layout and API design.

### Phase C - Rust Implementation

1. Add `crates/openwepp-landuse-migrate` with lib and bin.
2. Implement required-argument discovery.
3. Implement `--validate`.
4. Implement legacy cropland to `ow-lanuse-1` YAML migration with required
   disturbed-class authority.
5. Implement default output naming and lowercase `.yaml` producer enforcement.
6. Implement flat `ow-lanuse-1` to YAML bridge.
7. Implement native YAML `ow-lanuse-1` to `latest` pass-through migrator.
8. Add typed errors and no-output-on-failure behavior.

### Phase D - Tests And Crates.io Readiness

1. Add focused CLI/library tests and schema/runtime-consumer integration tests.
2. Verify package metadata, no external runtime dependency, and binary behavior
   outside a WEPPpy checkout.
3. Run required gates.

### Phase E - Review, Verification, Disposition

1. Complete dual review and dual verification.
2. Disposition all findings.
3. Record final gates and line-count governance.
4. Finalize worker handoff or close complete.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to read-only
review and verification subagents for scaffold/spec review, future
implementation review, and crates.io-readiness verification. Expected outputs
are package-local `artifacts/review-*.md` and
`artifacts/verification-*.md`. Write access is read-only unless a later prompt
explicitly assigns a bounded implementation write set.

Subagent requirement: REQUIRED for implementation review/verification when
available. Scaffold amendment may use local verification unless the operator
requests dispatch.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/spec-draft.md`
- `artifacts/implementation-plan.md`
- `artifacts/crate-design.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required for scaffold/spec draft:

- Draft CLI spec exists and names no-sidecar/no-compat-only policy.
- CLI spec names YAML-only output and `--validate`.
- Roadmap/catalog/spec index pointers updated.
- YAML authorization package exists and is referenced as a dependency.
- `markdown-doc lint` for touched docs.
- `git diff --check`.

Required for implementation closure:

- YAML authorization package closed or this package closes in HOLD.
- Finalize normative class-map, `--args-file`, validation-report,
  migration-report, and YAML output schemas in the spec or directly linked
  schema specs before claiming Rust closure.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Focused tests for CLI discovery, `--validate`, migration, fail-closed errors,
  YAML schema validation, real runtime YAML consumption, class-map conflict
  detection, partial maps, global-class admissibility, default `.man.yaml`
  naming, and producer rejection of non-lowercase-`.yaml` output extensions.
- Crates.io readiness check: package metadata, no WEPPpy/network runtime
  dependency, embedded coefficient table version/checksum.

## Exit Criteria

`SCAFFOLDED-SPEC-DRAFTED-AMENDED`:

- Work-package scaffold and active prompt exist.
- CLI specification draft exists and requires YAML-only output.
- `--validate` is specified.
- Roadmap/catalog/spec indexes point to the package/spec.
- YAML authorization package exists and is named as a dependency.
- Docs lint and diff hygiene pass.
- No Rust implementation claim is made.

`EXECUTED-COMPLETE`:

- Rust library and CLI are implemented.
- Legacy cropland migration to `ow-lanuse-1` requires disturbed-class authority
  and writes coefficient-complete native YAML.
- Producer output defaults to `.man.yaml` for flat `.man` inputs and emits only
  lowercase `.yaml`.
- Flat `ow-lanuse-1` to YAML bridge exists.
- Native YAML to latest migrator path exists.
- Real runtime consumer reads the YAML path and projects route coefficients.
- Required tests and closure gates pass.
- Review/verification findings are dispositioned.

`EXECUTED-HOLD-*`:

- Any contract, YAML authority, table authority, consumer-path, crates.io
  metadata, or validation blocker prevents implementation closure.

## Final Outcome

Executed complete. The Rust library/CLI, public spec, embedded Disturbed
coefficient table, focused tests, runtime consumer proof, package metadata
checks, reviews, verification, and closure gates are complete.
