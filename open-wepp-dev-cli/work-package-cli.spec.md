# open-wepp-dev-cli: Work-Package CLI and State Sidecar Spec (Draft)

## Document Control
- Status: Draft
- Version: `0.2.0`
- Scope: `owd wp` command group in `open-wepp-dev-cli`
- Audience: openWEPP contributors, package authors, agent workflows, CI maintainers
- Bin name: `owd`
- Namespace: `owd wp`

## 1. Problem Statement
`docs/work-packages` has strong scaffold consistency, but lifecycle truth is
currently spread across `package.md`, disposition artifacts, and prose fields.
This causes stale status drift, ambiguous hold/gap interpretation, and costly
manual dependency/blocker analysis.

The CLI must provide deterministic, machine-readable state evaluation with a
full audit trail that does not rely on git commit history quality.

## 2. Goals
1. Make package lifecycle state machine-readable and lintable.
2. Make HOLD and GAP states queryable and auditable.
3. Make dependency chains and blocker impact visible.
4. Provide deterministic conflict detection with stable warning codes.
5. Support agent workflows that continuously maintain state during execution.
6. Preserve markdown artifacts as narrative evidence while moving machine
   authority to sidecar files.

## 3. Non-Goals
1. Replacing work-package narrative content in `package.md`.
2. Rewriting historical artifact prose into a new format.
3. Enforcing git commit order as execution truth.
4. Replacing existing kernel/science correctness gates.

## 4. Authority Model
1. `state.events.yaml` is the authoritative, append-only event ledger.
2. `state.yaml` is a materialized snapshot derived from the ledger.
3. `package.md` and artifacts remain human-oriented evidence.
4. CLI decisions (`status`, `ready`, `holds`, `trace`) are derived from ledger
   reduction, not from markdown prose.

## 5. Package Discovery

### 5.1 Discovery Rule (Normative)
A package is discovered when a directory under `docs/work-packages/` contains:
1. `package.md`
2. `artifacts/README.md`

This rule supports legacy package names and future naming changes.

### 5.2 Naming Lint Rule (Policy)
For new packages, preferred naming pattern is:
- `^\d{8}-.+-\d{3}$`

Legacy names are allowed and must still be indexed.

### 5.3 Sidecar Paths
- `docs/work-packages/<package-id>/state.events.yaml`
- `docs/work-packages/<package-id>/state.yaml`

## 6. Data Model

### 6.1 Entity References (Normative)
Cross-package mutations must use fully-qualified refs:
- package: `pkg:<package_id>`
- hold: `hold:<package_id>#<hold_id>`
- gap: `gap:<package_id>#<gap_id>`

Bare hold/gap IDs are not valid mutation targets.

### 6.2 `state.events.yaml` Schema (Authoritative)

Example:
```yaml
schema_version: 1
package_id: 20260524-ws12-impoundment-physics-equivalence-port-001
events:
  - seq: 1
    occurred_at: "2026-05-24T22:10:00Z"
    op: package_state_set
    actor: codex
    target_ref: "pkg:20260524-ws12-impoundment-physics-equivalence-port-001"
    payload:
      state: active
      phase: B
    evidence:
      source_package_id: 20260524-ws12-impoundment-physics-equivalence-port-001
      source_artifact: artifacts/ws12-implementation-and-test-evidence.md
      source_locator: "# Execution Result"
    reason: "Phase B execution started"

  - seq: 2
    occurred_at: "2026-05-24T23:19:00Z"
    op: hold_open
    actor: codex
    target_ref: "hold:20260524-ws12-impoundment-physics-equivalence-port-001#WS12-HOLD-001"
    payload:
      summary: "WS12 parity-trace evidence not complete"
      severity: high
      lift_criteria:
        - "Publish accepted parity-trace artifact for WS12"
        - "Dual verification confirms closure"
      tags: [parity, trace]
    evidence:
      source_package_id: 20260524-ws12-impoundment-physics-equivalence-port-001
      source_artifact: artifacts/ws12_disposition.md
      source_locator: "# Hold State"
    reason: "Closeout blocked by unresolved parity trace"

  - seq: 3
    occurred_at: "2026-05-24T23:19:05Z"
    op: package_state_set
    actor: codex
    target_ref: "pkg:20260524-ws12-impoundment-physics-equivalence-port-001"
    payload:
      state: completed-with-hold
      phase: D
      risk_acceptance:
        accepted_by: "maintainer"
        accepted_at: "2026-05-24T23:19:05Z"
        authority_ref: "artifacts/ws12_disposition.md"
        scope: "WS12 only"
        rationale: "Implementation complete; parity-trace deferred"
    evidence:
      source_package_id: 20260524-ws12-impoundment-physics-equivalence-port-001
      source_artifact: artifacts/ws12_disposition.md
      source_locator: "# Status"
    reason: "Package closeout with explicit hold retention"
```

