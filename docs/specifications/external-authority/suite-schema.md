# External Authority Suite Schema

Status: Active
Last updated: 2026-05-31
Scope: normative schema for external-authority constitutive suites

## Purpose

Define a stable, machine-readable-friendly metadata schema for external
authority suites used by openWEPP correctness gates.

## Required Fields (Normative)

Each suite definition must provide the fields below.

| Field | Type | Required | Description |
|---|---|---|---|
| `suite_id` | string | yes | Stable identifier: `cas_l<level>_<domain>_<law>_<nnn>`. |
| `title` | string | yes | Human-readable suite name. |
| `status` | enum | yes | `draft`, `active`, or `deprecated`. |
| `authority_level` | enum | yes | `4`, `5`, or `6`. |
| `domain` | string | yes | Primary domain (`soil`, `watbal`, `subhyd`, etc.). |
| `process_family` | string | yes | Family under adjudication (`fc_wp`, `lateral_drain`, etc.). |
| `sc_invariant_refs` | list[string] | yes | One or more `SC-*#INV-*` links. |
| `external_citations` | list[object] | yes | Citation objects with provenance/version metadata. |
| `fixtures` | list[object] | yes | Fixture path/class/units manifest entries. |
| `tolerances` | object | yes | Explicit absolute/relative/mixed tolerance declarations. |
| `gate_lane` | enum | yes | `required`, `periodic`, or `manual`. |
| `failure_class` | enum | yes | `hard-fail` or `investigation`. |
| `runtime_cost_class` | enum | yes | `unit`, `component`, or `integration`. |
| `owner` | string | yes | Maintainer/reviewer owner tag. |
| `provenance` | object | yes | Author/date/update lineage metadata. |
| `notes` | string | no | Optional implementation notes. |

## Field Semantics (Normative)

1. `suite_id` is immutable once suite status is `active`.
2. `authority_level` controls default gate treatment:
   - `4`: constitutive correctness authority; blocking when in required lane.
   - `5`: measured/system validation; default non-blocking unless promoted.
   - `6`: independent-solver cross-check; default non-blocking unless promoted.
3. `sc_invariant_refs` must reference currently valid canonical invariant IDs.
4. `external_citations` entries must include source, edition/version, and
   retrieval/provenance details sufficient for independent audit.
5. `tolerances` must declare units basis and evaluation mode (`abs`, `rel`, or
   mixed).

## Citation Object Schema (Normative)

Each `external_citations[]` item must include:

| Field | Type | Required |
|---|---|---|
| `citation_id` | string | yes |
| `source_type` | enum (`book`, `paper`, `dataset`, `solver_doc`, `other`) | yes |
| `title` | string | yes |
| `locator` | string | yes |
| `version_or_edition` | string | yes |
| `retrieved_utc` | string (date) | yes |
| `notes` | string | no |

## Fixture Object Schema (Normative)

Each `fixtures[]` item must include:

| Field | Type | Required |
|---|---|---|
| `fixture_id` | string | yes |
| `path` | string | yes |
| `fixture_class` | enum (`unit`, `component`, `integration`) | yes |
| `units_basis` | string | yes |
| `seed_or_case` | string | no |
| `hash` | string (`sha256`) | yes |
| `source_repo` | string | yes |
| `source_commit` | string | yes |
| `source_path` | string | yes |
| `source_sha256` | string (`sha256`) | yes |
| `transform_note` | string | yes |

## Tolerance Object Schema (Normative)

`tolerances` must include:

| Field | Type | Required |
|---|---|---|
| `mode` | enum (`abs`, `rel`, `mixed`) | yes |
| `abs` | object | conditional (`abs` or `mixed`) |
| `rel` | object | conditional (`rel` or `mixed`) |
| `units` | string | yes |
| `notes` | string | no |

`abs` and `rel` sub-objects must include numeric threshold values and
comparison semantics.

## Naming Convention (Normative)

Suite IDs follow:

- `cas_l<authority_level>_<domain>_<law>_<nnn>`

Examples:

- `cas_l4_soil_fc_minus33_001`
- `cas_l4_soil_wp_minus1500_001`
- `cas_l4_watbal_relax_to_fc_001`

## Location Rules

- Suite definitions: `docs/specifications/external-authority/suites/`
- Fixture data: `tests/fixtures/constitutive/<suite_id>/`
- Contract-derived integration harness:
  `tests/integration/<suite_id>_contract.rs`

## Fixture Lock And Provenance Files (Normative)

Each active suite fixture root must include:

1. `fixtures.sha256`:
   - `sha256sum --check --strict` compatible manifest for all fixture payload
     files used by the suite.
2. `fixtures.provenance.yaml`:
   - per-fixture provenance entries keyed by `path` that include:
     - `sha256`,
     - `source_repo`,
     - `source_commit`,
     - `source_path`,
     - `source_sha256`,
     - `transform_note`.

Release-gate automation treats missing lock/provenance files, checksum
mismatches, or missing required provenance keys as blocking failures.
