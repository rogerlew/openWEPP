# WSHED-W7 Sediment-Active Watershed Fixture and Publication Closure

Status: `queued`

Date opened: `2026-07-02`

Package type: implementation, fixture-adoption, publication-closure, and
conservation-evidence package; watershed runtime architecture follow-on W7.

## Objective

Close the W6 residual sediment coverage risk by adopting or generating a
committed full watershed fixture with actual nonzero sediment response from
hillslope pass artifacts, then proving the public watershed supervisor routes
and publishes that fixture through the typed watershed path with serial/parallel
output identity and independent sediment-publication reconstruction.

## Rationale

WSHED-W6 completed typed watershed publication and large-watershed scaling, but
its accepted fixtures published zero detachment/deposition and sediment yield.
Those zeros were pass-backed fixture values, not synthetic fills, so W6 was
correct to close. They do not provide strong coverage for nonzero sediment
publication, sediment aggregation, or sediment-sensitive output identity.

W7 is a hardening package, not another watershed runtime rewrite. It must keep
the W2-W6 architecture intact: typed run plan, bounded hillslope worker pool,
typed pass inventory, typed network frame, direct watershed dispatch, and typed
publication. Any sediment values used for acceptance must be produced by actual
hillslope/routing physics and carried through production publication. Surrogate
sediment fills, proxy equations, fixture-only fake pass values, or aliases from
unrelated output columns are not acceptable.

## Dependencies

- `docs/ROADMAP.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/package.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/review-disposition.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling-matrix-evidence.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/publication-operand-lineage.md`
- `tests/fixtures/watershed/carnivorous-adobo/`
- `tests/fixtures/watershed/onshore-xenophobia/`

## Included Scope

- Inventory committed watershed fixtures and local candidate source substrates
  for full watershed runs with actual nonzero sediment response.
- Adopt or generate one committed full watershed fixture whose produced pass
  artifacts contain nonzero sediment response in at least one routed/publication
  row. Full means the whole watershed fixture selected for acceptance, not a
  subset, representative slice, `/wc1`, scratch, or partial channel cut.
- Preserve fixture provenance, topology summary, source checksum manifest, and
  enough replay instructions for future auditability.
- Run the public watershed supervisor on the accepted fixture with `--jobs 1`
  and an appropriate parallel `--jobs N`.
- Prove all required watershed parquet outputs are schema/row identical between
  serial and parallel runs unless a contract-governed delta is accepted.
- Record operand lineage for touched sediment/publication fields, including
  units, normalization basis, area or volume basis, source authority, and
  authoritative-vs-diagnostic status.
- Reconstruct sediment-sensitive publication operands independently from
  produced pass/output artifacts and reject plausible aliases or tautological
  producer-only checks.
- Add focused fixture/source-guard tests needed to keep the public path on the
  typed watershed frame and to prevent regression to zero-only sediment
  coverage.
- Update package artifacts, `docs/ROADMAP.md`, and
  `docs/work-packages/README.md` at final disposition.

## Excluded Scope

- Do not implement W8 channel-balance operand authority in this package except
  for recording that unavailable channel-balance operands remain null.
- Do not implement W9 canonical `NoEvent` semantics unless execution discovers
  that the selected valid sediment-active fixture cannot run without that
  authority; in that case hold with the exact blocker.
- Do not implement W10 `chan.inp` absence/default authority unless the selected
  valid sediment-active fixture is blocked solely by that authority; in that
  case hold with the exact blocker.
- Do not perform watershed CQR maintenance or broad behavior-preserving splits;
  that belongs to the watershed CQR burndown ExecPlan.
- Do not change hillslope, erosion, sediment, routing, impoundment,
  runoff-partition, water-balance, or publication physics for convenience.
- Do not add surrogate, provisional, proxy, empirical stand-in, heuristic, or
  fixture-only sediment physics.
- Do not make local `/wc1`, scratch, or wepppy paths persistent acceptance
  evidence. They may be source substrates only, with committed fixture outputs
  and manifests carrying the gate.

## Intended Write Set

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for fixture, source-guard, publication, and
  output-identity tests.
- `tests/fixtures/watershed/**`
- `docs/specifications/science-contracts/**` only if execution changes
  canonical sediment, publication, guard, unit, or output semantics.
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Any production edit outside this write set requires package amendment before
implementation.

## Science and Conservation Authority

W7 is conservation-sensitive publication work. Before production edits, record
an operand-lineage table for every touched sediment or publication field. The
table must state field name, units, normalization or denominator, area/volume
basis, source authority, and whether the operand is authoritative or
diagnostic.

