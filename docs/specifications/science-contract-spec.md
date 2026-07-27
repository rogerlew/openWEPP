# Science Contract Artifact Specification

Status: Active
Last updated: 2026-07-27
Scope: canonical artifact schema for openWEPP science contracts (`SC-*`)

## Purpose

Define the required structure of canonical openWEPP science-contract files. The
workflow for creating and promoting these files is defined in
`docs/specifications/science-contract-authoring-procedure.md`. Historical and
per-package narrative sidecars are defined in
`docs/specifications/science-contract-provenance-spec.md`.

## Canonical Location and Registry

Canonical contract files must live at:

- `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`

The canonical lifecycle registry must live at:

- `docs/specifications/science-contracts/index.md`

Stable references use:

- Contract: `SC-<DOMAIN>-<NNN>`
- Invariant: `SC-<DOMAIN>-<NNN>#INV-<DOMAIN>-<NNN>`
- Obligation: `SC-<DOMAIN>-<NNN>#OBL-<DOMAIN>-<NNN>` when obligation rows are
  separately identified.

## Required Front Matter

Existing contracts remain conformant when they satisfy the legacy lifecycle metadata baseline. New contracts and contracts migrated under this specification must carry the full target field set. Backfilling the target fields across existing contracts is migration work and must be tracked; this framework split does not silently render existing `SC-*` files non-conformant.

| Field | Required | Description |
|---|---|---|
| `contract_id` | yes | Stable ID matching the filename. |
| `title` | yes | Human-readable title. |
| `status` | yes | Lifecycle status such as `open`, `in_review`, `approved`, or `retired`. |
| `maturity` | yes | `proposed`, `draft`, `active`, or `deprecated`. |
| `owner` | yes | Maintainer or review group. |
| `contract_version` | yes | Monotonic version integer. |
| `producer_scope` | target | Producer surfaces governed by the contract. |
| `consumer_scope` | target | Consumer surfaces governed by the contract. |
| `evidence_level` | target | Highest evidence class supporting the contract. |
| `last_reviewed` | target | UTC date or `pending`. |
| `supersedes` | target | List of replaced contract IDs, otherwise empty. |
| `superseded_by` | target | List of replacement contract IDs, otherwise empty. |

Field requirement vocabulary: `yes` means required for all contracts; `target` means required for new contracts and contracts migrated under this schema, with legacy backfill tracked as follow-on migration work.

## Required Section Order

Each new contract and each existing contract materially amended after
ADR-0042 must contain, in order, these logical sections. Exact heading wording
may vary slightly during migration. Existing contracts remain conformant under
the legacy baseline until their next material scientific amendment; backfill
is tracked by the owning governance package.

1. Purpose.
2. Scientific scope and explicit out-of-scope boundaries.
3. Authority anchors with top-down citations.
4. Variables and units using canonical symbols first.
5. Algorithm state surfaces: required inputs, required outputs, and mutated
   state.
6. Algorithm specification with step sequence, branch conditions, and equations
   or pseudocode sufficient to reproduce the logic.
7. Branch and guard table.
8. Invariants and invariant guard map.
9. Producer obligations and consumer obligations.
10. Symbol alias map when boundary names differ.
11. Constants and parameters with provenance anchors.
12. Unit-governance map.
13. Tolerance and numeric notes.
14. Calibration and identifiability posture when scientifically applicable.
15. Test-vector obligations.
16. Binding Exposure Index when addenda or provenance sidecars exist.
17. Gap register and promotability labels.
18. Change log.

Kernel-affecting contracts must also satisfy
`docs/specifications/science-contracts/kernel-process-contract-profile.md`.

## Contract Draft Requirements

A draft is review-ready only when it contains:

1. Stable lifecycle metadata.
2. Scientific scope and excluded boundaries.
3. Variable/unit table for externally relevant symbols.
4. Invariant table with stable IDs and citation anchors.
5. Allowed degenerate states and invalid states.
6. Producer and consumer obligations.
7. Boundary disposition definitions per invariant family.
8. Tolerance statement or link to tolerance authority.
9. Gap register for unresolved science or evidence limits.
10. Guard map linking each invariant ID to enforcement path, failure behavior,
    and gate impact.
