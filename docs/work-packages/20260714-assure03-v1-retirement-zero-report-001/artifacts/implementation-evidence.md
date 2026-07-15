# ASSURE-03 Implementation And Consumer Evidence

Status: implemented; terminal heavy closure passed

Evidence class: Static + Ran

## Consumer Path

```text
assurance/catalog.yaml + assurance/templates/catalog.md
        |
        v
openwepp_assurance::Assurance (typed admission; nonempty rejected)
        |
        +--> validate / plan
        +--> build / check
        |       |
        |       +--> usersum/assurance/README.md
        |       +--> assurance/generated/wepppy-usersum.yaml
        |
        +--> explicit release-only immutable snapshot (report_count=0)

.github/workflows/release-gates.yml
        |
        +--> ordinary events: --mode validate --> validation evidence only
        +--> explicit manual release: --mode release
                    |
                    +--> check_assurance_release_transition.sh
                    +--> zero-report snapshot + binary assembly
```

The real consumers—not only source structure—were exercised.

## Ran Evidence

- `openwepp-assurance validate --all`: PASS, `reports: 0`.
- `plan --all`: two content-identified inputs and exactly two outputs.
- `check --all`: PASS against tracked bytes.
- Two independent output-root builds: byte-identical.
- Snapshot `260714assure03`: `report_count: 0`, `reports: []`, two inputs,
  exactly two copied outputs; manifest SHA-256
  `4e282e801590144b5f0a38a14585eb93425e6e842d8ac2d8f2afb089627b0b44`.
- Snapshot confirmation succeeded; mutated content, unsafe snapshot IDs,
  snapshot-ID symlinks, and descendant symlinks failed.
- Transition preflight passed in validation mode with assembly unauthorized,
  rejected release mode with a marker/nonempty catalog/retired route, and
  passed release mode on the terminal zero-report source.
- A copied real aggregate release script rejected the marker before creating
  its requested release directory.
- `check_assurance_dossier_exports.sh`: PASS with one public file, zero
  documents, and vendoring disabled.
- Latest focused Nextest remediation run
  `3143e492-8993-4a68-a8da-119765236e6f`: 13/13 passed.
- Focused crate Clippy with `-D warnings`: PASS.
- Renewed terminal validation aggregate: PASS, Nextest run
  `35e07ed8-ee99-4b26-89ef-2d675b5adb1d`, 1,974/1,974 passed.
- Renewed terminal release-mode transition aggregate: PASS, Nextest run
  `e3208b83-1287-4723-be48-ef6b600bf5fd`, 1,974/1,974 passed.
- Both fresh CRAP acquisitions: closure eligible, threshold 30,
  raw/adjudicated/actionable `2/2/0`, 13 touched production files.
- The renewed r4 ordered 40-file source manifest was
  `a5355bf907b0e23efae776ba3c464404e21e6c2f669d4ff1a39a00008c6248b8`
  and remained byte-identical through both aggregates and cleanup.

## Negative Proof

- `Selection::Dossier` and the v1 public library surface no longer exist.
- CLI `--dossier` fails with an explicit retirement error.
- A nonempty `dossiers` sequence fails typed admission.
- Retired active/public files fail both compiler admission and release preflight.
- Validation mode conditionally skips snapshotting and exits before binary,
  sidecar, or release-lint assembly.
- The workflow's only candidate-named upload requires explicit assembly,
  successful workspace validation, successful separately bound stability,
  successful transition preflight, and successful assembly. Failure evidence
  uses a non-candidate name.
- No tracked WEPPcloud vendor file changed; the dormant export explicitly says
  `vendoring_authorized: false`.