Required top-level fields:
- `schema_version` (int; must be `1`)
- `package_id` (must match directory name)
- `events` (array; append-only)

Required event fields:
- `seq` (int >= 1; strictly monotonic per package)
- `occurred_at` (RFC3339 UTC)
- `op` (event enum)
- `actor` (string)
- `target_ref` (entity ref)
- `reason` (string)

Required for state, hold, gap, and supersede mutations:
- `evidence.source_package_id`
- `evidence.source_artifact`
- `evidence.source_locator`

Event op enum:
- `package_state_set`
- `dependency_add`
- `dependency_remove`
- `hold_open`
- `hold_close`
- `hold_reopen`
- `gap_open`
- `gap_close`
- `gap_reopen`
- `package_supersede`
- `note`

### 6.2.1 Event Op Payload Contracts (Normative)
Each op has a required payload contract:

1. `package_state_set`
- Required: `state`
- Optional: `phase`
- Required when state reopens a terminal state (`completed`, `cancelled`, `superseded`): `reopen_reason`
- Required when `state=completed-with-hold`: `risk_acceptance`

2. `dependency_add`
- Required: `target_package_id`

3. `dependency_remove`
- Required: `target_package_id`

4. `hold_open`
- Required: `summary`, `lift_criteria`
- Optional: `severity`, `tags`

5. `hold_close`
- Required: `close_reason`

6. `hold_reopen`
- Required: `reopen_reason`, `lift_criteria`

7. `gap_open`
- Required: `summary`
- Optional: `severity`, `acceptance_required`, `tags`

8. `gap_close`
- Required: `close_reason`

9. `gap_reopen`
- Required: `reopen_reason`

10. `package_supersede`
- Required: `superseded_package_ref`, `replacement_package_ref`, `treatment`
- `treatment` enum: `retain-open-items|mark-open-items-superseded`

11. `note`
- Required: `message`

Unknown payload keys are lint warnings by default and strict errors in `--strict`.

### 6.3 `state.yaml` Schema (Materialized Snapshot)

Example:
```yaml
schema_version: 1
package_id: 20260524-ws12-impoundment-physics-equivalence-port-001
snapshot_generated_at: "2026-05-24T23:19:06Z"
last_event_seq: 3
state: completed-with-hold
phase: D
summary: "Implementation complete; hold retained pending parity-trace closure"

depends_on:
  - 20260524-ws11-channel-routing-physics-equivalence-port-001

open_holds:
  - ref: "hold:20260524-ws12-impoundment-physics-equivalence-port-001#WS12-HOLD-001"
    summary: "WS12 parity-trace evidence not complete"
    severity: high
    lift_criteria:
      - "Publish accepted parity-trace artifact for WS12"

open_gaps: []

risk_acceptance:
  accepted_by: "maintainer"
  accepted_at: "2026-05-24T23:19:05Z"
  authority_ref: "artifacts/ws12_disposition.md"
  scope: "WS12 only"
  rationale: "Implementation complete; parity-trace deferred"

artifacts:
  disposition: artifacts/ws12_disposition.md
```

Required fields:
- `schema_version`
- `package_id`
- `snapshot_generated_at`
- `last_event_seq`
- `state`
- `phase` (use `N/A` when not phase-scoped)
- `depends_on`
- `open_holds`
- `open_gaps`

State enum:
- `queued`
- `active`
- `hold`
- `completed`
- `completed-with-hold`
- `cancelled`
- `superseded`

Phase enum:
- `A`, `B`, `C`, `D`, `N/A`

Conditional requirements:
- `state=completed` requires no open holds and no open gaps.
- `state=completed-with-hold` requires at least one open hold and a
  `risk_acceptance` object.
- `state=superseded` requires supersede evidence in ledger.

`risk_acceptance` required fields:
- `accepted_by`
- `accepted_at` (RFC3339 UTC)
- `authority_ref` (must resolve to an existing artifact path)
- `scope`
- `rationale` (non-empty)

## 7. Reduction Semantics

### 7.1 Deterministic Event Ordering
Reducer invariants:
1. Within each package ledger, events are applied by `seq` ascending.
2. All `occurred_at` and `snapshot_generated_at` values must be valid RFC3339 UTC.
3. `occurred_at` must be non-decreasing as `seq` increases within a package.
4. Violations of invariant #2 emit `WPW001`.
5. Violations of invariant #3 emit `WPW017`.

