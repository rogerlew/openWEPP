# Science Contract Provenance Sidecar Specification

Status: Active
Last updated: 2026-06-08
Scope: sidecar format and lifecycle for science-contract provenance narratives

## Purpose

Define where historical, superseded, or package-local science-contract narrative
is retained after the canonical `SC-*` core exposes all binding obligations. This
specification preserves audit history without making sidecar prose a substitute
for canonical `INV-*` or `OBL-*` authority.

Workflow rules live in
`docs/specifications/science-contract-authoring-procedure.md`. Core contract
schema and the `Binding Exposure Index` live in
`docs/specifications/science-contract-spec.md`.

## Sidecar Location and Naming

Sidecars must live next to the canonical contract unless a package explicitly
records a different canonical sidecar root.

Recommended path:

- `docs/specifications/science-contracts/contracts/provenance/SC-<DOMAIN>-<NNN>-provenance.md`

A contract using a sidecar must reference it from the core `Binding Exposure
Index`.

## Sidecar Status Vocabulary

Sidecar entries use exactly these statuses:

| Status | Meaning | Binding posture |
|---|---|---|
| `active` | Entry describes live context whose binding residue is still relevant. | Binding only through mapped core `INV-*` / `OBL-*` IDs. |
| `superseded` | Entry has been replaced by later contract authority. | Non-binding unless mapped to current core IDs. |
| `historical` | Entry is retained for audit/provenance only. | Non-binding unless mapped to current core IDs. |

History statements are non-binding unless cross-referenced from the core
contract's `Binding Exposure Index` to canonical binding IDs.

## Required Sidecar Entry Fields

Each sidecar entry must contain:

| Field | Required | Description |
|---|---|---|
| `entry_id` | yes | Stable ID, usually package or addendum ID. |
| `title` | yes | Human-readable entry title. |
| `status` | yes | `active`, `superseded`, or `historical`. |
| `source_package` | yes | Work-package ID or source addendum heading. |
| `effective_date` | yes | UTC date when the entry became effective or historical. |
| `verdict` | yes | Short disposition such as `binding-exposed`, `superseded`, `historical`, `retracted`, or `hold`. |
| `superseded_by` | conditional | Required when status is `superseded`. |
| `canonical_binding_ids` | yes | Current `INV-*` / `OBL-*` IDs, or `none` for non-binding history. |
| `migration_target` | conditional | Required when entry routes implementation or contract follow-on work. |
| `provenance_anchors` | yes | Citation anchors, package artifacts, or source references. |
| `summary` | yes | Concise narrative of what the entry records. |

## Entry Template

Use this shape for each entry:

```md
## <entry_id> <title>

- status: active|superseded|historical
- source_package: <package-id-or-addendum-heading>
- effective_date: YYYY-MM-DD
- verdict: binding-exposed|superseded|historical|retracted|hold
- superseded_by: <entry-id-or-contract-id-or-none>
- canonical_binding_ids: <INV/OBL list or none>
- migration_target: <package/path or none>
- provenance_anchors: <citation/package/source list>

<summary prose>
```

## Lifecycle Rules

1. New binding authority is authored in the core contract first, not in the
   sidecar.
2. Sidecar entries may retain rationale, package history, rejected routes,
   superseded comparator interpretations, or migration provenance.
3. A sidecar entry cannot create a binding obligation by prose alone.
4. Binding residue must be exposed through core `INV-*` / `OBL-*` IDs and listed
   in the core `Binding Exposure Index`.
5. Superseding an entry requires `superseded_by` and a current core/index
   mapping or an explicit non-binding historical disposition.
6. Retiring an entry to `historical` requires a reason and an index row showing
   either canonical binding IDs or `none`.
7. Entries with `verdict: hold` must name an owner or follow-on and the next
   evidence gate.

## Retention Rules

1. Do not delete sidecar entries that explain why prior package routes were
   rejected, superseded, or reclassified.
2. Do not keep large narrative in the core contract when all binding residue is
   exposed through canonical IDs.
3. Sidecar retention is audit retention, not authority replacement.
4. If a sidecar grows enough to become a context burden, split it by dated or
   series-specific sidecar files and keep the core index links stable.

## Lint Requirements

Binding exposure lint must check sidecar entries for:

1. Required fields.
2. Valid status vocabulary.
3. `superseded_by` presence for `superseded` entries.
4. `canonical_binding_ids` presence or explicit `none`.
5. Core `Binding Exposure Index` coverage for every sidecar entry.
6. Existence of referenced core `INV-*` / `OBL-*` IDs.

A lint failure blocks promotion or consolidation closure.