11. Symbol alias map when canonical WEPP symbols and openWEPP boundary/API names
    differ.
12. Binding Exposure Index when sidecar or addendum material exists.
13. Calibration and identifiability posture, or an explicit
    `CALIBRATION_NOT_APPLICABLE` rationale.

The draft must exist at the canonical `SC-*` path before dual-agent review.

## Calibration and Identifiability Schema

Contracts with one or more parameters eligible for empirical estimation must
include a substantive `Calibration and Identifiability` section. Contracts
whose parameters are fixed entirely by science authority, external inputs, or
non-empirical specification may declare `CALIBRATION_NOT_APPLICABLE` only with
a short rationale. Applicable sections must identify:

1. calibratable, fixed, and externally supplied parameters with units;
2. mathematical validity domains separately from evidence-supported bounds and
   execution assumptions;
3. required observation types, temporal/spatial scale, units, uncertainty, and
   admission authority;
4. the observation operator mapping model state/output to observations;
5. parameters or combinations expected to be identifiable, partially
   identifiable, confounded, or non-identifiable;
6. calibration objective and holdout/validation posture when defined;
7. minimum calibration-readiness obligations, including deterministic
   parameter execution, objective reconstruction, sensitivity, boundary and
   failure reporting, and synthetic recovery where structurally meaningful;
8. evidence gaps and the additional observations needed to strengthen
   identification or validation; and
9. prohibited claims, including treating execution assumptions, comparator
   agreement, or synthetic recovery as empirical calibration.

The section must also bind the three package-reporting fields defined by
ADR-0042: `science_implementation_status`, `calibration_evidence_status`, and
`identifiability_status`.

Every calibration/readiness package must carry a readiness matrix with one row
for each obligation below:

| Obligation | Allowed disposition |
|---|---|
| typed/enumerable parameter surface | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| observation operator with units and scale | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| deterministic candidate execution | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| objective reconstruction | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| sensitivity analysis | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| identifiability/confounding analysis | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| boundary, saturation, and failure reporting | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| equifinality/uncertainty retention | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| synthetic recovery | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |
| additional-data inventory | `PASS`, `BLOCKED`, `NOT_APPLICABLE` |

Each row requires an evidence path and rationale. `NOT_APPLICABLE` requires
contract- or structure-backed justification. Any `BLOCKED` row that is a
required current-scope gate forces `calibration_evidence_status =
NOT_CALIBRATION_READY` and ordinary package `HOLD`.

Numeric fitted results and dataset-specific candidate ensembles remain
work-package evidence. Canonical parameter meaning, domains, observation
mapping, and claim limits remain in the contract.

## Invariant Table Schema

Invariant rows must include at least:

| Column | Required | Description |
|---|---|---|
| `Invariant ID` | yes | Stable `INV-<DOMAIN>-<NNN>` ID. |
| `Statement` | yes | Binding scientific or governance obligation. |
| `Authority` | yes | Citation anchor IDs or physical invariant basis. |
| `Evidence` | yes | `[DIRECT]` or `[INFERENCE]` plus evidence mode. |
| `Guard` | yes | Runtime or governance guard mapping. |
| `Failure posture` | yes | Typed failure, hard `HOLD`, or approved bounded normalization. |

## Guard Map Schema

Guard map rows must include:

| Column | Required | Description |
|---|---|---|
| `Invariant ID` | yes | Referenced canonical invariant. |
| `Enforcement path` | yes | Runtime function, test, governance gate, or checklist. |
| `Guard class` | yes | `runtime`, `test`, `governance`, or `profile`. |
| `Failure behavior` | yes | Typed error, blocked promotion, or explicit `HOLD`. |
| `Evidence artifact` | conditional | Required when enforcement is outside the contract. |

## Symbol Alias Map Schema

Alias map rows must include:

| Column | Required | Description |
|---|---|---|
| `Canonical symbol` | yes | WEPP/reference symbol. |
| `Boundary/API name` | yes | Rust field, CLI flag, JSON key, parquet column, sidecar field, or publication name. |
| `Scope` | yes | Runtime/publication surface where alias applies. |
| `Units check` | yes | Same-unit confirmation, conversion helper, or explicit gap. |
| `Owner contract` | yes | Owning `SC-*` contract when cross-domain. |

