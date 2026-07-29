# Gate-Planner Demotion Readiness Audit — 2026-07-27

Status: Final
Last updated: 2026-07-27
Evidence mode: Static
Scope: Independent verification of the Order-0 planning artifacts in
`docs/work-packages/20260727-gate-planner-advisory-linter-roadmap-001/` against
the actual authority surface of `crates/openwepp-gate-planner`,
`tools/local_ci/testgate.py`, and `gate-policy/v1`. Covers whether the planned
severance is complete and whether anything currently in flight is stranded by
it. Does **not** review ADR-0043's prose, the target linter interface, or the
friction metrics, and does **not** execute any gate, planner, or CAL command.

## Purpose

The Order-0 package declares the gate planner's demotion to a read-only
advisory linter and carries an open checklist item, "Obtain three independent
reviews and disposition every finding." This audit is one such independent
read. It answers a single question: does the Order-0 plan account for
everything that actually holds authority today, and does anything break or
strand that the plan has not named?

## Method

Static only. No `cargo` command, no planner invocation, no CAL command, and no
gate run was executed in this session.

- Read every artifact under
  `docs/work-packages/20260727-gate-planner-advisory-linter-roadmap-001/artifacts/`
  (13 files) plus `docs/work-packages/gate-planner-advisory-linter-roadmap.md`
  and the `docs/ROADMAP.md` Prospective Redirect.
- Enumerated the planner's write, subprocess, lifecycle-mutation, and
  fail-closed surfaces by reading `main.rs`, `pre_heavy.rs`, `executor.rs`,
  `publication.rs`, `external_dag.rs`, `external_dag/custody.rs`, `resume.rs`,
  and `tools/local_ci/testgate.py`.
- Grepped for consumers of planner output across `crates/`, `tests/`, `tools/`,
  and `.github/workflows/`.
- Read `Status:` lines from every `docs/work-packages/2026072*/package.md`
  dated 2026-07-24 onward.
- Spot-verified four load-bearing claims directly rather than by inference:
  the CAL-04B installer path, the gate-definitions schema constraint, the
  strategy document's self-protection clause, and the policy digest binding.

## Findings

### 1. The severance plan is sound and the largest hazard is already handled

The Order-0 migration map correctly classifies `publication.rs` and
`external_dag*.rs` as `MIGRATE` **then** `DELETE`, not `DELETE`, and makes
deletion conditional on a proven replacement owner. This is the correct
ordering and it forecloses the failure I expected to report.

Verified: CAL-04B's scientific outputs have exactly one installer into the
repository. Ten of its tooling scripts rebind their artifact root outside the
repo — `synthetic-gsi.py:58`, `observe.py:429`, `summarize.py:179`, and
siblings all resolve
`execution_root.parent / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"`
— and `tools/publish-results.py:44-54` is the only path back in, shelling
`openwepp-gate-plan publish-external-results`. A repo-wide grep for
`publish-external-results` returns exactly two hits: that script and
`crates/openwepp-gate-planner/src/main.rs:165`. Deleting `publication.rs`
without a replacement would leave CAL-04B unable to deliver its declared
artifacts.

