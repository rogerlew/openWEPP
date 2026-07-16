# Scientific Assurance Transition Source

Status: v1 retired; zero public reports; internal v2 staging assembly active

The v1 status-first dossier publisher and SNOTEL candidate were removed by
ASSURE-03 after exact source, generated, review, and compiler provenance was
bound to Git commit `3352388465f8b288aed4636e8f9752ca6c1cceb9` and the package
content manifest. The retirement concerns publication architecture, not the
quality or extent of openWEPP snow/frost science.

The root catalog, static public template, and dormant export still describe
exactly zero public reports. The retained public builder is deliberately fail-
closed and rejects any nonempty legacy dossier catalog.

ASSURE-04A added a separate internal source tree at `assurance/v2/`; ASSURE-04B
added typed dependency planning; ASSURE-04C adds deterministic assembly only
into an explicit disposable staging root. These capabilities are not approval
or publication: v2 sources cannot enter tracked `usersum`, export, snapshot,
release, or vendor surfaces. See `assurance/v2/README.md` for the source,
planning, and staging contracts.

From the repository root:

```bash
cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- validate \
  --report linear-groundwater-reservoir-recurrence
cargo run --quiet -p openwepp-assurance -- plan --all
cargo run --quiet -p openwepp-assurance -- build --all
cargo run --quiet -p openwepp-assurance -- check --all
```

The final two commands above deliberately exercise the protected zero-public
builder and checker. Internal v2 assembly is visibly separate and always names
a disposable root:

```bash
cargo run --quiet -p openwepp-assurance -- build --all \
  --staging-root /tmp/openwepp-assurance-stage
```

An explicit release operation may add `--snapshot <id> --snapshot-root <path>`
only after the release transition preflight passes. Ordinary CI never invokes
snapshot mode. Historical recovery is audit-only and does not authorize v1
publication, export, snapshotting, or vendoring.

Report-specific validation, planning, and disposable assembly are available.
Review locks and publication belong to ASSURE-04D and remain unavailable.