Reducer behavior:
- In `--strict` mode, any `WPW001` is fail-fast: reducer returns non-zero and
  emits no derived state output.
- In non-strict mode, `WPW001` remains a warning and affected packages are
  omitted from reduced output with diagnostic entries.

After per-package invariants pass, all events across packages are merged in:
1. `occurred_at` ascending
2. `package_id` ascending
3. `seq` ascending

`seq` is only compared after `package_id` tie-break, so it is never interpreted
as cross-package chronology.

### 7.2 Snapshot Derivation
`state.yaml` must be derivable from `state.events.yaml`.
- Lint emits drift warning when snapshot and derived state differ.
- `owd wp sync` regenerates snapshot from events.

### 7.3 Cross-Package Mutations
Cross-package mutations are only allowed through event ops targeting fully
qualified refs. Example:
- A package can close another package's hold only via `hold_close` event with
  `target_ref=hold:<owner_package>#<hold_id>` and evidence.

### 7.4 Supersede Semantics
`package_supersede` event payload must include:
- `superseded_package_ref` (`pkg:<id>`)
- `replacement_package_ref` (`pkg:<id>`)
- `treatment` (`retain-open-items|mark-open-items-superseded`)

Ownership rule:
- `package_supersede` is recorded in the superseding package ledger (the package
  initiating replacement), not in the superseded package ledger.

Reducer side effects are normative:
1. Target package state is set to `superseded`.
2. If `treatment=mark-open-items-superseded`, target open holds/gaps are marked
   `superseded` with provenance from this event.
3. If `treatment=retain-open-items`, target open holds/gaps remain open and
   retain existing ownership refs.
4. Dependencies targeting the superseded package are not auto-transferred;
   reducer emits `WPW014` until explicit dependency events update edges.

### 7.5 No Implicit Last-Write-Wins
No global overwrite by timestamp alone. Every mutation must be explicit and
traceable through event ops.

## 8. State Transition Rules (Normative)

Allowed package-state transitions:
- `queued -> active|cancelled`
- `active -> hold|completed|completed-with-hold|cancelled`
- `hold -> active|completed-with-hold|cancelled`
- `completed -> active` only with explicit reopen reason in `package_state_set`
- `completed-with-hold -> active|completed`
- `cancelled -> active` only with explicit reopen reason
- `queued|active|hold|completed|completed-with-hold -> superseded` only via
  `package_supersede`
- `superseded -> active` only with explicit `reopen_reason`

Additional invariants:
- Transition to `completed` is illegal if open hold/gap exists.
- Transition to `completed-with-hold` is illegal without `risk_acceptance`.
- If `package_state_set --phase` is omitted, reducer retains prior phase; if no
  prior phase exists, phase defaults to `N/A`.

## 9. Warning and Error Model

### 9.1 Warning Codes
- `WPW001`: malformed timestamp (`occurred_at` or `snapshot_generated_at`)
- `WPW002`: non-monotonic `seq` within package ledger
- `WPW003`: package ID mismatch between path and sidecar
- `WPW004`: unknown enum/op/ref shape
- `WPW005`: illegal package state transition
- `WPW006`: open hold missing lift criteria
- `WPW007`: mutation target ref not found
- `WPW008`: duplicate entity definition conflict
- `WPW009`: dependency target package not found
- `WPW010`: dependency cycle detected
- `WPW011`: terminal state invariant violation (`completed`, `completed-with-hold`)
- `WPW012`: referenced artifact path missing
- `WPW013`: snapshot drift from event-derived state
- `WPW014`: contradictory supersede semantics
- `WPW015`: inferred legacy state used (migration mode)
- `WPW016`: mutation missing required evidence
- `WPW017`: per-package `occurred_at` regression relative to `seq`
- `WPW018`: ambiguous bare hold/gap ID without owner
- `WPW019`: historical event mutation (non-append edit/delete)

### 9.2 Strict Mode
`owd wp lint --strict` upgrades at minimum:
- `WPW001`, `WPW002`, `WPW003`, `WPW004`, `WPW005`, `WPW006`, `WPW007`,
  `WPW008`, `WPW009`, `WPW010`, `WPW011`, `WPW012`, `WPW013`, `WPW014`,
  `WPW016`, `WPW017`, `WPW019`

to errors.

## 10. CLI Surface: `owd wp`

### 10.1 Global Options
- `--root <path>` default: repository root
- `--output table|json|yaml`
- `--strict`
- `--no-color`

