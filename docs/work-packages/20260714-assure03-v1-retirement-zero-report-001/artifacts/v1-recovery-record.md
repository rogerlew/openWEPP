# V1 Recovery And Retirement Record

Status: frozen before active-source removal

Evidence class: Static + Ran

## Preservation Identity

- Git commit: `3352388465f8b288aed4636e8f9752ca6c1cceb9`
- Manifest: `artifacts/v1-content-manifest.tsv`
- Manifest semantics: `path`, exact Git blob size, SHA-256 of exact blob bytes,
  and ASSURE-03 action.
- Retirement reason: the v1 status-first public architecture failed the reader-
  facing scientific communication objective; ADR-0038 supersedes it with a
  manuscript-first architecture. This is not an adverse scientific finding
  about the snow/frost model.

The manifest expands every tracked path under the active `assurance/`, public
`usersum/assurance/`, and compiler directories plus each individually named
source, consumer, release, workflow, and historical-handoff path in the
migration inventory. Historical search hits outside those active surfaces are
classified separately; they remain reachable through Git and are not copied
into a second active-looking archive.

## Source-To-Generated Mapping

At the frozen commit, the v1 compiler consumed `assurance/catalog.yaml`, six
dossier records, one method record, four templates, six schemas, the compiler
source, and the snow/frost narrative. It produced:

| Generated path | Frozen SHA-256 |
| --- | --- |
| `usersum/assurance/README.md` | `bfe57c9c65fcba174a543e0f5bc287124a292215aaf3f17f0148a58460d8b26e` |
| `usersum/assurance/application-context-worksheet.md` | `866774bf82baaaff90f63e8050cad8b9f3127f490b74fe24fca7fa6e7f269352` |
| `usersum/assurance/dossiers/snow-snotel-swe-depth-density.md` | `6d2dea9f676d996b7b1ddf8b6737cc61d80fbbf06ba473250fd8800842fdfbfd` |
| `usersum/assurance/methods/snow-snotel-evaluation-v1.md` | `15bd161a6b63515533fdb6aea651260fedb1556e81ca549530e56a4217dc5e82` |
| `assurance/generated/wepppy-usersum.yaml` | `828762f7ef5672a7e50b0e56184aac2d5a40530a3507b788cf4a58714612ee2a` |

The historical scientific review/lifecycle record is
`assurance/dossiers/snow-snotel-swe-depth-density/review.yaml`, SHA-256
`97aa711ffde48eb7c717179492899dd13076631066041f52ba9c0ac42eb4e126`.
The agent-assistance record is `authoring.yaml`, SHA-256
`c820e546eeec8b0744247dbcaf3d2d4d6054091822ee181482f69aa1d09335dc`.

## Reproduction Run

Ran from a detached worktree of the frozen commit, with the existing local
Cargo cache and shared target directory:

```text
CARGO_TARGET_DIR=/home/workdir/openWEPP/target \
  cargo run --quiet -p openwepp-assurance -- build --all
```

Observed `build: PASS`; all five generated hashes matched the manifest, and
`git diff --exit-code -- usersum/assurance assurance/generated/wepppy-usersum.yaml`
returned zero. The temporary detached worktree was removed after the check.

## Audit-Only Recovery

Recover one exact file without changing the worktree:

```text
git show 3352388465f8b288aed4636e8f9752ca6c1cceb9:<path> > <external-audit-path>
```

Verify it against the matching TSV row with `sha256sum`. Recover the full
historical tree only into a detached audit worktree:

```text
git worktree add --detach <external-audit-root> \
  3352388465f8b288aed4636e8f9752ca6c1cceb9
```

Recovery does not authorize public restoration, release snapshotting, export,
or vendoring of v1.
