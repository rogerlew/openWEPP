# Worker Handoff

Status: `complete`; focused evidence, the complete terminal workspace/CRAP
sequence, and dual accepted-fix verification pass.

## What Landed

- The canonical lifecycle, ownership, rebuild, rereview, and snapshot contract
  is `docs/governance/scientific-assurance-dossier-lifecycle.md`.
- `openwepp-assurance` is an offline, deterministic Rust compiler with
  `validate`, `plan`, `build`, and `check` operations.
- `assurance/` owns typed schemas, templates, method/dossier sources, evidence
  identities, authoring provenance, and review history.
- One public SNOTEL vertical slice proves the why/how/what/application route.
- Release-candidate automation fails on generated drift and records an explicit
  immutable assurance snapshot.
- openWEPP emits a deterministic wepppy handoff fragment without modifying
  wepppy.

This foundation is not a model release, scientific approval, deployed usersum
integration, or fitness determination.

## Routine Commands

From the openWEPP root:

```bash
cargo run -p openwepp-assurance -- validate --all
cargo run -p openwepp-assurance -- plan --all
cargo run -p openwepp-assurance -- build --all
cargo run -p openwepp-assurance -- check --all
bash tools/release/check_assurance_dossier_exports.sh
```

Use `--dossier snow-snotel-swe-depth-density` instead of `--all` for a targeted
validate, plan, build, or check. Normal commands are offline and agent-free.
`build` rewrites declared generated outputs; `check` builds in a clean temporary
directory and fails on missing, extra, or stale committed outputs.

## Ownership And Public Paths

- Why, hand-authored: `usersum/snow-frost-modeling-and-validation.md`.
- How, generated: `usersum/assurance/methods/snow-snotel-evaluation-v1.md`.
- What, generated: `usersum/assurance/dossiers/snow-snotel-swe-depth-density.md`.
- Application assessment, generated:
  `usersum/assurance/application-context-worksheet.md`.
- Discovery catalog, generated: `usersum/assurance/README.md`.
- Downstream fragment, generated: `assurance/generated/wepppy-usersum.yaml`.

Generated pages carry do-not-edit banners. Change the corresponding source,
template, or compiler and rebuild. The frozen candidate scientific root is
`bb4b8b5f6188613e22ca9a7bec301bd7d6a94f8ef5e3e2ed83f98ad532d45e8c`;
the publication root is
`9d3432db6eee33201c03d50ac9666bc050d46d4a0519170d05f05132ed5c32e8`.

## Change And Supersession Procedure

1. Edit only the owned source layer; do not hand-edit generated pages.
2. Run `validate`, inspect `plan`, then run `build` and `check`.
3. Apply the canonical material-change matrix. Scientific source changes
   invalidate scientific and publication approvals; renderer/schema/tooling
   changes invalidate publication approval.
4. Keep prior review records in order. Each approval binds its complete history
   prefix, and a locked publication approval terminates the history. A material
   change normally creates a new dossier version; do not mutate an immutable
   release snapshot.
5. A release uses its release tag as a new snapshot ID and retains the manifest
   digest with release evidence.

## Evidence Limits That Remain

- The pilot lifecycle is `CANDIDATE`.
- Empirical characterization is `INSUFFICIENT_EVIDENCE`.
- Aggregate software verification is `BLOCKED`, despite three narrow historical
  obligations that individually pass.
- Raw acquisition replay, numerical solution verification, current-release
  lineage, independent release reproduction, and external hydrologist review
  remain incomplete or unrun.
- No runoff, erosion, frost, channel, watershed, regulatory, or application
  fitness claim is made.

## Deferred And Next Work

Vendoring is not the next operational package. Do not open the wepppy package
while WEPPcloud and the dossier set remain immature. Preserve
`wepppy-handoff.md` as a dormant technical contract and refresh it when the
openWEPP beta release campaign in WEPPcloud is authorized.

Vendoring is nevertheless a mandatory pre-beta release gate. Before that beta
release is issued, a separate wepppy package must select the approved dossier
set, refresh all source and downstream identities, vendor the documents, merge
the export into its manifest and navigation, validate link rewriting, search,
and role visibility, and prove the real rendered consumer.

The binding next follow-on is documentation-only `ASSURE-02` in
`docs/ROADMAP.md`. It must establish and review the manuscript-first v2
architecture, scientific-report standard, public/machine separation,
pilot-selection inventory, real nonpublic process-kernel manuscript prototype,
v1 retirement plan, and implementation decomposition. Do not modify the
compiler, schemas, templates, catalog, generated pages, scientific results, or
downstream repository before that gate is accepted. Do not expand the
foundation into a generic evidence platform until repeated v2 report work shows
which caching or portfolio-query features are actually needed.