## Unit-Governance Map Schema

Unit-governance rows must include:

| Column | Required | Description |
|---|---|---|
| `Symbol` | yes | Canonical or boundary symbol. |
| `Declared units` | yes | Units in contract authority. |
| `Boundary registry entry` | conditional | Required when runtime wrapper/registry exists. |
| `Conversion helper` | conditional | Named directional helper when conversion is required. |
| `Scalar exception` | conditional | Required when no typed wrapper/helper exists. |
| `Publication metadata` | conditional | Required for output/publication symbols. |

## Binding Exposure Index

A contract core must include a `Binding Exposure Index` when historical addenda,
package-local addenda, or provenance sidecars exist. The index is the anti-drop
mechanism that proves consolidation preserves all binding obligations.

### Required Index Columns

| Column | Required | Description |
|---|---|---|
| `Entry ID` | yes | Stable sidecar/addendum entry ID. |
| `Source` | yes | Addendum heading or sidecar path + anchor. |
| `Status` | yes | `active`, `superseded`, or `historical`. |
| `Binding classification` | yes | `maps-to-existing-INV`, `unpromoted-binding`, `historical-or-superseded`, or `undecidable`. |
| `Canonical binding IDs` | yes | `INV-*` / `OBL-*` IDs, or `none` only for non-binding historical entries. |
| `Review gate` | yes | `none`, `flagged-binding-addition`, or `science-review-follow-on`. |
| `Notes` | no | Short rationale and provenance pointer. |

### Index Rules

1. Every active binding entry must map to at least one existing core `INV-*` or
   `OBL-*` ID before narrative can be relocated.
2. `unpromoted-binding` entries are flagged binding additions. They cannot be
   silently consolidated; they require review, disposition, and verification.
3. `historical` and `superseded` entries are non-binding unless the index maps
   them to canonical binding IDs.
4. `undecidable` entries block consolidation and require science-review follow-on.
5. The index must reference only binding IDs that exist in the core contract.

## Binding Exposure Lint Contract

Automation that checks Binding Exposure Index conformance must **fail** (hard
violation) when:

1. An `active` or `unpromoted-binding` entry has no canonical binding IDs **and is
   not routed to `science-review-follow-on`** (see deferral rule below).
2. The index references an `INV-*` or `OBL-*` ID absent from the core contract.
3. A sidecar entry lacks required provenance fields.
4. A sidecar entry marked `historical` or `superseded` is referenced as binding
   without canonical binding IDs.
5. Required status vocabulary is violated.
6. An `undecidable` entry is **not** routed to `science-review-follow-on`.

### Science-review deferral (normative)

A row whose `Review gate` is `science-review-follow-on` is a **deferral**, not a
hard violation: the obligation is acknowledged as unresolved and parked for a
science decision, with its narrative retained in the binding core (never
relocated). Deferral is permitted because the binding residue is conserved — it
is not dropped, only un-adjudicated. Deferral is **temporary and owned**: each
deferred row must be tracked in a science-review follow-on queue with an owner and
a next evidence gate; it is not a permanent parking state.

A row routed to `science-review-follow-on` therefore does **not** trip rules 1 or
6, but it does mean the contract is **not fully consolidated**.

### Verdicts (normative)

The lint reports exactly one verdict and exit code:

| Verdict | Meaning | Exit (default) | Exit (`--strict`) |
|---|---|---|---|
| `FAIL` | one or more hard violations above | 1 | 1 |
| `PASS-DEFERRED` | no hard violations, but ≥1 `science-review-follow-on` row remains (binding-safe, not fully consolidated) | 0 | 1 |
| `PASS` | no hard violations and zero deferred rows (fully consolidated) | 0 | 0 |

`PASS-DEFERRED` must be reported distinctly from `PASS` so a completion gate is not
satisfied by deferral. Default exit `0` makes deferral **binding-safe** (it does
not block unrelated work); `--strict` exit `1` lets a completion/promotion gate
require full consolidation. Plain `PASS` is the only verdict that means a contract
is actually consolidated.

The lint is a package-precondition and promotion gate for contracts using
sidecars or consolidated addenda. Use default mode as a safety gate (nothing
dropped) and `--strict` as a completion gate (consolidation done).
