# Assurance Editorial Normalization And Fast Gate

Status: EXECUTED-COMPLETE

Package ID: `20260716-assurance-editorial-fast-path-001`

Frozen base: `25bcb17f4a62924976a19381e974a36612ed4845`

This ExecPlan is maintained under `docs/codex_exec_plans.md`. The `Progress`,
`Surprises And Discoveries`, `Decision Log`, and `Outcomes And Retrospective`
sections are living execution records.

## Purpose And Big Picture

A DRAFT assurance report should be inexpensive to edit without weakening the
content identities that make it auditable. After this package, an author can
check or apply American-English normalization to one report with a single
command. The tool will use the canonical `uk2us` converter, update the exact
manuscript and supplement bytes, mechanically rebind every affected agent
packet, report descriptor, and catalog digest, validate the resulting source,
and either commit the complete transaction or restore the original bytes. A
cleanup error after the replacement generation validates is reported as a
distinct committed-cleanup condition rather than risking restoration from a
partially removed old generation.

The package also creates a proportional editorial gate. A lexical-only DRAFT
normalization runs focused assurance tests and deterministic build/check work;
it does not require dual scientific review or the full workspace gate. Any
change outside the converter-produced prose diff, any non-DRAFT lifecycle, or
any existing source drift fails closed and returns to the ordinary material-
change workflow. Content identities remain content identities: the fast path
removes manual bookkeeping, not review accountability.

## Progress

- [x] (2026-07-16) Read governing assurance, work-package, crate, test, and
  documentation instructions; inspect the existing hash cascade and flaky
  rollback test.
- [x] (2026-07-16) Scaffold and authorize this work package at the frozen base.
- [x] (2026-07-16) Implement transactional `normalize` check/apply behavior and
  receipt.
- [x] (2026-07-16) Add focused normalization contracts and deterministic assembly fault
  injection; delete the timing/polling rollback test mechanism.
- [x] (2026-07-16) Add the `assurance-editorial` nextest profile and document
  the fast path.
- [x] (2026-07-16) Run focused and heavy closure, fresh adjudicated CRAP, dual
  implementation review, and finding disposition; preserve and remediate the
  first heavy CRAP failure without waiver.
- [x] (2026-07-16) Run dual terminal verification over the closed candidate and
  evidence; both independent verifiers returned `PASS`.
- [x] (2026-07-16) Record final disposition, remove the package from the
  prospective roadmap, and catalog the completed package.

## Surprises And Discoveries

- Observation: the ASSURE-05 spelling edit touched two Markdown sources but
  required manual digest propagation through `agent-assistance-packet.json`,
  `report.yaml`, and `catalog.yaml`.
  Evidence: the active descriptor binds the packet twice and the catalog binds
  the descriptor.
- Observation: the post-install rollback contract relies on an 8 MiB padding
  dependency, a polling thread, and a 60-second deadline to observe a transient
  `.previous` directory.
  Evidence: `source_drift_after_install_rolls_back_prior_selected_bytes` in
  `tests/integration/assurance_v2_assembly_contract.rs`.
- Observation: initial independent review found that the first transaction
  draft did not bind the packet structure, full v2 tree, permission modes, or
  post-commit cleanup state tightly enough.
  Evidence: `artifacts/review-a.md`; accepted remediation now snapshots the
  complete tree, validates exact packet outputs even on no-op, preserves modes,
  and distinguishes rollback from committed cleanup.
- Observation: the first fresh adjudicated-CRAP closure found two actionable
  orchestration functions at `31.75489881112413` and
  `30.000000000000018` despite all behavior gates passing.
  Evidence: `artifacts/heavy-gate-runner.md` and the first-run evidence under
  `validation-evidence/adjudicated-crap/`.

## Decision Log

- Decision: preserve SHA-256 identity changes and automate their propagation.
  Rationale: lexical changes really do create new reviewed bytes, but humans and
  agents should not hand-edit derived digests.
  Date/Author: 2026-07-16 / Codex.
- Decision: permit normalization only for `DRAFT` sources and require exact
  `uk2us` output.
  Rationale: approved or in-review roots need the existing review-renewal
  process; the fast path must not become a semantic-change classifier.
  Date/Author: 2026-07-16 / Codex.
