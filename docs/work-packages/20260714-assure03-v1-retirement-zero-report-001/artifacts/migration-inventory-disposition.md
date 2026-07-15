# ASSURE-03 Migration Inventory Disposition

Status: complete; terminal implementation gates and dual verification passed

Evidence class: Static + Ran

## Active Source And Public Records

| Surface | Disposition | Reason / terminal behavior |
| --- | --- | --- |
| `assurance/catalog.yaml` | Rewritten | Typed transition source declares `v1_retired_zero_reports` and `dossiers: []`. |
| `assurance/README.md` | Rewritten | Documents audit-only Git recovery and the ASSURE-04 boundary. |
| `assurance/dossiers/**` | Removed | No active dossier, evidence, interpretation, authoring, or review record can be discovered. |
| `assurance/methods/**` | Removed | No active v1 method can be rendered or linked. |
| `assurance/schemas/**` | Removed | The transition tool uses the smallest typed Rust admission model; no v1 candidate schema remains active. |
| `assurance/templates/catalog.md` | Rewritten and retained | Only neutral, placeholder-free zero-report public output. |
| Application, dossier, and method templates | Removed | No candidate, method, or application worksheet route is generated. |
| `assurance/generated/wepppy-usersum.yaml` | Rewritten | Dormant export has `documents: []` and `vendoring_authorized: false`. |
| `usersum/assurance/README.md` | Rewritten | Exact generated neutral catalog. |
| Worksheet, dossier, and method public pages | Removed | Tracked public assurance tree has one file. |
| `usersum/README.md` | Repaired | Routes to model science and the zero-report catalog without v1 promises. |
| Snow/frost narrative | Repaired | Removes v1 routes/grade; retains the actual campaign design, positive selector/conservation results, and scientific limitations. |

## Compiler Files

The frozen compiler is recoverable from the manifest; only code required for a
fail-closed zero-report transition remains compiled.

| File | Disposition |
| --- | --- |
| `src/main.rs` | Retained unchanged as typed-error CLI boundary. |
| `src/lib.rs` | Rewritten to expose only the transition API. |
| `src/cli.rs` | Rewritten; admits only `--all`, no dossier selector. |
| `src/engine.rs` | Rewritten; validates zero reports, renders/checks two outputs, and snapshots zero reports. |
| `src/error.rs` | Reduced to errors reachable by the transition tool. |
| `src/hash.rs` | Reduced to SHA-256 helpers used by outputs, snapshots, and recovery tests. |
| `src/authoring.rs`, `graph.rs`, `model.rs`, `path.rs`, `publication.rs`, `render.rs`, `review.rs`, `snapshot.rs` | Removed; each belonged to the v1 status-first publication architecture. |

No root dependency or lockfile change was needed: all retained crate
dependencies remain exercised by typed YAML admission, JSON snapshot manifests,
and SHA-256 identities.

## Tests And Release Consumers

- The assurance integration target was replaced with 13 zero-report,
  candidate-rejection, drift, confinement, snapshot, exact-history, release-
  transition, workflow-consumer, and snow-science-preservation tests.
- `check_assurance_dossier_exports.sh` now enforces exactly one tracked public
  page, zero export documents, and vendoring prohibition.
- `check_assurance_release_transition.sh` is the shared pre-assembly guard.
- The aggregate gate script requires an explicit mode and exits validation
  before snapshot/binary assembly. Release preflight rejects marker and retired-
  route symlinks before a release directory is created.
- Ordinary workflow events run validation and upload
  `openwepp-validation-evidence-*`; candidate-named upload additionally
  requires explicit manual assembly, successful stability, preflight, and
  assembly. Failure evidence has a non-candidate name.

## Historical Records

Prior ADR/package statements remain factual history. The dormant WEPPcloud
handoff is unchanged and non-authorizing. The frozen commit and manifest, not a
copied archive tree, preserve exact v1 bytes. Current governance and standards
point to accepted v2 authority; the migration plan retains old paths only as the
execution inventory and recovery explanation.