The migration map's "Harvard Replacement Boundary" already states this
constraint with seven testable properties, and property 7 (the holdout process
must have no calibration-output write capability on any path, "including
parent, child, cleanup, recovery, and publication paths") is precisely the
right invariant. The instruction that Order 3 "must not link the current crate
as a library" is also correct: linking would drag execution, custody, and
publication back inside the advisory trust boundary.

No finding. Recorded because it is the plan's load-bearing decision and it
verifies.

### 2. Three registered test targets assert planner source text and will fail on any Order-1 deletion

Not addressed in any Order-0 artifact; grep for `align_authority`,
`ci_executor_contract`, `qualification.py`, and `literal` across the artifacts
directory returns no relevant hit.

| Guard | Coupling | Effect of Order-1 |
|---|---|---|
| `tests/integration/testgate_ci_executor_contract.rs:259-341` | Asserts ~40 literal substrings in planner sources, including `Command::new(program)` (`:264`), `GATE-EXEC-HEAVY-REQUIRES-AUDIT` (`:278`), `trusted_transition_command(` (`:296`), `no_open_tooling_defect_at_head` (`:317`). Also asserts an ordering property at `:302-308` requiring the final context recheck to precede `let execution = execute_nodes_for(` — which only holds while the planner executes. | Fails on removal of execution or transition authority |
| `tools/local_ci/testgate_qualification.py:69-79` | Reads `main.rs` and requires the literal tokens `validate-package-chain`, `package_authority_chain_id`, `pre-heavy-audit`, `"--stage", "transition"`, `verify_receipt` | Emits `result: "FAIL"` when any authoritative command is deleted |
| `tests/integration/testgate_align_authority_contract.rs:7-90` | Pins 12 `(name, schema, invalid-fixture, $id, schema_version)` tuples and asserts every schema retains a valid/invalid fixture pair | Fails when `gate-policy/v1` schemas or fixtures are retired |

Both `.rs` guards are registered integration targets in `Cargo.toml:454-459`.
Because they assert *source text* rather than behavior, they cannot be
satisfied by a correct advisory reimplementation — they must be amended or
deleted in the same change that removes the authority. If they are discovered
during Order 1 rather than planned for, they convert a deletion step into an
unplanned test-repair campaign, which is the failure mode the roadmap's
stop-loss rules exist to prevent.

### 3. The demotion is gated by the regime it demotes (circularity)

`docs/standards/testing-and-gate-strategy.md:1606-1607` states: "Changes to the
planner, impact map, receipt verifier, gate policy, test filters, coverage
acquisition, or anti-evasion checks are themselves critical."
`gate-policy/v1/impact-map.json:9-59` reinforces this with three entries —
`gate-policy-authority`, `gate-lifecycle-authority`, `gate-planner-authority` —
each at `risk_floor: CRITICAL`.

Every Order-1 edit therefore lands inside the CRITICAL-risk envelope of the
system being removed, and under the current standard a critical change requires
full-workspace regression under planner admission. The Order-0 artifacts do not
state which regime governs their own landing. ADR-0043 supersedes the authority
prospectively, but the transition itself needs an explicit disposition — either
ADR-0043 governs from ratification, or the final planner-admitted run is named
and bounded. Without that statement, the first Order-1 commit can legitimately
be argued into triggering the machinery being deleted.

### 4. The policy digest is content-bound to the standard's bytes

`gate-policy/v1/impact-map.json:4` pins
`policy_sha256: 74203b29…f5e9`, and `gate-policy/v1/README.md:52` states that
this digest "binds the exact bytes of `docs/standards/testing-and-gate-strategy.md`."

Order 1 edits that standard. The moment it does, the digest no longer matches,
which interacts directly with the `FREEZE_HISTORICAL_VERIFY` classification:
historical receipts carry the old digest and any verifier that re-derives it
from the current file will mismatch. The migration map's commitment to "never
rewrite" historical evidence is correct but insufficient on its own — the
verification path needs a declared disposition (pin the historical digest as a
literal, or scope historical verification to the retained bytes rather than the
live file).

### 5. Schema v1 cannot express an advisory gate

Verified at `gate-policy/v1/schemas/gate-definitions.schema.json:11`:
`"enforcement_status": { "const": "BLOCKING" }`, and at `:129`:
`"blocks_transition": { "enum": ["INCREMENT", "CHECKPOINT", "CAMPAIGN", "RELEASE"] }`
— no `NONE` member. A v1 gate-definitions file is structurally incapable of
declaring a non-blocking gate.

By contrast `impact-map.schema.json:13` admits
`["SCHEMA_ONLY_NONBLOCKING", "SHADOW", "BLOCKING"]`. The asymmetry matters: the
impact map can be downgraded in place, gate-definitions cannot. The Order-0 map
classifies `gate-policy/v1` schemas as "Mixed / Order 1/3/4," which is
compatible with this but does not name the constraint. Retiring rather than
downgrading `gate-definitions.json` is the only v1-legal option.

### 6. Live status drift between the catalog and package files

| Package | `docs/work-packages/README.md` | Its own `package.md` |
|---|---|---|
| `20260727-gate-planner-external-dag-transaction-adapter-001` | `HOLD / INVALID / SUPERSEDED` (`README.md:350-352`) | `ACTIVE / SCAFFOLD REVIEWED` (`package.md:6`) |
| `20260727-testgate-first-attempt-ledger-bootstrap-001` | `ACTIVE / SCAFFOLD REVIEW` (`README.md:366-369`) | `COMPLETE` (`package.md:6`) |

Three further packages (`external-dag-closeout-correction-001`,
`auth11-terminal-node-selection-001`, `auth11-fixed-inventory-test-provider-001`)
carry `Status: ACTIVE` while the catalog declares the whole gate-planner
closeout frozen (`README.md:339-342`). An executor reading only the package
file will resume frozen work.

## Caveats

- Static only. No claim here rests on observed runtime behavior. Statements
  about what "breaks" are derived from reading assertions and call graphs, not
  from running the affected suites.
- Finding 2's failure predictions are read from assertion text; the suites were
  not executed to confirm they fail, only that the asserted strings exist in
  source that Order 1 removes.
- The consumer sweep covered `crates/`, `tests/`, `tools/`, and
  `.github/workflows/`. Consumers living only in retained out-of-repo attempt
  roots were not enumerated exhaustively.
- Findings 1 and 6 reflect repository state at 2026-07-27 ~20:15 PDT. The
  Order-0 package was being actively edited during this audit (its checklist
  advanced from two to nine completed items between 20:00 and 20:11), so
  artifact content may have moved since.
- This audit did not review ADR-0043's text, the target linter interface
  contract, or whether the friction metrics are achievable.

## Recommended follow-ups (not performed in this audit)

1. Add the three source-text-coupled guards from Finding 2 to the Order-1
   write set explicitly, so their amendment is planned rather than discovered.
2. Add one sentence to ADR-0043 or the Order-1 package stating which regime
   governs the demotion's own landing (Finding 3).
3. Give historical receipt verification a declared disposition for the pinned
   `policy_sha256` (Finding 4).
4. Reconcile the six status lines in Finding 6 before any executor resumes.