- Decision: keep `validate`, `plan`, `build`, and `check` shell-free; invoking
  `uk2us` is confined to the explicit maintenance command.
  Rationale: ordinary deterministic assurance operations retain the source/build
  contract while normalization remains an author-requested mutation.
  Date/Author: 2026-07-16 / Codex.
- Decision: replace timing coordination with an explicit assembly transaction
  fault point used only by contract tests.
  Rationale: rollback correctness should not depend on CPU speed or observing a
  transient filesystem name.
  Date/Author: 2026-07-16 / Codex.
- Decision: amend the write set to include `v2/confined.rs` during accepted
  review remediation.
  Rationale: whole-tree exchange must preserve exact permission modes through
  descriptor-relative, no-follow operations; the existing confinement
  capability is the correct ownership seam.
  Date/Author: 2026-07-16 / Codex.
- Decision: distinguish rollback-capable transaction failures from cleanup
  failure after the replacement generation has validated.
  Rationale: restoring a partly removed old generation would corrupt a valid
  commit; the typed committed-cleanup result preserves the new generation and
  makes retained recovery state explicit.
  Date/Author: 2026-07-16 / Codex.
- Decision: decompose normalization preparation, application, candidate
  rebinding, and tree cloning rather than waive either CRAP result.
  Rationale: the threshold is a closure gate for touched production functions,
  including floating-point values that render as exactly 30.
  Date/Author: 2026-07-16 / Codex.

## Context And Orientation

`crates/openwepp-assurance/src/v2.rs` loads typed report sources and validates
their content identities. `v2/assembly.rs` renders current sources into an
explicit disposable staging root. `cli.rs` exposes the maintenance commands.
The current report's manuscript and supplement are bound by SHA-256 in
`report.yaml`; its disclosed agent-assistance packet also binds the draft
outputs, and `catalog.yaml` binds `report.yaml`. A spelling edit therefore
changes a small dependency graph even though its scientific meaning does not.

The new normalization operation is not a renderer and does not generate prose.
It sends only the report's authored Markdown bytes to the installed `uk2us`
executable, accepts only that returned byte sequence, and records old/new
digests in a deterministic JSON receipt. “Atomic” means readers observe either
all old source files or all new source files; a failed validation restores the
old bytes. The transaction preserves file and directory modes and syncs the
parent directory around exchange and cleanup for crash durability.

## Authority And Constraints

Binding authority is ADR-0038, the v2 source/build contract, the scientific
model-evaluation report standard, and the active lifecycle/review-lock rules.
This package changes editorial maintenance mechanics, not report science,
claims, results, lifecycle authority, publication approval, or application
fitness. It does not modify kernel physics or science contracts.

This package explicitly authorizes subagent spawning/delegation and requires:

- two read-only independent implementation reviewers over the final diff;
- one delegated heavy-gate runner for full workspace gates and fresh CRAP; and
- two read-only terminal verifiers after accepted findings are fixed.

Compact outputs belong under this package's `artifacts/` directory. Reviewers
and verifiers may write only their assigned package artifact. The heavy runner
may write logs/artifacts under this package and ordinary ignored build outputs;
it must not edit production source.

## Declared Write Set

- `crates/openwepp-assurance/src/{cli,error,lib,v2}.rs`
- `crates/openwepp-assurance/src/v2/{assembly,confined,normalization}.rs`
- `tests/integration/assurance_v2_{assembly,normalization}_contract.rs`
- `Cargo.toml`
- `.config/nextest.toml`
- `assurance/v2/README.md`
- `docs/governance/scientific-assurance-v2-source-build-contract.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260716-assurance-editorial-fast-path-001/**`

The current report, its evidence, `usersum`, publication snapshots, release
trees, vendor trees, schemas, principals, kernels, and science contracts are
read-only. Integration tests may copy and mutate report fixtures only in
temporary directories.

## Plan Of Work

First add a normalization module whose public repository methods check or apply
American-English normalization to a named DRAFT report. It must validate the
source before mutation, call `uk2us` with explicit standard-input bytes, reject
nonzero exit, unexpected output encoding, non-DRAFT lifecycle, unsupported
language, and missing identity edges, and compute the complete new byte set in
memory. It then updates packet, descriptor, and catalog digests without
reformatting unrelated YAML, installs the files transactionally, reopens and
validates the repository, and emits a deterministic receipt. `--check` is
read-only and returns drift when normalization would change bytes; `--apply`
performs the transaction. Exactly one mode is required.

