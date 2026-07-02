# WSHED-W6 Publication and Large-Watershed Scaling

Status: `QUEUED-HANDOFF-AUTHORED`

Date opened: `2026-07-02`

Package type: implementation, fixture-adoption, publication-contract, and
performance characterization package; watershed runtime architecture rung W6.

## Objective

Complete the watershed runtime performance arc after W5 by making publication
scale with typed watershed state and by adopting auditable larger watershed
fixtures. W6 must replace any remaining publication staging that is shaped like
a one-row compatibility seed with a typed publication/streaming path, then
record strict committed-fixture scaling evidence and a fresh same-scope
legacy/openWEPP comparison where runnable.

## Rationale

WSHED-W2 created the serial supervisor, WSHED-W3 added bounded worker-pool
fanout, WSHED-W4/W4DC01 moved routing onto `WatershedNetworkFrame`, and
WSHED-W5 deleted the old watershed request/writeback runtime. The next risk is
that publication and fixture coverage remain sized for tiny or 32-hillslope
cases. Large watersheds with over 1,000 hillslopes are common, and W6 is the
first package whose closure must prove the new runtime and publication path at
larger scale.

The performance answer must be measured on like-for-like surfaces. WSHEDPERF01
showed arboreal-dendrite openWEPP full chain was about `7.8x` slower than
legacy, while routed-stage-only timing was not comparable to legacy full
watershed timing. WSHED-W3 showed `--jobs 32` scaling on the committed
32-hillslope carnivorous-adobo fixture. W6 must update the record after W5 with
committed-fixture, sidecar-discovery-off timings and must not compare
discovery-on, route-only, or `/wc1`-only surfaces as if they were canonical
benchmark evidence.

## Dependencies

- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/`
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/`
- `tests/fixtures/watershed/carnivorous-adobo/`

## Included Scope

- Replace remaining watershed publication staging that converts typed
  publication into a compatibility-style row seed before writing outputs, if it
  is still present in the production public watershed path.
- Add a typed publication writer/streamer or equivalent direct typed writer so
  watershed outputs are produced from `WatershedPublicationFrame` or typed
  projection iterators without old runtime staging.
- Preserve the public output schema unless canonical contract authority
  requires a change.
- Record operand lineage for touched watershed output fields, including units,
  normalization basis, area/volume basis, source authority, and
  authoritative-vs-diagnostic status.
- Adopt at least one larger committed watershed fixture. Target fixture class is
  full `>=1000` hillslope coverage. User clarification during execution removed
  representative reduction as a closure path; if full fixture execution cannot
  close in W6, record an executed hold with the exact blocker.
- Ensure every fixture used as an acceptance gate is committed to this
  repository with provenance, README, topology summary, and checksum manifest.
  `/wc1`, scratch, or wepppy paths may be source substrates only.
- Run canonical strict-committed-fixture scaling evidence after W5, including
  at least carnivorous-adobo and the adopted full large fixture.
- Record fresh same-fixture legacy/openWEPP timing where runnable. If pinned
  legacy cannot run the committed fixture without an additional bridge, record
  the exact blocker and do not claim legacy-relative speedup for that fixture.
- Preserve `--jobs 1` deterministic output identity and `--jobs N` row/content
  identity across all required watershed output files.
- Update package artifacts, `docs/ROADMAP.md`, and
  `docs/work-packages/README.md`.

## Excluded Scope

- Do not change routing, impoundment, sediment, erosion, runoff-partition,
  water-balance, latest-event, or hillslope physics for performance.
- Do not add surrogate or proxy erosion/sediment physics to fill output gaps.
- Do not normalize fixture data unless explicitly recorded as fixture-only,
  contract-compatible normalization with checksum provenance.
- Do not make `/wc1`, scratch, or wepppy paths the persistent fixture or
  benchmark evidence.
- Do not reintroduce the deleted W5 watershed request/writeback runtime, public
  old-runtime selector, compatibility adapter, or shadow route.
- Do not default `--jobs` above `1`; ADR-0032 keeps CPU scaling explicit.

## Intended Write Set

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/src/watershed_wat.rs`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs` if totalwatsed3
  watershed publication is touched.
- `tests/integration/**` for fixture, publication, source-guard, conservation,
  and scaling contract tests.
- `tests/fixtures/watershed/**`
- `docs/specifications/science-contracts/**` only if execution discovers
  publication semantics, units, guards, or output authority must change.
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Any production edit outside this write set requires package amendment before
implementation.

## Science and Conservation Authority

W6 touches conservation-sensitive watershed publication. Before production
publication edits, execution must record operand lineage for touched outputs.
If implementation changes publication semantics, output schema, unit lineage,
normalization basis, domain guards, or process meaning, amend canonical `SC-*`
authority before production code and add contract-derived tests.

