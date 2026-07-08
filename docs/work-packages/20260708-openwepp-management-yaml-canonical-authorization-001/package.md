# openWEPP Management YAML Canonical Authorization

Status: `QUEUED`
Package ID: `20260708-openwepp-management-yaml-canonical-authorization-001`
Owner: Codex
Scaffold date: `2026-07-08`
Evidence mode: `Static scaffold; no Rust implementation executed`

## Objective

Authorize and implement the canonical producer-side openWEPP management YAML
surface with typed schemas for `ow-lanuse-1` and later native datvers.

The package establishes management YAML as a first-class native input format,
not a sidecar, and proves that the real openWEPP runtime consumer reads the YAML
path for Lane D/new-physics eligibility and route-coefficient projection.

## Rationale

Legacy WEPP management flat files are difficult to author safely and are a poor
long-term producer format. openWEPP should freeze the flat management parser as
a source/compatibility reader, then move native producer output to a typed YAML
schema that can be validated, versioned, distributed through crates.io, and
consumed directly by openWEPP.

This package is the authority/consumer prerequisite for
`20260708-landuse-migration-cli-spec-implementation-001`, which should emit
canonical YAML instead of native flat `.man` files.

## Scope

In scope:

- Finalize `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`.
- Author `SC-INFILE-MANAGEMENT-YAML-001` or explicitly amend the existing
  management input contract if review decides a sibling contract is not needed.
- Amend `docs/contracts/openwepp-management-lanuse-authority-contract.md` and
  `SC-OFEROUTE-001` only as needed to recognize YAML as canonical native
  producer authority for `ow-lanuse-1+`.
- Add a shared typed Rust schema/parser/validator surface for canonical
  management YAML.
- Ratify the file-extension policy: producers emit lowercase `.yaml` only,
  default migrated flat sources to `.man.yaml` naming when applicable, and
  consumers accept `.yaml`, `.YAML`, `.yml`, and `.YML`.
- Wire the real openWEPP runtime consumer to read management YAML and project
  typed route coefficients into the existing PL schedule surfaces.
- Add validation and fail-closed tests for missing, partial, mixed, or invalid
  route-coefficient authority.
- Update spec indexes, input-surface registry, roadmap, and work-package
  catalog.

Out of scope:

- No migration CLI implementation; that is owned by
  `20260708-landuse-migration-cli-spec-implementation-001`.
- No native flat `.man` writer.
- No sidecar-based coefficient authority.
- No legacy-field coefficient inference.
- No new routing numerics or groundwater/baseflow physics.
- No YAML support for unsupported legacy rangeland/forest/roads unless
  separately authorized by contract.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- this package's `artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md` before authoring
  a new `SC-INFILE-MANAGEMENT-YAML-001` contract.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before amending kernel-facing obligations.
- `crates/openwepp-input-contract/src/parsers/management.rs` when mapping the
  flat parser model into YAML schema types.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
  before runtime consumer edits.
- `tests/integration/infile_management_parser_contract.rs` before integration
  test additions.

## Intended Write Set

Scaffold/spec:

- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/specifications/wepp-input-files/specs/README.md`
- `docs/specifications/wepp-input-files/README.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/**`

Future authority/implementation:

- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `Cargo.toml`
- `crates/openwepp-management-schema/**` or an explicitly dispositioned
  equivalent shared schema module
- `crates/openwepp-input-contract/**` if the schema lives there
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- runfile or runner input-binding files required for the real consumer path
- focused integration tests under `tests/integration/**`

Protected:

- Do not add a native flat `.man` writer.
- Do not add optional sidecars for YAML authority.
- Do not allow YAML emission to close production readiness unless the runtime
  consumes YAML directly.
- Do not infer route coefficients from legacy fields.
- Do not emit `.yml`, `.YML`, or `.YAML` from openWEPP producer tools.

## Phase Plan

### Phase A - Scaffold And Draft Spec

1. Create this work-package scaffold and execution prompt.
2. Add or refine `management-yaml.spec.md`.
3. Register the planned YAML input surface.
4. Update roadmap and package catalog.

### Phase B - Contract Authorization

1. Decide whether YAML gets a sibling `SC-INFILE-MANAGEMENT-YAML-001` contract or
   an explicit amendment to `SC-INFILE-MANAGEMENT-001`.
2. Author/amend the contract before production code.
3. Amend management-lanuse and `SC-OFEROUTE-001` authority only where needed to
   recognize YAML as a canonical native producer surface.
4. Record a source-vs-target policy that freezes flat `.man` as source-only for
   native producer evolution.

### Phase C - Typed Schema And Validator

1. Confirm crate ownership using `artifacts/crate-ownership-assessment.md`.
2. Add the shared Rust schema crate or module.
3. Define strict serde types for YAML schema version 1.
4. Add typed validation errors with field paths.
5. Add generated or checked schema artifacts if selected.
6. Add unit tests for schema success/failure and route-coefficient completeness.
7. Add extension-policy tests: producer emits lowercase `.yaml` only; consumer
   accepts `.yaml`, `.YAML`, `.yml`, and `.YML`.

### Phase D - Runtime Consumer Path

1. Add runfile/runner binding for management YAML input.
2. Wire the hillslope runtime management intake to parse the shared YAML schema.
3. Project YAML route coefficients into the same PL schedule surfaces used by
   flat `ow-lanuse-1`.
4. Prove old flat compatibility paths, reports, and optional sidecars are not
   carrying the YAML production-readiness claim.

### Phase E - Review, Verification, And Closure

1. Complete dual review and dual verification.
2. Run Rust and docs closure gates.
3. Record consumer-path proof, line-count governance, and final disposition.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to read-only
review and verification subagents for contract review, schema review, runtime
consumer-path verification, and crates.io/schema-readiness verification.
Expected outputs are package-local `artifacts/review-*.md` and
`artifacts/verification-*.md`. Write access is read-only unless a later prompt
explicitly assigns a bounded implementation write set.

Subagent requirement: REQUIRED for implementation review/verification when
available. Scaffold authoring may use local verification unless the operator
requests dispatch.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/schema-outline.md`
- `artifacts/authority-plan.md`
- `artifacts/crate-ownership-assessment.md`
- `artifacts/consumer-path-plan.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required for scaffold:

- Package scaffold and active prompt exist.
- Draft `management-yaml.spec.md` exists and is indexed.
- Input surface registry contains `infile-management-yaml` as planned.
- Roadmap/catalog pointers added.
- `markdown-doc lint` for touched docs.
- `git diff --check`.

Required for implementation closure:

- Contract-first authorization completed before code.
- Typed Rust schema/parser/validator exists.
- Crate ownership is dispositioned before implementation, with a publishable
  shared schema crate preferred unless review records a stronger local reason.
- YAML schema validation rejects unknown schema versions, missing required
  fields, missing route coefficients, invalid coefficient domains, and mixed
  authority.
- Producer tests prove emitted files use lowercase `.yaml`, default flat
  management output naming derives `.man.yaml`, and explicit `.yml`/uppercase
  output requests fail closed.
- Consumer tests prove `.yaml`, `.YAML`, `.yml`, and `.YML` are accepted for
  input and still require schema validation.
- Runtime consumer reads YAML directly and projects route coefficients into PL
  schedule surfaces.
- Consumer-path proof names producer, YAML document, parser/schema type,
  runtime intake call site, PL schedule projection, and negative proof that the
  original `.man`, optional reports, or sidecars are not carrying the claim.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `markdown-doc lint` for touched docs.
- `git diff --check`

## Exit Criteria

`SCAFFOLDED-AUTHORIZATION-PACKAGE`:

- Package scaffold and active prompt exist.
- Draft YAML spec and registry/index pointers exist.
- Roadmap/catalog point to the package.
- Docs lint and diff hygiene pass.
- No Rust implementation claim is made.

`EXECUTED-COMPLETE`:

- Canonical YAML authority is ratified.
- Typed schema and validator are implemented.
- Runtime consumer reads YAML directly.
- Lane D route-coefficient projection from YAML is proven.
- Required gates pass and review/verification findings are dispositioned.

`EXECUTED-HOLD-*`:

- Contract, schema, runtime consumer, or validation authority blocks closure.

## Final Outcome

Queued scaffold. Authorization and implementation have not started.
