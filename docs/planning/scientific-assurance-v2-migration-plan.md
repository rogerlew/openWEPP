# Scientific Assurance V2 Migration Plan

Status: prospective — blocked until ASSURE-02 human acceptance

Owner: scientific-assurance steward and release maintainer

## Objective

Remove the v1 status-first SNOTEL candidate from every active public and export
surface, preserve its exact engineering and scientific provenance, and establish
a neutral zero-report state before any v2 compiler or report is implemented.

The migration retires a publication architecture. It does not delete or
downgrade snow/frost science, datasets, contracts, campaign results, or the
canonical model narrative.

## Exact V1 Action Inventory

ASSURE-03 freezes one pre-removal Git commit plus a path, size, and SHA-256
manifest for every `preserve` or `remove` row below. That commit and manifest
are the preservation identity unless a row names an additional generated or
release object. The executing package must expand every directory row to its
individual tracked files before editing; glob text is not final evidence.

| Current path or surface | ASSURE-03 action | Required zero-report proof |
| --- | --- | --- |
| `assurance/README.md` | Update from v1 canonical/rebuild instructions to historical/empty-catalog guidance | No instruction can regenerate or publish the SNOTEL candidate |
| `assurance/catalog.yaml` | Remove active candidate entry; retain the smallest empty-catalog source the v1 tool needs | Validation and build enumerate zero public reports |
| `assurance/dossiers/snow-snotel-swe-depth-density/authoring.yaml` | Remove from active source after manifesting | Catalog and planner cannot discover it |
| `assurance/dossiers/snow-snotel-swe-depth-density/dossier.yaml` | Remove from active source after manifesting | No dossier identity remains active |
| `assurance/dossiers/snow-snotel-swe-depth-density/evidence.yaml` | Remove from active source after manifesting; preserve science through Git and named campaign references | No active/public claim reads it; later snow synthesis can recover it exactly |
| `assurance/dossiers/snow-snotel-swe-depth-density/interpretation.md` | Remove from active source after manifesting | No renderer reads status-first prose |
| `assurance/dossiers/snow-snotel-swe-depth-density/limitations.md` | Remove from active source after manifesting | No renderer reads status-first prose |
| `assurance/dossiers/snow-snotel-swe-depth-density/review.yaml` | Remove from active source after manifesting; preserve exact review provenance | No review lock authorizes candidate publication |
| `assurance/methods/snow-snotel-evaluation-v1.yaml` | Remove from active source after manifesting | No active method entry or generated method page |
| `assurance/schemas/authoring.schema.json`, `catalog.schema.json`, `dossier.schema.json`, `evidence.schema.json`, `method.schema.json`, and `review.schema.json` | Retain only as v1 engineering support needed for empty-catalog migration and historical tests; otherwise remove under a recorded per-file decision | No retained schema can make a candidate publishable |
| `assurance/templates/application-context-worksheet.md`, `catalog.md`, `dossier.md`, and `method.md` | Retain only the minimum neutral empty-catalog template; remove or quarantine candidate/method/application templates under recorded per-file decisions | Empty build produces only the neutral catalog |
| `assurance/generated/wepppy-usersum.yaml` | Regenerate as an explicitly empty dormant-export manifest or remove if the consumer contract permits | Export enumerates zero reports and cannot carry candidate routes |
| `usersum/assurance/README.md` | Replace with neutral zero-report catalog | Public catalog contains no grade, candidate, or implication that snow lacks evidence |
| `usersum/assurance/application-context-worksheet.md` | Remove from current assurance navigation and normally remove the generated page | No orphaned public assurance output remains |
| `usersum/assurance/dossiers/snow-snotel-swe-depth-density.md` | Remove after manifesting exact bytes | Path absent and no public/catalog/search reference resolves to it |
| `usersum/assurance/methods/snow-snotel-evaluation-v1.md` | Remove after manifesting exact bytes | Path absent and no public/catalog/search reference resolves to it |
| `usersum/README.md` | Replace v1 dossier/catalog wording and links with the neutral zero-report entry | Public root exposes only the zero-report state and model narratives |
| `usersum/snow-frost-modeling-and-validation.md` | Preserve science; remove only premature v1 report/method links and explain that v2 synthesis is pending | Narrative remains substantive and has no broken/candidate link |
| `crates/openwepp-assurance/Cargo.toml` and every tracked file under `crates/openwepp-assurance/src/` | Retain the minimum v1 engineering code needed to validate/build the empty catalog until v2 replacement; identify retained versus deleted files individually | Real CLI produces no candidate/public dossier and fails closed on candidate input |
| Root `Cargo.toml` and `Cargo.lock` | Keep workspace/package bindings only while retained compiler/tests require them; update exact dependency lock if code changes | Workspace and empty-catalog gates pass with no dormant candidate fixture dependency |
| `tests/integration/assurance_dossier_build_contract.rs` | Rewrite current-public-candidate assertions as empty-catalog, candidate-rejection, release-negative, and exact-history recovery tests; retain isolated v1 engineering fixtures only when clearly historical | Test names and assertions no longer present the candidate as current public content |
| `tools/release/check_assurance_dossier_exports.sh` | Convert to zero-report validation and candidate-negative check | Exits nonzero if a candidate/public v1 route or export record exists |
| `tools/release/run_release_candidate_gates.sh` | First install the `ASSURE03-REL-001` transition guard, then adopt zero-report snapshot behavior | Validation mode creates/uploads no release candidate; release mode fails until zero-report migration passes |
| `tools/release/README.md` | Document separate validation and release modes plus the transition guard | No command implies current candidate snapshotting is safe |
| `.github/workflows/release-gates.yml` | Split ordinary PR/push validation from explicit release assembly and rename nonrelease artifacts | PR/push creates no assurance snapshot and uploads no `openwepp-release-candidate-*` artifact |
| `docs/governance/openwepp-release-procedure-draft.md` | Replace transition warning with the proven zero-report release procedure only after executable gates pass | Runbook and executable consumer agree |
| `docs/standards/scientific-assurance-dossier.md` | Finalize the proposed retirement notice after ASSURE-02 acceptance | V1 status and accepted ADR/v2 standard transition atomically |
| `docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/artifacts/wepppy-handoff.md` | Preserve unchanged as dormant historical compatibility evidence | Current docs label it non-authorizing; no export executes from it |
| Historical ADR and work-package evidence containing v1 statements | Preserve factual history; classify every search hit and annotate only where current routing is ambiguous | Search report separates historical references from active/public authority |