Comparator agreement is a flag, not a target. Legacy timing and output
comparison may identify investigation targets, but `OPENWEPP-DEFECTIVE`
requires independent correctness authority and like-for-like unit/surface
proof. W6 must not tune output values or physics to match legacy timing or
legacy magnitudes.

## Fixture and Benchmark Policy

Canonical W6 benchmark mode is `strict-committed-fixture` from ADR-0032:

- committed fixture inputs under `tests/fixtures/watershed/`;
- legacy sidecar discovery disabled for canonical openWEPP runs;
- timing scope, sidecar/input-discovery mode, job count, CPU inventory, binary
  provenance, fixture checksum, and output identity evidence recorded;
- `/wc1` and scratch paths recorded only as source provenance or contextual
  exploratory evidence.

W6 complete closure requires at least:

- carnivorous-adobo post-W5 scaling matrix, including `--jobs 1` and a high
  CPU-count `--jobs N` appropriate for the host;
- one adopted full `>=1000` hillslope committed fixture with topology summary
  and checksum manifest;
- output row/content identity between `--jobs 1` and `--jobs N`;
- fresh legacy/openWEPP comparison on any fixture where both surfaces are
  runnable and comparable, with non-comparable surfaces labeled honestly.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `science_contract_reviewer`, and `fixture_inventory_agent`
subagents. `comparator_suite_runner` is required for heavy full-closure gates,
release builds, scaling matrices, legacy/openWEPP timing comparisons, and
protected-output comparator runs when available. `fixture_inventory_agent` is
authorized for read-only discovery of candidate `/wc1` source substrates and
bounded package artifact updates for fixture inventory if the tool session
allows it. Review and verification subagents are read-only. Expected outputs
are compact metrics/findings plus log or artifact paths.

## Phase Plan

1. Preparation and inventory:
   - read required authority and package documents;
   - inventory current watershed publication call path from
     `WatershedPublicationFrame` to parquet outputs;
   - inventory current committed watershed fixtures and candidate larger source
     substrates;
   - record benchmark-surface taxonomy before running timings.
2. Operand lineage and contract gate:
   - record operand lineage for all touched watershed publication fields;
   - decide whether W6 is schema/semantics preserving;
   - amend contracts and add contract-derived tests first if semantics change.
3. Typed publication streaming:
   - implement direct typed writer/streamer or equivalent direct publication
     path;
   - remove production dependence on compatibility-shaped row seed staging when
     it exists;
   - preserve required parquet schemas and metadata.
4. Fixture adoption:
   - select a full `>=1000` hillslope source substrate or record an executed
     hold if no full substrate can be committed and run in W6;
   - commit fixture inputs/manifests under `tests/fixtures/watershed/`;
   - add fixture-contract tests proving gates read committed paths only.
5. Scaling and comparison evidence:
   - build release binaries;
   - run carnivorous-adobo post-W5 scaling matrix;
   - run full large fixture scaling matrix;
   - run same-fixture legacy/openWEPP comparison where runnable;
   - record output identity, CPU inventory, timing scope, and RSS.
6. Review and closure:
   - run focused publication/fixture/scaling gates and final Rust closure gates;
   - complete dual review, dual verification, line-count governance, gate
     results, worker handoff, and final disposition;
   - update roadmap and work-package index state after W6.

## Exit Criteria

- Production watershed publication consumes typed publication/projection state
  directly; no old runtime staging or compatibility-shaped row seed carries the
  W6 closure claim.
- Required watershed parquet outputs keep their documented schema or carry
  contract-governed schema migration evidence.
- Operand lineage and independent reconstruction are recorded for
  conservation-sensitive touched outputs.
- A full `>=1000` hillslope committed fixture exists under
  `tests/fixtures/watershed/`, or W6 records an executed hold with the named
  blocker preventing full fixture adoption/execution.
- Fixture manifests validate from committed files.
- `--jobs 1` and `--jobs N` outputs are row/content identical for required
  fixtures unless a contract-governed delta is accepted.
- Fresh performance evidence records post-W5 openWEPP scaling and any runnable
  same-fixture legacy comparison without cross-scope speedup claims.
- Full gates pass, or the package closes `EXECUTED-HOLD` with a named blocker
  outside the W6 envelope.

## Required Final Gates

Complete closure requires:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. focused W6 publication, source-guard, fixture-contract, output-identity,
   scaling, and legacy-comparison gates
6. fixture checksum manifest validation for every adopted committed fixture
7. scoped docs lint for touched package/index/fixture docs
8. `git diff --check`

## Security and Safety

Do not introduce network dependencies, credential handling, broad path
discovery beyond local fixture candidates, silent dependency fallbacks, or shell
interpolation. Preserve typed fail-closed behavior for invalid plans, child
failures, missing pass inventory, missing latest-event payloads, stale
generated artifacts, and watershed domain violations. Do not commit generated
heavy output caches as fixtures unless package review explicitly approves them
as bounded audit artifacts with owner and sunset plan.