Next add integration contracts for no-op checking, detected British spelling,
successful rebinding, idempotence, non-DRAFT refusal, missing converter/error
handling through a controllable executable path, rollback, and proof that no
claim/result/lifecycle fields changed. Refactor assembly transaction execution
to admit an explicit test-only post-install fault, then replace the polling
rollback test with deterministic injection and remove the padding/deadline.

Finally add an `assurance-editorial` nextest profile selecting the focused v2
source, planner, assembly, normalization, and report contracts. Document the
workflow as `draft -> normalize check/apply -> validate -> focused build/check
-> review freeze`. State clearly that the profile is sufficient only for an
exact converter-produced DRAFT prose change; all mixed, material, lifecycle,
schema, builder, or authority changes use the ordinary package gates.

## Validation And Acceptance

From `/home/workdir/openWEPP`, acceptance requires:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile assurance-editorial
    cargo run --quiet -p openwepp-assurance -- normalize --report linear-groundwater-reservoir-recurrence --language en-US --check
    cargo run --quiet -p openwepp-assurance -- validate --report linear-groundwater-reservoir-recurrence

The named normalization check must pass without modifying the already
normalized production DRAFT. A disposable British-spelling fixture must fail
`--check`, pass `--apply`, contain the expected American spelling, carry current packet,
descriptor, and catalog hashes, validate, build, check, and produce the same
receipt on an equivalent input. A forced post-install assembly error must
restore prior selected staging bytes without threads, sleeps, polling, padding,
or wall-clock deadlines.

Package closure also requires the root implementation gates:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check

Run the repository's adjudicated CRAP closure procedure against the frozen base
for the exact touched Rust/test manifest. Every touched production function
must be at or below CRAP 30; record raw, adjudicated, and actionable counts.
Record `.rs` line counts: 2000+ is WARN with decomposition rationale and split
intent; 3000+ blocks closure unless an approved generated/fixture exception has
an owner and sunset. Run `uk2us` preview/apply as appropriate and
`markdown-doc lint` for every changed Markdown path, then `git diff --check`.

Every review finding must be dispositioned as `accepted`, `rejected`,
`deferred`, or `follow-up` with rationale. Accepted findings are fixed and
reverified. No current-scope evidence may be deferred to a later package.

## Idempotence And Recovery

`--check` is read-only. Repeating `--apply` after a successful normalization is
a no-op with a receipt that reports no changed files. Before install, all new
bytes are computed and validated structurally. During install, original bytes
are retained until every replacement succeeds; any command, write, rename, or
post-install validation error restores them. Parent directory entries are
synced around exchange, restoration, and cleanup. Once the new generation has
validated, cleanup failure leaves it active, returns a typed committed-cleanup
error containing the committed receipt, and retains any remaining old-generation
directory for explicit operator disposition. Retained recovery state blocks
all later normalization operations. Recovery artifacts must remain confined to
the source repository and be removed after successful cleanup or recovery.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/gate-results.md`
- `artifacts/review-a.md`
- `artifacts/review-b.md`
- `artifacts/finding-disposition.md`
- `artifacts/heavy-gate-runner.md`
- `artifacts/terminal-verification-a.md`
- `artifacts/terminal-verification-b.md`
- `artifacts/final-disposition.md`

## Outcomes And Retrospective

Implemented a single-command, canonical `uk2us` check/apply transaction for one
pre-review DRAFT report. It preserves full-tree bytes and permission modes,
rebinds only the declared packet/report/catalog identity chain, validates the
installed generation, distinguishes rollback from committed cleanup, emits a
deterministic receipt, and blocks unresolved recovery state.

The proportional `assurance-editorial` profile selects 65 tests and normally
completes in about 10 seconds. It is authorized only when the normalizer
produced the complete prose/digest diff; mixed, scientific, lifecycle,
authority, builder, or publication changes retain ordinary full gates. The
former polling/thread/8 MiB assembly rollback test is now a deterministic
private fault contract.

Two implementation-review HOLD cycles, a scheduling-sensitive test-fixture
failure, and the first heavy CRAP failure were accepted and corrected in scope.
Final full Nextest passed 2,063/2,063. Fresh adjudicated CRAP passed at 2 raw / 2
adjudicated / 0 actionable, with maximum touched CRAP exactly 30.0 and maximum
normalization CRAP `15.101256515775034`. Both terminal verifiers returned
`PASS`; protected production report bytes remain unchanged.