Git history plus the content manifest is the historical archive. Do not copy a
second active-looking v1 tree into public documentation.

### Current executable conflict: `ASSURE03-REL-001`

At ASSURE-02 review, `tools/release/run_release_candidate_gates.sh` still
unconditionally built a v1 candidate assurance snapshot, and
`.github/workflows/release-gates.yml` still invoked that script and uploaded a
release-candidate artifact during ordinary PR/push CI. A prose prohibition does
not close this conflict. ASSURE-03 must correct it before any other release gate
can be treated as release-safe:

1. create a validation-only mode that cannot snapshot or label/upload a release
   candidate;
2. make explicit release assembly fail while the v1 candidate or transition
   marker exists;
3. add negative integration tests for both routes; and
4. align the release README, runbook, script, and workflow.

The later zero-report migration then replaces the temporary transition failure
with a positive empty-catalog release path. Until both stages pass, openWEPP is
not eligible for release-candidate assembly.

## ASSURE-03 Write Boundary

ASSURE-03 may change active v1 source/catalog records, generated/public
assurance content, relevant `usersum` links, v1 export manifests, release checks,
and package evidence. It may make the existing compiler and gates support an
empty active catalog if needed. It must not implement v2 manuscript schemas,
rendering, dependency planning, or a new report.

## Target Zero-Report State

After migration:

- no v1 dossier or method page exists in the tracked public `usersum` tree;
- public assurance navigation lists zero reports and explains that reviewed
  reports are in development;
- the snow/frost model narrative remains available, with broken v1 links
  replaced by neutral language pointing to future evaluation work;
- no release, export, or WEPPcloud handoff can include the v1 candidate;
- an empty assurance build/check succeeds deterministically;
- the v1 source/generated manifest and recovery commit are package-local
  historical evidence; and
- search and reference checks find no current claim that the v1 candidate is an
  openWEPP scientific assessment.

The zero-report page must not say or imply that snow/frost lacks evidence. It
states only that no report has completed the v2 publication process.

## Preservation Contract

ASSURE-03 records:

- exact Git commit before deletion;
- file inventory, sizes, and SHA-256 digests;
- compiler/tool version and command used to reconstruct the generated bytes;
- review and lifecycle records;
- source-to-generated mapping;
- the reason for retirement and the superseding ADR/standards; and
- recovery commands using Git history.

Restricted or copyrighted scientific references remain governed by the
reference-vendoring policy; their protected content is not copied into the
migration artifact.

## Link And Catalog Repair

ASSURE-03 searches all tracked Markdown, manifests, release scripts, and source
records for v1 routes and terminology. Each occurrence is classified as:

- current public navigation — remove or replace;
- model-science narrative — preserve science, remove premature report link;
- current governance/standard — point to v2 authority;
- historical work-package/ADR evidence — retain unchanged or annotate without
  rewriting history; or
- code/test fixture — retain only when required to prove empty-catalog or
  migration behavior.

Historical work-package statements remain true records of what v1 implemented.
They are not rewritten to claim that v1 never existed.

## Gates

ASSURE-03 closes only when:

1. the v1 content manifest and recovery commit independently reconstruct every
   removed source and generated byte;
2. no active/public catalog, navigation, export, release snapshot, or vendor
   manifest includes the candidate;
3. the neutral zero-report page and snow/frost narrative pass domain-language
   review without diminishing existing science;
4. empty-catalog validate, build, check, release, link, and search gates pass;
5. public outputs are deterministic and no draft/candidate route is generated;
6. two independent reviews are dispositioned and two verifications pass; and
7. rollback is proven from the recorded Git commit without making v1 current.

## Rollback

Rollback means recovering exact historical files for audit or diagnosing a
migration defect. It does not authorize restoring v1 to public navigation.
Reinstating a public v1 candidate would conflict with ADR-0038 and requires a
new explicit scientific-governance decision.
