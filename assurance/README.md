# Scientific Assurance Transition Source

Status: v1 retired; zero public reports; internal v2 source admission active

The v1 status-first dossier publisher and SNOTEL candidate were removed by
ASSURE-03 after exact source, generated, review, and compiler provenance was
bound to Git commit `3352388465f8b288aed4636e8f9752ca6c1cceb9` and the package
content manifest. The retirement concerns publication architecture, not the
quality or extent of openWEPP snow/frost science.

The root catalog, static public template, and dormant export still describe
exactly zero public reports. The retained public builder is deliberately fail-
closed and rejects any nonempty legacy dossier catalog.

ASSURE-04A adds a separate internal source tree at `assurance/v2/`. It admits a
strictly content-identified, manuscript-first `DRAFT` fixture for architecture
testing. Admission is not approval or publication: v2 sources cannot render,
export, snapshot, vendor, or enter `usersum` through this package. See
`assurance/v2/README.md` for the source contract and package boundaries.

From the repository root:

```bash
cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- validate \
  --report linear-groundwater-reservoir-recurrence
cargo run --quiet -p openwepp-assurance -- plan --all
cargo run --quiet -p openwepp-assurance -- build --all
cargo run --quiet -p openwepp-assurance -- check --all
```

An explicit release operation may add `--snapshot <id> --snapshot-root <path>`
only after the release transition preflight passes. Ordinary CI never invokes
snapshot mode. Historical recovery is audit-only and does not authorize v1
publication, export, snapshotting, or vendoring.

Report-specific planning belongs to ASSURE-04B, assembly to ASSURE-04C, and
review locks and publication to ASSURE-04D. Until those packages close, only
v2 source validation is available.