### 10.2 Read Commands
1. `owd wp status`
   - List effective package state and unresolved blocker counts.
2. `owd wp show <package-id>`
   - Show reduced snapshot plus sidecar provenance.
3. `owd wp holds [--open|--all]`
   - Show hold registry with refs and owners.
4. `owd wp gaps [--open|--all]`
   - Show gap registry with refs and owners.
5. `owd wp deps <package-id> [--reverse] [--transitive] [--depth N]`
   - Show dependency chain and impact.
6. `owd wp graph [--graph-format mermaid|dot|json]`
   - Emit package dependency graph.
7. `owd wp ready [<package-id>]`
   - Evaluate execution readiness (deps, holds, invariants).
8. `owd wp trace <package-id>`
   - Show package event timeline.
9. `owd wp trace hold <entity-ref|hold-id> [--owner <package-id>] [--allow-ambiguous]`
   - Show hold open/close/reopen/supersede chain.
10. `owd wp trace gap <entity-ref|gap-id> [--owner <package-id>] [--allow-ambiguous]`
    - Show gap chain.
11. `owd wp events <package-id>`
    - Print raw ledger events with line/file pointers.

Trace argument rules:
- Fully-qualified `entity-ref` is always valid.
- Bare `hold-id`/`gap-id` requires `--owner` unless `--allow-ambiguous` is set.
- Bare IDs without owner are hard errors (non-zero exit) with `WPW018`.
- With `--allow-ambiguous`, command returns all matching refs and emits `WPW018`.

### 10.3 Validation and Repair Commands
1. `owd wp lint`
   - Validate schemas, transitions, refs, deps, and evidence requirements.
2. `owd wp doctor`
   - Summarize normalization hotspots and migration risk.
3. `owd wp sync [<package-id>]`
   - Regenerate `state.yaml` from `state.events.yaml`.
   - `--check`: no writes; exits non-zero when derived snapshot differs from
     committed `state.yaml`, with diff summary in output.

### 10.4 Authoring Commands (Agent-Facing)
Authoring commands append ledger events and run snapshot sync.

1. `owd wp state init <package-id> [--state queued] [--phase N/A] --reason <text> --evidence-artifact <path> --evidence-locator <text> [--source-package-id <package-id>]`
2. `owd wp state set <package-id> --state <enum> [--phase <phase>] --reason <text> --evidence-artifact <path> --evidence-locator <text> [--source-package-id <package-id>] [--reopen-reason <text>] [--accepted-by <text> --accepted-at <rfc3339> --authority-ref <path> --scope <text> --rationale <text>]`
3. `owd wp hold open <package-id> --id <hold-id> --summary <text> --lift <text> --evidence-artifact <path> --evidence-locator <text>`
4. `owd wp hold close <package-id> --target <entity-ref> --reason <text> --evidence-artifact <path> --evidence-locator <text>`
5. `owd wp hold reopen <package-id> --target <entity-ref> --reason <text> --lift <text> --evidence-artifact <path> --evidence-locator <text>`
6. `owd wp gap open <package-id> --id <gap-id> --summary <text> --evidence-artifact <path> --evidence-locator <text>`
7. `owd wp gap close <package-id> --target <entity-ref> --reason <text> --evidence-artifact <path> --evidence-locator <text>`
8. `owd wp gap reopen <package-id> --target <entity-ref> --reason <text> --evidence-artifact <path> --evidence-locator <text>`
9. `owd wp dependency add <package-id> --target-package <package-id>`
10. `owd wp dependency remove <package-id> --target-package <package-id>`
11. `owd wp supersede --superseding-package <package-id> --superseded-package <package-id> --replacement-package <package-id> --treatment <retain-open-items|mark-open-items-superseded> --reason <text> --evidence-artifact <path> --evidence-locator <text>`
12. `owd wp note add <package-id> --message <text> --evidence-artifact <path> --evidence-locator <text>`

Command invariants:
- `state set --state completed-with-hold` requires all risk-acceptance flags.
- `state set` transitions from terminal states require `--reopen-reason`.
- `state init/state set` require evidence fields to satisfy ledger auditability.

Authoring guarantees:
- append-only event writes,
- non-tail edits/deletes are rejected with `WPW019`,
- atomic writes,
- automatic `snapshot_generated_at` update via `sync`.

## 11. Output Contract

### 11.1 Status Table Columns
- `package_id`
- `state`
- `phase`
- `open_holds`
- `open_gaps`
- `depends_on_count`
- `last_event_at`
- `warning_count`

