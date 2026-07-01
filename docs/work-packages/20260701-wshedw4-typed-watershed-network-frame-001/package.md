# WSHED-W4 Typed Watershed Network Frame

Status: `QUEUED-HANDOFF-AUTHORED`

Date opened: `2026-07-01`

Package type: implementation package; watershed runtime architecture rung W4.

## Objective

Replace production watershed routing and publication reads/writes through
`WatershedWritebackSurface` with a typed `WatershedNetworkFrame` and
`WatershedPublicationFrame`. W4 must cut the real public
`openwepp-cli-watershed` path over to typed channel, impoundment, hillslope
contribution, routed-state, and publication fields while preserving the W2/W3
run-plan, worker-pool, pass-inventory, and fail-closed behavior.

## Rationale

W2 introduced the public serial supervisor and pass inventory. W3 added bounded
`--jobs N` fanout and proved worker-pool output identity on the committed
carnivorous-adobo fixture. The remaining routed stage still builds and consumes
`WatershedWritebackSurface` maps keyed by `BoundarySymbol`; W3 explicitly left
that as W4's consumer-path boundary. W4 is the typed routed-stage rewrite: the
production router should no longer populate, look up, or publish from symbol-map
state.

The package should be aggressive. Do not keep the old production runtime alive
by wrapping symbol maps in a new facade. If tests assert only obsolete map keys
or internal symbol-surface behavior, delete or migrate those tests with
explicit protected-coverage backfill rather than preserving the old surface.
W5 owns final old-runtime deletion, but W4 owns the production cutover away from
`WatershedWritebackSurface`.

## Included Scope

- Define typed routed-stage data structures:
  `WatershedNetworkFrame`, typed channel/impoundment control records,
  typed hillslope contribution records, routed channel/impoundment state, and
  `WatershedPublicationFrame`.
- Build the typed frame from current validated sources:
  parsed structure/channel/impoundment/slope inputs, validated
  `PassInventory`, topology validation, and runfile output contract.
- Replace production routing-loop reads and writes currently performed through
  `WatershedWritebackSurface`, `BoundarySymbol`, and `BoundaryValue` with typed
  frame fields.
- Replace production publication reads currently performed from
  `WatershedKernelExecutionReport`/writeback surfaces with typed publication
  frame operands for the protected watershed output files.
- Keep explicit compatibility projections only at named edge boundaries for
  replay, comparator, diagnostic, or temporary migration checks; these
  projections cannot be used by the production routing/publication claim.
- Add negative source checks proving no production routing-loop symbol lookup
  remains for the W4 claim.
- Add protected output identity tests or contract-governed delta disposition
  for routed-stage outputs.
- Record operand lineage, anti-alias validation, independent reconstruction,
  and closure/magnitude evidence for conservation-sensitive publication fields.
- Record consumer-path proof showing routing and publication consume the typed
  frame in the real public runner path.

## Excluded Scope

- No worker-pool or `--jobs` scheduling changes except as needed to preserve W3
  handoff into typed routing.
- No full deletion of obsolete old runtime files outside the production cutover;
  W5 owns full runtime deletion and deletion manifest.
- No large 1,000+ hillslope fixture adoption; W6 owns large-scaling fixtures.
- No erosion, sediment equation, impoundment equation, routing physics, or
  output schema changes for performance.
- No `NoEvent` semantic change unless implemented contract-first under
  canonical science-contract authority.
- No fixture climate/data normalization unless explicitly authorized in package
  amendment and recorded with checksum provenance.
- No producer-only, shadow-only, counter-only, or adapter-only closure.

## Intended Write Set

- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs` only for pass-inventory
  or frame handoff changes.
- `crates/openwepp-runner/src/lib.rs` if public exports change.
- `crates/openwepp-watershed-output/src/**` if publication helpers move to the
  typed publication frame.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for typed frame, protected output, and source-guard
  tests if needed.
- `tests/fixtures/watershed/carnivorous-adobo/**` only for committed metadata
  or manifest updates required by W4 gates.
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Any production edit outside this write set requires package amendment before
implementation.

## Science and Conservation Authority

W4 controls conservation-sensitive routed water/sediment publication surfaces.
Before production edits, execution must record an operand-lineage table for the
typed publication frame covering field name, units, normalization/denominator,
area or volume basis, source authority, and authoritative-vs-diagnostic status.

If implementation changes routing/publication semantics, aliases, guards,
units, or output meaning, amend canonical `SC-*` authority before production
code. At minimum, consult relevant watershed/routing contracts on demand:
`SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-RUNOFFPART-001`, `SC-SED-001`,
`SC-WATBAL-001`, and watershed infile contracts.

One-sided bounds and exact self-consistency checks are supporting sanity
evidence only. Closure requires independent reconstruction from produced
outputs plus a real closure or magnitude audit on committed fixtures.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `science_contract_reviewer` subagents.
`comparator_suite_runner` is authorized and required for heavy final closure
gates, protected-output comparator runs, and release-style fixture runs when
available; expected output is compact metrics plus log/artifact paths.
`rust_code_reviewer`, `rust_qa_reviewer`, and `science_contract_reviewer` are
authorized for read-only review and verification; expected outputs are compact
findings with file/path references and contract-authority disposition. Write
access is read-only for subagents, with parent disposition recorded in
`artifacts/review-disposition.md` and `artifacts/verification.md`.

## Phase Plan

1. Preparation and inventory:
   - read required authorities and this package;
   - inventory current `WatershedWritebackSurface` construction, routing reads,
     routing writes, and publication reads in the public watershed CLI and
     orchestrator crates;
   - classify each surface as production routing, production publication,
     compatibility edge, test-only, or obsolete-internal.
2. Operand lineage and contract gate:
   - author typed publication operand lineage before production edits;
   - decide whether the package is behavior-preserving or requires
     contract-governed deltas;
   - if semantics change, update canonical contracts and contract-derived tests
     before production code.
3. Typed frame implementation:
   - add typed frame/control/contribution/publication structures;
   - build the frame from parsed inputs and pass inventory;
   - migrate routing dispatch and publication consumers to typed fields;
   - keep any compatibility projection outside the production claim.
4. Test migration and source guards:
   - migrate protected tests to typed frame or public output surfaces;
   - delete obsolete map-key tests only with protected-coverage backfill;
   - add source guards proving no production routing-loop symbol lookup remains.
5. Output and conservation evidence:
   - run protected output identity tests on focused fixtures;
   - record contract-governed deltas if identity is not preserved;
   - run independent reconstruction and closure/magnitude audit for touched
     conservation-sensitive outputs.
6. Review and closure:
   - run focused local iteration gates;
   - run or delegate required final closure gates;
   - complete dual review, science/QA verification, line-count governance,
     consumer-path proof, and final disposition.

## Exit Criteria

- The real public `openwepp-cli-watershed` path builds and routes through
  `WatershedNetworkFrame`.
- Production routing loops do not read or write `WatershedWritebackSurface`,
  `BoundarySymbol`, or `BoundaryValue`.
- Production publication for protected watershed outputs consumes typed
  `WatershedPublicationFrame` operands.
- Any remaining `WatershedWritebackSurface` use is explicitly labeled as
  compatibility, replay, diagnostic, or obsolete-test code and is not used by
  the production W4 claim.
- Protected routed-stage outputs remain identity-equivalent on committed
  fixtures, or deltas are contract-governed with accepted review disposition.
- Conservation-sensitive outputs have operand lineage, anti-alias tests,
  independent reconstruction, and closure/magnitude evidence.
- Consumer-path proof names producer source, typed frame object, runner handoff,
  downstream routing/publication consumer, output/API surface, and negative
  proof that old symbol-map routing is not used.
- Source guards prove no production routing-loop symbol lookup remains.
- Focused tests and final closure gates pass, or the package closes
  `EXECUTED-HOLD` with a named blocker.

## Required Final Gates

Because W4 edits production Rust and routed publication behavior, final closure
must run and record:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. focused W4 typed-frame, source-guard, protected-output, and
   conservation/publication gates.

If a gate cannot run, the package must close `EXECUTED-HOLD` with the exact
blocker unless a canonical decision explicitly authorizes a narrower closure.

## Security and Safety

Do not introduce network dependencies, credential handling, silent dependency
fallbacks, broad path discovery, or shell interpolation. Preserve W2/W3
fail-closed behavior for invalid plans, child failures, pass inventory failures,
and domain violations. Do not canonicalize-and-proceed on physics/routing
domain violations unless a canonical `SC-*` contract explicitly authorizes
bounded normalization with units, thresholds, provenance, tests, and evidence.