If W7 changes publication semantics, output schema, unit lineage, process
meaning, guards, or sediment/routing physics, amend canonical `SC-*` authority
and add contract-derived tests before production code changes.

Comparator agreement is a flag, not a target. Legacy outputs may identify
investigation targets, but `OPENWEPP-DEFECTIVE` requires independent
correctness authority and like-for-like unit/surface proof.

## Fixture and Publication Policy

Complete closure requires a committed full watershed acceptance fixture with
actual nonzero sediment response. The fixture may be:

- an existing committed watershed fixture proven to produce nonzero sediment
  after a current rebuild;
- a newly committed watershed fixture generated from local source substrates;
  or
- an amended committed fixture only when the amendment is input-provenance
  preserving and does not fake pass/output sediment values.

The package may not close on a subset, a representative slice, manually edited
pass sediment values, route-only evidence, or fixture values filled outside the
production hillslope/pass/routing path.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `fixture_inventory_agent`,
`rust_code_reviewer`, `rust_qa_reviewer`, and `science_contract_reviewer`
subagents. `comparator_suite_runner` is required for heavy release builds,
full watershed serial/parallel runs, output-identity comparison, protected
output comparator runs, and final full closure gates when available.
`fixture_inventory_agent` is authorized for read-only discovery of local
candidate source substrates and bounded package-artifact updates for fixture
inventory if the tool session allows it. Review and verification subagents are
read-only. Expected outputs are compact metrics/findings plus log or artifact
paths.

## Phase Plan

1. Preparation and authority map:
   - read required authority, package, and W6 residual-risk artifacts;
   - update `artifacts/required-reading-map.md` if scope changes;
   - inventory current public watershed publication and sediment call paths.
2. Sediment-active fixture inventory:
   - rebuild or inspect committed fixtures for nonzero sediment response;
   - inspect local candidate full watershed source substrates if needed;
   - record source provenance, topology, sediment signal, and rejection reasons.
3. Operand lineage and contract gate:
   - record sediment/publication operand lineage before production edits;
   - decide whether W7 is schema/semantics preserving;
   - amend contracts and add contract-derived tests first if semantics change.
4. Fixture adoption and focused tests:
   - commit the selected full watershed fixture, or prove an existing committed
     fixture is the W7 acceptance fixture;
   - update checksum manifests and README provenance;
   - add focused tests/source guards for nonzero sediment coverage and typed
     public publication consumption.
5. Full watershed execution and reconstruction:
   - build required binaries;
   - run the accepted full fixture with `--jobs 1`;
   - run the accepted full fixture with an appropriate parallel `--jobs N`;
   - compare all required parquet schemas and rows;
   - independently reconstruct sediment-sensitive publication operands and
     record rejected alias formulas.
6. Review and closure:
   - run focused W7 gates and final Rust closure gates;
   - complete dual review, finding disposition, dual verification, line-count
     governance, gate results, worker handoff, and final disposition;
   - update roadmap and work-package index state after W7 closes.

## Exit Criteria

- A committed full watershed acceptance fixture exists and produces actual
  nonzero sediment response from production-generated pass artifacts.
- Public watershed execution for that fixture uses the W2-W6 typed runtime and
  typed publication path.
- `--jobs 1` and `--jobs N` outputs are schema/row identical across all
  required watershed parquet outputs unless a contract-governed delta is
  accepted.
- Sediment-sensitive publication fields have operand lineage and independent
  reconstruction evidence.
- Evidence rejects at least the plausible wrong formulas/aliases identified in
  the operand-lineage artifact, including zero-fill, adjacent diagnostic alias,
  inflow/outflow alias when applicable, and producer-self-consistency only.
- No surrogate/proxy/heuristic sediment or routing physics is introduced.
- Full gates pass, or the package closes `EXECUTED-HOLD` with a named blocker
  outside the W7 envelope.

## Required Final Gates

Complete closure requires:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. focused W7 fixture, source-guard, output-identity, publication, and
   conservation-reconstruction gates
6. fixture checksum manifest validation for every accepted or newly adopted
   fixture
7. scoped docs lint for touched package/index/fixture docs
8. `git diff --check`

## Security and Safety

Do not introduce network dependencies, credential handling, broad unbounded
path discovery, silent dependency fallbacks, or shell interpolation. Preserve
typed fail-closed behavior for invalid run plans, child failures, missing pass
inventory, missing latest-event payloads, stale generated artifacts, malformed
fixture manifests, non-finite sediment operands, and watershed domain
violations.