### 11.2 JSON Output
All read commands support stable JSON with:
- `version`
- `generated_at`
- `root`
- `records[]`
- `warnings[]` (`code`, `message`, `file`, `line`, `package_id`)

## 12. Agent Workflow Integration (Required)

Agent package execution workflows must call `owd wp` commands at phase
boundaries and blockers, not only at closeout.

Required event points:
1. kickoff: `state set -> active`
2. blocker discovered: `hold open`
3. gap discovered: `gap open`
4. blocker/gap resolved: corresponding close event
5. closeout: `state set -> completed` or `completed-with-hold`
6. pre-handoff: `lint --strict` and `trace`

`completed-with-hold` requires explicit `risk_acceptance` payload in the
state-set event.

## 13. CI Integration

Recommended gates:
1. `owd wp lint --strict`
2. `owd wp doctor --output json`
3. `owd wp sync --check` (fails if snapshot drift exists)

Changed-package enforcement:
1. CI computes changed package set from `merge-base(main, HEAD)..HEAD`.
2. Local runs may override base with `--since <git-ref>`.
3. Path filter scope is `docs/work-packages/<id>/**`.
4. If any file under a package changes, that package must also contain a new
   event append unless change is docs-only under `prompts/archived/`.
5. `seq` must increase for changed packages.
6. `snapshot_generated_at` and `last_event_seq` must match ledger head.
7. For `state.events.yaml`, all historical events present at base ref must be
   byte-identical in HEAD; only tail appends are allowed (`WPW019` on violation).

## 14. Migration Plan

### Phase M0: Read-Only Legacy Index
- Discover packages by structure.
- If sidecars missing, infer temporary virtual state from markdown and emit
  `WPW015` warnings.
- No writes in M0.

### Phase M1: Sidecar Bootstrap
- Generate initial `state.events.yaml` and `state.yaml` for all packages.
- Bootstrap rules:
  1. Create `seq=1` `package_state_set` from best-known status.
  2. Set `occurred_at` from strongest available source in order:
     disposition timestamp > package status timestamp > fixed sentinel
     `1970-01-01T00:00:00Z` (emit `WPW015` when sentinel used).
  3. Create `target_ref` deterministically from package directory and discovered
     hold/gap IDs.
  4. For any mutation event (`package_state_set`, hold/gap events,
     `package_supersede`) where evidence artifact/locator cannot be resolved:
     use fallback evidence (`source_package_id=<package_id>`,
     `source_artifact=artifacts/README.md`,
     `source_locator=# Migration Bootstrap (Inferred)`), emit `WPW015`, and
     append `op=note` describing the inferred evidence.
  5. If hold/gap prose exists with no explicit ID, mint deterministic synthetic
     IDs: `<PACKAGE_KEY>-HOLD-AUTO-<NNN>` / `<PACKAGE_KEY>-GAP-AUTO-<NNN>` and
     emit `WPW015`.
  6. If inferred state is `completed-with-hold` and no risk-accept evidence is
     available, set state to `hold`, emit `WPW015`, and require manual closeout
     decision before transition to `completed-with-hold`.
  7. Mark uncertain imports with `op=note` and migration warnings.

### Phase M2: Authority Shift
- New packages require both sidecar files.
- CI strict lint enforced for changed packages.

### Phase M3: Legacy Reconciliation
- Prioritize active/hold/recent packages.
- Resolve inferred-state warnings and remove ambiguous legacy imports.

## 15. Backward Compatibility
1. During migration, commands report `state_source`:
   - `event_ledger`, `snapshot`, or `inferred_legacy`.
2. Once a package has `state.events.yaml`, ledger is authoritative.
3. Markdown status text is advisory and not machine authority.

## 16. Security and Safety
1. Flat-file reads/writes only.
2. No network requirement.
3. Atomic sidecar writes (`temp + rename`).
4. `--check` dry-run mode for CI and review.

## 17. Open Questions
1. Should `actor` be constrained to known identities (`codex`, `human`, `ci`)?
2. Should entity refs be case-sensitive for IDs after `#`?
3. Should migration auto-create risk-acceptance placeholders or force manual fill?

## 18. Acceptance Criteria for Initial Implementation
1. `status`, `holds`, `gaps`, `deps`, `trace`, `lint`, `sync` implemented.
2. Both sidecar schemas validated with stable error codes.
3. Conflict/warning model implemented as specified.
4. Deterministic event reduction implemented with ordering:
   `occurred_at`, `package_id`, `seq`.
5. Migration utility implemented for sidecar bootstrap from legacy packages.
