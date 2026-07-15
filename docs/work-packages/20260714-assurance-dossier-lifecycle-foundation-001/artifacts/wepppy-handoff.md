# wepppy Usersum Handoff

Status: `deferred`; dormant handoff, mandatory pre-beta release gate, and not
currently authorized for vendoring or deployment.

Static: openWEPP owns the source content and emits
`assurance/generated/wepppy-usersum.yaml`. Current SHA-256:
`828762f7ef5672a7e50b0e56184aac2d5a40530a3507b788cf4a58714612ee2a`.
The fragment contains five stable document records and the current candidate
assurance lifecycle. It is source-relative and sorted by `doc_id`.

This proves an export shape, not content or integration readiness. WEPPcloud
and the dossier set are still under development. Do not execute the downstream
steps in this record now.

The downstream `status` field uses the current wepppy vocabulary: `draft` for
an openWEPP `DRAFT`, `active` for `CANDIDATE` or `PUBLISHED`, and `deprecated`
for `SUPERSEDED` or `WITHDRAWN`. The exact source lifecycle is preserved in the
additional `assurance_lifecycle` field. A future wepppy package may retain that
extension as metadata or explicitly adopt it; it must not reinterpret
`status: active` as scientific corroboration or application fitness.

## Activation Gate

Vendoring is deferred until the openWEPP beta release campaign in WEPPcloud.
Activate this handoff only when the release authority explicitly determines
that the WEPPcloud documentation surface and the selected dossier set are ready
for integration and authorizes a dedicated package in the wepppy repository.

That downstream package is a mandatory pre-release gate: it must complete and
prove the real rendered consumer before the openWEPP beta release is issued in
WEPPcloud. At activation, refresh every source digest, selected document,
review state, wepppy contract, parser assumption, navigation target, and test
command. The identities and read-only parser proof below are historical
foundation evidence and cannot be reused as beta-release acceptance.

## openWEPP-Owned Inputs

- `usersum/assurance/README.md`
- `usersum/assurance/application-context-worksheet.md`
- `usersum/assurance/methods/snow-snotel-evaluation-v1.md`
- `usersum/assurance/dossiers/snow-snotel-swe-depth-density.md`
- `usersum/snow-frost-modeling-and-validation.md`
- `assurance/generated/wepppy-usersum.yaml`

The first four files are generated and carry source banners. The narrative is
hand-authored. From openWEPP, rebuild and then verify committed drift with:

```bash
cargo run -p openwepp-assurance -- build --all
cargo run -p openwepp-assurance -- check --all
```

## Deferred Pre-Beta wepppy-Owned Work

A separate package in `/home/workdir/wepppy` must:

1. Add an `openwepp` entry to
   `wepppy/weppcloud/routes/usersum/vendors.yaml`. In this workspace its source
   checkout is `/home/workdir/openWEPP`; deployment configuration must use its
   actual checkout path and a reviewed source ref. Include the five Markdown
   paths above, exclude `.git/**`, and use target root
   `wepppy/weppcloud/routes/usersum/vendor/openwepp` with route prefix
   `/usersum/vendor/openwepp`.
2. Sync those exact source files. Do not vendor the internal `assurance/`
   source tree or work-package evidence.
3. Merge the export records into `docs_manifest.yaml`, mechanically prefixing
   each source-relative `rel_path` with the configured target root. Preserve
   `doc_id`, title, role, category, audience tags, compatible status,
   `assurance_lifecycle`, and navigation key.
4. Add one navigation section with one leaf per exported `nav_key`; wepppy owns
   final placement and wording. Candidate status must remain visible rather
   than being translated into a favorable maturity state.
5. Validate link rewriting between the narrative, catalog, method, dossier,
   and worksheet; verify user-role visibility, canonical document routes,
   breadcrumbs, full-text search, and vendor aliases.
6. Add a source/export congruence test so an openWEPP export change cannot
   silently drift from the merged manifest.

Suggested wepppy commands, from that repository's own instructions:

```bash
PYTHONPATH=/home/workdir/wepppy python3 tools/usersum_docs_tool.py sync-vendors --write
PYTHONPATH=/home/workdir/wepppy python3 tools/usersum_docs_tool.py validate --require-vendor-files
PYTHONPATH=/home/workdir/wepppy python3 tools/usersum_docs_tool.py build-index --write --require-vendor-files
```

The wepppy package must run its own focused tests and documentation checks; the
commands above are not a substitute for its local `AGENTS.md` gates.

## Read-Only Consumer Proof

Ran: with bytecode writes disabled, loaded the current export, mechanically
renamed only its container keys from `schema_version` / `documents` to the
wepppy manifest's `version` / `docs`, and passed all five records to the real
wepppy
`wepppy.weppcloud.usersum_docs.docs_contracts._parse_docs_manifest` parser at
HEAD `b2b6d62c3472c324263c55597c7ee5ccc9545942`. It returned five vendor
documents, all with the accepted `active` status and `vendor_id: openwepp`.
The extra `assurance_lifecycle` field was safely ignored by that current
parser. This is compatibility evidence for the handoff record shape, not proof
of vendoring, navigation, rendering, indexing, deployment, or scientific
meaning.

## No-Mutation Evidence

Ran at openWEPP intake against wepppy HEAD
`b2b6d62c3472c324263c55597c7ee5ccc9545942`:

- tracked-status SHA-256:
  `d948ce3d022e21f9cbe30174014ca1b9f645630f8aac148b38161cff5cca8a50`;
- binary-diff SHA-256:
  `ee6cdc17fde5bb7c709fab6a4ad166ddd60daddb3be8709bacf5c9405e9dc70d`.

The intake modified-path list is recorded in `owned-file-manifest.md`. At the
openWEPP re-freeze, concurrent wepppy work had advanced HEAD to
`5da847b406c83708846bc63da8bf927e688c291d` and changed both status hashes; the
terminal observation is recorded there rather than misrepresented as a
package-caused diff. This package issued no wepppy write command. This handoff
is not evidence that wepppy has consumed, rendered, indexed, deployed, or
released the documents.
